// DO NOT EDIT
// This file was @generated by Stone

#![allow(
    clippy::too_many_arguments,
    clippy::large_enum_variant,
    clippy::result_large_err,
    clippy::doc_markdown,
)]

/// Error occurred because the account doesn't have permission to access the resource.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive] // variants may be added in the future
pub enum AccessError {
    /// Current account type cannot access the resource.
    InvalidAccountType(InvalidAccountTypeError),
    /// Current account cannot access Paper.
    PaperAccessDenied(PaperAccessError),
    /// Catch-all used for unrecognized values returned from the server. Encountering this value
    /// typically indicates that this SDK version is out of date.
    Other,
}

impl<'de> ::serde::de::Deserialize<'de> for AccessError {
    fn deserialize<D: ::serde::de::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        // union deserializer
        use serde::de::{self, MapAccess, Visitor};
        struct EnumVisitor;
        impl<'de> Visitor<'de> for EnumVisitor {
            type Value = AccessError;
            fn expecting(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                f.write_str("a AccessError structure")
            }
            fn visit_map<V: MapAccess<'de>>(self, mut map: V) -> Result<Self::Value, V::Error> {
                let tag: &str = match map.next_key()? {
                    Some(".tag") => map.next_value()?,
                    _ => return Err(de::Error::missing_field(".tag"))
                };
                let value = match tag {
                    "invalid_account_type" => {
                        match map.next_key()? {
                            Some("invalid_account_type") => AccessError::InvalidAccountType(map.next_value()?),
                            None => return Err(de::Error::missing_field("invalid_account_type")),
                            _ => return Err(de::Error::unknown_field(tag, VARIANTS))
                        }
                    }
                    "paper_access_denied" => {
                        match map.next_key()? {
                            Some("paper_access_denied") => AccessError::PaperAccessDenied(map.next_value()?),
                            None => return Err(de::Error::missing_field("paper_access_denied")),
                            _ => return Err(de::Error::unknown_field(tag, VARIANTS))
                        }
                    }
                    _ => AccessError::Other,
                };
                crate::eat_json_fields(&mut map)?;
                Ok(value)
            }
        }
        const VARIANTS: &[&str] = &["invalid_account_type",
                                    "paper_access_denied",
                                    "other"];
        deserializer.deserialize_struct("AccessError", VARIANTS, EnumVisitor)
    }
}

impl ::serde::ser::Serialize for AccessError {
    fn serialize<S: ::serde::ser::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        // union serializer
        use serde::ser::SerializeStruct;
        match self {
            AccessError::InvalidAccountType(x) => {
                // union or polymporphic struct
                let mut s = serializer.serialize_struct("AccessError", 2)?;
                s.serialize_field(".tag", "invalid_account_type")?;
                s.serialize_field("invalid_account_type", x)?;
                s.end()
            }
            AccessError::PaperAccessDenied(x) => {
                // union or polymporphic struct
                let mut s = serializer.serialize_struct("AccessError", 2)?;
                s.serialize_field(".tag", "paper_access_denied")?;
                s.serialize_field("paper_access_denied", x)?;
                s.end()
            }
            AccessError::Other => Err(::serde::ser::Error::custom("cannot serialize 'Other' variant"))
        }
    }
}

impl ::std::error::Error for AccessError {
    fn source(&self) -> Option<&(dyn ::std::error::Error + 'static)> {
        match self {
            AccessError::InvalidAccountType(inner) => Some(inner),
            AccessError::PaperAccessDenied(inner) => Some(inner),
            _ => None,
        }
    }
}

impl ::std::fmt::Display for AccessError {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match self {
            AccessError::InvalidAccountType(inner) => write!(f, "Current account type cannot access the resource: {}", inner),
            AccessError::PaperAccessDenied(inner) => write!(f, "Current account cannot access Paper: {}", inner),
            _ => write!(f, "{:?}", *self),
        }
    }
}

