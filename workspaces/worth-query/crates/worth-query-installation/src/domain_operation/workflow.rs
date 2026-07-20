#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthQueryOperationWorkflowContract {
    NotRequired,
    Declared(WorthQueryPortableWorkflowDefinition),
}

impl WorthQueryOperationWorkflowContract {
    pub(crate) fn canonicalize(&mut self) {
        if let Self::Declared(workflow) = self {
            for stage in &mut workflow.stages {
                stage.semantics.canonicalize();
            }
            workflow
                .stages
                .sort_by(|left, right| left.identity.cmp(&right.identity));
        }
    }
}

impl WorthQueryWorkflowStageSemantics {
    fn canonicalize(&mut self) {
        for values in [
            &mut self.graph_read_roles,
            &mut self.touch_roles,
            &mut self.invariant_roles,
        ] {
            values.sort();
            values.dedup();
        }
        self.effect_roles.sort();
        self.effect_roles.dedup();
        self.cost_roles.sort();
        self.cost_roles.dedup();
        self.required_domain_roles.sort();
        self.required_domain_roles.dedup();
        self.terminal_result_states.sort();
        self.terminal_result_states.dedup();
        self.failure_classes.sort();
        self.failure_classes.dedup();
        super::conditional_node::canonicalize_conditional_nodes(&mut self.conditional_nodes);
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryPortableWorkflowDefinition {
    entry_stage: String,
    stages: Vec<WorthQueryPortableWorkflowStage>,
}

impl WorthQueryPortableWorkflowDefinition {
    pub fn new(
        entry_stage: impl Into<String>,
        stages: impl IntoIterator<Item = WorthQueryPortableWorkflowStage>,
    ) -> Self {
        Self {
            entry_stage: entry_stage.into(),
            stages: stages.into_iter().collect(),
        }
    }

    pub fn entry_stage(&self) -> &str {
        &self.entry_stage
    }

    pub fn stages(&self) -> &[WorthQueryPortableWorkflowStage] {
        &self.stages
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryPortableWorkflowStage {
    identity: String,
    predecessors: Vec<String>,
    terminal: bool,
    publishable: bool,
    required_capabilities: Vec<super::WorthQueryOperationCapabilityRequirement>,
    semantics: WorthQueryWorkflowStageSemantics,
}

impl WorthQueryPortableWorkflowStage {
    pub fn new(
        identity: impl Into<String>,
        predecessors: impl IntoIterator<Item = impl Into<String>>,
        terminal: bool,
        publishable: bool,
        required_capabilities: impl IntoIterator<Item = super::WorthQueryOperationCapabilityRequirement>,
    ) -> Self {
        let mut predecessors = predecessors.into_iter().map(Into::into).collect::<Vec<_>>();
        predecessors.sort();
        predecessors.dedup();
        let mut required_capabilities = required_capabilities.into_iter().collect::<Vec<_>>();
        required_capabilities.sort();
        required_capabilities.dedup();
        Self {
            identity: identity.into(),
            predecessors,
            terminal,
            publishable,
            required_capabilities,
            semantics: WorthQueryWorkflowStageSemantics::default(),
        }
    }

    pub fn with_semantics(mut self, semantics: WorthQueryWorkflowStageSemantics) -> Self {
        self.semantics = semantics;
        self
    }

    pub fn identity(&self) -> &str {
        &self.identity
    }

    pub fn predecessors(&self) -> &[String] {
        &self.predecessors
    }

    pub fn is_terminal(&self) -> bool {
        self.terminal
    }

    pub fn is_publishable(&self) -> bool {
        self.publishable
    }

    pub fn required_capabilities(&self) -> &[super::WorthQueryOperationCapabilityRequirement] {
        &self.required_capabilities
    }

    pub fn semantics(&self) -> &WorthQueryWorkflowStageSemantics {
        &self.semantics
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct WorthQueryWorkflowStageSemantics {
    pub input: WorthQueryWorkflowValueContract,
    pub output: WorthQueryWorkflowValueContract,
    pub required_domain_roles: Vec<super::WorthQueryOperationRequiredDomainRole>,
    pub graph_read_roles: Vec<String>,
    pub touch_roles: Vec<String>,
    pub effect_roles: Vec<super::WorthQueryOperationEffectFamily>,
    pub invariant_roles: Vec<String>,
    pub cost_roles: Vec<WorthQueryWorkflowCostRole>,
    pub terminal_result_states: Vec<super::WorthQueryOperationResultState>,
    pub failure_classes: Vec<super::WorthQueryOperationFailureClass>,
    pub conditional_nodes: Vec<super::WorthQueryPortableConditionalNodeDeclaration>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum WorthQueryWorkflowCostRole {
    Admission,
    GraphRead,
    TouchEffect,
    CommitAdmission,
    Effect,
    Invariant,
    Execution,
    ResultValidation,
}

impl WorthQueryWorkflowCostRole {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Admission => "admission",
            Self::GraphRead => "graph-read",
            Self::TouchEffect => "touch-effect",
            Self::CommitAdmission => "commit-admission",
            Self::Effect => "effect",
            Self::Invariant => "invariant",
            Self::Execution => "execution",
            Self::ResultValidation => "result-validation",
        }
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum WorthQueryWorkflowValueContract {
    #[default]
    NotRequired,
    Bool,
    I64,
    U64,
    Text,
    EntityIdentity,
    Projection,
}
