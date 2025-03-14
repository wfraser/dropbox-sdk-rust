// DO NOT EDIT
// This file was @generated by Stone

#![allow(
    clippy::too_many_arguments,
    clippy::large_enum_variant,
    clippy::result_large_err,
    clippy::doc_markdown,
)]

#[allow(unused_imports)]
pub use crate::generated::types::check::*;

/// This endpoint performs App Authentication, validating the supplied app key and secret, and
/// returns the supplied string, to allow you to test your code and connection to the Dropbox API.
/// It has no other effect. If you receive an HTTP 200 response with the supplied query, it
/// indicates at least part of the Dropbox API infrastructure is working and that the app key and
/// secret valid.
///
/// # Stability
/// *PREVIEW*: This function may change or disappear without notice.
#[cfg(feature = "unstable")]
#[cfg_attr(docsrs, doc(cfg(feature = "unstable")))]
pub fn app(
    client: &impl crate::client_trait::AppAuthClient,
    arg: &EchoArg,
) -> Result<EchoResult, crate::Error<crate::NoError>> {
    crate::client_helpers::unwrap_async(
        crate::client_helpers::request(
            client,
            crate::client_trait_common::Endpoint::Api,
            crate::client_trait_common::Style::Rpc,
            "check/app",
            arg,
            None)
    )
}

/// This endpoint performs User Authentication, validating the supplied access token, and returns
/// the supplied string, to allow you to test your code and connection to the Dropbox API. It has no
/// other effect. If you receive an HTTP 200 response with the supplied query, it indicates at least
/// part of the Dropbox API infrastructure is working and that the access token is valid.
///
/// # Stability
/// *PREVIEW*: This function may change or disappear without notice.
#[cfg(feature = "unstable")]
#[cfg_attr(docsrs, doc(cfg(feature = "unstable")))]
pub fn user(
    client: &impl crate::client_trait::UserAuthClient,
    arg: &EchoArg,
) -> Result<EchoResult, crate::Error<crate::NoError>> {
    crate::client_helpers::unwrap_async(
        crate::client_helpers::request(
            client,
            crate::client_trait_common::Endpoint::Api,
            crate::client_trait_common::Style::Rpc,
            "check/user",
            arg,
            None)
    )
}