/// Errors occurred during authentication.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive] // variants may be added in the future
pub enum AuthError {
    /// The access token is invalid.
    InvalidAccessToken,
    /// The user specified in 'Dropbox-API-Select-User' is no longer on the team.
    InvalidSelectUser,
    /// The user specified in 'Dropbox-API-Select-Admin' is not a Dropbox Business team admin.
    InvalidSelectAdmin,
    /// The user has been suspended.
    UserSuspended,
    /// The access token has expired.
    ExpiredAccessToken,
    /// The access token does not have the required scope to access the route.
    MissingScope(TokenScopeError),
    /// The route is not available to public.
    RouteAccessDenied,
    /// Catch-all used for unrecognized values returned from the server. Encountering this value
    /// typically indicates that this SDK version is out of date.
    Other,
}

impl<'de> ::serde::de::Deserialize<'de> for AuthError {
    fn deserialize<D: ::serde::de::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        // union deserializer
        use serde::de::{self, MapAccess, Visitor};
        struct EnumVisitor;
        impl<'de> Visitor<'de> for EnumVisitor {
            type Value = AuthError;
            fn expecting(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                f.write_str("a AuthError structure")
            }
            fn visit_map<V: MapAccess<'de>>(self, mut map: V) -> Result<Self::Value, V::Error> {
                let tag: &str = match map.next_key()? {
                    Some(".tag") => map.next_value()?,
                    _ => return Err(de::Error::missing_field(".tag"))
                };
                let value = match tag {
                    "invalid_access_token" => AuthError::InvalidAccessToken,
                    "invalid_select_user" => AuthError::InvalidSelectUser,
                    "invalid_select_admin" => AuthError::InvalidSelectAdmin,
                    "user_suspended" => AuthError::UserSuspended,
                    "expired_access_token" => AuthError::ExpiredAccessToken,
                    "missing_scope" => AuthError::MissingScope(TokenScopeError::internal_deserialize(&mut map)?),
                    "route_access_denied" => AuthError::RouteAccessDenied,
                    _ => AuthError::Other,
                };
                crate::eat_json_fields(&mut map)?;
                Ok(value)
            }
        }
        const VARIANTS: &[&str] = &["invalid_access_token",
                                    "invalid_select_user",
                                    "invalid_select_admin",
                                    "user_suspended",
                                    "expired_access_token",
                                    "missing_scope",
                                    "route_access_denied",
                                    "other"];
        deserializer.deserialize_struct("AuthError", VARIANTS, EnumVisitor)
    }
}

impl ::serde::ser::Serialize for AuthError {
    fn serialize<S: ::serde::ser::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        // union serializer
        use serde::ser::SerializeStruct;
        match self {
            AuthError::InvalidAccessToken => {
                // unit
                let mut s = serializer.serialize_struct("AuthError", 1)?;
                s.serialize_field(".tag", "invalid_access_token")?;
                s.end()
            }
            AuthError::InvalidSelectUser => {
                // unit
                let mut s = serializer.serialize_struct("AuthError", 1)?;
                s.serialize_field(".tag", "invalid_select_user")?;
                s.end()
            }
            AuthError::InvalidSelectAdmin => {
                // unit
                let mut s = serializer.serialize_struct("AuthError", 1)?;
                s.serialize_field(".tag", "invalid_select_admin")?;
                s.end()
            }
            AuthError::UserSuspended => {
                // unit
                let mut s = serializer.serialize_struct("AuthError", 1)?;
                s.serialize_field(".tag", "user_suspended")?;
                s.end()
            }
            AuthError::ExpiredAccessToken => {
                // unit
                let mut s = serializer.serialize_struct("AuthError", 1)?;
                s.serialize_field(".tag", "expired_access_token")?;
                s.end()
            }
            AuthError::MissingScope(x) => {
                // struct
                let mut s = serializer.serialize_struct("AuthError", 2)?;
                s.serialize_field(".tag", "missing_scope")?;
                x.internal_serialize::<S>(&mut s)?;
                s.end()
            }
            AuthError::RouteAccessDenied => {
                // unit
                let mut s = serializer.serialize_struct("AuthError", 1)?;
                s.serialize_field(".tag", "route_access_denied")?;
                s.end()
            }
            AuthError::Other => Err(::serde::ser::Error::custom("cannot serialize 'Other' variant"))
        }
    }
}

impl ::std::error::Error for AuthError {
}

