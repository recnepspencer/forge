use forge_runtime_bridge::facade::{
    BridgeCausalEnvelopeAssemblyRequest, BridgeCausalEnvelopeDenial, BridgeCausalEvidenceFamily,
    BridgeCausalEvidenceOwner, BridgeCausalEvidenceReference,
    BridgeCausalInspectionAdmissionSummary, RuntimeBridge,
};

use super::admission::{
    AdmittedCausalInspection, AdvisoryCausalInspection, CausalInspectionProofFlow,
};
use super::builder::CausalInspectionPlan;
use super::inventory::{CausalEvidenceFamily, CausalEvidenceOwner};
use super::materialization::{
    materialize_admitted_causal_inspection, materialize_advisory_causal_inspection,
    materialize_bridge_denied_admitted_causal_inspection,
    materialize_bridge_denied_advisory_causal_inspection, materialize_denied_causal_inspection,
    CausalInspectionMaterializationError, QueryCausalInspectionArtifact,
};
use super::reference::CausalEvidenceReference;

impl CausalInspectionPlan {
    pub fn materialize_with_bridge(
        &self,
        bridge: &RuntimeBridge,
    ) -> Result<QueryCausalInspectionArtifact, CausalInspectionMaterializationError> {
        match &self.admission {
            CausalInspectionProofFlow::Denied(inspection) => {
                Ok(materialize_denied_causal_inspection(
                    inspection,
                    None,
                    self.redaction_policy,
                    self.materialization_policy,
                ))
            }
            CausalInspectionProofFlow::Admitted(inspection) => {
                self.materialize_admitted_with_bridge(inspection, bridge)
            }
            CausalInspectionProofFlow::Advisory(inspection) => {
                self.materialize_advisory_with_bridge(inspection, bridge)
            }
        }
    }

    fn materialize_admitted_with_bridge(
        &self,
        inspection: &AdmittedCausalInspection,
        bridge: &RuntimeBridge,
    ) -> Result<QueryCausalInspectionArtifact, CausalInspectionMaterializationError> {
        match bridge
            .diagnostics()
            .assemble_causal_explanation_envelope(self.bridge_request_for_admitted(inspection)?)
        {
            Ok(envelope) => materialize_admitted_causal_inspection(
                inspection,
                &envelope,
                self.redaction_policy,
                self.materialization_policy,
            ),
            Err(denial) => Ok(materialize_bridge_denied_admitted_causal_inspection(
                inspection,
                &denial,
                self.redaction_policy,
                self.materialization_policy,
            )),
        }
    }

    fn materialize_advisory_with_bridge(
        &self,
        inspection: &AdvisoryCausalInspection,
        bridge: &RuntimeBridge,
    ) -> Result<QueryCausalInspectionArtifact, CausalInspectionMaterializationError> {
        match bridge
            .diagnostics()
            .assemble_causal_explanation_envelope(self.bridge_request_for_advisory(inspection)?)
        {
            Ok(envelope) => materialize_advisory_causal_inspection(
                inspection,
                &envelope,
                self.redaction_policy,
                self.materialization_policy,
            ),
            Err(denial) => Ok(materialize_bridge_denied_advisory_causal_inspection(
                inspection,
                &denial,
                self.redaction_policy,
                self.materialization_policy,
            )),
        }
    }

    fn bridge_request_for_admitted(
        &self,
        inspection: &AdmittedCausalInspection,
    ) -> Result<BridgeCausalEnvelopeAssemblyRequest, CausalInspectionMaterializationError> {
        let summary = BridgeCausalInspectionAdmissionSummary::admitted(
            inspection.admitted_inspection_digest(),
            inspection.subject().anchor_digest(),
        )
        .map_err(materialization_error_from_bridge_denial)?;
        bridge_request_from_summary(
            summary,
            inspection.subject().query_observation_digest(),
            self.reference_set.references(),
        )
    }

    fn bridge_request_for_advisory(
        &self,
        inspection: &AdvisoryCausalInspection,
    ) -> Result<BridgeCausalEnvelopeAssemblyRequest, CausalInspectionMaterializationError> {
        let summary = BridgeCausalInspectionAdmissionSummary::advisory(
            inspection.advisory_inspection_digest(),
            inspection.subject().anchor_digest(),
        )
        .map_err(materialization_error_from_bridge_denial)?;
        bridge_request_from_summary(
            summary,
            inspection.subject().query_observation_digest(),
            self.reference_set.references(),
        )
    }
}

