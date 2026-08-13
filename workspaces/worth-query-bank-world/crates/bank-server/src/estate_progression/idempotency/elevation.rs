use bank_domain::{
    estate::{EmergencyAccessReason, EstateAction, RestrictedBankField},
    proposals::BankIdempotencyKey,
};
use worth_foundational::facade::{CanonicalBasisDomain, CanonicalBasisEntry};
use worth_query_host::facade::primary_graph::WorthQueryApplicationIdempotencyBinding;

use super::{derive_identity, digest_entry, text_entry, unsigned_entry};
use crate::estate_progression::BankEstateProgressionDenial;

const ELEVATION_KEY_DOMAIN: CanonicalBasisDomain =
    CanonicalBasisDomain::Future("worth-bank.estate-elevation-key");
const ELEVATION_INTENT_DOMAIN: CanonicalBasisDomain =
    CanonicalBasisDomain::Future("worth-bank.estate-elevation-intent");

#[derive(Clone, Copy)]
pub(in crate::estate_progression) enum EstateElevationTransition {
    Request,
    Approve,
    Revoke,
    CompleteReview,
}

pub(in crate::estate_progression) fn elevation_binding(
    key: &BankIdempotencyKey,
    transition: EstateElevationTransition,
    action: EstateAction,
) -> Result<WorthQueryApplicationIdempotencyBinding, BankEstateProgressionDenial> {
    let transition_label = transition.label();
    let key_identity = derive_identity(
        ELEVATION_KEY_DOMAIN,
        "worth-bank-estate-elevation-key-v1",
        [
            text_entry(ELEVATION_KEY_DOMAIN, "transition", transition_label),
            text_entry(ELEVATION_KEY_DOMAIN, "client-key", key.as_str()),
        ],
    );
    let mut entries = vec![
        digest_entry(ELEVATION_INTENT_DOMAIN, "key", key_identity),
        text_entry(ELEVATION_INTENT_DOMAIN, "transition", transition_label),
    ];
    transition.append_action_entries(action, &mut entries)?;
    let intent_identity = derive_identity(
        ELEVATION_INTENT_DOMAIN,
        "worth-bank-estate-elevation-intent-v1",
        entries,
    );
    Ok(WorthQueryApplicationIdempotencyBinding::new(
        *key_identity.bytes(),
        *intent_identity.bytes(),
    ))
}

impl EstateElevationTransition {
    const fn label(self) -> &'static str {
        match self {
            Self::Request => "request",
            Self::Approve => "approve",
            Self::Revoke => "revoke",
            Self::CompleteReview => "complete-review",
        }
    }

    fn append_action_entries(
        self,
        action: EstateAction,
        entries: &mut Vec<CanonicalBasisEntry>,
    ) -> Result<(), BankEstateProgressionDenial> {
        match (self, action) {
            (
                Self::Request,
                EstateAction::RequestEmergencyAccess {
                    estate,
                    access,
                    review,
                    grant,
                    reason,
                    field,
                    duration,
                },
            ) => {
                entries.extend([
                    unsigned_entry(ELEVATION_INTENT_DOMAIN, "estate", estate.get()),
                    unsigned_entry(ELEVATION_INTENT_DOMAIN, "access", access.get()),
                    unsigned_entry(ELEVATION_INTENT_DOMAIN, "review", review.get()),
                    unsigned_entry(ELEVATION_INTENT_DOMAIN, "grant", grant.get()),
                    text_entry(ELEVATION_INTENT_DOMAIN, "reason", reason_text(reason)),
                    text_entry(ELEVATION_INTENT_DOMAIN, "field", field_text(field)),
                    unsigned_entry(
                        ELEVATION_INTENT_DOMAIN,
                        "duration-seconds",
                        duration.as_secs(),
                    ),
                    unsigned_entry(
                        ELEVATION_INTENT_DOMAIN,
                        "duration-nanoseconds",
                        u64::from(duration.subsec_nanos()),
                    ),
                ]);
                Ok(())
            }
            (Self::Approve, EstateAction::ApproveEmergencyAccess { estate, access })
            | (Self::Revoke, EstateAction::RevokeEmergencyAccess { estate, access }) => {
                entries.extend([
                    unsigned_entry(ELEVATION_INTENT_DOMAIN, "estate", estate.get()),
                    unsigned_entry(ELEVATION_INTENT_DOMAIN, "access", access.get()),
                ]);
                Ok(())
            }
            (
                Self::CompleteReview,
                EstateAction::CompleteMandatoryReview {
                    estate,
                    access,
                    review,
                },
            ) => {
                entries.extend([
                    unsigned_entry(ELEVATION_INTENT_DOMAIN, "estate", estate.get()),
                    unsigned_entry(ELEVATION_INTENT_DOMAIN, "access", access.get()),
                    unsigned_entry(ELEVATION_INTENT_DOMAIN, "review", review.get()),
                ]);
                Ok(())
            }
            _ => Err(BankEstateProgressionDenial::CommandInput(
                "EstateEmergencyAccessTransition",
            )),
        }
    }
}

const fn reason_text(reason: EmergencyAccessReason) -> &'static str {
    match reason {
        EmergencyAccessReason::PreventImmediateLoss => "prevent-immediate-loss",
        EmergencyAccessReason::ProtectVulnerableCustomer => "protect-vulnerable-customer",
        EmergencyAccessReason::MeetLegalDeadline => "meet-legal-deadline",
    }
}

const fn field_text(field: RestrictedBankField) -> &'static str {
    match field {
        RestrictedBankField::CustomerIdentity => "customer-identity",
        RestrictedBankField::BeneficiaryIdentity => "beneficiary-identity",
        RestrictedBankField::LegalDocument => "legal-document",
        RestrictedBankField::AccountDetails => "account-details",
        RestrictedBankField::PostingHistory => "posting-history",
        RestrictedBankField::AuditTrail => "audit-trail",
        RestrictedBankField::GovernanceMetadata => "governance-metadata",
        RestrictedBankField::EmergencyAccessActivity => "emergency-access-activity",
    }
}