impl ::std::fmt::Display for AuthError {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match self {
            AuthError::InvalidAccessToken => f.write_str("The access token is invalid."),
            AuthError::InvalidSelectUser => f.write_str("The user specified in 'Dropbox-API-Select-User' is no longer on the team."),
            AuthError::InvalidSelectAdmin => f.write_str("The user specified in 'Dropbox-API-Select-Admin' is not a Dropbox Business team admin."),
            AuthError::UserSuspended => f.write_str("The user has been suspended."),
            AuthError::ExpiredAccessToken => f.write_str("The access token has expired."),
            AuthError::MissingScope(inner) => write!(f, "The access token does not have the required scope to access the route: {:?}", inner),
            AuthError::RouteAccessDenied => f.write_str("The route is not available to public."),
            _ => write!(f, "{:?}", *self),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive] // variants may be added in the future
pub enum InvalidAccountTypeError {
    /// Current account type doesn't have permission to access this route endpoint.
    Endpoint,
    /// Current account type doesn't have permission to access this feature.
    Feature,
    /// Catch-all used for unrecognized values returned from the server. Encountering this value
    /// typically indicates that this SDK version is out of date.
    Other,
}

impl<'de> ::serde::de::Deserialize<'de> for InvalidAccountTypeError {
    fn deserialize<D: ::serde::de::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        // union deserializer
        use serde::de::{self, MapAccess, Visitor};
        struct EnumVisitor;
        impl<'de> Visitor<'de> for EnumVisitor {
            type Value = InvalidAccountTypeError;
            fn expecting(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                f.write_str("a InvalidAccountTypeError structure")
            }
            fn visit_map<V: MapAccess<'de>>(self, mut map: V) -> Result<Self::Value, V::Error> {
                let tag: &str = match map.next_key()? {
                    Some(".tag") => map.next_value()?,
                    _ => return Err(de::Error::missing_field(".tag"))
                };
                let value = match tag {
                    "endpoint" => InvalidAccountTypeError::Endpoint,
                    "feature" => InvalidAccountTypeError::Feature,
                    _ => InvalidAccountTypeError::Other,
                };
                crate::eat_json_fields(&mut map)?;
                Ok(value)
            }
        }
        const VARIANTS: &[&str] = &["endpoint",
                                    "feature",
                                    "other"];
        deserializer.deserialize_struct("InvalidAccountTypeError", VARIANTS, EnumVisitor)
    }
}

impl ::serde::ser::Serialize for InvalidAccountTypeError {
    fn serialize<S: ::serde::ser::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        // union serializer
        use serde::ser::SerializeStruct;
        match self {
            InvalidAccountTypeError::Endpoint => {
                // unit
                let mut s = serializer.serialize_struct("InvalidAccountTypeError", 1)?;
                s.serialize_field(".tag", "endpoint")?;
                s.end()
            }
            InvalidAccountTypeError::Feature => {
                // unit
                let mut s = serializer.serialize_struct("InvalidAccountTypeError", 1)?;
                s.serialize_field(".tag", "feature")?;
                s.end()
            }
            InvalidAccountTypeError::Other => Err(::serde::ser::Error::custom("cannot serialize 'Other' variant"))
        }
    }
}

impl ::std::error::Error for InvalidAccountTypeError {
}

impl ::std::fmt::Display for InvalidAccountTypeError {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match self {
            InvalidAccountTypeError::Endpoint => f.write_str("Current account type doesn't have permission to access this route endpoint."),
            InvalidAccountTypeError::Feature => f.write_str("Current account type doesn't have permission to access this feature."),
            _ => write!(f, "{:?}", *self),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive] // variants may be added in the future
pub enum PaperAccessError {
    /// Paper is disabled.
    PaperDisabled,
    /// The provided user has not used Paper yet.
    NotPaperUser,
    /// Catch-all used for unrecognized values returned from the server. Encountering this value
    /// typically indicates that this SDK version is out of date.
    Other,
}

impl<'de> ::serde::de::Deserialize<'de> for PaperAccessError {
    fn deserialize<D: ::serde::de::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        // union deserializer
        use serde::de::{self, MapAccess, Visitor};
        struct EnumVisitor;
        impl<'de> Visitor<'de> for EnumVisitor {
            type Value = PaperAccessError;
            fn expecting(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                f.write_str("a PaperAccessError structure")
            }
            fn visit_map<V: MapAccess<'de>>(self, mut map: V) -> Result<Self::Value, V::Error> {
                let tag: &str = match map.next_key()? {
                    Some(".tag") => map.next_value()?,
                    _ => return Err(de::Error::missing_field(".tag"))
                };
                let value = match tag {
                    "paper_disabled" => PaperAccessError::PaperDisabled,
                    "not_paper_user" => PaperAccessError::NotPaperUser,
                    _ => PaperAccessError::Other,
                };
                crate::eat_json_fields(&mut map)?;
                Ok(value)
            }
        }
        const VARIANTS: &[&str] = &["paper_disabled",
                                    "not_paper_user",
                                    "other"];
        deserializer.deserialize_struct("PaperAccessError", VARIANTS, EnumVisitor)
    }
}