fn bridge_request_from_summary(
    summary: BridgeCausalInspectionAdmissionSummary,
    query_observation_digest: &str,
    references: &[CausalEvidenceReference],
) -> Result<BridgeCausalEnvelopeAssemblyRequest, CausalInspectionMaterializationError> {
    let mut bridge_references = vec![BridgeCausalEvidenceReference::new(
        BridgeCausalEvidenceOwner::Query,
        BridgeCausalEvidenceFamily::QueryObservation,
        query_observation_digest,
    )
    .map_err(materialization_error_from_bridge_denial)?];
    for reference in references {
        if let Some((owner, family)) =
            bridge_reference_family(reference.owner(), reference.family())
        {
            bridge_references.push(
                BridgeCausalEvidenceReference::new(
                    owner,
                    family,
                    reference.reference_digest().as_str(),
                )
                .map_err(materialization_error_from_bridge_denial)?,
            );
        }
    }
    BridgeCausalEnvelopeAssemblyRequest::from_query_admission(summary, bridge_references)
        .map_err(materialization_error_from_bridge_denial)
}

fn bridge_reference_family(
    owner: CausalEvidenceOwner,
    family: CausalEvidenceFamily,
) -> Option<(BridgeCausalEvidenceOwner, BridgeCausalEvidenceFamily)> {
    let bridge_owner = match owner {
        CausalEvidenceOwner::Query => return None,
        CausalEvidenceOwner::RuntimeBridge => BridgeCausalEvidenceOwner::RuntimeBridge,
        CausalEvidenceOwner::Relational => BridgeCausalEvidenceOwner::Relational,
        CausalEvidenceOwner::Signal => BridgeCausalEvidenceOwner::Signal,
    };
    let bridge_family = bridge_family_for(family)?;
    Some((bridge_owner, bridge_family))
}

fn bridge_family_for(family: CausalEvidenceFamily) -> Option<BridgeCausalEvidenceFamily> {
    Some(match family {
        CausalEvidenceFamily::RelationalAuthority | CausalEvidenceFamily::RelationalDecision => {
            BridgeCausalEvidenceFamily::RelationalAuthority
        }
        CausalEvidenceFamily::BridgeRoute => BridgeCausalEvidenceFamily::BridgeRoute,
        CausalEvidenceFamily::BridgeEvaluation => {
            BridgeCausalEvidenceFamily::BridgeHistoricalEvaluation
        }
        CausalEvidenceFamily::BridgeSourceMaterialization => {
            BridgeCausalEvidenceFamily::BridgeSourceMaterialization
        }
        CausalEvidenceFamily::BridgeSourceFailure => {
            BridgeCausalEvidenceFamily::BridgeSourceFailure
        }
        CausalEvidenceFamily::BridgeContinuity => BridgeCausalEvidenceFamily::BridgeContinuity,
        CausalEvidenceFamily::BridgeMerge => BridgeCausalEvidenceFamily::BridgeMerge,
        CausalEvidenceFamily::BridgeStructural => BridgeCausalEvidenceFamily::BridgeStructuralRemap,
        CausalEvidenceFamily::BridgeStream | CausalEvidenceFamily::BridgeReplay => {
            BridgeCausalEvidenceFamily::BridgeStreamReplay
        }
        CausalEvidenceFamily::BridgePreview => BridgeCausalEvidenceFamily::BridgePreviewExecution,
        CausalEvidenceFamily::BridgeWriteback => {
            BridgeCausalEvidenceFamily::BridgeWritebackExecution
        }
        CausalEvidenceFamily::BridgeMapper => BridgeCausalEvidenceFamily::BridgeWritebackMapper,
        CausalEvidenceFamily::SignalInvalidation => BridgeCausalEvidenceFamily::SignalInvalidation,
        CausalEvidenceFamily::SignalEvaluation => BridgeCausalEvidenceFamily::SignalEvaluation,
        CausalEvidenceFamily::SignalForensicAvailability => {
            BridgeCausalEvidenceFamily::SignalForensicAvailability
        }
        CausalEvidenceFamily::SignalReplayCursor => BridgeCausalEvidenceFamily::SignalReplayCursor,
        CausalEvidenceFamily::SignalLineage | CausalEvidenceFamily::Lineage => {
            BridgeCausalEvidenceFamily::SignalLineage
        }
        CausalEvidenceFamily::SignalProvenance => BridgeCausalEvidenceFamily::SignalProvenance,
        CausalEvidenceFamily::QueryInspection
        | CausalEvidenceFamily::QueryMutationCausality
        | CausalEvidenceFamily::QueryMutationProvenance
        | CausalEvidenceFamily::Provenance
        | CausalEvidenceFamily::Policy
        | CausalEvidenceFamily::Redaction => return None,
    })
}

fn materialization_error_from_bridge_denial(
    denial: BridgeCausalEnvelopeDenial,
) -> CausalInspectionMaterializationError {
    CausalInspectionMaterializationError::from_bridge_assembly_denial(&denial)
}
