use super::{
    WorthQueryArtifactPosture, WorthQueryArtifactReuseEquivalence, WorthQueryComparatorRequirement,
    WorthQueryConditionalEvaluationCondition, WorthQueryConditionalNodeContext,
    WorthQueryConditionalNodeOutput, WorthQueryConditionalNodeRole, WorthQueryConditionalTrigger,
    WorthQueryMaintenancePosture, WorthQueryOutputEquivalenceRequirement,
    WorthQueryOutputRelationship, WorthQuerySemanticTruthDependency,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryPortableConditionalNodeDeclaration {
    identity: String,
    role: WorthQueryConditionalNodeRole,
    dependencies: Vec<WorthQuerySemanticTruthDependency>,
    outputs: Vec<WorthQueryConditionalNodeOutput>,
    required_context: Vec<WorthQueryConditionalNodeContext>,
    condition: WorthQueryConditionalEvaluationCondition,
    trigger: WorthQueryConditionalTrigger,
    dependency_comparator: WorthQueryComparatorRequirement,
    output_equivalence: WorthQueryOutputEquivalenceRequirement,
    artifact_reuse_equivalence: WorthQueryArtifactReuseEquivalence,
    maintenance: WorthQueryMaintenancePosture,
    artifact: WorthQueryArtifactPosture,
    output_relationship: WorthQueryOutputRelationship,
}

impl WorthQueryPortableConditionalNodeDeclaration {
    pub fn declare(
        identity: impl Into<String>,
        role: WorthQueryConditionalNodeRole,
    ) -> super::WorthQueryPortableConditionalNodeBuilder {
        super::WorthQueryPortableConditionalNodeBuilder::new(identity.into(), role)
    }

    #[allow(clippy::too_many_arguments)]
    pub(super) fn new(
        identity: impl Into<String>,
        role: WorthQueryConditionalNodeRole,
        dependencies: impl IntoIterator<Item = WorthQuerySemanticTruthDependency>,
        outputs: impl IntoIterator<Item = WorthQueryConditionalNodeOutput>,
        required_context: impl IntoIterator<Item = WorthQueryConditionalNodeContext>,
        condition: WorthQueryConditionalEvaluationCondition,
        trigger: WorthQueryConditionalTrigger,
        dependency_comparator: WorthQueryComparatorRequirement,
        output_equivalence: WorthQueryOutputEquivalenceRequirement,
        artifact_reuse_equivalence: WorthQueryArtifactReuseEquivalence,
        maintenance: WorthQueryMaintenancePosture,
        artifact: WorthQueryArtifactPosture,
        output_relationship: WorthQueryOutputRelationship,
    ) -> Result<Self, &'static str> {
        let mut declaration = Self {
            identity: identity.into(),
            role,
            dependencies: dependencies.into_iter().collect(),
            outputs: outputs.into_iter().collect(),
            required_context: required_context.into_iter().collect(),
            condition,
            trigger,
            dependency_comparator,
            output_equivalence,
            artifact_reuse_equivalence,
            maintenance,
            artifact,
            output_relationship,
        };
        declaration.canonicalize();
        super::validation::validate_conditional_node(&declaration)?;
        Ok(declaration)
    }

    pub fn identity(&self) -> &str {
        &self.identity
    }
    pub const fn role(&self) -> WorthQueryConditionalNodeRole {
        self.role
    }
    pub fn dependencies(&self) -> &[WorthQuerySemanticTruthDependency] {
        &self.dependencies
    }
    pub fn outputs(&self) -> &[WorthQueryConditionalNodeOutput] {
        &self.outputs
    }
    pub fn required_context(&self) -> &[WorthQueryConditionalNodeContext] {
        &self.required_context
    }
    pub fn condition(&self) -> &WorthQueryConditionalEvaluationCondition {
        &self.condition
    }
    pub fn trigger(&self) -> &WorthQueryConditionalTrigger {
        &self.trigger
    }
    pub fn dependency_comparator(&self) -> &WorthQueryComparatorRequirement {
        &self.dependency_comparator
    }
    pub fn output_equivalence(&self) -> &WorthQueryOutputEquivalenceRequirement {
        &self.output_equivalence
    }
    pub fn artifact_reuse_equivalence(&self) -> &WorthQueryArtifactReuseEquivalence {
        &self.artifact_reuse_equivalence
    }
    pub const fn maintenance(&self) -> WorthQueryMaintenancePosture {
        self.maintenance
    }
    pub const fn artifact(&self) -> WorthQueryArtifactPosture {
        self.artifact
    }
    pub const fn output_relationship(&self) -> WorthQueryOutputRelationship {
        self.output_relationship
    }

    pub(crate) fn canonicalize(&mut self) {
        self.dependencies.sort_by_key(super::dependency_token);
        self.dependencies.dedup();
        for output in &mut self.outputs {
            super::output_contract::canonicalize_output(output);
        }
        self.outputs
            .sort_by_key(super::output_contract::output_token);
        self.outputs.dedup();
        self.required_context.sort();
        self.required_context.dedup();
        self.condition.canonicalize();
    }

    pub(crate) fn canonical_token(&self) -> String {
        let mut material = String::new();
        super::push_token(&mut material, "identity", &self.identity);
        super::push_token(&mut material, "role", role_name(self.role));
        for dependency in &self.dependencies {
            super::push_token(
                &mut material,
                "dependency",
                &super::dependency_token(dependency),
            );
        }
        for output in &self.outputs {
            super::push_token(
                &mut material,
                "output",
                &super::output_contract::output_token(output),
            );
        }
        for context in &self.required_context {
            super::push_token(
                &mut material,
                "context",
                super::node_posture::context_name(*context),
            );
        }
        super::push_token(
            &mut material,
            "condition",
            &self.condition.canonical_token(),
        );
        super::push_token(
            &mut material,
            "trigger",
            &super::trigger::trigger_token(&self.trigger),
        );
        super::push_token(
            &mut material,
            "dependency-comparator",
            &super::comparison::comparator_token(&self.dependency_comparator),
        );
        super::push_token(
            &mut material,
            "output-equivalence",
            &super::comparison::output_equivalence_token(&self.output_equivalence),
        );
        super::push_token(
            &mut material,
            "artifact-reuse-equivalence",
            &super::comparison::artifact_reuse_token(&self.artifact_reuse_equivalence),
        );
        super::push_token(
            &mut material,
            "maintenance",
            maintenance_name(self.maintenance),
        );
        super::push_token(&mut material, "artifact", artifact_name(self.artifact));
        super::push_token(
            &mut material,
            "output-relationship",
            output_relationship_name(self.output_relationship),
        );
        material
    }
}

