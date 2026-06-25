use forge_query::facade::{
    ForgeQueryGraphObligationKind, ForgeQueryGraphObligationSupportLane,
    ForgeQueryGraphObligationSupportPosture,
};

use crate::validator_invariant_catalog::family_record::WorthTopologyLegalityFamilyRecordInput;
use crate::validator_invariant_catalog::source_catalog::{
    WorthTopologyLegalityFamilySourceAuthorityKind, WorthTopologyLegalityFamilySourceProofInput,
};
use crate::validator_invariant_catalog::{
    WorthTopologyDiagnosticProjectionPosture, WorthTopologyEnforcementPhase,
    WorthTopologyInvariantFamilyIdentity, WorthTopologyInvariantFamilyRecord,
    WorthTopologyLegalityCatalog, WorthTopologyLegalityCatalogCloseout,
    WorthTopologyLegalityCatalogError, WorthTopologyLegalityFamilyRecord,
    WorthTopologyLegalityFamilySourceProof, WorthTopologyRequiredAccessPosture,
    WorthTopologyTouchedApplicability, WorthTopologyWitnessPosture,
};

#[derive(Clone)]
pub(crate) struct WorthTopologyLegalityTestFamilyRow {
    name: String,
    semantic_version: String,
    query_obligation_kind: ForgeQueryGraphObligationKind,
    touched_applicability: WorthTopologyTouchedApplicability,
    support_posture: ForgeQueryGraphObligationSupportPosture,
}

impl WorthTopologyLegalityTestFamilyRow {
    pub(crate) fn invariant(
        name: impl Into<String>,
        touched_applicability: WorthTopologyTouchedApplicability,
    ) -> Self {
        Self {
            name: name.into(),
            semantic_version: "v1".to_string(),
            query_obligation_kind: ForgeQueryGraphObligationKind::BlockingInvariant,
            touched_applicability,
            support_posture: ForgeQueryGraphObligationSupportPosture::supported(
                ForgeQueryGraphObligationSupportLane::WorthTopoOperatorCatalog,
            ),
        }
    }

    pub(crate) fn with_support_posture(
        mut self,
        support_posture: ForgeQueryGraphObligationSupportPosture,
    ) -> Self {
        self.support_posture = support_posture;
        self
    }

    fn into_record_and_source_proof(
        self,
    ) -> Result<
        (
            WorthTopologyLegalityFamilyRecord,
            WorthTopologyLegalityFamilySourceProof,
        ),
        WorthTopologyLegalityCatalogError,
    > {
        let identity = WorthTopologyInvariantFamilyIdentity::registered(
            self.name.clone(),
            self.semantic_version.clone(),
        );
        let input = WorthTopologyLegalityFamilyRecordInput {
            identity: identity.clone(),
            query_obligation_kind: self.query_obligation_kind,
            touched_applicability: Some(self.touched_applicability.clone()),
            required_access_posture: Some(
                WorthTopologyRequiredAccessPosture::milestone_eight_receipt_backed(
                    "test-milestone-eight-posture",
                ),
            ),
            enforcement_phase: Some(WorthTopologyEnforcementPhase::SelectedObligationExecution),
            witness_posture: Some(WorthTopologyWitnessPosture::TouchedNeighborhood),
            diagnostic_projection: Some(WorthTopologyDiagnosticProjectionPosture::ViolationWitness),
            query_support_posture: self.support_posture,
        };
        let source_proof = WorthTopologyLegalityFamilySourceProof::new(
            WorthTopologyLegalityFamilySourceProofInput {
                authority_kind:
                    WorthTopologyLegalityFamilySourceAuthorityKind::RuntimeInvariantRegistration,
                source_identity_digest: identity.identity_digest().to_string(),
                rule_name: identity.name().to_string(),
                semantic_version: identity.semantic_version().to_string(),
                execution_point: Some("phase-three-test-support".to_string()),
                applicability_digest: self.touched_applicability.digest_part(),
                enforcement_phase: WorthTopologyEnforcementPhase::SelectedObligationExecution,
                witness_posture: WorthTopologyWitnessPosture::TouchedNeighborhood,
            },
        );
        let record = WorthTopologyLegalityFamilyRecord::Invariant(
            WorthTopologyInvariantFamilyRecord::from_input(input)?,
        );
        Ok((record, source_proof))
    }
}

pub(crate) fn catalog_closeout_from_test_family_rows(
    rows: impl IntoIterator<Item = WorthTopologyLegalityTestFamilyRow>,
) -> Result<WorthTopologyLegalityCatalogCloseout, WorthTopologyLegalityCatalogError> {
    let mut records = Vec::new();
    let mut source_proofs = Vec::new();
    for row in rows {
        let (record, source_proof) = row.into_record_and_source_proof()?;
        records.push(record);
        source_proofs.push(source_proof);
    }
    let catalog = WorthTopologyLegalityCatalog::from_test_family_records(records, source_proofs)?;
    Ok(WorthTopologyLegalityCatalogCloseout::from_test_catalog(
        catalog,
    ))
}
