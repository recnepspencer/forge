use crate::bindings::query_native_rebinding_neighborhood_replacement::TopologyNeighborhoodReplacementFactReceipt;
use crate::planar_contracts::motion_posture::PlanarMotionPostureReceipt;
use crate::planar_contracts::planar_diagnostics::PlanarDiagnosticBundleReceipt;
use crate::planar_contracts::planar_recovery::PlanarRecoveryPostureReceipt;
use crate::planar_contracts::projection_consumed_facts::ProjectionConsumedPlanarFactsReceipt;
use crate::planar_contracts::retained_planar_facts::RetainedPlanarFactsReceipt;
use crate::planar_contracts::structural_identity::PlanarStructuralIdentityReceipt;
use crate::planar_contracts::topology_contract_completeness::PlanarTopologyContractCompletenessReceipt;
use std::sync::Arc;

use super::validation::validate_planar_local_rebuild_parity_basis;
use super::{PlanarLocalRebuildParityDenial, PlanarRebindingContinuityEvidence};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarLocalRebuildScope {
    scope_identity: String,
}

impl PlanarLocalRebuildScope {
    pub fn named(scope_identity: impl Into<String>) -> Self {
        Self {
            scope_identity: scope_identity.into(),
        }
    }

    pub fn scope_identity(&self) -> &str {
        &self.scope_identity
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct PlanarLocalRebuildParityBasis {
    rebuild_scope: PlanarLocalRebuildScope,
    neighborhood: Arc<TopologyNeighborhoodReplacementFactReceipt>,
    rebinding: PlanarRebindingContinuityEvidence,
    structural_identity: Arc<PlanarStructuralIdentityReceipt>,
    retained: Arc<RetainedPlanarFactsReceipt>,
    projection_consumed: Arc<ProjectionConsumedPlanarFactsReceipt>,
    motion: Arc<PlanarMotionPostureReceipt>,
    topology: Arc<PlanarTopologyContractCompletenessReceipt>,
    recovery: Arc<PlanarRecoveryPostureReceipt>,
    diagnostics: Arc<PlanarDiagnosticBundleReceipt>,
}

impl PlanarLocalRebuildParityBasis {
    pub fn builder(rebuild_scope: PlanarLocalRebuildScope) -> PlanarLocalRebuildParityBuilder {
        PlanarLocalRebuildParityBuilder::new(rebuild_scope)
    }

    pub(crate) fn from_builder(
        builder: PlanarLocalRebuildParityBuilder,
    ) -> Result<Self, PlanarLocalRebuildParityDenial> {
        let basis = Self {
            rebuild_scope: builder.rebuild_scope,
            neighborhood: Arc::new(
                builder
                    .neighborhood
                    .ok_or_else(|| missing("local neighborhood"))?,
            ),
            rebinding: builder
                .rebinding
                .ok_or_else(|| missing("rebinding continuity"))?,
            structural_identity: Arc::new(
                builder
                    .structural_identity
                    .ok_or_else(|| missing("structural identity"))?,
            ),
            retained: Arc::new(
                builder
                    .retained
                    .ok_or_else(|| missing("retained planar facts"))?,
            ),
            projection_consumed: Arc::new(
                builder
                    .projection_consumed
                    .ok_or_else(|| missing("projection-consumed planar facts"))?,
            ),
            motion: Arc::new(builder.motion.ok_or_else(|| missing("motion posture"))?),
            topology: Arc::new(
                builder
                    .topology
                    .ok_or_else(|| missing("topology completeness"))?,
            ),
            recovery: Arc::new(
                builder
                    .recovery
                    .ok_or_else(|| missing("recovery posture"))?,
            ),
            diagnostics: Arc::new(builder.diagnostics.ok_or_else(|| missing("diagnostics"))?),
        };
        validate_planar_local_rebuild_parity_basis(&basis)?;
        Ok(basis)
    }

    pub fn rebuild_scope(&self) -> &PlanarLocalRebuildScope {
        &self.rebuild_scope
    }

    pub fn neighborhood(&self) -> &TopologyNeighborhoodReplacementFactReceipt {
        &self.neighborhood
    }

    pub fn rebinding(&self) -> &PlanarRebindingContinuityEvidence {
        &self.rebinding
    }

    pub fn structural_identity(&self) -> &PlanarStructuralIdentityReceipt {
        &self.structural_identity
    }

    pub fn retained(&self) -> &RetainedPlanarFactsReceipt {
        &self.retained
    }

    pub fn projection_consumed(&self) -> &ProjectionConsumedPlanarFactsReceipt {
        &self.projection_consumed
    }

    pub fn motion(&self) -> &PlanarMotionPostureReceipt {
        &self.motion
    }

    pub fn topology(&self) -> &PlanarTopologyContractCompletenessReceipt {
        &self.topology
    }

    pub fn recovery(&self) -> &PlanarRecoveryPostureReceipt {
        &self.recovery
    }

    pub fn diagnostics(&self) -> &PlanarDiagnosticBundleReceipt {
        &self.diagnostics
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct PlanarLocalRebuildParityBuilder {
    rebuild_scope: PlanarLocalRebuildScope,
    neighborhood: Option<TopologyNeighborhoodReplacementFactReceipt>,
    rebinding: Option<PlanarRebindingContinuityEvidence>,
    structural_identity: Option<PlanarStructuralIdentityReceipt>,
    retained: Option<RetainedPlanarFactsReceipt>,
    projection_consumed: Option<ProjectionConsumedPlanarFactsReceipt>,
    motion: Option<PlanarMotionPostureReceipt>,
    topology: Option<PlanarTopologyContractCompletenessReceipt>,
    recovery: Option<PlanarRecoveryPostureReceipt>,
    diagnostics: Option<PlanarDiagnosticBundleReceipt>,
}

impl PlanarLocalRebuildParityBuilder {
    fn new(rebuild_scope: PlanarLocalRebuildScope) -> Self {
        Self {
            rebuild_scope,
            neighborhood: None,
            rebinding: None,
            structural_identity: None,
            retained: None,
            projection_consumed: None,
            motion: None,
            topology: None,
            recovery: None,
            diagnostics: None,
        }
    }

    pub fn local_neighborhood(
        mut self,
        receipt: TopologyNeighborhoodReplacementFactReceipt,
    ) -> Self {
        self.neighborhood = Some(receipt);
        self
    }

    pub fn rebinding_continuity(mut self, evidence: PlanarRebindingContinuityEvidence) -> Self {
        self.rebinding = Some(evidence);
        self
    }

    pub fn structural_identity(mut self, receipt: PlanarStructuralIdentityReceipt) -> Self {
        self.structural_identity = Some(receipt);
        self
    }

    pub fn retained_planar_facts(mut self, receipt: RetainedPlanarFactsReceipt) -> Self {
        self.retained = Some(receipt);
        self
    }

    pub fn projection_consumed_planar_facts(
        mut self,
        receipt: ProjectionConsumedPlanarFactsReceipt,
    ) -> Self {
        self.projection_consumed = Some(receipt);
        self
    }

    pub fn motion_posture(mut self, receipt: PlanarMotionPostureReceipt) -> Self {
        self.motion = Some(receipt);
        self
    }

    pub fn topology_contract(mut self, receipt: PlanarTopologyContractCompletenessReceipt) -> Self {
        self.topology = Some(receipt);
        self
    }

    pub fn recovery_posture(mut self, receipt: PlanarRecoveryPostureReceipt) -> Self {
        self.recovery = Some(receipt);
        self
    }

    pub fn diagnostics(mut self, receipt: PlanarDiagnosticBundleReceipt) -> Self {
        self.diagnostics = Some(receipt);
        self
    }

    pub fn build(self) -> Result<PlanarLocalRebuildParityBasis, PlanarLocalRebuildParityDenial> {
        PlanarLocalRebuildParityBasis::from_builder(self)
    }
}

fn missing(_label: &'static str) -> PlanarLocalRebuildParityDenial {
    PlanarLocalRebuildParityDenial::new(
        super::PlanarLocalRebuildParityDenialKind::MissingPlanarReceipt,
        "planar local rebuild parity requires every Phase 15-18 planar receipt",
    )
}