impl ::serde::ser::Serialize for PaperAccessError {
    fn serialize<S: ::serde::ser::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        // union serializer
        use serde::ser::SerializeStruct;
        match self {
            PaperAccessError::PaperDisabled => {
                // unit
                let mut s = serializer.serialize_struct("PaperAccessError", 1)?;
                s.serialize_field(".tag", "paper_disabled")?;
                s.end()
            }
            PaperAccessError::NotPaperUser => {
                // unit
                let mut s = serializer.serialize_struct("PaperAccessError", 1)?;
                s.serialize_field(".tag", "not_paper_user")?;
                s.end()
            }
            PaperAccessError::Other => Err(::serde::ser::Error::custom("cannot serialize 'Other' variant"))
        }
    }
}

impl ::std::error::Error for PaperAccessError {
}

impl ::std::fmt::Display for PaperAccessError {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match self {
            PaperAccessError::PaperDisabled => f.write_str("Paper is disabled."),
            PaperAccessError::NotPaperUser => f.write_str("The provided user has not used Paper yet."),
            _ => write!(f, "{:?}", *self),
        }
    }
}

/// Error occurred because the app is being rate limited.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive] // structs may have more fields added in the future.
pub struct RateLimitError {
    /// The reason why the app is being rate limited.
    pub reason: RateLimitReason,
    /// The number of seconds that the app should wait before making another request.
    pub retry_after: u64,
}

impl RateLimitError {
    pub fn new(reason: RateLimitReason) -> Self {
        RateLimitError {
            reason,
            retry_after: 1,
        }
    }

    pub fn with_retry_after(mut self, value: u64) -> Self {
        self.retry_after = value;
        self
    }
}

const RATE_LIMIT_ERROR_FIELDS: &[&str] = &["reason",
                                           "retry_after"];
impl RateLimitError {
    pub(crate) fn internal_deserialize<'de, V: ::serde::de::MapAccess<'de>>(
        map: V,
    ) -> Result<RateLimitError, V::Error> {
        Self::internal_deserialize_opt(map, false).map(Option::unwrap)
    }

    pub(crate) fn internal_deserialize_opt<'de, V: ::serde::de::MapAccess<'de>>(
        mut map: V,
        optional: bool,
    ) -> Result<Option<RateLimitError>, V::Error> {
        let mut field_reason = None;
        let mut field_retry_after = None;
        let mut nothing = true;
        while let Some(key) = map.next_key::<&str>()? {
            nothing = false;
            match key {
                "reason" => {
                    if field_reason.is_some() {
                        return Err(::serde::de::Error::duplicate_field("reason"));
                    }
                    field_reason = Some(map.next_value()?);
                }
                "retry_after" => {
                    if field_retry_after.is_some() {
                        return Err(::serde::de::Error::duplicate_field("retry_after"));
                    }
                    field_retry_after = Some(map.next_value()?);
                }
                _ => {
                    // unknown field allowed and ignored
                    map.next_value::<::serde_json::Value>()?;
                }
            }
        }
        if optional && nothing {
            return Ok(None);
        }
        let result = RateLimitError {
            reason: field_reason.ok_or_else(|| ::serde::de::Error::missing_field("reason"))?,
            retry_after: field_retry_after.unwrap_or(1),
        };
        Ok(Some(result))
    }

    pub(crate) fn internal_serialize<S: ::serde::ser::Serializer>(
        &self,
        s: &mut S::SerializeStruct,
    ) -> Result<(), S::Error> {
        use serde::ser::SerializeStruct;
        s.serialize_field("reason", &self.reason)?;
        if self.retry_after != 1 {
            s.serialize_field("retry_after", &self.retry_after)?;
        }
        Ok(())
    }
}

