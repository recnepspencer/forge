use crate::workload_platform::evidence_lookup_family_catalog::{
    EvidenceLookupEvidenceClassSet, EvidenceLookupFamilyDeclaration,
};
use crate::workload_platform::evidence_lookup_query_surface_contract::EvidenceLookupQuerySurfaceContract;

use super::query_posture::EvidenceLookupPlanQueryPosture;
use super::strategy::EvidenceLookupSelectedStrategy;
use super::topology_posture::EvidenceLookupPlanTopologyPosture;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EvidenceLookupPlanRowOutcome {
    Selected,
    Unaffected,
    RequiredQueryPosture,
    Denied,
    CappedResidue,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EvidenceLookupSelectedPlanRow {
    family_identity: String,
    family_declaration_digest: String,
    spatial_touch_digest: String,
    stage_receipt_digest: String,
    evidence_classes: EvidenceLookupEvidenceClassSet,
    topology_posture: EvidenceLookupPlanTopologyPosture,
    query_posture: EvidenceLookupPlanQueryPosture,
    query_surface_contract: Option<EvidenceLookupQuerySurfaceContract>,
    strategy: Option<EvidenceLookupSelectedStrategy>,
    outcome: EvidenceLookupPlanRowOutcome,
    row_digest: String,
}

pub(crate) struct EvidenceLookupSelectedPlanRowParts {
    pub(crate) family: EvidenceLookupFamilyDeclaration,
    pub(crate) spatial_touch_digest: String,
    pub(crate) stage_receipt_digest: String,
    pub(crate) topology_posture: EvidenceLookupPlanTopologyPosture,
    pub(crate) query_posture: EvidenceLookupPlanQueryPosture,
    pub(crate) strategy: Option<EvidenceLookupSelectedStrategy>,
    pub(crate) outcome: EvidenceLookupPlanRowOutcome,
}

impl EvidenceLookupSelectedPlanRow {
    pub(crate) fn from_parts(parts: EvidenceLookupSelectedPlanRowParts) -> Self {
        let row_digest = row_digest(&parts);
        Self {
            family_identity: parts.family.identity().as_str().to_string(),
            family_declaration_digest: parts.family.declaration_digest().to_string(),
            spatial_touch_digest: parts.spatial_touch_digest,
            stage_receipt_digest: parts.stage_receipt_digest,
            evidence_classes: parts.family.evidence_classes().clone(),
            topology_posture: parts.topology_posture,
            query_posture: parts.query_posture,
            query_surface_contract: EvidenceLookupQuerySurfaceContract::from_family_query_posture(
                parts.family.query_posture(),
            ),
            strategy: parts.strategy,
            outcome: parts.outcome,
            row_digest,
        }
    }

    pub fn family_identity(&self) -> &str {
        &self.family_identity
    }

    pub fn family_declaration_digest(&self) -> &str {
        &self.family_declaration_digest
    }

    pub fn spatial_touch_digest(&self) -> &str {
        &self.spatial_touch_digest
    }

    pub fn stage_receipt_digest(&self) -> &str {
        &self.stage_receipt_digest
    }

    pub const fn evidence_classes(&self) -> &EvidenceLookupEvidenceClassSet {
        &self.evidence_classes
    }

    pub const fn topology_posture(&self) -> &EvidenceLookupPlanTopologyPosture {
        &self.topology_posture
    }

    pub const fn query_posture(&self) -> &EvidenceLookupPlanQueryPosture {
        &self.query_posture
    }

    pub const fn query_surface_contract(&self) -> Option<&EvidenceLookupQuerySurfaceContract> {
        self.query_surface_contract.as_ref()
    }

    pub const fn strategy(&self) -> Option<&EvidenceLookupSelectedStrategy> {
        self.strategy.as_ref()
    }

    pub const fn outcome(&self) -> EvidenceLookupPlanRowOutcome {
        self.outcome
    }

    pub fn row_digest(&self) -> &str {
        &self.row_digest
    }
}

fn row_digest(parts: &EvidenceLookupSelectedPlanRowParts) -> String {
    let strategy_part = parts
        .strategy
        .as_ref()
        .map(|strategy| format!("strategy:{:?}", strategy.kind()))
        .unwrap_or_else(|| "strategy:none".to_string());
    worth_primitives::truth_digest_parts(
        worth_primitives::TruthDigestScope::ArtifactIdentity,
        &[
            "worth-spatial:evidence-lookup-selected-plan-row:v1".to_string(),
            format!("family:{}", parts.family.identity().digest()),
            format!("declaration:{}", parts.family.declaration_digest()),
            format!("spatial-touch:{}", parts.spatial_touch_digest),
            format!("stage-receipt:{}", parts.stage_receipt_digest),
            parts.topology_posture.digest_part(),
            parts.query_posture.digest_part(),
            strategy_part,
            format!("outcome:{:?}", parts.outcome),
        ],
    )
}
