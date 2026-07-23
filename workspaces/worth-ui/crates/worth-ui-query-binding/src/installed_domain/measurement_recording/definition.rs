use worth_foundational::facade::{AspectMask, ProjectionMask};
use worth_query::facade::domain;

use crate::{
    installed_domain::snapshot_measurement::snapshot_measurement_definition,
    native_aspect_contracts, WorthUiDomainEntry,
};

use super::{
    WorthUiMeasurementRecording, WorthUiMeasurementRecordingFamily, IDENTIFY_STAGE,
    LOWERING_FAMILY, RECORD_STAGE,
};

pub(crate) fn measurement_recording_definition() -> domain::WorthQueryDomainOperationDefinition<
    WorthUiDomainEntry,
    WorthUiMeasurementRecording,
    WorthUiMeasurementRecordingFamily,
> {
    let read_meaning = snapshot_measurement_definition();
    let mut semantics = read_meaning.semantics().clone();
    semantics.parameters = domain::WorthQueryOperationParameterContract::NotRequired;
    semantics.collection = domain::WorthQueryOperationCollectionContract::NotCollection;
    semantics.required_capabilities =
        vec![domain::WorthQueryOperationCapabilityRequirement::WorkflowOrchestration];
    semantics.workflow = domain::WorthQueryOperationWorkflowContract::Declared(workflow());
    semantics.graph_reads = domain::WorthQueryOperationGraphReadContract::NotRequired;
    semantics.touches = domain::WorthQueryOperationTouchContract::NotRequired;
    semantics.effects = domain::WorthQueryOperationEffectContract::Declared {
        effect_families: vec![domain::WorthQueryOperationEffectFamily::Mutation],
    };
    semantics.publication = domain::WorthQueryOperationPublicationContract::NotRequired;
    semantics.projection_consumption =
        domain::WorthQueryOperationProjectionConsumptionContract::NotRequired;
    semantics.native_projection = domain::WorthQueryOperationNativeProjectionContract::new(
        native_aspect_contracts::worth_ui_native_aspect_contract("measurement")
            .expect("the installed Worth UI measurement aspect must exist"),
        AspectMask::<ProjectionMask>::whole_aspect(),
    )
    .expect("the whole measurement aspect mask must be admissible");
    semantics.terminal = domain::WorthQueryOperationTerminalContract {
        result_states: vec![domain::WorthQueryOperationResultState::Ready],
        failure_classes: vec![
            domain::WorthQueryOperationFailureClass::InvalidInput,
            domain::WorthQueryOperationFailureClass::Dependency,
        ],
    };
    semantics.support.projection_consumption = domain::WorthQuerySupportRequirement::NotRequired;
    semantics.lowering = domain::WorthQueryOperationLoweringContract {
        family: LOWERING_FAMILY.into(),
        deterministic: true,
    };
    domain::WorthQueryDomainOperationDefinition::new(
        domain::WorthQueryDomainOperationIdentity::new("measurement-recording", 1),
        semantics,
    )
}

fn workflow() -> domain::WorthQueryPortableWorkflowDefinition {
    domain::WorthQueryPortableWorkflowDefinition::new(
        IDENTIFY_STAGE,
        [
            domain::WorthQueryPortableWorkflowStage::new(
                IDENTIFY_STAGE,
                std::iter::empty::<&str>(),
                false,
                false,
                std::iter::empty::<domain::WorthQueryOperationCapabilityRequirement>(),
            )
            .with_semantics(domain::WorthQueryWorkflowStageSemantics {
                input: domain::WorthQueryWorkflowValueContract::Text,
                output: domain::WorthQueryWorkflowValueContract::Text,
                cost_roles: vec![
                    domain::WorthQueryWorkflowCostRole::Admission,
                    domain::WorthQueryWorkflowCostRole::Execution,
                    domain::WorthQueryWorkflowCostRole::ResultValidation,
                ],
                failure_classes: vec![domain::WorthQueryOperationFailureClass::InvalidInput],
                ..Default::default()
            }),
            domain::WorthQueryPortableWorkflowStage::new(
                RECORD_STAGE,
                [IDENTIFY_STAGE],
                true,
                false,
                std::iter::empty::<domain::WorthQueryOperationCapabilityRequirement>(),
            )
            .with_semantics(domain::WorthQueryWorkflowStageSemantics {
                input: domain::WorthQueryWorkflowValueContract::U64,
                output: domain::WorthQueryWorkflowValueContract::Text,
                effect_roles: vec![domain::WorthQueryOperationEffectFamily::Mutation],
                cost_roles: vec![
                    domain::WorthQueryWorkflowCostRole::Admission,
                    domain::WorthQueryWorkflowCostRole::Effect,
                    domain::WorthQueryWorkflowCostRole::Execution,
                    domain::WorthQueryWorkflowCostRole::ResultValidation,
                ],
                terminal_result_states: vec![domain::WorthQueryOperationResultState::Ready],
                failure_classes: vec![
                    domain::WorthQueryOperationFailureClass::InvalidInput,
                    domain::WorthQueryOperationFailureClass::Dependency,
                ],
                ..Default::default()
            }),
        ],
    )
}
