use worth_query_decl::facade::authentication::{
    WorthQueryExternalPrincipalIdentity, WorthQueryPrincipalMappingStatus,
};
use worth_query_decl::facade::{
    worth_query_aspect, worth_query_field, worth_query_principal_binding,
};

use crate::model::BankPrincipalId;

use super::entities::{ExternalPrincipalMapping, Principal};
use super::relations::ExternalPrincipal;
use super::BankSchema;

worth_query_aspect!(
    pub ExternalPrincipalIdentity in BankSchema,
    ExternalPrincipalMapping
);
worth_query_field!(
    pub ExternalIdentityKey in BankSchema,
    ExternalPrincipalMapping,
    ExternalPrincipalIdentity:
    WorthQueryExternalPrincipalIdentity, read_only, equality
);
worth_query_field!(
    pub ExternalMappingStatus in BankSchema,
    ExternalPrincipalMapping,
    ExternalPrincipalIdentity:
    WorthQueryPrincipalMappingStatus, read_write, equality
);
worth_query_aspect!(pub PrincipalIdentity in BankSchema, Principal);
worth_query_field!(
    pub PrincipalIdentityField in BankSchema,
    Principal,
    PrincipalIdentity:
    BankPrincipalId, read_only, equality
);
worth_query_principal_binding!(
    pub BankPrincipalBinding in BankSchema,
    mapping ExternalPrincipalMapping {
        identity: ExternalIdentityKey,
        status: ExternalMappingStatus,
        target: ExternalPrincipal => Principal,
        principal_identity: PrincipalIdentityField
    }
);
