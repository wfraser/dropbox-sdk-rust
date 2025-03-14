// DO NOT EDIT
// This file was @generated by Stone

#![allow(
    clippy::too_many_arguments,
    clippy::large_enum_variant,
    clippy::result_large_err,
    clippy::doc_markdown,
)]

pub type GroupExternalId = String;
pub type GroupId = String;
pub type MemberExternalId = String;
pub type ResellerId = String;
pub type TeamId = String;
pub type TeamMemberId = String;

/// The group type determines how a group is managed.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive] // variants may be added in the future
pub enum GroupManagementType {
    /// A group which is managed by selected users.
    UserManaged,
    /// A group which is managed by team admins only.
    CompanyManaged,
    /// A group which is managed automatically by Dropbox.
    SystemManaged,
    /// Catch-all used for unrecognized values returned from the server. Encountering this value
    /// typically indicates that this SDK version is out of date.
    Other,
}

impl<'de> ::serde::de::Deserialize<'de> for GroupManagementType {
    fn deserialize<D: ::serde::de::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        // union deserializer
        use serde::de::{self, MapAccess, Visitor};
        struct EnumVisitor;
        impl<'de> Visitor<'de> for EnumVisitor {
            type Value = GroupManagementType;
            fn expecting(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                f.write_str("a GroupManagementType structure")
            }
            fn visit_map<V: MapAccess<'de>>(self, mut map: V) -> Result<Self::Value, V::Error> {
                let tag: &str = match map.next_key()? {
                    Some(".tag") => map.next_value()?,
                    _ => return Err(de::Error::missing_field(".tag"))
                };
                let value = match tag {
                    "user_managed" => GroupManagementType::UserManaged,
                    "company_managed" => GroupManagementType::CompanyManaged,
                    "system_managed" => GroupManagementType::SystemManaged,
                    _ => GroupManagementType::Other,
                };
                crate::eat_json_fields(&mut map)?;
                Ok(value)
            }
        }
        const VARIANTS: &[&str] = &["user_managed",
                                    "company_managed",
                                    "system_managed",
                                    "other"];
        deserializer.deserialize_struct("GroupManagementType", VARIANTS, EnumVisitor)
    }
}

impl ::serde::ser::Serialize for GroupManagementType {
    fn serialize<S: ::serde::ser::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        // union serializer
        use serde::ser::SerializeStruct;
        match self {
            GroupManagementType::UserManaged => {
                // unit
                let mut s = serializer.serialize_struct("GroupManagementType", 1)?;
                s.serialize_field(".tag", "user_managed")?;
                s.end()
            }
            GroupManagementType::CompanyManaged => {
                // unit
                let mut s = serializer.serialize_struct("GroupManagementType", 1)?;
                s.serialize_field(".tag", "company_managed")?;
                s.end()
            }
            GroupManagementType::SystemManaged => {
                // unit
                let mut s = serializer.serialize_struct("GroupManagementType", 1)?;
                s.serialize_field(".tag", "system_managed")?;
                s.end()
            }
            GroupManagementType::Other => Err(::serde::ser::Error::custom("cannot serialize 'Other' variant"))
        }
    }
}

/// Information about a group.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive] // structs may have more fields added in the future.
pub struct GroupSummary {
    pub group_name: String,
    pub group_id: GroupId,
    /// Who is allowed to manage the group.
    pub group_management_type: GroupManagementType,
    /// External ID of group. This is an arbitrary ID that an admin can attach to a group.
    pub group_external_id: Option<GroupExternalId>,
    /// The number of members in the group.
    pub member_count: Option<u32>,
}

impl GroupSummary {
    pub fn new(
        group_name: String,
        group_id: GroupId,
        group_management_type: GroupManagementType,
    ) -> Self {
        GroupSummary {
            group_name,
            group_id,
            group_management_type,
            group_external_id: None,
            member_count: None,
        }
    }

    pub fn with_group_external_id(mut self, value: GroupExternalId) -> Self {
        self.group_external_id = Some(value);
        self
    }

    pub fn with_member_count(mut self, value: u32) -> Self {
        self.member_count = Some(value);
        self
    }
}

