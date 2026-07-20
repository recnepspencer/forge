use super::{
    WorthQueryArtifactPosture, WorthQueryArtifactReuseEquivalence, WorthQueryComparatorRequirement,
    WorthQueryConditionalEvaluationCondition, WorthQueryConditionalNodeContext,
    WorthQueryConditionalNodeOutput, WorthQueryConditionalNodeRole, WorthQueryConditionalTrigger,
    WorthQueryMaintenancePosture, WorthQueryOutputEquivalenceRequirement,
    WorthQueryOutputRelationship, WorthQueryPortableConditionalNodeDeclaration,
    WorthQuerySemanticTruthDependency,
};

/// Named authoring grammar for one portable conditional node. Every semantic
/// dimension is required; this builder deliberately supplies no policy
/// defaults that could silently become executable meaning later.
pub struct WorthQueryPortableConditionalNodeBuilder {
    identity: String,
    role: WorthQueryConditionalNodeRole,
    dependencies: Option<Vec<WorthQuerySemanticTruthDependency>>,
    outputs: Option<Vec<WorthQueryConditionalNodeOutput>>,
    required_context: Option<Vec<WorthQueryConditionalNodeContext>>,
    condition: Option<WorthQueryConditionalEvaluationCondition>,
    trigger: Option<WorthQueryConditionalTrigger>,
    dependency_comparator: Option<WorthQueryComparatorRequirement>,
    output_equivalence: Option<WorthQueryOutputEquivalenceRequirement>,
    artifact_reuse_equivalence: Option<WorthQueryArtifactReuseEquivalence>,
    maintenance: Option<WorthQueryMaintenancePosture>,
    artifact: Option<WorthQueryArtifactPosture>,
    output_relationship: Option<WorthQueryOutputRelationship>,
}

impl WorthQueryPortableConditionalNodeBuilder {
    pub(crate) fn new(identity: String, role: WorthQueryConditionalNodeRole) -> Self {
        Self {
            identity,
            role,
            dependencies: None,
            outputs: None,
            required_context: None,
            condition: None,
            trigger: None,
            dependency_comparator: None,
            output_equivalence: None,
            artifact_reuse_equivalence: None,
            maintenance: None,
            artifact: None,
            output_relationship: None,
        }
    }

    pub fn dependencies(
        mut self,
        dependencies: impl IntoIterator<Item = WorthQuerySemanticTruthDependency>,
    ) -> Self {
        self.dependencies = Some(dependencies.into_iter().collect());
        self
    }

    pub fn outputs(
        mut self,
        outputs: impl IntoIterator<Item = WorthQueryConditionalNodeOutput>,
    ) -> Self {
        self.outputs = Some(outputs.into_iter().collect());
        self
    }

    pub fn required_context(
        mut self,
        required_context: impl IntoIterator<Item = WorthQueryConditionalNodeContext>,
    ) -> Self {
        self.required_context = Some(required_context.into_iter().collect());
        self
    }

    pub fn evaluation(
        mut self,
        condition: WorthQueryConditionalEvaluationCondition,
        trigger: WorthQueryConditionalTrigger,
    ) -> Self {
        self.condition = Some(condition);
        self.trigger = Some(trigger);
        self
    }

    pub fn comparison(
        mut self,
        dependency: WorthQueryComparatorRequirement,
        output: WorthQueryOutputEquivalenceRequirement,
    ) -> Self {
        self.dependency_comparator = Some(dependency);
        self.output_equivalence = Some(output);
        self
    }

    pub fn artifact_policy(
        mut self,
        reuse: WorthQueryArtifactReuseEquivalence,
        maintenance: WorthQueryMaintenancePosture,
        artifact: WorthQueryArtifactPosture,
    ) -> Self {
        self.artifact_reuse_equivalence = Some(reuse);
        self.maintenance = Some(maintenance);
        self.artifact = Some(artifact);
        self
    }

    pub fn output_relationship(mut self, relationship: WorthQueryOutputRelationship) -> Self {
        self.output_relationship = Some(relationship);
        self
    }

    pub fn finish(self) -> Result<WorthQueryPortableConditionalNodeDeclaration, &'static str> {
        WorthQueryPortableConditionalNodeDeclaration::new(
            self.identity,
            self.role,
            self.dependencies
                .ok_or("conditional-node-missing-dependencies")?,
            self.outputs.ok_or("conditional-node-missing-outputs")?,
            self.required_context
                .ok_or("conditional-node-missing-required-context")?,
            self.condition.ok_or("conditional-node-missing-condition")?,
            self.trigger.ok_or("conditional-node-missing-trigger")?,
            self.dependency_comparator
                .ok_or("conditional-node-missing-dependency-comparator")?,
            self.output_equivalence
                .ok_or("conditional-node-missing-output-equivalence")?,
            self.artifact_reuse_equivalence
                .ok_or("conditional-node-missing-artifact-reuse-equivalence")?,
            self.maintenance
                .ok_or("conditional-node-missing-maintenance")?,
            self.artifact.ok_or("conditional-node-missing-artifact")?,
            self.output_relationship
                .ok_or("conditional-node-missing-output-relationship")?,
        )
    }
}
