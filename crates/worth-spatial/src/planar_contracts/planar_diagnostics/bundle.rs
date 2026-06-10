use crate::planar_contracts::motion_posture::PlanarMotionPostureReceipt;
use crate::planar_contracts::projection_consumed_facts::ProjectionConsumedPlanarFactsReceipt;
use crate::planar_contracts::retained_planar_facts::RetainedPlanarFactsReceipt;
use crate::planar_contracts::topology_contract_completeness::PlanarTopologyContractCompletenessReceipt;

use super::{
    validate_planar_diagnostic_bundle_basis, PlanarDiagnosticCausalEvidence,
    PlanarDiagnosticDenial, PlanarDiagnosticEvidence, PlanarDiagnosticEvidenceKind,
    PlanarDiagnosticSubject, PlanarDiagnosticTopologyEvidence, PlanarDiagnosticTruthEffect,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarDiagnosticBundleBasis {
    subject: PlanarDiagnosticSubject,
    topology_evidence: Option<PlanarDiagnosticTopologyEvidence>,
    causal_evidence: Option<PlanarDiagnosticCausalEvidence>,
    materialized_causal_archive_requested: bool,
    truth_effect: PlanarDiagnosticTruthEffect,
}

impl PlanarDiagnosticBundleBasis {
    pub fn builder(subject: PlanarDiagnosticSubject) -> PlanarDiagnosticBundleBuilder {
        PlanarDiagnosticBundleBuilder::new(subject)
    }

    pub(crate) fn from_builder(
        builder: PlanarDiagnosticBundleBuilder,
    ) -> Result<Self, PlanarDiagnosticDenial> {
        let basis = Self {
            subject: builder.subject,
            topology_evidence: builder.topology_evidence,
            causal_evidence: builder.causal_evidence,
            materialized_causal_archive_requested: builder.materialized_causal_archive_requested,
            truth_effect: PlanarDiagnosticTruthEffect::DoesNotChangePlanarTruth,
        };
        validate_planar_diagnostic_bundle_basis(&basis)?;
        Ok(basis)
    }

    pub fn subject(&self) -> &PlanarDiagnosticSubject {
        &self.subject
    }

    pub fn topology_evidence(&self) -> Option<&PlanarDiagnosticTopologyEvidence> {
        self.topology_evidence.as_ref()
    }

    pub fn causal_evidence(&self) -> Option<&PlanarDiagnosticCausalEvidence> {
        self.causal_evidence.as_ref()
    }

    pub fn materialized_causal_archive_requested(&self) -> bool {
        self.materialized_causal_archive_requested
    }

    pub fn truth_effect(&self) -> PlanarDiagnosticTruthEffect {
        self.truth_effect
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarDiagnosticBundleBuilder {
    subject: PlanarDiagnosticSubject,
    topology_evidence: Option<PlanarDiagnosticTopologyEvidence>,
    causal_evidence: Option<PlanarDiagnosticCausalEvidence>,
    materialized_causal_archive_requested: bool,
}

impl PlanarDiagnosticBundleBuilder {
    fn new(subject: PlanarDiagnosticSubject) -> Self {
        Self {
            subject,
            topology_evidence: None,
            causal_evidence: None,
            materialized_causal_archive_requested: false,
        }
    }

    pub fn topology_declared_surface(mut self, evidence: PlanarDiagnosticTopologyEvidence) -> Self {
        self.subject.push_evidence(PlanarDiagnosticEvidence::new(
            PlanarDiagnosticEvidenceKind::TopologyDeclaredSurface,
            evidence.declared_surface_digest(),
        ));
        self.topology_evidence = Some(evidence);
        self
    }

    pub fn query_causal_inspection(mut self, evidence: PlanarDiagnosticCausalEvidence) -> Self {
        self.subject.push_evidence(PlanarDiagnosticEvidence::new(
            PlanarDiagnosticEvidenceKind::QueryCausalInspection,
            evidence.reference_digest(),
        ));
        self.causal_evidence = Some(evidence);
        self
    }

    pub fn retained_planar_facts(mut self, receipt: RetainedPlanarFactsReceipt) -> Self {
        self.subject.push_evidence(PlanarDiagnosticEvidence::new(
            PlanarDiagnosticEvidenceKind::BasisLifecycleReceipt,
            receipt.retained_fact_digest(),
        ));
        self
    }

    pub fn projection_consumed_planar_facts(
        mut self,
        receipt: ProjectionConsumedPlanarFactsReceipt,
    ) -> Self {
        self.subject.push_evidence(PlanarDiagnosticEvidence::new(
            PlanarDiagnosticEvidenceKind::ProjectionConsumptionReceipt,
            receipt.projection_consumption_digest(),
        ));
        self
    }

    pub fn topology_contract(mut self, receipt: PlanarTopologyContractCompletenessReceipt) -> Self {
        self.subject.push_evidence(PlanarDiagnosticEvidence::new(
            PlanarDiagnosticEvidenceKind::TopologyDeclaredSurface,
            receipt.fact_digest(),
        ));
        self
    }

    pub fn motion_posture(mut self, receipt: PlanarMotionPostureReceipt) -> Self {
        self.subject.push_evidence(PlanarDiagnosticEvidence::new(
            PlanarDiagnosticEvidenceKind::BasisLifecycleReceipt,
            receipt.retained_motion_digest(),
        ));
        self
    }

    pub fn request_materialized_causal_archive(mut self) -> Self {
        self.materialized_causal_archive_requested = true;
        self
    }

    pub fn build(self) -> Result<PlanarDiagnosticBundleBasis, PlanarDiagnosticDenial> {
        PlanarDiagnosticBundleBasis::from_builder(self)
    }
}
