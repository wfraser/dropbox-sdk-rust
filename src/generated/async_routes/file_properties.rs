// DO NOT EDIT
// This file was @generated by Stone

#![allow(
    clippy::too_many_arguments,
    clippy::large_enum_variant,
    clippy::result_large_err,
    clippy::doc_markdown,
)]

#[allow(unused_imports)]
pub use crate::generated::types::file_properties::*;

/// Add property groups to a Dropbox file. See
/// [`templates_add_for_user()`](crate::file_properties::templates_add_for_user) or
/// [`templates_add_for_team()`](crate::file_properties::templates_add_for_team) to create new
/// templates.
pub fn properties_add<'a>(
    client: &'a impl crate::async_client_trait::UserAuthClient,
    arg: &'a AddPropertiesArg,
) -> impl std::future::Future<Output=Result<(), crate::Error<AddPropertiesError>>> + Send + 'a {
    crate::client_helpers::request(
        client,
        crate::client_trait_common::Endpoint::Api,
        crate::client_trait_common::Style::Rpc,
        "file_properties/properties/add",
        arg,
        None)
}

/// Overwrite property groups associated with a file. This endpoint should be used instead of
/// [`properties_update()`](crate::file_properties::properties_update) when property groups are
/// being updated via a "snapshot" instead of via a "delta". In other words, this endpoint will
/// delete all omitted fields from a property group, whereas
/// [`properties_update()`](crate::file_properties::properties_update) will only delete fields that
/// are explicitly marked for deletion.
pub fn properties_overwrite<'a>(
    client: &'a impl crate::async_client_trait::UserAuthClient,
    arg: &'a OverwritePropertyGroupArg,
) -> impl std::future::Future<Output=Result<(), crate::Error<InvalidPropertyGroupError>>> + Send + 'a {
    crate::client_helpers::request(
        client,
        crate::client_trait_common::Endpoint::Api,
        crate::client_trait_common::Style::Rpc,
        "file_properties/properties/overwrite",
        arg,
        None)
}

/// Permanently removes the specified property group from the file. To remove specific property
/// field key value pairs, see [`properties_update()`](crate::file_properties::properties_update).
/// To update a template, see
/// [`templates_update_for_user()`](crate::file_properties::templates_update_for_user) or
/// [`templates_update_for_team()`](crate::file_properties::templates_update_for_team). To remove a
/// template, see [`templates_remove_for_user()`](crate::file_properties::templates_remove_for_user)
/// or [`templates_remove_for_team()`](crate::file_properties::templates_remove_for_team).
pub fn properties_remove<'a>(
    client: &'a impl crate::async_client_trait::UserAuthClient,
    arg: &'a RemovePropertiesArg,
) -> impl std::future::Future<Output=Result<(), crate::Error<RemovePropertiesError>>> + Send + 'a {
    crate::client_helpers::request(
        client,
        crate::client_trait_common::Endpoint::Api,
        crate::client_trait_common::Style::Rpc,
        "file_properties/properties/remove",
        arg,
        None)
}

/// Search across property templates for particular property field values.
pub fn properties_search<'a>(
    client: &'a impl crate::async_client_trait::UserAuthClient,
    arg: &'a PropertiesSearchArg,
) -> impl std::future::Future<Output=Result<PropertiesSearchResult, crate::Error<PropertiesSearchError>>> + Send + 'a {
    crate::client_helpers::request(
        client,
        crate::client_trait_common::Endpoint::Api,
        crate::client_trait_common::Style::Rpc,
        "file_properties/properties/search",
        arg,
        None)
}

/// Once a cursor has been retrieved from
/// [`properties_search()`](crate::file_properties::properties_search), use this to paginate through
/// all search results.
pub fn properties_search_continue<'a>(
    client: &'a impl crate::async_client_trait::UserAuthClient,
    arg: &'a PropertiesSearchContinueArg,
) -> impl std::future::Future<Output=Result<PropertiesSearchResult, crate::Error<PropertiesSearchContinueError>>> + Send + 'a {
    crate::client_helpers::request(
        client,
        crate::client_trait_common::Endpoint::Api,
        crate::client_trait_common::Style::Rpc,
        "file_properties/properties/search/continue",
        arg,
        None)
}

/// Add, update or remove properties associated with the supplied file and templates. This endpoint
/// should be used instead of
/// [`properties_overwrite()`](crate::file_properties::properties_overwrite) when property groups
/// are being updated via a "delta" instead of via a "snapshot" . In other words, this endpoint will
/// not delete any omitted fields from a property group, whereas
/// [`properties_overwrite()`](crate::file_properties::properties_overwrite) will delete any fields
/// that are omitted from a property group.
pub fn properties_update<'a>(
    client: &'a impl crate::async_client_trait::UserAuthClient,
    arg: &'a UpdatePropertiesArg,
) -> impl std::future::Future<Output=Result<(), crate::Error<UpdatePropertiesError>>> + Send + 'a {
    crate::client_helpers::request(
        client,
        crate::client_trait_common::Endpoint::Api,
        crate::client_trait_common::Style::Rpc,
        "file_properties/properties/update",
        arg,
        None)
}

/// Add a template associated with a team. See
/// [`properties_add()`](crate::file_properties::properties_add) to add properties to a file or
/// folder. Note: this endpoint will create team-owned templates.
pub fn templates_add_for_team<'a>(
    client: &'a impl crate::async_client_trait::TeamAuthClient,
    arg: &'a AddTemplateArg,
) -> impl std::future::Future<Output=Result<AddTemplateResult, crate::Error<ModifyTemplateError>>> + Send + 'a {
    crate::client_helpers::request(
        client,
        crate::client_trait_common::Endpoint::Api,
        crate::client_trait_common::Style::Rpc,
        "file_properties/templates/add_for_team",
        arg,
        None)
}

