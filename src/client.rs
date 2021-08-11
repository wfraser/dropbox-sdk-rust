//! Clients for the various types of authentication supported by Dropbox API routes.

use crate::auth::AuthError;
use crate::client_trait::{Endpoint, HttpClient, HttpRequestResultRaw, ParamsType, Style, TeamSelect,
    TokenType};
use crate::oauth2::{Authorization, TokenCache};
use std::sync::Arc;

macro_rules! impl_set_path_root {
    ($self:ident) => {
        /// Set a root which all subsequent paths are evaluated relative to.
        ///
        /// The default, if this function is not called, is to behave as if it was called with
        /// [`PathRoot::Home`](crate::common::PathRoot::Home).
        ///
        /// See <https://www.dropbox.com/developers/reference/path-root-header-modes> for more
        /// information.
        #[cfg(feature = "dbx_common")]
        pub fn set_path_root(&mut $self, path_root: &crate::common::PathRoot) {
            // Only way this can fail is if PathRoot::Other was specified, which is a programmer
            // error, so panic if that happens.
            $self.path_root = Some(serde_json::to_string(path_root).expect("invalid path root"));
        }
    }
}

/// Common interface to each of the types of client.
///
/// This is a subset of the [`HttpClient`](crate::client_trait::HttpClient) trait, with the client-
/// specific fields of the `request` method abstracted away.
pub(crate) trait Client {
    #[allow(clippy::too_many_arguments)]
    fn request(
        &self,
        endpoint: Endpoint,
        style: Style,
        function: &str,
        params: &str,
        params_type: ParamsType,
        body: Option<&[u8]>,
        range_start: Option<u64>,
        range_end: Option<u64>,
    ) -> crate::Result<HttpRequestResultRaw>;
}

/// Client for unauthenticated API calls.
pub struct NoauthClient<H: HttpClient> {
    http: H,
    path_root: Option<String>,
}

impl<H: HttpClient> Client for NoauthClient<H> {
    fn request(
        &self,
        endpoint: Endpoint,
        style: Style,
        function: &str,
        params: &str,
        params_type: ParamsType,
        body: Option<&[u8]>,
        range_start: Option<u64>,
        range_end: Option<u64>,
    ) -> crate::Result<HttpRequestResultRaw> {
        self.http.request(endpoint, style, function, params, params_type, body, range_start,
            range_end, None, self.path_root.as_deref(), None)
    }
}

impl<H: HttpClient> NoauthClient<H> {
    impl_set_path_root!(self);

    /// Make a new unauthenticated client, using the given HTTP client implementation.
    pub fn new(http: H) -> Self {
        Self {
            http,
            path_root: None,
        }
    }
}

impl<H: HttpClient + Default> Default for NoauthClient<H> {
    fn default() -> Self {
        Self {
            http: H::default(),
            path_root: None,
        }
    }
}

/// Takes care of making requests with up-to-date auth tokens, by refreshing and retrying when the
/// token expires.
struct AuthedClient<H: HttpClient> {
    http: H,
    tokens: Arc<TokenCache>,
}

impl<H: HttpClient> AuthedClient<H> {
    #[allow(clippy::too_many_arguments)]
    fn request(
        &self,
        endpoint: Endpoint,
        style: Style,
        function: &str,
        params: &str,
        params_type: ParamsType,
        body: Option<&[u8]>,
        range_start: Option<u64>,
        range_end: Option<u64>,
        path_root: Option<&str>,
        team_select: Option<&TeamSelect>,
    ) -> crate::Result<HttpRequestResultRaw> {
        let mut token = self.tokens.get_token(&NoauthClient::new(&self.http))?;

        let mut retried = false;
        loop {
            let result = self.http.request(endpoint, style, function, params, params_type,
                body, range_start, range_end, Some((TokenType::Bearer, &token)), path_root,
                team_select);

            if retried {
                break result;
            }

            if let Err(crate::Error::Authentication(AuthError::ExpiredAccessToken)) = &result {
                info!("refreshing auth token");
                let old_token = token;
                token = self.tokens.update_token(&NoauthClient::new(&self.http), old_token)?;
                retried = true;
                continue;
            }

            break result;
        }
    }
}

/// Client for user-authenticated API calls.
pub struct UserAuthClient<H: HttpClient> {
    authed: AuthedClient<H>,
    path_root: Option<String>, // a serialized PathRoot enum
}

