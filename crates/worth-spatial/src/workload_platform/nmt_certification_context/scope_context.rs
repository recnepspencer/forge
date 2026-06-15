use topology::facade::NmtTopologyScopeReceipt;

use super::{
    scope_set_denial, NmtCertificationDenial, NmtCertificationDenialKind, NmtCertifiedScopeSet,
    NmtScopeAttackCounters, NmtScopeMotionReceipt, NmtScopeParityReceipt,
    NmtScopeProjectionReceipt, NmtScopeRetainedReplayReceipt, NmtScopeSurfaceSupportReceipt,
};
use crate::workload_platform::transform_workload::UnsupportedTransformWorkload;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NmtScopeBoundaryIdentity {
    identity: String,
}

impl NmtScopeBoundaryIdentity {
    pub(crate) fn from_scope(scope: &NmtTopologyScopeReceipt) -> Self {
        Self {
            identity: scope.open_boundary_identity().to_string(),
        }
    }

    pub fn identity(&self) -> &str {
        &self.identity
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NmtScopePredicateBasis {
    scope_identity: String,
    boundary_identity: String,
    local_frame_identity: String,
    motion_identity: String,
    precision_policy_identity: String,
}

impl NmtScopePredicateBasis {
    pub fn scope_identity(&self) -> &str {
        &self.scope_identity
    }

    pub fn boundary_identity(&self) -> &str {
        &self.boundary_identity
    }

    pub fn local_frame_identity(&self) -> &str {
        &self.local_frame_identity
    }

    pub fn motion_identity(&self) -> &str {
        &self.motion_identity
    }

    pub fn precision_policy_identity(&self) -> &str {
        &self.precision_policy_identity
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NmtCertifiedScopeContext {
    topology_scope: NmtTopologyScopeReceipt,
    surface_support: NmtScopeSurfaceSupportReceipt,
    projection: NmtScopeProjectionReceipt,
    motion: NmtScopeMotionReceipt,
    retained_replay: NmtScopeRetainedReplayReceipt,
    parity: NmtScopeParityReceipt,
}

impl NmtCertifiedScopeContext {
    pub(crate) fn new(
        topology_scope: NmtTopologyScopeReceipt,
        surface_support: NmtScopeSurfaceSupportReceipt,
        projection: NmtScopeProjectionReceipt,
        motion: NmtScopeMotionReceipt,
        retained_replay: NmtScopeRetainedReplayReceipt,
        parity: NmtScopeParityReceipt,
    ) -> Self {
        Self {
            topology_scope,
            surface_support,
            projection,
            motion,
            retained_replay,
            parity,
        }
    }

    pub fn topology_scope(&self) -> &NmtTopologyScopeReceipt {
        &self.topology_scope
    }

    pub fn surface_support(&self) -> &NmtScopeSurfaceSupportReceipt {
        &self.surface_support
    }

    pub fn projection(&self) -> &NmtScopeProjectionReceipt {
        &self.projection
    }

    pub fn motion(&self) -> &NmtScopeMotionReceipt {
        &self.motion
    }

    pub fn retained_replay(&self) -> &NmtScopeRetainedReplayReceipt {
        &self.retained_replay
    }

    pub fn parity(&self) -> &NmtScopeParityReceipt {
        &self.parity
    }

    pub fn boundary_identity(&self) -> NmtScopeBoundaryIdentity {
        NmtScopeBoundaryIdentity::from_scope(&self.topology_scope)
    }

    pub fn predicate_basis_for_boundary(
        &self,
        boundary: &NmtScopeBoundaryIdentity,
    ) -> Result<NmtScopePredicateBasis, super::NmtCertificationDenial> {
        if boundary.identity() != self.topology_scope.open_boundary_identity() {
            return Err(super::scope_set_denial(
                super::NmtCertificationDenialKind::MissingScopeEvidence,
                Some(self.topology_scope.scope_identity()),
                None,
                Some(self.topology_scope.kind()),
                boundary.identity(),
                "NMT predicate basis requires the boundary identity from the certified scope.",
                super::NmtScopeAttackCounters::new(
                    1,
                    self.topology_scope.counters().scope_entity_count(),
                    self.projection
                        .counters()
                        .scope_projected_entities_consumed(),
                    0,
                    0,
                    1,
                ),
            ));
        }
        Ok(NmtScopePredicateBasis {
            scope_identity: self.topology_scope.scope_identity().to_string(),
            boundary_identity: boundary.identity().to_string(),
            local_frame_identity: self.projection.local_frame_identity().to_string(),
            motion_identity: self.motion.scope_motion_identity().to_string(),
            precision_policy_identity: format!(
                "nmt-scope-local-feature-scale:{}",
                self.topology_scope.scope_identity()
            ),
        })
    }
}

impl NmtCertifiedScopeSet {
    pub fn attempt_cross_scope_projection(
        &self,
        source: &NmtScopeProjectionReceipt,
        target: &NmtCertifiedScopeContext,
    ) -> Result<(), NmtCertificationDenial> {
        same_scope_or_deny(
            source.scope_identity(),
            target,
            source.scope_projection_identity(),
            NmtCertificationDenialKind::CrossScopeProjection,
            "Projection evidence from one NMT scope cannot certify another NMT scope.",
            source.counters().scope_projected_entities_consumed(),
            0,
            0,
        )
    }

    pub fn attempt_cross_scope_retained_replay(
        &self,
        source: &NmtScopeRetainedReplayReceipt,
        target: &NmtCertifiedScopeContext,
    ) -> Result<(), NmtCertificationDenial> {
        same_scope_or_deny(
            source.scope_identity(),
            target,
            source.scope_replay_identity(),
            NmtCertificationDenialKind::CrossScopeRetainedReplay,
            "Retained replay evidence from one NMT scope cannot replay another NMT scope.",
            0,
            source.counters().scope_checkpoints_consumed(),
            0,
        )
    }

    pub fn attempt_cross_scope_surface_support(
        &self,
        source: &NmtScopeSurfaceSupportReceipt,
        target: &NmtCertifiedScopeContext,
    ) -> Result<(), NmtCertificationDenial> {
        same_scope_or_deny(
            source.scope_identity(),
            target,
            source.scope_surface_support_identity(),
            NmtCertificationDenialKind::CrossScopeSurfaceSupport,
            "Surface support evidence from one NMT scope cannot certify another NMT scope.",
            source.counters().scope_face_carriers()
                + source.counters().scope_edge_carriers()
                + source.counters().scope_loop_carriers(),
            0,
            0,
        )
    }

    pub fn attempt_cross_scope_parity(
        &self,
        source: &NmtScopeParityReceipt,
        target: &NmtCertifiedScopeContext,
    ) -> Result<(), NmtCertificationDenial> {
        same_scope_or_deny(
            source.scope_identity(),
            target,
            source.parity_identity(),
            NmtCertificationDenialKind::CrossScopeParity,
            "Parity evidence from one NMT scope cannot certify another NMT scope.",
            0,
            0,
            source.counters().receipt_backed_lanes(),
        )
    }

    pub fn attempt_missing_scope_evidence(
        &self,
        target: &NmtCertifiedScopeContext,
        missing_kind: &str,
    ) -> Result<(), NmtCertificationDenial> {
        Err(scope_set_denial(
            NmtCertificationDenialKind::MissingScopeEvidence,
            Some(target.topology_scope().scope_identity()),
            None,
            Some(target.topology_scope().kind()),
            target.topology_scope().scope_identity(),
            format!(
                "{} is missing {} evidence.",
                target.topology_scope().kind().human_name(),
                missing_kind
            ),
            scope_counter(target, 1, 0, 0),
        ))
    }

    pub fn attempt_label_only_motion(
        &self,
        target: &NmtCertifiedScopeContext,
        denied_motion: &UnsupportedTransformWorkload,
    ) -> Result<(), NmtCertificationDenial> {
        Err(NmtScopeMotionReceipt::denial_from_unsupported(
            target.topology_scope(),
            denied_motion,
        ))
    }

    pub fn attempt_false_closure(
        &self,
        target: &NmtCertifiedScopeContext,
    ) -> Result<(), NmtCertificationDenial> {
        Err(scope_set_denial(
            NmtCertificationDenialKind::FalseClosure,
            Some(target.topology_scope().scope_identity()),
            None,
            Some(target.topology_scope().kind()),
            target.topology_scope().scope_identity(),
            format!(
                "{} cannot be upgraded to closed topology by aggregate stack evidence.",
                target.topology_scope().kind().human_name()
            ),
            scope_counter(target, 1, 0, 0),
        ))
    }

    pub fn attempt_storm_overlap_smuggling(
        &self,
        target: &NmtCertifiedScopeContext,
        storm_evidence_digest: &str,
    ) -> Result<(), NmtCertificationDenial> {
        Err(scope_set_denial(
            NmtCertificationDenialKind::StormOverlapSmuggling,
            Some(target.topology_scope().scope_identity()),
            None,
            Some(target.topology_scope().kind()),
            storm_evidence_digest,
            "Storm overlap extraction evidence cannot certify an NMT topology scope.",
            scope_counter(target, 1, 0, 0),
        ))
    }
}

fn same_scope_or_deny(
    source_scope_identity: &str,
    target: &NmtCertifiedScopeContext,
    evidence_digest: &str,
    kind: NmtCertificationDenialKind,
    human_reason: &str,
    projection_entities_read: usize,
    retained_checkpoints_read: usize,
    parity_lanes_read: usize,
) -> Result<(), NmtCertificationDenial> {
    if source_scope_identity == target.topology_scope().scope_identity() {
        Ok(())
    } else {
        Err(scope_set_denial(
            kind,
            Some(target.topology_scope().scope_identity()),
            Some(source_scope_identity),
            Some(target.topology_scope().kind()),
            evidence_digest,
            human_reason,
            NmtScopeAttackCounters::new(
                2,
                target.topology_scope().counters().scope_entity_count(),
                projection_entities_read,
                retained_checkpoints_read,
                parity_lanes_read,
                1,
            ),
        ))
    }
}

fn scope_counter(
    target: &NmtCertifiedScopeContext,
    projection_entities_read: usize,
    retained_checkpoints_read: usize,
    parity_lanes_read: usize,
) -> NmtScopeAttackCounters {
    NmtScopeAttackCounters::new(
        1,
        target.topology_scope().counters().scope_entity_count(),
        projection_entities_read,
        retained_checkpoints_read,
        parity_lanes_read,
        1,
    )
}
