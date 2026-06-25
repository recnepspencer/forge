use sha2::{Digest, Sha256};

use crate::graph_read_access_inventory::{
    WorthGraphReadAccessInventoryRowContext, WorthGraphReadDeclarationCandidate,
    WorthGraphReadRequirementVocabulary,
};

use super::super::touched_authority_lowering::{
    lower_touched_authority_for_catalog_candidate, WorthGraphReadLoweredTouchedAuthority,
};
use super::catalog_dimensions::{
    WorthGraphReadCatalogAccessShape, WorthGraphReadCatalogBasisSnapshotPosture,
    WorthGraphReadCatalogMilestoneEightAdoptionTarget, WorthGraphReadCatalogPolicyTenantPosture,
    WorthGraphReadCatalogSelectivityPosture, WorthGraphReadCatalogSupportPosture,
};
use super::errors::{
    WorthGraphReadAccessDeclarationPhaseTwoError, WorthGraphReadAccessDeclarationPhaseTwoErrorKind,
};

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct WorthGraphReadDeclarationCatalogKey {
    touched_authority_input: String,
    read_family_target: String,
    access_shape: WorthGraphReadCatalogAccessShape,
    selectivity_posture: WorthGraphReadCatalogSelectivityPosture,
    basis_snapshot_posture: WorthGraphReadCatalogBasisSnapshotPosture,
    lowered_authority: WorthGraphReadLoweredTouchedAuthority,
    policy_tenant_posture: WorthGraphReadCatalogPolicyTenantPosture,
    requirement_evidence_digest: String,
    support_posture: WorthGraphReadCatalogSupportPosture,
    milestone_eight_adoption_target: WorthGraphReadCatalogMilestoneEightAdoptionTarget,
    lowering_target: String,
    key_digest: String,
}

impl WorthGraphReadDeclarationCatalogKey {
    pub(crate) fn from_candidate(
        candidate: &WorthGraphReadDeclarationCandidate,
    ) -> Result<Self, WorthGraphReadAccessDeclarationPhaseTwoError> {
        let lowered_authority = lower_touched_authority_for_catalog_candidate(candidate)
            .map_err(|lowering_error| {
                WorthGraphReadAccessDeclarationPhaseTwoError::touched_authority_lowering_failed(
                    lowering_error.kind(),
                )
            })?
            .lowered_authority()
            .clone();
        Self::new(CatalogKeyParts {
            touched_authority_input: candidate.touched_authority_input().to_string(),
            read_family_target: candidate.read_family_target().as_str().to_string(),
            access_shape: WorthGraphReadCatalogAccessShape::from_target(
                candidate.read_family_target(),
            ),
            selectivity_posture: WorthGraphReadCatalogSelectivityPosture::from_context(
                candidate.inventory_row_context(),
            ),
            basis_snapshot_posture: WorthGraphReadCatalogBasisSnapshotPosture::from_vocabulary(
                candidate.requirement_vocabulary(),
            ),
            lowered_authority,
            policy_tenant_posture: WorthGraphReadCatalogPolicyTenantPosture::from_context(
                candidate.inventory_row_context(),
            ),
            requirement_evidence_digest: requirement_evidence_digest(
                candidate.requirement_vocabulary(),
                candidate.inventory_row_context(),
            )?,
            support_posture: WorthGraphReadCatalogSupportPosture::from_context(
                candidate.inventory_row_context(),
            ),
            milestone_eight_adoption_target:
                WorthGraphReadCatalogMilestoneEightAdoptionTarget::from_target(
                    candidate.read_family_target(),
                ),
            lowering_target: candidate.milestone_seven_lowering_target().to_string(),
        })
    }