const GROUP_SUMMARY_FIELDS: &[&str] = &["group_name",
                                        "group_id",
                                        "group_management_type",
                                        "group_external_id",
                                        "member_count"];
impl GroupSummary {
    pub(crate) fn internal_deserialize<'de, V: ::serde::de::MapAccess<'de>>(
        map: V,
    ) -> Result<GroupSummary, V::Error> {
        Self::internal_deserialize_opt(map, false).map(Option::unwrap)
    }

    pub(crate) fn internal_deserialize_opt<'de, V: ::serde::de::MapAccess<'de>>(
        mut map: V,
        optional: bool,
    ) -> Result<Option<GroupSummary>, V::Error> {
        let mut field_group_name = None;
        let mut field_group_id = None;
        let mut field_group_management_type = None;
        let mut field_group_external_id = None;
        let mut field_member_count = None;
        let mut nothing = true;
        while let Some(key) = map.next_key::<&str>()? {
            nothing = false;
            match key {
                "group_name" => {
                    if field_group_name.is_some() {
                        return Err(::serde::de::Error::duplicate_field("group_name"));
                    }
                    field_group_name = Some(map.next_value()?);
                }
                "group_id" => {
                    if field_group_id.is_some() {
                        return Err(::serde::de::Error::duplicate_field("group_id"));
                    }
                    field_group_id = Some(map.next_value()?);
                }
                "group_management_type" => {
                    if field_group_management_type.is_some() {
                        return Err(::serde::de::Error::duplicate_field("group_management_type"));
                    }
                    field_group_management_type = Some(map.next_value()?);
                }
                "group_external_id" => {
                    if field_group_external_id.is_some() {
                        return Err(::serde::de::Error::duplicate_field("group_external_id"));
                    }
                    field_group_external_id = Some(map.next_value()?);
                }
                "member_count" => {
                    if field_member_count.is_some() {
                        return Err(::serde::de::Error::duplicate_field("member_count"));
                    }
                    field_member_count = Some(map.next_value()?);
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
        let result = GroupSummary {
            group_name: field_group_name.ok_or_else(|| ::serde::de::Error::missing_field("group_name"))?,
            group_id: field_group_id.ok_or_else(|| ::serde::de::Error::missing_field("group_id"))?,
            group_management_type: field_group_management_type.ok_or_else(|| ::serde::de::Error::missing_field("group_management_type"))?,
            group_external_id: field_group_external_id.and_then(Option::flatten),
            member_count: field_member_count.and_then(Option::flatten),
        };
        Ok(Some(result))
    }

    pub(crate) fn internal_serialize<S: ::serde::ser::Serializer>(
        &self,
        s: &mut S::SerializeStruct,
    ) -> Result<(), S::Error> {
        use serde::ser::SerializeStruct;
        s.serialize_field("group_name", &self.group_name)?;
        s.serialize_field("group_id", &self.group_id)?;
        s.serialize_field("group_management_type", &self.group_management_type)?;
        if let Some(val) = &self.group_external_id {
            s.serialize_field("group_external_id", val)?;
        }
        if let Some(val) = &self.member_count {
            s.serialize_field("member_count", val)?;
        }
        Ok(())
    }
}

impl<'de> ::serde::de::Deserialize<'de> for GroupSummary {
    fn deserialize<D: ::serde::de::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        // struct deserializer
        use serde::de::{MapAccess, Visitor};
        struct StructVisitor;
        impl<'de> Visitor<'de> for StructVisitor {
            type Value = GroupSummary;
            fn expecting(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                f.write_str("a GroupSummary struct")
            }
            fn visit_map<V: MapAccess<'de>>(self, map: V) -> Result<Self::Value, V::Error> {
                GroupSummary::internal_deserialize(map)
            }
        }
        deserializer.deserialize_struct("GroupSummary", GROUP_SUMMARY_FIELDS, StructVisitor)
    }
}

impl ::serde::ser::Serialize for GroupSummary {
    fn serialize<S: ::serde::ser::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        // struct serializer
        use serde::ser::SerializeStruct;
        let mut s = serializer.serialize_struct("GroupSummary", 5)?;
        self.internal_serialize::<S>(&mut s)?;
        s.end()
    }
}