impl<'de> ::serde::de::Deserialize<'de> for RateLimitError {
    fn deserialize<D: ::serde::de::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        // struct deserializer
        use serde::de::{MapAccess, Visitor};
        struct StructVisitor;
        impl<'de> Visitor<'de> for StructVisitor {
            type Value = RateLimitError;
            fn expecting(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                f.write_str("a RateLimitError struct")
            }
            fn visit_map<V: MapAccess<'de>>(self, map: V) -> Result<Self::Value, V::Error> {
                RateLimitError::internal_deserialize(map)
            }
        }
        deserializer.deserialize_struct("RateLimitError", RATE_LIMIT_ERROR_FIELDS, StructVisitor)
    }
}

impl ::serde::ser::Serialize for RateLimitError {
    fn serialize<S: ::serde::ser::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        // struct serializer
        use serde::ser::SerializeStruct;
        let mut s = serializer.serialize_struct("RateLimitError", 2)?;
        self.internal_serialize::<S>(&mut s)?;
        s.end()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive] // variants may be added in the future
pub enum RateLimitReason {
    /// You are making too many requests in the past few minutes.
    TooManyRequests,
    /// There are currently too many write operations happening in the user's Dropbox.
    TooManyWriteOperations,
    /// Catch-all used for unrecognized values returned from the server. Encountering this value
    /// typically indicates that this SDK version is out of date.
    Other,
}

impl<'de> ::serde::de::Deserialize<'de> for RateLimitReason {
    fn deserialize<D: ::serde::de::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        // union deserializer
        use serde::de::{self, MapAccess, Visitor};
        struct EnumVisitor;
        impl<'de> Visitor<'de> for EnumVisitor {
            type Value = RateLimitReason;
            fn expecting(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                f.write_str("a RateLimitReason structure")
            }
            fn visit_map<V: MapAccess<'de>>(self, mut map: V) -> Result<Self::Value, V::Error> {
                let tag: &str = match map.next_key()? {
                    Some(".tag") => map.next_value()?,
                    _ => return Err(de::Error::missing_field(".tag"))
                };
                let value = match tag {
                    "too_many_requests" => RateLimitReason::TooManyRequests,
                    "too_many_write_operations" => RateLimitReason::TooManyWriteOperations,
                    _ => RateLimitReason::Other,
                };
                crate::eat_json_fields(&mut map)?;
                Ok(value)
            }
        }
        const VARIANTS: &[&str] = &["too_many_requests",
                                    "too_many_write_operations",
                                    "other"];
        deserializer.deserialize_struct("RateLimitReason", VARIANTS, EnumVisitor)
    }
}

impl ::serde::ser::Serialize for RateLimitReason {
    fn serialize<S: ::serde::ser::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        // union serializer
        use serde::ser::SerializeStruct;
        match self {
            RateLimitReason::TooManyRequests => {
                // unit
                let mut s = serializer.serialize_struct("RateLimitReason", 1)?;
                s.serialize_field(".tag", "too_many_requests")?;
                s.end()
            }
            RateLimitReason::TooManyWriteOperations => {
                // unit
                let mut s = serializer.serialize_struct("RateLimitReason", 1)?;
                s.serialize_field(".tag", "too_many_write_operations")?;
                s.end()
            }
            RateLimitReason::Other => Err(::serde::ser::Error::custom("cannot serialize 'Other' variant"))
        }
    }
}

impl ::std::fmt::Display for RateLimitReason {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match self {
            RateLimitReason::TooManyRequests => f.write_str("You are making too many requests in the past few minutes."),
            RateLimitReason::TooManyWriteOperations => f.write_str("There are currently too many write operations happening in the user's Dropbox."),
            _ => write!(f, "{:?}", *self),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive] // structs may have more fields added in the future.
pub struct TokenFromOAuth1Arg {
    /// The supplied OAuth 1.0 access token.
    pub oauth1_token: String,
    /// The token secret associated with the supplied access token.
    pub oauth1_token_secret: String,
}

impl TokenFromOAuth1Arg {
    pub fn new(oauth1_token: String, oauth1_token_secret: String) -> Self {
        TokenFromOAuth1Arg {
            oauth1_token,
            oauth1_token_secret,
        }
    }
}

const TOKEN_FROM_O_AUTH1_ARG_FIELDS: &[&str] = &["oauth1_token",
                                                 "oauth1_token_secret"];
impl TokenFromOAuth1Arg {
    pub(crate) fn internal_deserialize<'de, V: ::serde::de::MapAccess<'de>>(
        map: V,
    ) -> Result<TokenFromOAuth1Arg, V::Error> {
        Self::internal_deserialize_opt(map, false).map(Option::unwrap)
    }

