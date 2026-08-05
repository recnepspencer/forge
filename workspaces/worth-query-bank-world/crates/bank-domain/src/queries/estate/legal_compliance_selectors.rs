use worth_query_decl::facade::{
    application_query::{
        ApplicationQueryResultFieldRef, ApplicationQueryResultRelationRef, ExactlyOneResult,
        ForwardResultTraversal, ManyResults, ReverseResultTraversal,
    },
    application_schema::{EqualityPredicate, NoApplicationCurrency, ReadOnly, ReadWrite},
};

use crate::{
    estate::{EstateCaseId, LegalAuthorityId, LegalAuthorityKind},
    model::BankPrincipalId,
    schema::{
        BankSchema, EstateCase, EstateCaseIdentityField, EstateCaseRecord, LegalAuthority,
        LegalAuthorityEstate, LegalAuthorityHolder, LegalAuthorityIdentityField,
        LegalAuthorityKindField, LegalAuthorityRecognizedField, LegalAuthorityRecord, Principal,
        PrincipalIdentity, PrincipalIdentityField,
    },
};

use super::legal_compliance::EstateLegalComplianceQuery;

pub(super) struct EstateIdentitySlot;
pub(super) struct AuthoritiesSlot;
pub(super) struct AuthorityIdentitySlot;
pub(super) struct AuthorityKindSlot;
pub(super) struct AuthorityRecognizedSlot;
pub(super) struct AuthorityHolderSlot;
pub(super) struct AuthorityHolderIdentitySlot;

macro_rules! selector {
    ($name:ident, $slot:ty, $entity:ty, $aspect:ty, $field:ty, $value:ty, $write:ty, $alias:literal) => {
        pub(super) fn $name() -> ApplicationQueryResultFieldRef<
            EstateLegalComplianceQuery,
            $slot,
            BankSchema,
            $entity,
            $aspect,
            $field,
            $value,
            $write,
            EqualityPredicate,
            NoApplicationCurrency,
        > {
            ApplicationQueryResultFieldRef::new($alias, <$field>::reference())
        }
    };
}

selector!(
    estate_identity,
    EstateIdentitySlot,
    EstateCase,
    EstateCaseRecord,
    EstateCaseIdentityField,
    EstateCaseId,
    ReadOnly,
    "estate"
);
selector!(
    authority_identity,
    AuthorityIdentitySlot,
    LegalAuthority,
    LegalAuthorityRecord,
    LegalAuthorityIdentityField,
    LegalAuthorityId,
    ReadOnly,
    "authority"
);
selector!(
    authority_kind,
    AuthorityKindSlot,
    LegalAuthority,
    LegalAuthorityRecord,
    LegalAuthorityKindField,
    LegalAuthorityKind,
    ReadWrite,
    "kind"
);
selector!(
    authority_recognized,
    AuthorityRecognizedSlot,
    LegalAuthority,
    LegalAuthorityRecord,
    LegalAuthorityRecognizedField,
    bool,
    ReadWrite,
    "recognized"
);
selector!(
    authority_holder_identity,
    AuthorityHolderIdentitySlot,
    Principal,
    PrincipalIdentity,
    PrincipalIdentityField,
    BankPrincipalId,
    ReadOnly,
    "holder"
);

pub(super) fn estate_authorities() -> ApplicationQueryResultRelationRef<
    EstateLegalComplianceQuery,
    AuthoritiesSlot,
    BankSchema,
    LegalAuthorityEstate,
    LegalAuthority,
    EstateCase,
    ReverseResultTraversal,
    ManyResults,
> {
    ApplicationQueryResultRelationRef::reverse_many(
        "legal_authorities",
        LegalAuthorityEstate::reference(),
    )
}

pub(super) fn authority_holder() -> ApplicationQueryResultRelationRef<
    EstateLegalComplianceQuery,
    AuthorityHolderSlot,
    BankSchema,
    LegalAuthorityHolder,
    LegalAuthority,
    Principal,
    ForwardResultTraversal,
    ExactlyOneResult,
> {
    ApplicationQueryResultRelationRef::forward_one("holder", LegalAuthorityHolder::reference())
}
