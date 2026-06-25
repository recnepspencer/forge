use forge_query::facade::{ForgeQueryGraphObligationKind, ForgeQueryGraphObligationSupportPosture};

use crate::validator_invariant_catalog::{
    WorthTopologyDiagnosticProjectionPosture, WorthTopologyEnforcementPhase,
    WorthTopologyLegalityCatalogError, WorthTopologyRequiredAccessPosture,
    WorthTopologyTouchedApplicability, WorthTopologyWitnessPosture,
};

pub(in crate::validator_invariant_catalog) struct WorthTopologyLegalityFamilyRecordInput<I> {
    pub(in crate::validator_invariant_catalog) identity: I,
    pub(in crate::validator_invariant_catalog) query_obligation_kind: ForgeQueryGraphObligationKind,
    pub(in crate::validator_invariant_catalog) touched_applicability:
        Option<WorthTopologyTouchedApplicability>,
    pub(in crate::validator_invariant_catalog) required_access_posture:
        Option<WorthTopologyRequiredAccessPosture>,
    pub(in crate::validator_invariant_catalog) enforcement_phase:
        Option<WorthTopologyEnforcementPhase>,
    pub(in crate::validator_invariant_catalog) witness_posture: Option<WorthTopologyWitnessPosture>,
    pub(in crate::validator_invariant_catalog) diagnostic_projection:
        Option<WorthTopologyDiagnosticProjectionPosture>,
    pub(in crate::validator_invariant_catalog) query_support_posture:
        ForgeQueryGraphObligationSupportPosture,
}

pub(in crate::validator_invariant_catalog) struct ValidatedWorthTopologyLegalityFamilyRecordInput<I>
{
    pub(in crate::validator_invariant_catalog) identity: I,
    pub(in crate::validator_invariant_catalog) query_obligation_kind: ForgeQueryGraphObligationKind,
    pub(in crate::validator_invariant_catalog) touched_applicability:
        WorthTopologyTouchedApplicability,
    pub(in crate::validator_invariant_catalog) required_access_posture:
        WorthTopologyRequiredAccessPosture,
    pub(in crate::validator_invariant_catalog) enforcement_phase: WorthTopologyEnforcementPhase,
    pub(in crate::validator_invariant_catalog) witness_posture: WorthTopologyWitnessPosture,
    pub(in crate::validator_invariant_catalog) diagnostic_projection:
        WorthTopologyDiagnosticProjectionPosture,
    pub(in crate::validator_invariant_catalog) query_support_posture:
        ForgeQueryGraphObligationSupportPosture,
}

impl<I> WorthTopologyLegalityFamilyRecordInput<I> {
    pub(in crate::validator_invariant_catalog) fn validate(
        self,
        family_key: &str,
    ) -> Result<ValidatedWorthTopologyLegalityFamilyRecordInput<I>, WorthTopologyLegalityCatalogError>
    {
        Ok(ValidatedWorthTopologyLegalityFamilyRecordInput {
            identity: self.identity,
            query_obligation_kind: self.query_obligation_kind,
            touched_applicability: self.touched_applicability.ok_or_else(|| {
                WorthTopologyLegalityCatalogError::MissingTouchedApplicability(
                    family_key.to_string(),
                )
            })?,
            required_access_posture: self.required_access_posture.ok_or_else(|| {
                WorthTopologyLegalityCatalogError::MissingRequiredAccessPosture(
                    family_key.to_string(),
                )
            })?,
            enforcement_phase: self.enforcement_phase.ok_or_else(|| {
                WorthTopologyLegalityCatalogError::MissingEnforcementPhase(family_key.to_string())
            })?,
            witness_posture: self.witness_posture.ok_or_else(|| {
                WorthTopologyLegalityCatalogError::MissingWitnessPosture(family_key.to_string())
            })?,
            diagnostic_projection: self.diagnostic_projection.ok_or_else(|| {
                WorthTopologyLegalityCatalogError::MissingDiagnosticProjection(
                    family_key.to_string(),
                )
            })?,
            query_support_posture: self.query_support_posture,
        })
    }
}