/// Add a template associated with a user. See
/// [`properties_add()`](crate::file_properties::properties_add) to add properties to a file. This
/// endpoint can't be called on a team member or admin's behalf.
pub fn templates_add_for_user<'a>(
    client: &'a impl crate::async_client_trait::UserAuthClient,
    arg: &'a AddTemplateArg,
) -> impl std::future::Future<Output=Result<AddTemplateResult, crate::Error<ModifyTemplateError>>> + Send + 'a {
    crate::client_helpers::request(
        client,
        crate::client_trait_common::Endpoint::Api,
        crate::client_trait_common::Style::Rpc,
        "file_properties/templates/add_for_user",
        arg,
        None)
}

/// Get the schema for a specified template.
pub fn templates_get_for_team<'a>(
    client: &'a impl crate::async_client_trait::TeamAuthClient,
    arg: &'a GetTemplateArg,
) -> impl std::future::Future<Output=Result<GetTemplateResult, crate::Error<TemplateError>>> + Send + 'a {
    crate::client_helpers::request(
        client,
        crate::client_trait_common::Endpoint::Api,
        crate::client_trait_common::Style::Rpc,
        "file_properties/templates/get_for_team",
        arg,
        None)
}

/// Get the schema for a specified template. This endpoint can't be called on a team member or
/// admin's behalf.
pub fn templates_get_for_user<'a>(
    client: &'a impl crate::async_client_trait::UserAuthClient,
    arg: &'a GetTemplateArg,
) -> impl std::future::Future<Output=Result<GetTemplateResult, crate::Error<TemplateError>>> + Send + 'a {
    crate::client_helpers::request(
        client,
        crate::client_trait_common::Endpoint::Api,
        crate::client_trait_common::Style::Rpc,
        "file_properties/templates/get_for_user",
        arg,
        None)
}

/// Get the template identifiers for a team. To get the schema of each template use
/// [`templates_get_for_team()`](crate::file_properties::templates_get_for_team).
pub fn templates_list_for_team(
    client: &impl crate::async_client_trait::TeamAuthClient,
) -> impl std::future::Future<Output=Result<ListTemplateResult, crate::Error<TemplateError>>> + Send + '_ {
    crate::client_helpers::request(
        client,
        crate::client_trait_common::Endpoint::Api,
        crate::client_trait_common::Style::Rpc,
        "file_properties/templates/list_for_team",
        &(),
        None)
}

/// Get the template identifiers for a team. To get the schema of each template use
/// [`templates_get_for_user()`](crate::file_properties::templates_get_for_user). This endpoint
/// can't be called on a team member or admin's behalf.
pub fn templates_list_for_user(
    client: &impl crate::async_client_trait::UserAuthClient,
) -> impl std::future::Future<Output=Result<ListTemplateResult, crate::Error<TemplateError>>> + Send + '_ {
    crate::client_helpers::request(
        client,
        crate::client_trait_common::Endpoint::Api,
        crate::client_trait_common::Style::Rpc,
        "file_properties/templates/list_for_user",
        &(),
        None)
}

/// Permanently removes the specified template created from
/// [`templates_add_for_user()`](crate::file_properties::templates_add_for_user). All properties
/// associated with the template will also be removed. This action cannot be undone.
pub fn templates_remove_for_team<'a>(
    client: &'a impl crate::async_client_trait::TeamAuthClient,
    arg: &'a RemoveTemplateArg,
) -> impl std::future::Future<Output=Result<(), crate::Error<TemplateError>>> + Send + 'a {
    crate::client_helpers::request(
        client,
        crate::client_trait_common::Endpoint::Api,
        crate::client_trait_common::Style::Rpc,
        "file_properties/templates/remove_for_team",
        arg,
        None)
}

/// Permanently removes the specified template created from
/// [`templates_add_for_user()`](crate::file_properties::templates_add_for_user). All properties
/// associated with the template will also be removed. This action cannot be undone.
pub fn templates_remove_for_user<'a>(
    client: &'a impl crate::async_client_trait::UserAuthClient,
    arg: &'a RemoveTemplateArg,
) -> impl std::future::Future<Output=Result<(), crate::Error<TemplateError>>> + Send + 'a {
    crate::client_helpers::request(
        client,
        crate::client_trait_common::Endpoint::Api,
        crate::client_trait_common::Style::Rpc,
        "file_properties/templates/remove_for_user",
        arg,
        None)
}

/// Update a template associated with a team. This route can update the template name, the template
/// description and add optional properties to templates.
pub fn templates_update_for_team<'a>(
    client: &'a impl crate::async_client_trait::TeamAuthClient,
    arg: &'a UpdateTemplateArg,
) -> impl std::future::Future<Output=Result<UpdateTemplateResult, crate::Error<ModifyTemplateError>>> + Send + 'a {
    crate::client_helpers::request(
        client,
        crate::client_trait_common::Endpoint::Api,
        crate::client_trait_common::Style::Rpc,
        "file_properties/templates/update_for_team",
        arg,
        None)
}

/// Update a template associated with a user. This route can update the template name, the template
/// description and add optional properties to templates. This endpoint can't be called on a team
/// member or admin's behalf.
pub fn templates_update_for_user<'a>(
    client: &'a impl crate::async_client_trait::UserAuthClient,
    arg: &'a UpdateTemplateArg,
) -> impl std::future::Future<Output=Result<UpdateTemplateResult, crate::Error<ModifyTemplateError>>> + Send + 'a {
    crate::client_helpers::request(
        client,
        crate::client_trait_common::Endpoint::Api,
        crate::client_trait_common::Style::Rpc,
        "file_properties/templates/update_for_user",
        arg,
        None)
}
