use topology::facade::{
    NmtTopologyConstructionCounters, NmtTopologyConstructionReceipt, NmtTopologyScopeKind,
    NmtTopologyScopeSet,
};

use super::{
    NmtCertificationDenial, NmtCertificationDenialInput, NmtCertificationDenialKind,
    NmtCertifiedScopeContext, NmtScopeAttackCounters, NmtScopeMotionReceipt, NmtScopeParityReceipt,
    NmtScopeProjectionReceipt, NmtScopeRetainedReplayReceipt, NmtScopeSurfaceSupportReceipt,
};
use crate::workload_platform::evidence_ledger::{
    CompleteWorkloadEvidenceLedger, WorkloadEvidenceStage,
};
use crate::workload_platform::geometry_binding::BoundGeometryWorkload;
use crate::workload_platform::projection_workload::ProjectedPlanarWorkload;
use crate::workload_platform::retained_replay_workload::ReplayReceiptSet;
use crate::workload_platform::surface_support::{SurfaceFamily, SurfaceSupportWorkload};
use crate::workload_platform::transform_workload::TransformReceiptSet;
use worth_primitives::{truth_digest_parts, TruthDigestScope};

pub struct NmtCertifiedScopeSetBuilder<'a> {
    topology_construction: &'a NmtTopologyConstructionReceipt,
    evidence_ledger: &'a CompleteWorkloadEvidenceLedger,
    bound_geometry: &'a BoundGeometryWorkload,
    projected: &'a ProjectedPlanarWorkload,
    transform: &'a TransformReceiptSet,
    replay: &'a ReplayReceiptSet,
    scopes: NmtTopologyScopeSet,
    surface_family: SurfaceFamily,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NmtCertifiedScopeSet {
    parent_construction_identity: String,
    topology_counters: NmtTopologyConstructionCounters,
    scopes: Vec<NmtCertifiedScopeContext>,
}

impl NmtCertifiedScopeSet {
    pub fn from_platform_evidence<'a>(
        topology_construction: &'a NmtTopologyConstructionReceipt,
        evidence_ledger: &'a CompleteWorkloadEvidenceLedger,
        bound_geometry: &'a BoundGeometryWorkload,
        projected: &'a ProjectedPlanarWorkload,
        transform: &'a TransformReceiptSet,
        replay: &'a ReplayReceiptSet,
        scopes: NmtTopologyScopeSet,
    ) -> NmtCertifiedScopeSetBuilder<'a> {
        NmtCertifiedScopeSetBuilder {
            topology_construction,
            evidence_ledger,
            bound_geometry,
            projected,
            transform,
            replay,
            scopes,
            surface_family: SurfaceFamily::Plane,
        }
    }

    pub fn parent_construction_identity(&self) -> &str {
        &self.parent_construction_identity
    }

    pub fn scopes(&self) -> &[NmtCertifiedScopeContext] {
        &self.scopes
    }

    pub fn topology_counters(&self) -> NmtTopologyConstructionCounters {
        self.topology_counters
    }

    pub fn single_scope(
        &self,
        kind: NmtTopologyScopeKind,
    ) -> Result<&NmtCertifiedScopeContext, NmtCertificationDenial> {
        let matches = self
            .scopes
            .iter()
            .filter(|context| context.topology_scope().kind() == kind)
            .collect::<Vec<_>>();
        if matches.len() == 1 {
            Ok(matches[0])
        } else {
            Err(scope_set_denial(
                NmtCertificationDenialKind::MissingScopeEvidence,
                None,
                None,
                Some(kind),
                "nmt-scope-set",
                format!(
                    "NMT certified scope set requires exactly one {}.",
                    kind.human_name()
                ),
                NmtScopeAttackCounters::new(0, 0, 0, 0, 0, 1),
            ))
        }
    }

    pub fn layer(&self, layer_index: usize) -> Option<&NmtCertifiedScopeContext> {
        self.scopes
            .iter()
            .find(|context| context.topology_scope().layer_index() == Some(layer_index))
    }

    pub fn from_certified_open_class_members(
        members: &[&NmtCertifiedScopeSet],
    ) -> Result<Self, NmtCertificationDenial> {
        if members.is_empty() {
            return Err(scope_set_denial(
                NmtCertificationDenialKind::MissingScopeEvidence,
                None,
                None,
                None,
                "nmt-open-class-members",
                "NMT open-class certification requires certified member scope sets.",
                NmtScopeAttackCounters::new(0, 0, 0, 0, 0, 1),
            ));
        }
        let mut parent_parts = vec!["nmt-certified-open-class-members".to_string()];
        let mut contexts = Vec::new();
        for member in members {
            parent_parts.push(member.parent_construction_identity().to_string());
            contexts.extend(member.scopes().iter().cloned());
        }
        if contexts.is_empty() {
            return Err(scope_set_denial(
                NmtCertificationDenialKind::MissingScopeEvidence,
                None,
                None,
                None,
                "nmt-open-class-members",
                "NMT open-class certification cannot merge empty member scope sets.",
                NmtScopeAttackCounters::new(0, 0, 0, 0, 0, 1),
            ));
        }
        let topology_counters = members[0].topology_counters();
        Ok(Self {
            parent_construction_identity: truth_digest_parts(
                TruthDigestScope::ArtifactIdentity,
                &parent_parts,
            ),
            topology_counters,
            scopes: contexts,
        })
    }
}

