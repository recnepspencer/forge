use topology::facade::NmtTopologyScopeReceipt;
use worth_primitives::{truth_digest_parts, TruthDigestScope};

use super::{
    NmtCertificationDenial, NmtCertificationDenialInput, NmtCertificationDenialKind,
    NmtScopeAttackCounters,
};
use crate::workload_platform::transform_workload::{
    TransformReceiptSet, UnsupportedTransformWorkload,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NmtScopeMotionCounters {
    transform_steps: usize,
    changed_coordinate_rows: usize,
    transformed_entities: usize,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NmtScopeMotionReceipt {
    parent_transform_identity: String,
    scope_identity: String,
    scope_motion_identity: String,
    transform_posture_identity: String,
    counters: NmtScopeMotionCounters,
}

impl NmtScopeMotionReceipt {
    pub(crate) fn from_transform_scope(
        transform: &TransformReceiptSet,
        scope: &NmtTopologyScopeReceipt,
    ) -> Result<Self, NmtCertificationDenial> {
        let counters = transform.counters();
        if counters.transform_steps() == 0 || counters.changed_coordinate_rows() == 0 {
            return Err(NmtCertificationDenial::new(NmtCertificationDenialInput {
                kind: NmtCertificationDenialKind::LabelOnlyMotion,
                target_scope_identity: Some(scope.scope_identity().to_string()),
                source_scope_identity: None,
                target_scope_kind: Some(scope.kind()),
                consumed_evidence_digest: transform.stage_identity().receipt_identity(),
                human_reason: format!(
                    "{} motion certification requires transform evidence that changed geometry.",
                    scope.kind().human_name()
                ),
                counters: NmtScopeAttackCounters::new(
                    1,
                    scope.counters().scope_entity_count(),
                    0,
                    0,
                    0,
                    1,
                ),
            }));
        }
        let scope_motion_identity = truth_digest_parts(
            TruthDigestScope::ArtifactIdentity,
            &[
                "nmt-scope-motion".to_string(),
                transform.stage_identity().receipt_identity(),
                scope.scope_identity().to_string(),
                transform
                    .transform_posture_receipt()
                    .posture_identity()
                    .to_string(),
            ],
        );
        Ok(Self {
            parent_transform_identity: transform.stage_identity().receipt_identity(),
            scope_identity: scope.scope_identity().to_string(),
            scope_motion_identity,
            transform_posture_identity: transform
                .transform_posture_receipt()
                .posture_identity()
                .to_string(),
            counters: NmtScopeMotionCounters {
                transform_steps: counters.transform_steps(),
                changed_coordinate_rows: counters.changed_coordinate_rows(),
                transformed_entities: counters.transformed_entities(),
            },
        })
    }

    pub(crate) fn denial_from_unsupported(
        scope: &NmtTopologyScopeReceipt,
        unsupported: &UnsupportedTransformWorkload,
    ) -> NmtCertificationDenial {
        NmtCertificationDenial::new(NmtCertificationDenialInput {
            kind: NmtCertificationDenialKind::LabelOnlyMotion,
            target_scope_identity: Some(scope.scope_identity().to_string()),
            source_scope_identity: None,
            target_scope_kind: Some(scope.kind()),
            consumed_evidence_digest: format!(
                "unsupported-transform:{:?}:{}",
                unsupported.reason_code(),
                unsupported.human_reason()
            ),
            human_reason: format!(
                "{} rejected motion before NMT certification: {}",
                scope.kind().human_name(),
                unsupported.human_reason()
            ),
            counters: NmtScopeAttackCounters::new(
                1,
                scope.counters().scope_entity_count(),
                0,
                0,
                0,
                1,
            ),
        })
    }

    pub fn parent_transform_identity(&self) -> &str {
        &self.parent_transform_identity
    }

    pub fn scope_identity(&self) -> &str {
        &self.scope_identity
    }

    pub fn scope_motion_identity(&self) -> &str {
        &self.scope_motion_identity
    }

    pub fn transform_posture_identity(&self) -> &str {
        &self.transform_posture_identity
    }

    pub fn counters(&self) -> NmtScopeMotionCounters {
        self.counters
    }
}

impl NmtScopeMotionCounters {
    pub fn transform_steps(self) -> usize {
        self.transform_steps
    }

    pub fn changed_coordinate_rows(self) -> usize {
        self.changed_coordinate_rows
    }

    pub fn transformed_entities(self) -> usize {
        self.transformed_entities
    }
}
