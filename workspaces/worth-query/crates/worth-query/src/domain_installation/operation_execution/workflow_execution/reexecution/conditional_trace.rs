use crate::domain_installation::{
    WorthQueryConditionalOutcomeClass, WorthQueryConditionalProvenance,
};

#[derive(Clone, Debug)]
pub struct WorthQueryConditionalTraceMeaning {
    pub(super) location: worth_query_installation::facade::WorthQueryConditionalNodeLocation,
    pub(super) declaration:
        worth_query_installation::facade::WorthQueryPortableConditionalNodeDeclaration,
    pub(super) outcome: WorthQueryConditionalOutcomeClass,
    pub(super) artifact_reuse_admitted: bool,
    pub(super) signal_projection: std::sync::Arc<str>,
    pub(super) observations: Vec<WorthQueryConditionalObservationMeaning>,
}

impl PartialEq for WorthQueryConditionalTraceMeaning {
    fn eq(&self, candidate: &Self) -> bool {
        self.location == candidate.location
            && self.declaration == candidate.declaration
            && self.outcome == candidate.outcome
            && self.artifact_reuse_admitted == candidate.artifact_reuse_admitted
            && self.observations == candidate.observations
    }
}

impl Eq for WorthQueryConditionalTraceMeaning {}

impl WorthQueryConditionalTraceMeaning {
    pub fn location(&self) -> &worth_query_installation::facade::WorthQueryConditionalNodeLocation {
        &self.location
    }

    pub fn declaration(
        &self,
    ) -> &worth_query_installation::facade::WorthQueryPortableConditionalNodeDeclaration {
        &self.declaration
    }

    pub const fn outcome(&self) -> WorthQueryConditionalOutcomeClass {
        self.outcome
    }

    pub const fn artifact_reuse_admitted(&self) -> bool {
        self.artifact_reuse_admitted
    }

    pub fn observations(&self) -> &[WorthQueryConditionalObservationMeaning] {
        &self.observations
    }

    pub fn signal_projection(&self) -> &str {
        &self.signal_projection
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryConditionalObservationMeaning {
    pub(super) dependency_ordinal: usize,
    pub(super) previous: Option<worth_foundational::facade::ContractValidatedAspectArtifact>,
    pub(super) current: worth_foundational::facade::ContractValidatedAspectArtifact,
}

impl WorthQueryConditionalObservationMeaning {
    pub const fn dependency_ordinal(&self) -> usize {
        self.dependency_ordinal
    }

    pub const fn previous(
        &self,
    ) -> Option<&worth_foundational::facade::ContractValidatedAspectArtifact> {
        self.previous.as_ref()
    }

    pub const fn current(&self) -> &worth_foundational::facade::ContractValidatedAspectArtifact {
        &self.current
    }
}

pub(crate) fn conditional_trace_meaning(
    item: &WorthQueryConditionalProvenance,
) -> WorthQueryConditionalTraceMeaning {
    WorthQueryConditionalTraceMeaning {
        location: item.location().clone(),
        declaration: item.declaration().clone(),
        outcome: item.class(),
        artifact_reuse_admitted: item.artifact_reuse_admitted(),
        signal_projection: std::sync::Arc::clone(item.signal_projection().label()),
        observations: (0..item.semantic_observation_count())
            .filter_map(|ordinal| item.semantic_observation(ordinal))
            .map(|observation| WorthQueryConditionalObservationMeaning {
                dependency_ordinal: observation.dependency_ordinal(),
                previous: observation.previous().cloned(),
                current: observation.current().clone(),
            })
            .collect(),
    }
}

pub(crate) fn conditional_trace_semantic_material(
    item: &WorthQueryConditionalProvenance,
) -> String {
    let meaning = conditional_trace_meaning(item);
    conditional_meaning_semantic_material(&meaning)
}

pub(super) fn conditional_meaning_semantic_material(
    meaning: &WorthQueryConditionalTraceMeaning,
) -> String {
    crate::domain_installation::operation_identity_basis::canonical_operation_material(vec![
        (
            "conditional.location",
            location_semantic_material(&meaning.location),
        ),
        (
            "conditional.declaration",
            worth_query_installation::facade::portable_conditional_node_canonical_material(
                &meaning.declaration,
            ),
        ),
        (
            "conditional.outcome",
            outcome_semantic_material(meaning.outcome).into(),
        ),
        (
            "conditional.artifact_reuse",
            meaning.artifact_reuse_admitted.to_string(),
        ),
        (
            "conditional.observations",
            crate::domain_installation::operation_identity_basis::canonical_indexed_operation_material(
                "conditional.observation",
                meaning.observations.iter().map(observation_semantic_material),
            ),
        ),
    ])
}

pub(super) fn conditional_trace_operational_material(
    item: &WorthQueryConditionalProvenance,
) -> String {
    conditional_trace_semantic_material(item)
}

fn location_semantic_material(
    location: &worth_query_installation::facade::WorthQueryConditionalNodeLocation,
) -> String {
    crate::domain_installation::operation_identity_basis::canonical_operation_material(vec![
        (
            "location.scope",
            if location.stage_identity().is_some() {
                "workflow-stage"
            } else {
                "operation"
            }
            .into(),
        ),
        (
            "location.stage",
            location.stage_identity().unwrap_or("not-applicable").into(),
        ),
        ("location.node", location.node_identity().into()),
    ])
}

fn outcome_semantic_material(outcome: WorthQueryConditionalOutcomeClass) -> &'static str {
    match outcome {
        WorthQueryConditionalOutcomeClass::ComputedChanged => "computed-changed",
        WorthQueryConditionalOutcomeClass::ComputedRevertedClean => "computed-reverted-clean",
        WorthQueryConditionalOutcomeClass::DependencyUnchanged => "dependency-unchanged",
        WorthQueryConditionalOutcomeClass::Suppressed => "suppressed",
        WorthQueryConditionalOutcomeClass::DeferredByCondition => "deferred-by-condition",
        WorthQueryConditionalOutcomeClass::DeferredTemporal => "deferred-temporal",
        WorthQueryConditionalOutcomeClass::DeferredOnDemand => "deferred-on-demand",
    }
}

fn observation_semantic_material(observation: &WorthQueryConditionalObservationMeaning) -> String {
    crate::domain_installation::operation_identity_basis::canonical_operation_material(vec![
        (
            "observation.dependency",
            observation.dependency_ordinal.to_string(),
        ),
        (
            "observation.previous",
            observation
                .previous
                .as_ref()
                .map(validated_artifact_semantic_material)
                .unwrap_or_else(|| "explicitly-absent".into()),
        ),
        (
            "observation.current",
            validated_artifact_semantic_material(&observation.current),
        ),
    ])
}

fn validated_artifact_semantic_material(
    artifact: &worth_foundational::facade::ContractValidatedAspectArtifact,
) -> String {
    let value = artifact.payload();
    let canonical_value = match value.view() {
        worth_foundational::facade::ContractValidatedAspectValueView::Scalar(value) => {
            worth_foundational::facade::prepare_aspect_value_identity_basis(value)
        }
        worth_foundational::facade::ContractValidatedAspectValueView::Struct(value) => {
            worth_foundational::facade::prepare_struct_aspect_value_identity_basis(value)
        }
    };
    crate::domain_installation::operation_identity_basis::canonical_operation_material(vec![
        ("artifact.key", value.key().as_str().into()),
        (
            "artifact.contract_identity",
            value.contract_identity().0.to_string(),
        ),
        (
            "artifact.contract_revision",
            value.contract_revision().0.to_string(),
        ),
        ("artifact.value", canonical_value.as_str().into()),
    ])
}
