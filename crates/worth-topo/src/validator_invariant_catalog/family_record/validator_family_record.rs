use forge_query::facade::{ForgeQueryGraphObligationKind, ForgeQueryGraphObligationSupportPosture};

use super::family_record_input::WorthTopologyLegalityFamilyRecordInput;
use crate::validator_invariant_catalog::{
    WorthTopologyDiagnosticProjectionPosture, WorthTopologyEnforcementPhase,
    WorthTopologyLegalityCatalogError, WorthTopologyRequiredAccessPosture,
    WorthTopologyTouchedApplicability, WorthTopologyValidatorFamilyIdentity,
    WorthTopologyWitnessPosture,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorthTopologyValidatorFamilyRecord {
    identity: WorthTopologyValidatorFamilyIdentity,
    query_obligation_kind: ForgeQueryGraphObligationKind,
    touched_applicability: WorthTopologyTouchedApplicability,
    required_access_posture: WorthTopologyRequiredAccessPosture,
    enforcement_phase: WorthTopologyEnforcementPhase,
    witness_posture: WorthTopologyWitnessPosture,
    diagnostic_projection: WorthTopologyDiagnosticProjectionPosture,
    query_support_posture: ForgeQueryGraphObligationSupportPosture,
    family_digest: String,
}

impl WorthTopologyValidatorFamilyRecord {
    pub(in crate::validator_invariant_catalog) fn from_input(
        input: WorthTopologyLegalityFamilyRecordInput<WorthTopologyValidatorFamilyIdentity>,
    ) -> Result<Self, WorthTopologyLegalityCatalogError> {
        let family_key = input.identity.stable_key().to_string();
        let input = input.validate(&family_key)?;
        let family_digest = family_digest(
            "validator",
            input.identity.identity_digest(),
            input.query_obligation_kind.as_str(),
            input.touched_applicability.digest_part().as_str(),
            input.required_access_posture.posture_digest(),
            input.enforcement_phase.as_str(),
            input.witness_posture.as_str(),
            input.diagnostic_projection.as_str(),
            input.query_support_posture.posture_digest(),
        );
        Ok(Self {
            identity: input.identity,
            query_obligation_kind: input.query_obligation_kind,
            touched_applicability: input.touched_applicability,
            required_access_posture: input.required_access_posture,
            enforcement_phase: input.enforcement_phase,
            witness_posture: input.witness_posture,
            diagnostic_projection: input.diagnostic_projection,
            query_support_posture: input.query_support_posture,
            family_digest,
        })
    }

    pub fn identity(&self) -> &WorthTopologyValidatorFamilyIdentity {
        &self.identity
    }

    pub const fn query_obligation_kind(&self) -> ForgeQueryGraphObligationKind {
        self.query_obligation_kind
    }

    pub const fn touched_applicability(&self) -> &WorthTopologyTouchedApplicability {
        &self.touched_applicability
    }

    pub const fn required_access_posture(&self) -> &WorthTopologyRequiredAccessPosture {
        &self.required_access_posture
    }

    pub const fn enforcement_phase(&self) -> WorthTopologyEnforcementPhase {
        self.enforcement_phase
    }

    pub const fn witness_posture(&self) -> WorthTopologyWitnessPosture {
        self.witness_posture
    }

    pub const fn diagnostic_projection(&self) -> WorthTopologyDiagnosticProjectionPosture {
        self.diagnostic_projection
    }

    pub const fn query_support_posture(&self) -> &ForgeQueryGraphObligationSupportPosture {
        &self.query_support_posture
    }

    pub fn family_digest(&self) -> &str {
        &self.family_digest
    }
}

pub(super) fn family_digest(
    family_kind: &str,
    identity_digest: &str,
    obligation_kind: &str,
    applicability: &str,
    access_posture: &str,
    enforcement_phase: &str,
    witness_posture: &str,
    diagnostic_projection: &str,
    support_posture: &str,
) -> String {
    [
        "worth-topo-legality-family-record-v1",
        family_kind,
        identity_digest,
        obligation_kind,
        applicability,
        access_posture,
        enforcement_phase,
        witness_posture,
        diagnostic_projection,
        support_posture,
    ]
    .join("|")
}
