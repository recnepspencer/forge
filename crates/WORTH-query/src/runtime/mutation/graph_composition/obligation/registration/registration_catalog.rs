use std::collections::BTreeMap;

use crate::evidence_identity::{
    worth_query_evidence_identity, WorthQueryEvidenceIdentity, WorthQueryEvidenceScope,
    WorthQueryEvidenceTag,
};

use super::registration::WorthQueryGraphObligationRegistration;
use super::registration_denial::{
    WorthQueryGraphObligationRegistrationDenial, WorthQueryGraphObligationRegistrationDenialKind,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryGraphObligationRegistrationCatalog {
    registrations: Vec<WorthQueryGraphObligationRegistration>,
    catalog_digest: WorthQueryEvidenceIdentity,
}

impl WorthQueryGraphObligationRegistrationCatalog {
    pub fn from_registrations(
        registrations: Vec<WorthQueryGraphObligationRegistration>,
    ) -> Result<Self, WorthQueryGraphObligationRegistrationDenial> {
        if registrations.is_empty() {
            return Ok(Self::empty());
        }
        let registrations = canonicalize_registrations(registrations)?;
        Ok(Self::from_canonicalized_registrations(registrations))
    }

    pub fn empty() -> Self {
        Self::from_canonicalized_registrations(Vec::new())
    }

    fn from_canonicalized_registrations(
        registrations: Vec<WorthQueryGraphObligationRegistration>,
    ) -> Self {
        let digests = registrations
            .iter()
            .map(WorthQueryGraphObligationRegistration::registration_evidence_digest)
            .collect::<Vec<_>>();
        let catalog_digest = worth_query_evidence_identity(
            WorthQueryEvidenceScope::GraphObligationRegistrationCatalog,
        )
        .field_usize(
            WorthQueryEvidenceTag::new("registration_count"),
            registrations.len(),
        )
        .field_evidence_identity_sequence(WorthQueryEvidenceTag::new("registration"), digests)
        .seal();
        Self {
            registrations,
            catalog_digest,
        }
    }

    pub fn registrations(&self) -> &[WorthQueryGraphObligationRegistration] {
        &self.registrations
    }

    pub fn registration_count(&self) -> usize {
        self.registrations.len()
    }

    pub fn catalog_digest(&self) -> &str {
        self.catalog_digest.as_str()
    }
}

fn canonicalize_registrations(
    mut registrations: Vec<WorthQueryGraphObligationRegistration>,
) -> Result<Vec<WorthQueryGraphObligationRegistration>, WorthQueryGraphObligationRegistrationDenial>
{
    registrations
        .sort_by(|left, right| left.registration_digest().cmp(right.registration_digest()));
    registrations.dedup_by(|left, right| left.registration_digest() == right.registration_digest());
    reject_conflicting_rule_registrations(&registrations)?;
    Ok(registrations)
}

fn reject_conflicting_rule_registrations(
    registrations: &[WorthQueryGraphObligationRegistration],
) -> Result<(), WorthQueryGraphObligationRegistrationDenial> {
    let mut by_registration_slot =
        BTreeMap::<String, &WorthQueryGraphObligationRegistration>::new();
    for registration in registrations {
        let slot_key = registration_slot_key(registration);
        if let Some(existing) = by_registration_slot.insert(slot_key, registration) {
            if existing != registration {
                return Err(WorthQueryGraphObligationRegistrationDenial::new(
                    WorthQueryGraphObligationRegistrationDenialKind::ConflictingRegistrationForRule,
                    format!(
                        "graph obligation rule `{}` has conflicting registrations for the same touch selector and operating world selector",
                        registration.rule_identity().domain_invariant_family()
                    ),
                ));
            }
        }
    }
    Ok(())
}

fn registration_slot_key(registration: &WorthQueryGraphObligationRegistration) -> String {
    format!(
        "{}|{}|{}",
        registration.rule_identity().identity_digest(),
        registration.touch_selector().selector_digest(),
        registration.operating_world_selector().as_str(),
    )
}