    fn new(parts: CatalogKeyParts) -> Result<Self, WorthGraphReadAccessDeclarationPhaseTwoError> {
        Self::require_complete_key_parts(&parts)?;
        let key_digest = stable_digest(&[
            "worth_graph_read_declaration_catalog_key_v1".to_string(),
            format!("touched_authority:{}", parts.touched_authority_input),
            format!("read_family_target:{}", parts.read_family_target),
            format!("access_shape:{}", parts.access_shape.digest_part()),
            format!(
                "selectivity_posture:{}",
                parts.selectivity_posture.digest_part()
            ),
            format!(
                "basis_snapshot_posture:{}",
                parts.basis_snapshot_posture.digest_part()
            ),
            format!(
                "source_family:{}",
                parts.lowered_authority.source_family_label()
            ),
            format!(
                "query_touch_descriptor:{}",
                parts.lowered_authority.query_touch_descriptor_digest()
            ),
            format!(
                "query_touch_collection:{}",
                parts.lowered_authority.query_touch_collection_label()
            ),
            format!(
                "query_touch_read_verbs:{}",
                parts.lowered_authority.query_touch_read_verb_digest()
            ),
            format!(
                "operating_world:{}",
                parts.lowered_authority.operating_world_digest()
            ),
            format!(
                "policy_tenant_posture:{}",
                parts.policy_tenant_posture.digest_part()
            ),
            format!("requirement_evidence:{}", parts.requirement_evidence_digest),
            format!("support_posture:{}", parts.support_posture.digest_part()),
            format!(
                "milestone_eight_adoption_target:{}",
                parts.milestone_eight_adoption_target.digest_part()
            ),
            format!("lowering_target:{}", parts.lowering_target),
        ]);

        Ok(Self {
            touched_authority_input: parts.touched_authority_input,
            read_family_target: parts.read_family_target,
            access_shape: parts.access_shape,
            selectivity_posture: parts.selectivity_posture,
            basis_snapshot_posture: parts.basis_snapshot_posture,
            lowered_authority: parts.lowered_authority,
            policy_tenant_posture: parts.policy_tenant_posture,
            requirement_evidence_digest: parts.requirement_evidence_digest,
            support_posture: parts.support_posture,
            milestone_eight_adoption_target: parts.milestone_eight_adoption_target,
            lowering_target: parts.lowering_target,
            key_digest,
        })
    }

    fn require_complete_key_parts(
        parts: &CatalogKeyParts,
    ) -> Result<(), WorthGraphReadAccessDeclarationPhaseTwoError> {
        require_non_empty(
            &parts.touched_authority_input,
            WorthGraphReadAccessDeclarationPhaseTwoErrorKind::MissingTouchedAuthority,
        )?;
        require_non_empty(
            &parts.read_family_target,
            WorthGraphReadAccessDeclarationPhaseTwoErrorKind::MissingReadFamilyTarget,
        )?;
        require_non_empty(
            &parts.requirement_evidence_digest,
            WorthGraphReadAccessDeclarationPhaseTwoErrorKind::MissingRequirementEvidence,
        )?;
        require_non_empty(
            parts.support_posture.digest_part(),
            WorthGraphReadAccessDeclarationPhaseTwoErrorKind::MissingSupportPosture,
        )?;
        require_non_empty(
            parts.milestone_eight_adoption_target.digest_part(),
            WorthGraphReadAccessDeclarationPhaseTwoErrorKind::MissingMilestoneEightAdoptionTarget,
        )?;
        require_non_empty(
            &parts.lowering_target,
            WorthGraphReadAccessDeclarationPhaseTwoErrorKind::MissingLoweringTarget,
        )
    }

    pub fn touched_authority_input(&self) -> &str {
        &self.touched_authority_input
    }

    pub fn read_family_target(&self) -> &str {
        &self.read_family_target
    }

    pub fn access_shape(&self) -> &str {
        self.access_shape.digest_part()
    }

    pub fn requirement_evidence_digest(&self) -> &str {
        &self.requirement_evidence_digest
    }

    pub fn key_digest(&self) -> &str {
        &self.key_digest
    }