    pub(crate) fn internal_deserialize_opt<'de, V: ::serde::de::MapAccess<'de>>(
        mut map: V,
        optional: bool,
    ) -> Result<Option<TokenFromOAuth1Arg>, V::Error> {
        let mut field_oauth1_token = None;
        let mut field_oauth1_token_secret = None;
        let mut nothing = true;
        while let Some(key) = map.next_key::<&str>()? {
            nothing = false;
            match key {
                "oauth1_token" => {
                    if field_oauth1_token.is_some() {
                        return Err(::serde::de::Error::duplicate_field("oauth1_token"));
                    }
                    field_oauth1_token = Some(map.next_value()?);
                }
                "oauth1_token_secret" => {
                    if field_oauth1_token_secret.is_some() {
                        return Err(::serde::de::Error::duplicate_field("oauth1_token_secret"));
                    }
                    field_oauth1_token_secret = Some(map.next_value()?);
                }
                _ => {
                    // unknown field allowed and ignored
                    map.next_value::<::serde_json::Value>()?;
                }
            }
        }
        if optional && nothing {
            return Ok(None);
        }
        let result = TokenFromOAuth1Arg {
            oauth1_token: field_oauth1_token.ok_or_else(|| ::serde::de::Error::missing_field("oauth1_token"))?,
            oauth1_token_secret: field_oauth1_token_secret.ok_or_else(|| ::serde::de::Error::missing_field("oauth1_token_secret"))?,
        };
        Ok(Some(result))
    }

    pub(crate) fn internal_serialize<S: ::serde::ser::Serializer>(
        &self,
        s: &mut S::SerializeStruct,
    ) -> Result<(), S::Error> {
        use serde::ser::SerializeStruct;
        s.serialize_field("oauth1_token", &self.oauth1_token)?;
        s.serialize_field("oauth1_token_secret", &self.oauth1_token_secret)?;
        Ok(())
    }
}

impl<'de> ::serde::de::Deserialize<'de> for TokenFromOAuth1Arg {
    fn deserialize<D: ::serde::de::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        // struct deserializer
        use serde::de::{MapAccess, Visitor};
        struct StructVisitor;
        impl<'de> Visitor<'de> for StructVisitor {
            type Value = TokenFromOAuth1Arg;
            fn expecting(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                f.write_str("a TokenFromOAuth1Arg struct")
            }
            fn visit_map<V: MapAccess<'de>>(self, map: V) -> Result<Self::Value, V::Error> {
                TokenFromOAuth1Arg::internal_deserialize(map)
            }
        }
        deserializer.deserialize_struct("TokenFromOAuth1Arg", TOKEN_FROM_O_AUTH1_ARG_FIELDS, StructVisitor)
    }
}

impl ::serde::ser::Serialize for TokenFromOAuth1Arg {
    fn serialize<S: ::serde::ser::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        // struct serializer
        use serde::ser::SerializeStruct;
        let mut s = serializer.serialize_struct("TokenFromOAuth1Arg", 2)?;
        self.internal_serialize::<S>(&mut s)?;
        s.end()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive] // variants may be added in the future
pub enum TokenFromOAuth1Error {
    /// Part or all of the OAuth 1.0 access token info is invalid.
    InvalidOauth1TokenInfo,
    /// The authorized app does not match the app associated with the supplied access token.
    AppIdMismatch,
    /// Catch-all used for unrecognized values returned from the server. Encountering this value
    /// typically indicates that this SDK version is out of date.
    Other,
}