impl<'a> NmtCertifiedScopeSetBuilder<'a> {
    pub fn with_surface_family(mut self, surface_family: SurfaceFamily) -> Self {
        self.surface_family = surface_family;
        self
    }

    pub fn compile(self) -> Result<NmtCertifiedScopeSet, NmtCertificationDenial> {
        self.require_topology_identity_match()?;
        self.require_stage_identity(
            WorkloadEvidenceStage::Topology,
            self.topology_construction
                .topology_seed_receipt()
                .query_receipts()
                .declaration_receipt()
                .identity()
                .name(),
        )?;
        self.require_stage_identity(
            WorkloadEvidenceStage::GeometryBinding,
            &self
                .bound_geometry
                .receipts()
                .stage_identity()
                .receipt_identity(),
        )?;
        self.require_stage_identity(
            WorkloadEvidenceStage::Projection,
            &self
                .projected
                .receipts()
                .stage_identity()
                .receipt_identity(),
        )?;
        self.require_stage_identity(
            WorkloadEvidenceStage::Transform,
            &self.transform.stage_identity().receipt_identity(),
        )?;
        self.require_stage_identity(
            WorkloadEvidenceStage::RetainedReplay,
            &self.replay.stage_identity().receipt_identity(),
        )?;
        let support = SurfaceSupportWorkload::for_bound_geometry(self.bound_geometry.clone())
            .declared("NMT certified scope surface support")
            .with_surface_family(self.surface_family)
            .certify()
            .map_err(|unsupported| {
                scope_set_denial(
                    NmtCertificationDenialKind::UnsupportedSurface,
                    None,
                    None,
                    None,
                    unsupported.human_reason(),
                    unsupported.human_reason(),
                    NmtScopeAttackCounters::new(0, 0, 0, 0, 0, 1),
                )
            })?;
        let scopes = self
            .scopes
            .scopes()
            .iter()
            .map(|scope| {
                let surface = NmtScopeSurfaceSupportReceipt::from_certified_surface_scope(
                    &support,
                    scope,
                    self.surface_family,
                )?;
                let projection = NmtScopeProjectionReceipt::from_projected_workload_scope(
                    self.projected,
                    scope,
                )?;
                let motion = NmtScopeMotionReceipt::from_transform_scope(self.transform, scope)?;
                let retained = NmtScopeRetainedReplayReceipt::from_replay_scope(
                    self.replay,
                    scope,
                    &projection,
                )?;
                let parity = NmtScopeParityReceipt::from_scope_receipts(
                    scope,
                    &projection,
                    &retained,
                    &motion,
                );
                Ok(NmtCertifiedScopeContext::new(
                    scope.clone(),
                    surface,
                    projection,
                    motion,
                    retained,
                    parity,
                ))
            })
            .collect::<Result<Vec<_>, _>>()?;
        Ok(NmtCertifiedScopeSet {
            parent_construction_identity: self.scopes.parent_construction_identity().to_string(),
            topology_counters: self.topology_construction.counters(),
            scopes,
        })
    }

    fn require_topology_identity_match(&self) -> Result<(), NmtCertificationDenial> {
        if self.scopes.parent_construction_identity()
            == self
                .topology_construction
                .pattern_identity()
                .identity_digest()
        {
            Ok(())
        } else {
            Err(scope_set_denial(
                NmtCertificationDenialKind::MismatchedTopologyConstruction,
                None,
                None,
                None,
                self.scopes.parent_construction_identity(),
                "NMT topology scopes must come from the same construction receipt as the workload.",
                NmtScopeAttackCounters::new(0, 0, 0, 0, 0, 1),
            ))
        }
    }

    fn require_stage_identity(
        &self,
        stage: WorkloadEvidenceStage,
        expected_identity: &str,
    ) -> Result<(), NmtCertificationDenial> {
        let Some(row) = self.evidence_ledger.row_for_stage(stage) else {
            return Err(scope_set_denial(
                NmtCertificationDenialKind::MissingReceiptBackedStage,
                None,
                None,
                None,
                expected_identity,
                format!("NMT scope certification is missing {}.", stage.human_name()),
                NmtScopeAttackCounters::new(0, 0, 0, 0, 0, 1),
            ));
        };
        if row.is_receipt_backed()
            && row.is_admitted()
            && row.evidence_identity() == expected_identity
        {
            Ok(())
        } else {
            Err(scope_set_denial(
                NmtCertificationDenialKind::MissingReceiptBackedStage,
                None,
                None,
                None,
                row.evidence_identity(),
                format!(
                    "NMT scope certification requires receipt-backed admitted {} matching the consumed workload receipt.",
                    stage.human_name()
                ),
                NmtScopeAttackCounters::new(0, 0, 0, 0, 0, 1),
            ))
        }
    }
}

pub(crate) fn scope_set_denial(
    kind: NmtCertificationDenialKind,
    target_scope_identity: Option<&str>,
    source_scope_identity: Option<&str>,
    target_scope_kind: Option<NmtTopologyScopeKind>,
    consumed_evidence_digest: &str,
    human_reason: impl Into<String>,
    counters: NmtScopeAttackCounters,
) -> NmtCertificationDenial {
    NmtCertificationDenial::new(NmtCertificationDenialInput {
        kind,
        target_scope_identity: target_scope_identity.map(str::to_string),
        source_scope_identity: source_scope_identity.map(str::to_string),
        target_scope_kind,
        consumed_evidence_digest: consumed_evidence_digest.to_string(),
        human_reason: human_reason.into(),
        counters,
    })
}