    pub fn has_complete_declaration_dimensions(&self) -> bool {
        !self.touched_authority_input.is_empty()
            && !self.read_family_target.is_empty()
            && !self.access_shape.digest_part().is_empty()
            && !self.selectivity_posture.digest_part().is_empty()
            && !self.basis_snapshot_posture.digest_part().is_empty()
            && !self.lowered_authority.source_family_label().is_empty()
            && !self
                .lowered_authority
                .query_touch_descriptor_digest()
                .is_empty()
            && !self.lowered_authority.operating_world_digest().is_empty()
            && !self.policy_tenant_posture.digest_part().is_empty()
            && !self.requirement_evidence_digest.is_empty()
            && !self.support_posture.digest_part().is_empty()
            && !self
                .milestone_eight_adoption_target
                .digest_part()
                .is_empty()
            && !self.lowering_target.is_empty()
            && !self.key_digest.is_empty()
    }

    pub(crate) fn conflict_identity(&self) -> String {
        stable_digest(&[
            "worth_graph_read_declaration_catalog_conflict_identity_v1".to_string(),
            format!("touched_authority:{}", self.touched_authority_input),
            format!("read_family_target:{}", self.read_family_target),
            format!("access_shape:{}", self.access_shape.digest_part()),
        ])
    }

    pub fn lowered_authority(&self) -> &WorthGraphReadLoweredTouchedAuthority {
        &self.lowered_authority
    }

    pub fn query_touch_descriptor_digest(&self) -> &str {
        self.lowered_authority.query_touch_descriptor_digest()
    }

    pub fn operating_world_digest(&self) -> &str {
        self.lowered_authority.operating_world_digest()
    }
}

struct CatalogKeyParts {
    touched_authority_input: String,
    read_family_target: String,
    access_shape: WorthGraphReadCatalogAccessShape,
    selectivity_posture: WorthGraphReadCatalogSelectivityPosture,
    basis_snapshot_posture: WorthGraphReadCatalogBasisSnapshotPosture,
    lowered_authority: WorthGraphReadLoweredTouchedAuthority,
    policy_tenant_posture: WorthGraphReadCatalogPolicyTenantPosture,
    requirement_evidence_digest: String,
    support_posture: WorthGraphReadCatalogSupportPosture,
    milestone_eight_adoption_target: WorthGraphReadCatalogMilestoneEightAdoptionTarget,
    lowering_target: String,
}

fn requirement_evidence_digest(
    vocabulary: &WorthGraphReadRequirementVocabulary,
    context: &WorthGraphReadAccessInventoryRowContext,
) -> Result<String, WorthGraphReadAccessDeclarationPhaseTwoError> {
    if vocabulary.requirement_kinds().is_empty() {
        return Err(error(
            WorthGraphReadAccessDeclarationPhaseTwoErrorKind::MissingRequirementEvidence,
        ));
    }

    let mut parts = vec![
        "worth_graph_read_declaration_requirement_evidence_v1".to_string(),
        format!("rebuild_basis:{:?}", vocabulary.rebuild_basis()),
        format!("invalidation_basis:{:?}", vocabulary.invalidation_basis()),
        format!("complexity_contract:{:?}", vocabulary.complexity_contract()),
        format!(
            "memory_estimate_basis:{:?}",
            vocabulary.memory_estimate_basis()
        ),
        format!("cost_posture:{:?}", context.cost_posture()),
    ];
    parts.extend(
        vocabulary
            .requirement_kinds()
            .iter()
            .map(|kind| format!("requirement_kind:{kind:?}")),
    );
    Ok(stable_digest(&parts))
}

fn require_non_empty(
    value: &str,
    kind: WorthGraphReadAccessDeclarationPhaseTwoErrorKind,
) -> Result<(), WorthGraphReadAccessDeclarationPhaseTwoError> {
    if value.is_empty() {
        return Err(error(kind));
    }
    Ok(())
}

pub(crate) fn stable_digest(parts: &[String]) -> String {
    let mut hasher = Sha256::new();
    for part in parts {
        hasher.update(part.as_bytes());
        hasher.update([0]);
    }
    format!("{:x}", hasher.finalize())
}

const fn error(
    kind: WorthGraphReadAccessDeclarationPhaseTwoErrorKind,
) -> WorthGraphReadAccessDeclarationPhaseTwoError {
    WorthGraphReadAccessDeclarationPhaseTwoError::new(kind)
}