impl<'de> ::serde::de::Deserialize<'de> for TokenFromOAuth1Error {
    fn deserialize<D: ::serde::de::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        // union deserializer
        use serde::de::{self, MapAccess, Visitor};
        struct EnumVisitor;
        impl<'de> Visitor<'de> for EnumVisitor {
            type Value = TokenFromOAuth1Error;
            fn expecting(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                f.write_str("a TokenFromOAuth1Error structure")
            }
            fn visit_map<V: MapAccess<'de>>(self, mut map: V) -> Result<Self::Value, V::Error> {
                let tag: &str = match map.next_key()? {
                    Some(".tag") => map.next_value()?,
                    _ => return Err(de::Error::missing_field(".tag"))
                };
                let value = match tag {
                    "invalid_oauth1_token_info" => TokenFromOAuth1Error::InvalidOauth1TokenInfo,
                    "app_id_mismatch" => TokenFromOAuth1Error::AppIdMismatch,
                    _ => TokenFromOAuth1Error::Other,
                };
                crate::eat_json_fields(&mut map)?;
                Ok(value)
            }
        }
        const VARIANTS: &[&str] = &["invalid_oauth1_token_info",
                                    "app_id_mismatch",
                                    "other"];
        deserializer.deserialize_struct("TokenFromOAuth1Error", VARIANTS, EnumVisitor)
    }
}

impl ::serde::ser::Serialize for TokenFromOAuth1Error {
    fn serialize<S: ::serde::ser::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        // union serializer
        use serde::ser::SerializeStruct;
        match self {
            TokenFromOAuth1Error::InvalidOauth1TokenInfo => {
                // unit
                let mut s = serializer.serialize_struct("TokenFromOAuth1Error", 1)?;
                s.serialize_field(".tag", "invalid_oauth1_token_info")?;
                s.end()
            }
            TokenFromOAuth1Error::AppIdMismatch => {
                // unit
                let mut s = serializer.serialize_struct("TokenFromOAuth1Error", 1)?;
                s.serialize_field(".tag", "app_id_mismatch")?;
                s.end()
            }
            TokenFromOAuth1Error::Other => Err(::serde::ser::Error::custom("cannot serialize 'Other' variant"))
        }
    }
}

impl ::std::error::Error for TokenFromOAuth1Error {
}

impl ::std::fmt::Display for TokenFromOAuth1Error {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match self {
            TokenFromOAuth1Error::InvalidOauth1TokenInfo => f.write_str("Part or all of the OAuth 1.0 access token info is invalid."),
            TokenFromOAuth1Error::AppIdMismatch => f.write_str("The authorized app does not match the app associated with the supplied access token."),
            _ => write!(f, "{:?}", *self),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive] // structs may have more fields added in the future.
pub struct TokenFromOAuth1Result {
    /// The OAuth 2.0 token generated from the supplied OAuth 1.0 token.
    pub oauth2_token: String,
}

impl TokenFromOAuth1Result {
    pub fn new(oauth2_token: String) -> Self {
        TokenFromOAuth1Result {
            oauth2_token,
        }
    }
}

const TOKEN_FROM_O_AUTH1_RESULT_FIELDS: &[&str] = &["oauth2_token"];
impl TokenFromOAuth1Result {
    pub(crate) fn internal_deserialize<'de, V: ::serde::de::MapAccess<'de>>(
        map: V,
    ) -> Result<TokenFromOAuth1Result, V::Error> {
        Self::internal_deserialize_opt(map, false).map(Option::unwrap)
    }

    pub(crate) fn internal_deserialize_opt<'de, V: ::serde::de::MapAccess<'de>>(
        mut map: V,
        optional: bool,
    ) -> Result<Option<TokenFromOAuth1Result>, V::Error> {
        let mut field_oauth2_token = None;
        let mut nothing = true;
        while let Some(key) = map.next_key::<&str>()? {
            nothing = false;
            match key {
                "oauth2_token" => {
                    if field_oauth2_token.is_some() {
                        return Err(::serde::de::Error::duplicate_field("oauth2_token"));
                    }
                    field_oauth2_token = Some(map.next_value()?);
                }
                _ => {
                    // unknown field allowed and ignored
                    map.next_value::<::serde_json::Value>()?;
                }
            }
        }
        if optional && nothing {
            return Ok(None);
        }
        let result = TokenFromOAuth1Result {
            oauth2_token: field_oauth2_token.ok_or_else(|| ::serde::de::Error::missing_field("oauth2_token"))?,
        };
        Ok(Some(result))
    }