/// The group type determines how a group is created and managed.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive] // variants may be added in the future
pub enum GroupType {
    /// A group to which team members are automatically added. Applicable to [team
    /// folders](https://www.dropbox.com/help/986) only.
    Team,
    /// A group is created and managed by a user.
    UserManaged,
    /// Catch-all used for unrecognized values returned from the server. Encountering this value
    /// typically indicates that this SDK version is out of date.
    Other,
}

impl<'de> ::serde::de::Deserialize<'de> for GroupType {
    fn deserialize<D: ::serde::de::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        // union deserializer
        use serde::de::{self, MapAccess, Visitor};
        struct EnumVisitor;
        impl<'de> Visitor<'de> for EnumVisitor {
            type Value = GroupType;
            fn expecting(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                f.write_str("a GroupType structure")
            }
            fn visit_map<V: MapAccess<'de>>(self, mut map: V) -> Result<Self::Value, V::Error> {
                let tag: &str = match map.next_key()? {
                    Some(".tag") => map.next_value()?,
                    _ => return Err(de::Error::missing_field(".tag"))
                };
                let value = match tag {
                    "team" => GroupType::Team,
                    "user_managed" => GroupType::UserManaged,
                    _ => GroupType::Other,
                };
                crate::eat_json_fields(&mut map)?;
                Ok(value)
            }
        }
        const VARIANTS: &[&str] = &["team",
                                    "user_managed",
                                    "other"];
        deserializer.deserialize_struct("GroupType", VARIANTS, EnumVisitor)
    }
}

impl ::serde::ser::Serialize for GroupType {
    fn serialize<S: ::serde::ser::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        // union serializer
        use serde::ser::SerializeStruct;
        match self {
            GroupType::Team => {
                // unit
                let mut s = serializer.serialize_struct("GroupType", 1)?;
                s.serialize_field(".tag", "team")?;
                s.end()
            }
            GroupType::UserManaged => {
                // unit
                let mut s = serializer.serialize_struct("GroupType", 1)?;
                s.serialize_field(".tag", "user_managed")?;
                s.end()
            }
            GroupType::Other => Err(::serde::ser::Error::custom("cannot serialize 'Other' variant"))
        }
    }
}

/// The type of the space limit imposed on a team member.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive] // variants may be added in the future
pub enum MemberSpaceLimitType {
    /// The team member does not have imposed space limit.
    Off,
    /// The team member has soft imposed space limit - the limit is used for display and for
    /// notifications.
    AlertOnly,
    /// The team member has hard imposed space limit - Dropbox file sync will stop after the limit
    /// is reached.
    StopSync,
    /// Catch-all used for unrecognized values returned from the server. Encountering this value
    /// typically indicates that this SDK version is out of date.
    Other,
}

impl<'de> ::serde::de::Deserialize<'de> for MemberSpaceLimitType {
    fn deserialize<D: ::serde::de::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        // union deserializer
        use serde::de::{self, MapAccess, Visitor};
        struct EnumVisitor;
        impl<'de> Visitor<'de> for EnumVisitor {
            type Value = MemberSpaceLimitType;
            fn expecting(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                f.write_str("a MemberSpaceLimitType structure")
            }
            fn visit_map<V: MapAccess<'de>>(self, mut map: V) -> Result<Self::Value, V::Error> {
                let tag: &str = match map.next_key()? {
                    Some(".tag") => map.next_value()?,
                    _ => return Err(de::Error::missing_field(".tag"))
                };
                let value = match tag {
                    "off" => MemberSpaceLimitType::Off,
                    "alert_only" => MemberSpaceLimitType::AlertOnly,
                    "stop_sync" => MemberSpaceLimitType::StopSync,
                    _ => MemberSpaceLimitType::Other,
                };
                crate::eat_json_fields(&mut map)?;
                Ok(value)
            }
        }
        const VARIANTS: &[&str] = &["off",
                                    "alert_only",
                                    "stop_sync",
                                    "other"];
        deserializer.deserialize_struct("MemberSpaceLimitType", VARIANTS, EnumVisitor)
    }
}