fn role_name(role: WorthQueryConditionalNodeRole) -> &'static str {
    match role {
        WorthQueryConditionalNodeRole::Computed => "computed",
        WorthQueryConditionalNodeRole::WorkflowStage => "workflow-stage",
        WorthQueryConditionalNodeRole::OperationGate => "operation-gate",
    }
}

fn maintenance_name(posture: WorthQueryMaintenancePosture) -> &'static str {
    match posture {
        WorthQueryMaintenancePosture::EagerOnEligibleInvalidation => {
            "eager-on-eligible-invalidation"
        }
        WorthQueryMaintenancePosture::LazyUntilObserved => "lazy-until-observed",
        WorthQueryMaintenancePosture::OnDemandOnly => "on-demand-only",
        WorthQueryMaintenancePosture::Temporal => "temporal",
    }
}

fn artifact_name(posture: WorthQueryArtifactPosture) -> &'static str {
    match posture {
        WorthQueryArtifactPosture::Ephemeral => "ephemeral",
        WorthQueryArtifactPosture::ReusableWhenEquivalent => "reusable-when-equivalent",
        WorthQueryArtifactPosture::Durable => "durable",
    }
}

fn output_relationship_name(relationship: WorthQueryOutputRelationship) -> &'static str {
    match relationship {
        WorthQueryOutputRelationship::IntermediateOnly => "intermediate-only",
        WorthQueryOutputRelationship::ContributesToOperationOutput => {
            "contributes-to-operation-output"
        }
        WorthQueryOutputRelationship::IsOperationOutput => "is-operation-output",
        WorthQueryOutputRelationship::IsWorkflowStageOutput => "is-workflow-stage-output",
    }
}
