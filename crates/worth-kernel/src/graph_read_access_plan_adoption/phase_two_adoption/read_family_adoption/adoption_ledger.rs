use crate::graph_read_access_declarations::{
    WorthGraphReadAdmissionCapabilityGap, WorthGraphReadDeclarationReadFamilyIdentity,
    WorthGraphReadRequirementRowDigestProjection,
};

use super::super::errors::{
    WorthGraphReadAccessPlanAdoptionPhaseTwoError,
    WorthGraphReadAccessPlanAdoptionPhaseTwoErrorKind,
};
use super::super::query_admission::{
    WorthGraphReadAccessPlanAdoptionAdmissionInput, WorthGraphReadAccessPlanAdoptionAttempt,
};
use super::super::stable_digest;
use super::carried_gap_projection::WorthGraphReadAccessPlanAdoptionCarriedGapRow;
use super::pairing_uniqueness::reject_duplicate_pairings;
use super::structured_seed_pairing::WorthGraphReadAccessPlanAdoptionSeedPairing;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthGraphReadAccessPlanAdoptionLedger {
    pairings: Vec<WorthGraphReadAccessPlanAdoptionSeedPairing>,
    attempts: Vec<WorthGraphReadAccessPlanAdoptionAttempt>,
    carried_capability_gaps: Vec<WorthGraphReadAccessPlanAdoptionCarriedGapRow>,
    ledger_digest: String,
}

impl WorthGraphReadAccessPlanAdoptionLedger {
    pub(crate) fn from_phase_one_closeout(
        milestone_seven_closeout_digest: &str,
        declaration_catalog_digest: &str,
        read_family_identities: &[WorthGraphReadDeclarationReadFamilyIdentity],
        requirement_rows: &[WorthGraphReadRequirementRowDigestProjection],
        carried_capability_gaps: &[WorthGraphReadAdmissionCapabilityGap],
    ) -> Result<Self, WorthGraphReadAccessPlanAdoptionPhaseTwoError> {
        if read_family_identities.is_empty() {
            return Err(error(
                WorthGraphReadAccessPlanAdoptionPhaseTwoErrorKind::MissingReadFamilyIdentity,
            ));
        }
        if requirement_rows.is_empty() {
            return Err(error(
                WorthGraphReadAccessPlanAdoptionPhaseTwoErrorKind::MissingRequirementRowEvidence,
            ));
        }

        let pairings = read_family_identities
            .iter()
            .flat_map(|identity| {
                requirement_rows.iter().filter_map(move |requirement_row| {
                    WorthGraphReadAccessPlanAdoptionSeedPairing::from_seed_rows(
                        milestone_seven_closeout_digest,
                        declaration_catalog_digest,
                        identity,
                        requirement_row,
                    )
                })
            })
            .collect::<Vec<_>>();

        if pairings.is_empty() {
            return Err(error(
                WorthGraphReadAccessPlanAdoptionPhaseTwoErrorKind::MissingStructuredSeedPairing,
            ));
        }
        reject_duplicate_pairings(&pairings)?;

        let attempts = pairings
            .iter()
            .map(|pairing| {
                WorthGraphReadAccessPlanAdoptionAttempt::missing_query_read_family_artifact(
                    WorthGraphReadAccessPlanAdoptionAdmissionInput::missing_query_read_family_artifact(pairing),
                )
            })
            .collect::<Vec<_>>();
        let carried_capability_gaps = carried_capability_gaps
            .iter()
            .map(WorthGraphReadAccessPlanAdoptionCarriedGapRow::from_admission_gap)
            .collect::<Vec<_>>();

        let mut digest_parts = vec![
            "worth_graph_read_access_plan_adoption_ledger_v1".to_string(),
            format!("closeout:{milestone_seven_closeout_digest}"),
            format!("catalog:{declaration_catalog_digest}"),
            format!("pairing_count:{}", pairings.len()),
            format!("attempt_count:{}", attempts.len()),
            format!(
                "carried_capability_gap_count:{}",
                carried_capability_gaps.len()
            ),
        ];
        digest_parts.extend(
            pairings
                .iter()
                .map(|pairing| format!("pairing:{}", pairing.pairing_digest())),
        );
        digest_parts.extend(
            attempts
                .iter()
                .map(|attempt| format!("attempt:{}", attempt.attempt_digest())),
        );
        digest_parts.extend(
            carried_capability_gaps
                .iter()
                .map(|gap| format!("carried_gap:{}", gap.row_digest())),
        );

        Ok(Self {
            pairings,
            attempts,
            carried_capability_gaps,
            ledger_digest: stable_digest(&digest_parts),
        })
    }

    #[cfg(test)]
    pub(crate) fn from_attempts_for_posture_matrix_tests(
        attempts: Vec<WorthGraphReadAccessPlanAdoptionAttempt>,
        carried_capability_gaps: Vec<WorthGraphReadAccessPlanAdoptionCarriedGapRow>,
    ) -> Self {
        let mut digest_parts = vec![
            "worth_graph_read_access_plan_adoption_ledger_v1".to_string(),
            "closeout:phase-one-posture-matrix-test".to_string(),
            "catalog:declaration-catalog-posture-matrix-test".to_string(),
            "pairing_count:0".to_string(),
            format!("attempt_count:{}", attempts.len()),
            format!(
                "carried_capability_gap_count:{}",
                carried_capability_gaps.len()
            ),
        ];
        digest_parts.extend(
            attempts
                .iter()
                .map(|attempt| format!("attempt:{}", attempt.attempt_digest())),
        );
        digest_parts.extend(
            carried_capability_gaps
                .iter()
                .map(|gap| format!("carried_gap:{}", gap.row_digest())),
        );

        Self {
            pairings: Vec::new(),
            attempts,
            carried_capability_gaps,
            ledger_digest: stable_digest(&digest_parts),
        }
    }

    pub fn pairings(&self) -> &[WorthGraphReadAccessPlanAdoptionSeedPairing] {
        &self.pairings
    }

    pub fn attempts(&self) -> &[WorthGraphReadAccessPlanAdoptionAttempt] {
        &self.attempts
    }

    pub const fn carried_capability_gap_count(&self) -> usize {
        self.carried_capability_gaps.len()
    }

    pub fn carried_capability_gaps(&self) -> &[WorthGraphReadAccessPlanAdoptionCarriedGapRow] {
        &self.carried_capability_gaps
    }

    pub const fn duplicate_pairing_count(&self) -> usize {
        0
    }

    pub fn ledger_digest(&self) -> &str {
        &self.ledger_digest
    }
}

const fn error(
    kind: WorthGraphReadAccessPlanAdoptionPhaseTwoErrorKind,
) -> WorthGraphReadAccessPlanAdoptionPhaseTwoError {
    WorthGraphReadAccessPlanAdoptionPhaseTwoError::new(kind)
}