    pub(crate) fn internal_serialize<S: ::serde::ser::Serializer>(
        &self,
        s: &mut S::SerializeStruct,
    ) -> Result<(), S::Error> {
        use serde::ser::SerializeStruct;
        s.serialize_field("oauth2_token", &self.oauth2_token)?;
        Ok(())
    }
}

impl<'de> ::serde::de::Deserialize<'de> for TokenFromOAuth1Result {
    fn deserialize<D: ::serde::de::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        // struct deserializer
        use serde::de::{MapAccess, Visitor};
        struct StructVisitor;
        impl<'de> Visitor<'de> for StructVisitor {
            type Value = TokenFromOAuth1Result;
            fn expecting(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                f.write_str("a TokenFromOAuth1Result struct")
            }
            fn visit_map<V: MapAccess<'de>>(self, map: V) -> Result<Self::Value, V::Error> {
                TokenFromOAuth1Result::internal_deserialize(map)
            }
        }
        deserializer.deserialize_struct("TokenFromOAuth1Result", TOKEN_FROM_O_AUTH1_RESULT_FIELDS, StructVisitor)
    }
}

impl ::serde::ser::Serialize for TokenFromOAuth1Result {
    fn serialize<S: ::serde::ser::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        // struct serializer
        use serde::ser::SerializeStruct;
        let mut s = serializer.serialize_struct("TokenFromOAuth1Result", 1)?;
        self.internal_serialize::<S>(&mut s)?;
        s.end()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive] // structs may have more fields added in the future.
pub struct TokenScopeError {
    /// The required scope to access the route.
    pub required_scope: String,
}

impl TokenScopeError {
    pub fn new(required_scope: String) -> Self {
        TokenScopeError {
            required_scope,
        }
    }
}

const TOKEN_SCOPE_ERROR_FIELDS: &[&str] = &["required_scope"];
impl TokenScopeError {
    pub(crate) fn internal_deserialize<'de, V: ::serde::de::MapAccess<'de>>(
        map: V,
    ) -> Result<TokenScopeError, V::Error> {
        Self::internal_deserialize_opt(map, false).map(Option::unwrap)
    }

    pub(crate) fn internal_deserialize_opt<'de, V: ::serde::de::MapAccess<'de>>(
        mut map: V,
        optional: bool,
    ) -> Result<Option<TokenScopeError>, V::Error> {
        let mut field_required_scope = None;
        let mut nothing = true;
        while let Some(key) = map.next_key::<&str>()? {
            nothing = false;
            match key {
                "required_scope" => {
                    if field_required_scope.is_some() {
                        return Err(::serde::de::Error::duplicate_field("required_scope"));
                    }
                    field_required_scope = Some(map.next_value()?);
                }
                _ => {
                    // unknown field allowed and ignored
                    map.next_value::<::serde_json::Value>()?;
                }
            }
        }
        if optional && nothing {
            return Ok(None);
        }
        let result = TokenScopeError {
            required_scope: field_required_scope.ok_or_else(|| ::serde::de::Error::missing_field("required_scope"))?,
        };
        Ok(Some(result))
    }

    pub(crate) fn internal_serialize<S: ::serde::ser::Serializer>(
        &self,
        s: &mut S::SerializeStruct,
    ) -> Result<(), S::Error> {
        use serde::ser::SerializeStruct;
        s.serialize_field("required_scope", &self.required_scope)?;
        Ok(())
    }
}

impl<'de> ::serde::de::Deserialize<'de> for TokenScopeError {
    fn deserialize<D: ::serde::de::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        // struct deserializer
        use serde::de::{MapAccess, Visitor};
        struct StructVisitor;
        impl<'de> Visitor<'de> for StructVisitor {
            type Value = TokenScopeError;
            fn expecting(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                f.write_str("a TokenScopeError struct")
            }
            fn visit_map<V: MapAccess<'de>>(self, map: V) -> Result<Self::Value, V::Error> {
                TokenScopeError::internal_deserialize(map)
            }
        }
        deserializer.deserialize_struct("TokenScopeError", TOKEN_SCOPE_ERROR_FIELDS, StructVisitor)
    }
}

impl ::serde::ser::Serialize for TokenScopeError {
    fn serialize<S: ::serde::ser::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        // struct serializer
        use serde::ser::SerializeStruct;
        let mut s = serializer.serialize_struct("TokenScopeError", 1)?;
        self.internal_serialize::<S>(&mut s)?;
        s.end()
    }
}