impl<H: HttpClient> Client for UserAuthClient<H> {
    fn request(
        &self,
        endpoint: Endpoint,
        style: Style,
        function: &str,
        params: &str,
        params_type: ParamsType,
        body: Option<&[u8]>,
        range_start: Option<u64>,
        range_end: Option<u64>,
    ) -> crate::Result<HttpRequestResultRaw> {
        self.authed.request(endpoint, style, function, params, params_type, body, range_start,
            range_end, self.path_root.as_deref(), None)
    }
}

impl<H: HttpClient> UserAuthClient<H> {
    impl_set_path_root!(self);

    /// Construct a new client from the given client implementation and OAuth2 authorization.
    pub fn from_auth(http: H, auth: Authorization) -> Self {
        Self {
            authed: AuthedClient {
                http,
                tokens: Arc::new(TokenCache::new(auth)),
            },
            path_root: None,
        }
    }

    /// Construct a new client from the given client implementation, and an OAuth2 authorization
    /// token cache which may be shared between multiple clients (i.e. this one and also a
    /// [`TeamAuthClient`]).
    pub fn from_shared_tokens(http: H, tokens: Arc<TokenCache>) -> Self {
        Self {
            authed: AuthedClient {
                http,
                tokens,
            },
            path_root: None,
        }
    }
}

/// Client for team-authenticated API calls.
pub struct TeamAuthClient<H: HttpClient> {
    authed: AuthedClient<H>,
    path_root: Option<String>, // a serialized PathRoot enum
    team_select: Option<TeamSelect>,
}

impl<H: HttpClient> Client for TeamAuthClient<H> {
    fn request(
        &self,
        endpoint: Endpoint,
        style: Style,
        function: &str,
        params: &str,
        params_type: ParamsType,
        body: Option<&[u8]>,
        range_start: Option<u64>,
        range_end: Option<u64>,
    ) -> crate::Result<HttpRequestResultRaw> {
        self.authed.request(endpoint, style, function, params, params_type, body, range_start,
            range_end, self.path_root.as_deref(), self.team_select.as_ref())
    }
}

impl<H: HttpClient> TeamAuthClient<H> {
    impl_set_path_root!(self);

    /// Select a user or team context to operate in.
    pub fn select(&mut self, team_select: Option<TeamSelect>) {
        self.team_select = team_select;
    }

    /// Construct a new client from the given client implementation and OAuth2 authorization.
    pub fn from_auth(http: H, auth: Authorization) -> Self {
        Self {
            authed: AuthedClient {
                http,
                tokens: Arc::new(TokenCache::new(auth)),
            },
            path_root: None,
            team_select: None,
        }
    }

    /// Construct a new client from the given client implementation, and an OAuth2 authorization
    /// token cache which may be shared between multiple clients (i.e. this one and also a
    /// [`UserAuthClient`]).
    pub fn from_shared_tokens(http: H, tokens: Arc<TokenCache>) -> Self {
        Self {
            authed: AuthedClient {
                http,
                tokens,
            },
            path_root: None,
            team_select: None,
        }
    }
}

/// Client for app-authenticated API calls.
pub struct AppAuthClient<H: HttpClient> {
    http: H,
    path_root: Option<String>,
    base64_key_secret_pair: String,
}

impl<H: HttpClient> Client for AppAuthClient<H> {
    fn request(
        &self,
        endpoint: Endpoint,
        style: Style,
        function: &str,
        params: &str,
        params_type: ParamsType,
        body: Option<&[u8]>,
        range_start: Option<u64>,
        range_end: Option<u64>,
    ) -> crate::Result<HttpRequestResultRaw> {
        self.http.request(endpoint, style, function, params, params_type, body, range_start,
            range_end, Some((TokenType::Basic, &self.base64_key_secret_pair)),
            self.path_root.as_deref(), None)
    }
}

impl<H: HttpClient> AppAuthClient<H> {
    /// Construct a new client from the given client implementation, app key, and app secret.
    pub fn new(http: H, app_key: &str, app_secret: &str) -> Self {
        let base64_key_secret_pair = base64::encode(format!("{}:{}", app_key, app_secret));
        Self {
            http,
            base64_key_secret_pair,
            path_root: None,
        }
    }

    impl_set_path_root!(self);
}
