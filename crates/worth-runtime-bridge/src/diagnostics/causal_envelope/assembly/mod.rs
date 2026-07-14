use super::authority::{BridgeCausalEvidenceFamily, BridgeCausalEvidenceOwner};
use super::binding::BridgeCausalEvidenceBinding;
use super::counters::BridgeCausalEnvelopeCounters;
use super::denial::{BridgeCausalEnvelopeDenial, BridgeCausalEnvelopeDenialKind};
use super::evidence_reference::BridgeCausalEvidenceReference;
use super::explanation_envelope::BridgeCausalExplanationEnvelope;
use super::retained_mapping;
use crate::diagnostics::BridgeDiagnosticsFacade;
use crate::identity::BridgeIdentityEvidence;

mod request;

pub use request::{
    BridgeCausalEnvelopeAssemblyRequest, BridgeCausalInspectionAdmissionSummary,
    BridgeCausalInspectionAdmissionSummaryKind,
};

impl BridgeDiagnosticsFacade {
    pub fn assemble_causal_explanation_envelope(
        &self,
        request: BridgeCausalEnvelopeAssemblyRequest,
    ) -> Result<BridgeCausalExplanationEnvelope, BridgeCausalEnvelopeDenial> {
        let mut progress = BridgeCausalEnvelopeAssemblyProgress::for_request(&request);

        for reference in request.references() {
            progress.bind_reference(self, reference)?;
        }

        if !progress.has_required_bridge_route_evidence() {
            return Err(
                self.missing_required_bridge_route_evidence(&request, progress.success_counters())
            );
        }
        let counters = progress.success_counters();
        Ok(BridgeCausalExplanationEnvelope::new(
            request,
            progress.into_bindings(),
            counters,
        ))
    }

    fn retained_record_evidence_identity(
        &self,
        reference: &BridgeCausalEvidenceReference,
    ) -> Result<Option<BridgeIdentityEvidence>, BridgeCausalEnvelopeDenial> {
        retained_mapping::retained_record_evidence_identity(self, reference)
    }

    fn missing_retained_record(
        &self,
        reference: &BridgeCausalEvidenceReference,
        counters: BridgeCausalEnvelopeCounters,
    ) -> BridgeCausalEnvelopeDenial {
        BridgeCausalEnvelopeDenial::new(
            BridgeCausalEnvelopeDenialKind::MissingRetainedBridgeRecord,
            reference.family(),
            reference.owner(),
            reference.family().expected_owner(),
            reference.reference_evidence_identity().clone(),
            counters,
        )
    }

    fn missing_required_bridge_route_evidence(
        &self,
        request: &BridgeCausalEnvelopeAssemblyRequest,
        counters: BridgeCausalEnvelopeCounters,
    ) -> BridgeCausalEnvelopeDenial {
        BridgeCausalEnvelopeDenial::new(
            BridgeCausalEnvelopeDenialKind::MissingRequiredBridgeRouteEvidence,
            BridgeCausalEvidenceFamily::BridgeRoute,
            BridgeCausalEvidenceOwner::RuntimeBridge,
            BridgeCausalEvidenceOwner::RuntimeBridge,
            request.request_evidence_identity().clone(),
            counters,
        )
    }
}

struct BridgeCausalEnvelopeAssemblyProgress {
    evidence_reference_count: usize,
    bridge_retained_lookup_count: usize,
    retained_bridge_binding_count: usize,
    external_authority_reference_count: usize,
    retained_bridge_route_binding_count: usize,
    bindings: Vec<BridgeCausalEvidenceBinding>,
}

impl BridgeCausalEnvelopeAssemblyProgress {
    fn for_request(request: &BridgeCausalEnvelopeAssemblyRequest) -> Self {
        Self {
            evidence_reference_count: request.references().len(),
            bridge_retained_lookup_count: 0,
            retained_bridge_binding_count: 0,
            external_authority_reference_count: 0,
            retained_bridge_route_binding_count: 0,
            bindings: Vec::new(),
        }
    }

    fn bind_reference(
        &mut self,
        facade: &BridgeDiagnosticsFacade,
        reference: &BridgeCausalEvidenceReference,
    ) -> Result<(), BridgeCausalEnvelopeDenial> {
        match reference.owner() {
            BridgeCausalEvidenceOwner::RuntimeBridge => {
                self.bind_retained_bridge_record(facade, reference)
            }
            BridgeCausalEvidenceOwner::Query
            | BridgeCausalEvidenceOwner::Relational
            | BridgeCausalEvidenceOwner::Signal => {
                self.bind_external_authority_reference(reference);
                Ok(())
            }
        }
    }

    fn bind_retained_bridge_record(
        &mut self,
        facade: &BridgeDiagnosticsFacade,
        reference: &BridgeCausalEvidenceReference,
    ) -> Result<(), BridgeCausalEnvelopeDenial> {
        self.bridge_retained_lookup_count += 1;
        let retained_identity = match facade.retained_record_evidence_identity(reference)? {
            Some(retained_identity) => retained_identity,
            None => return Err(facade.missing_retained_record(reference, self.failure_counters())),
        };
        self.retained_bridge_binding_count += 1;
        if reference.family() == BridgeCausalEvidenceFamily::BridgeRoute {
            self.retained_bridge_route_binding_count += 1;
        }
        self.bindings.push(BridgeCausalEvidenceBinding::retained(
            reference,
            retained_identity,
        ));
        Ok(())
    }

    fn bind_external_authority_reference(&mut self, reference: &BridgeCausalEvidenceReference) {
        self.external_authority_reference_count += 1;
        self.bindings
            .push(BridgeCausalEvidenceBinding::external(reference));
    }

    fn has_required_bridge_route_evidence(&self) -> bool {
        self.retained_bridge_route_binding_count > 0
    }

    fn success_counters(&self) -> BridgeCausalEnvelopeCounters {
        self.counters(0)
    }

    fn failure_counters(&self) -> BridgeCausalEnvelopeCounters {
        self.counters(1)
    }

    fn counters(&self, missing_bridge_record_count: usize) -> BridgeCausalEnvelopeCounters {
        BridgeCausalEnvelopeCounters::new(
            self.evidence_reference_count,
            self.lower_runtime_family_count(),
            self.bridge_retained_lookup_count,
            self.retained_bridge_binding_count,
            self.external_authority_reference_count,
            self.materialized_detail_count(),
            missing_bridge_record_count,
        )
    }

    fn into_bindings(self) -> Vec<BridgeCausalEvidenceBinding> {
        self.bindings
    }

    fn lower_runtime_family_count(&self) -> usize {
        let mut families = Vec::new();
        for binding in &self.bindings {
            if binding.family() == BridgeCausalEvidenceFamily::QueryObservation {
                continue;
            }
            if !families.contains(&binding.family()) {
                families.push(binding.family());
            }
        }
        families.len()
    }

    fn materialized_detail_count(&self) -> usize {
        self.bindings.len()
    }
}
