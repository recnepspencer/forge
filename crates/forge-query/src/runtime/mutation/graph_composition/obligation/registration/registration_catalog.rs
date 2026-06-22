use std::collections::BTreeMap;

use crate::evidence_identity::{
    forge_query_evidence_identity, ForgeQueryEvidenceIdentity, ForgeQueryEvidenceScope,
    ForgeQueryEvidenceTag,
};

use super::registration::ForgeQueryGraphObligationRegistration;
use super::registration_denial::{
    ForgeQueryGraphObligationRegistrationDenial, ForgeQueryGraphObligationRegistrationDenialKind,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryGraphObligationRegistrationCatalog {
    registrations: Vec<ForgeQueryGraphObligationRegistration>,
    catalog_digest: ForgeQueryEvidenceIdentity,
}

impl ForgeQueryGraphObligationRegistrationCatalog {
    pub fn from_registrations(
        registrations: Vec<ForgeQueryGraphObligationRegistration>,
    ) -> Result<Self, ForgeQueryGraphObligationRegistrationDenial> {
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
        registrations: Vec<ForgeQueryGraphObligationRegistration>,
    ) -> Self {
        let digests = registrations
            .iter()
            .map(ForgeQueryGraphObligationRegistration::registration_evidence_digest)
            .collect::<Vec<_>>();
        let catalog_digest = forge_query_evidence_identity(
            ForgeQueryEvidenceScope::GraphObligationRegistrationCatalog,
        )
        .field_usize(
            ForgeQueryEvidenceTag::new("registration_count"),
            registrations.len(),
        )
        .field_evidence_identity_sequence(ForgeQueryEvidenceTag::new("registration"), digests)
        .seal();
        Self {
            registrations,
            catalog_digest,
        }
    }

    pub fn registrations(&self) -> &[ForgeQueryGraphObligationRegistration] {
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
    mut registrations: Vec<ForgeQueryGraphObligationRegistration>,
) -> Result<Vec<ForgeQueryGraphObligationRegistration>, ForgeQueryGraphObligationRegistrationDenial>
{
    registrations
        .sort_by(|left, right| left.registration_digest().cmp(right.registration_digest()));
    registrations.dedup_by(|left, right| left.registration_digest() == right.registration_digest());
    reject_conflicting_rule_registrations(&registrations)?;
    Ok(registrations)
}

fn reject_conflicting_rule_registrations(
    registrations: &[ForgeQueryGraphObligationRegistration],
) -> Result<(), ForgeQueryGraphObligationRegistrationDenial> {
    let mut by_registration_slot =
        BTreeMap::<String, &ForgeQueryGraphObligationRegistration>::new();
    for registration in registrations {
        let slot_key = registration_slot_key(registration);
        if let Some(existing) = by_registration_slot.insert(slot_key, registration) {
            if existing != registration {
                return Err(ForgeQueryGraphObligationRegistrationDenial::new(
                    ForgeQueryGraphObligationRegistrationDenialKind::ConflictingRegistrationForRule,
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

fn registration_slot_key(registration: &ForgeQueryGraphObligationRegistration) -> String {
    format!(
        "{}|{}|{}",
        registration.rule_identity().identity_digest(),
        registration.touch_selector().selector_digest(),
        registration.operating_world_selector().as_str(),
    )
}
