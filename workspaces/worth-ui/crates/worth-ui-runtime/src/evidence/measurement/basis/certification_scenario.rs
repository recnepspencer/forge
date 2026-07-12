use worth_query::facade::ProjectionFactConsumptionAttempt;
use worth_ui_host_contract::{
    UiMeasurementEvidenceFamily, UiMeasurementRequestIdentity, WorthUiHostCapabilityReport,
    WorthUiMeasurementHostAdapter,
};
use worth_ui_inspection::UiEvidenceAuthorityGeneration;
use worth_ui_query_binding::WorthUiQueryPrerequisiteEvidence;

use crate::declaration::{UiDeclarationIdentity, UiDeclaredMeasurementPolicyPosture};
use crate::graph::{UiGraphNodeIdentity, UiGraphWorldProfile};
use crate::host::{
    UiHostMeasurementEvidenceDenial, UiHostMeasurementNeed, UiHostMeasurementNormalizationContext,
};

use super::{
    admit::admit_measurement_basis, certification::certify_measurement_basis_determinism,
    UiMeasurementBasis, UiMeasurementBasisCertificationReport,
};
use crate::evidence::measurement::{
    consume_declared_measurement_projection_facts, MeasurementEvidenceInput,
    UiProjectionFactReceiptDenial,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiMeasurementBasisCertificationHostRequest {
    request_identity: UiMeasurementRequestIdentity,
    evidence_family: UiMeasurementEvidenceFamily,
    need: UiHostMeasurementNeed,
    normalization_context: UiHostMeasurementNormalizationContext,
}

#[derive(Clone, Debug, PartialEq)]
pub struct UiMeasurementBasisCertificationScenario {
    declaration_identity: UiDeclarationIdentity,
    graph_node_identity: UiGraphNodeIdentity,
    world_profile: UiGraphWorldProfile,
    declaration_support_authority_generation: UiEvidenceAuthorityGeneration,
    query_receipt_authority_generation: Option<UiEvidenceAuthorityGeneration>,
    declared_measurement_policy: UiDeclaredMeasurementPolicyPosture,
    query_prerequisites: Option<WorthUiQueryPrerequisiteEvidence>,
    projection_consumption: Option<ProjectionFactConsumptionAttempt>,
    host_capability_report: WorthUiHostCapabilityReport,
    host_requests: Box<[UiMeasurementBasisCertificationHostRequest]>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct UiMeasurementBasisCertificationOutcome {
    first_basis: UiMeasurementBasis,
    second_basis: UiMeasurementBasis,
    report: UiMeasurementBasisCertificationReport,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum UiMeasurementBasisCertificationScenarioError {
    MissingQueryPrerequisites,
    MissingProjectionConsumption,
    ProjectionFactReceiptDenied(UiProjectionFactReceiptDenial),
    HostMeasurementEvidenceDenied(UiHostMeasurementEvidenceDenial),
}

impl UiMeasurementBasisCertificationHostRequest {
    pub fn new(
        request_identity: UiMeasurementRequestIdentity,
        evidence_family: UiMeasurementEvidenceFamily,
        need: UiHostMeasurementNeed,
        normalization_context: UiHostMeasurementNormalizationContext,
    ) -> Self {
        Self {
            request_identity,
            evidence_family,
            need,
            normalization_context,
        }
    }
}

impl UiMeasurementBasisCertificationScenario {
    pub fn new(
        declaration_identity: UiDeclarationIdentity,
        graph_node_identity: UiGraphNodeIdentity,
        world_profile: UiGraphWorldProfile,
        declaration_support_authority_generation: UiEvidenceAuthorityGeneration,
        declared_measurement_policy: UiDeclaredMeasurementPolicyPosture,
        host_capability_report: WorthUiHostCapabilityReport,
    ) -> Self {
        Self {
            declaration_identity,
            graph_node_identity,
            world_profile,
            declaration_support_authority_generation,
            query_receipt_authority_generation: None,
            declared_measurement_policy,
            query_prerequisites: None,
            projection_consumption: None,
            host_capability_report,
            host_requests: Box::new([]),
        }
    }

    pub fn with_query_projection_consumption(
        mut self,
        query_prerequisites: WorthUiQueryPrerequisiteEvidence,
        projection_consumption: ProjectionFactConsumptionAttempt,
    ) -> Self {
        self.query_prerequisites = Some(query_prerequisites);
        self.projection_consumption = Some(projection_consumption);
        self
    }

    pub fn with_query_receipt_authority_generation(
        mut self,
        authority_generation: UiEvidenceAuthorityGeneration,
    ) -> Self {
        self.query_receipt_authority_generation = Some(authority_generation);
        self
    }

    pub fn with_host_requests(
        mut self,
        host_requests: impl Into<Box<[UiMeasurementBasisCertificationHostRequest]>>,
    ) -> Self {
        self.host_requests = host_requests.into();
        self
    }
}

impl UiMeasurementBasisCertificationOutcome {
    pub fn first_basis(&self) -> &UiMeasurementBasis {
        &self.first_basis
    }

    pub fn second_basis(&self) -> &UiMeasurementBasis {
        &self.second_basis
    }

    pub fn report(&self) -> &UiMeasurementBasisCertificationReport {
        &self.report
    }
}

pub fn certify_measurement_basis_determinism_for_scenarios<
    FirstAdapter: WorthUiMeasurementHostAdapter,
    SecondAdapter: WorthUiMeasurementHostAdapter,
>(
    first: &UiMeasurementBasisCertificationScenario,
    first_adapter: &FirstAdapter,
    second: &UiMeasurementBasisCertificationScenario,
    second_adapter: &SecondAdapter,
) -> Result<UiMeasurementBasisCertificationOutcome, UiMeasurementBasisCertificationScenarioError> {
    let first_basis = materialize_measurement_basis_for_certification(first, first_adapter)?;
    let second_basis = materialize_measurement_basis_for_certification(second, second_adapter)?;
    let report = certify_measurement_basis_determinism(&first_basis, &second_basis);
    Ok(UiMeasurementBasisCertificationOutcome {
        first_basis,
        second_basis,
        report,
    })
}

fn materialize_measurement_basis_for_certification<Adapter: WorthUiMeasurementHostAdapter>(
    scenario: &UiMeasurementBasisCertificationScenario,
    host_adapter: &Adapter,
) -> Result<UiMeasurementBasis, UiMeasurementBasisCertificationScenarioError> {
    let mut inputs = Vec::new();

    match (
        &scenario.query_prerequisites,
        &scenario.projection_consumption,
    ) {
        (Some(prerequisites), Some(consumption)) => {
            let receipt = consume_declared_measurement_projection_facts(
                scenario.declaration_identity.clone(),
                scenario
                    .query_receipt_authority_generation
                    .unwrap_or(scenario.declaration_support_authority_generation),
                &scenario.declared_measurement_policy,
                prerequisites.clone(),
                consumption,
            )
            .map_err(UiMeasurementBasisCertificationScenarioError::ProjectionFactReceiptDenied)?;
            inputs.push(MeasurementEvidenceInput::query_projection_fact(&receipt));
        }
        (Some(_), None) => {
            return Err(UiMeasurementBasisCertificationScenarioError::MissingProjectionConsumption);
        }
        (None, Some(_)) => {
            return Err(UiMeasurementBasisCertificationScenarioError::MissingQueryPrerequisites);
        }
        (None, None) => {}
    }

    inputs.push(MeasurementEvidenceInput::host_capability_report(
        &scenario.host_capability_report,
    ));

    let host_measurement_collector =
        crate::host::WorthUiHostMeasurementCollector::for_internal_proof();
    for host_request in scenario.host_requests.iter() {
        let result = host_measurement_collector
            .collect(
                host_adapter,
                host_request.request_identity,
                host_request.evidence_family,
                host_request.need.clone(),
                &scenario.host_capability_report,
                scenario.declaration_support_authority_generation,
                host_request.normalization_context,
            )
            .map_err(UiMeasurementBasisCertificationScenarioError::HostMeasurementEvidenceDenied)?;
        inputs.push(MeasurementEvidenceInput::host_measurement_result(&result));
    }

    Ok(admit_measurement_basis(
        scenario.declaration_identity.clone(),
        scenario.graph_node_identity,
        scenario.world_profile.clone(),
        scenario.declaration_support_authority_generation,
        &scenario.declared_measurement_policy,
        &inputs,
    ))
}