impl ::serde::ser::Serialize for MemberSpaceLimitType {
    fn serialize<S: ::serde::ser::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        // union serializer
        use serde::ser::SerializeStruct;
        match self {
            MemberSpaceLimitType::Off => {
                // unit
                let mut s = serializer.serialize_struct("MemberSpaceLimitType", 1)?;
                s.serialize_field(".tag", "off")?;
                s.end()
            }
            MemberSpaceLimitType::AlertOnly => {
                // unit
                let mut s = serializer.serialize_struct("MemberSpaceLimitType", 1)?;
                s.serialize_field(".tag", "alert_only")?;
                s.end()
            }
            MemberSpaceLimitType::StopSync => {
                // unit
                let mut s = serializer.serialize_struct("MemberSpaceLimitType", 1)?;
                s.serialize_field(".tag", "stop_sync")?;
                s.end()
            }
            MemberSpaceLimitType::Other => Err(::serde::ser::Error::custom("cannot serialize 'Other' variant"))
        }
    }
}

/// Time range.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
#[non_exhaustive] // structs may have more fields added in the future.
pub struct TimeRange {
    /// Optional starting time (inclusive).
    pub start_time: Option<crate::types::common::DropboxTimestamp>,
    /// Optional ending time (exclusive).
    pub end_time: Option<crate::types::common::DropboxTimestamp>,
}

impl TimeRange {
    pub fn with_start_time(mut self, value: crate::types::common::DropboxTimestamp) -> Self {
        self.start_time = Some(value);
        self
    }

    pub fn with_end_time(mut self, value: crate::types::common::DropboxTimestamp) -> Self {
        self.end_time = Some(value);
        self
    }
}

const TIME_RANGE_FIELDS: &[&str] = &["start_time",
                                     "end_time"];
impl TimeRange {
    // no _opt deserializer
    pub(crate) fn internal_deserialize<'de, V: ::serde::de::MapAccess<'de>>(
        mut map: V,
    ) -> Result<TimeRange, V::Error> {
        let mut field_start_time = None;
        let mut field_end_time = None;
        while let Some(key) = map.next_key::<&str>()? {
            match key {
                "start_time" => {
                    if field_start_time.is_some() {
                        return Err(::serde::de::Error::duplicate_field("start_time"));
                    }
                    field_start_time = Some(map.next_value()?);
                }
                "end_time" => {
                    if field_end_time.is_some() {
                        return Err(::serde::de::Error::duplicate_field("end_time"));
                    }
                    field_end_time = Some(map.next_value()?);
                }
                _ => {
                    // unknown field allowed and ignored
                    map.next_value::<::serde_json::Value>()?;
                }
            }
        }
        let result = TimeRange {
            start_time: field_start_time.and_then(Option::flatten),
            end_time: field_end_time.and_then(Option::flatten),
        };
        Ok(result)
    }

    pub(crate) fn internal_serialize<S: ::serde::ser::Serializer>(
        &self,
        s: &mut S::SerializeStruct,
    ) -> Result<(), S::Error> {
        use serde::ser::SerializeStruct;
        if let Some(val) = &self.start_time {
            s.serialize_field("start_time", val)?;
        }
        if let Some(val) = &self.end_time {
            s.serialize_field("end_time", val)?;
        }
        Ok(())
    }
}

impl<'de> ::serde::de::Deserialize<'de> for TimeRange {
    fn deserialize<D: ::serde::de::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        // struct deserializer
        use serde::de::{MapAccess, Visitor};
        struct StructVisitor;
        impl<'de> Visitor<'de> for StructVisitor {
            type Value = TimeRange;
            fn expecting(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                f.write_str("a TimeRange struct")
            }
            fn visit_map<V: MapAccess<'de>>(self, map: V) -> Result<Self::Value, V::Error> {
                TimeRange::internal_deserialize(map)
            }
        }
        deserializer.deserialize_struct("TimeRange", TIME_RANGE_FIELDS, StructVisitor)
    }
}

impl ::serde::ser::Serialize for TimeRange {
    fn serialize<S: ::serde::ser::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        // struct serializer
        use serde::ser::SerializeStruct;
        let mut s = serializer.serialize_struct("TimeRange", 2)?;
        self.internal_serialize::<S>(&mut s)?;
        s.end()
    }
}

