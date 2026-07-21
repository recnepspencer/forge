use super::{
    WorthQueryWorkflowEffectEvidence, WorthQueryWorkflowPrimaryReadEvidence,
    WorthQueryWorkflowStageExecutionContext, WorthQueryWorkflowStageWorkspace,
};

#[derive(Debug)]
pub enum WorthQueryWorkflowValue {
    NotRequired,
    Bool(bool),
    I64(i64),
    U64(u64),
    Text(String),
    EntityIdentity(String),
    Projection(Box<crate::ordinary::read::WorthQueryReadCompletion>),
}

impl WorthQueryWorkflowValue {
    pub(crate) fn satisfies(
        &self,
        contract: worth_query_installation::facade::WorthQueryWorkflowValueContract,
    ) -> bool {
        use worth_query_installation::facade::WorthQueryWorkflowValueContract as Contract;
        matches!(
            (self, contract),
            (Self::NotRequired, Contract::NotRequired)
                | (Self::Bool(_), Contract::Bool)
                | (Self::I64(_), Contract::I64)
                | (Self::U64(_), Contract::U64)
                | (Self::Text(_), Contract::Text)
                | (Self::EntityIdentity(_), Contract::EntityIdentity)
                | (Self::Projection(_), Contract::Projection)
        )
    }

    pub(crate) fn semantic_part(&self) -> String {
        match self {
            Self::NotRequired => "not-required".into(),
            Self::Bool(value) => format!("bool:{value}"),
            Self::I64(value) => format!("i64:{value}"),
            Self::U64(value) => format!("u64:{value}"),
            Self::Text(value) => format!("text:{value}"),
            Self::EntityIdentity(value) => format!("entity:{value}"),
            Self::Projection(completion) => format!(
                "projection:{}:{}",
                completion.result().receipt().canonical_query_digest(),
                completion.result().receipt().result_digest(),
            ),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthQueryWorkflowStageWarning {
    Advisory(String),
    Partial(String),
}

pub struct WorthQueryWorkflowStageMaterial {
    output: WorthQueryWorkflowValue,
    warnings: Vec<WorthQueryWorkflowStageWarning>,
    result_state: Option<crate::domain_installation::WorthQueryOperationResultState>,
    primary_graph_reads: Vec<WorthQueryWorkflowPrimaryReadEvidence>,
    effects: Vec<WorthQueryWorkflowEffectEvidence>,
    executed_effects: Vec<WorthQueryWorkflowEffectEvidence>,
    lineage: Vec<crate::identity_evolution::InstalledIdentityEvolutionOutcome>,
}

pub(crate) struct WorthQueryWorkflowStageMaterialParts {
    pub(crate) output: WorthQueryWorkflowValue,
    pub(crate) warnings: Vec<WorthQueryWorkflowStageWarning>,
    pub(crate) result_state: Option<crate::domain_installation::WorthQueryOperationResultState>,
    pub(crate) primary_graph_reads: Vec<WorthQueryWorkflowPrimaryReadEvidence>,
    pub(crate) effects: Vec<WorthQueryWorkflowEffectEvidence>,
    pub(crate) executed_effects: Vec<WorthQueryWorkflowEffectEvidence>,
    pub(crate) lineage: Vec<crate::identity_evolution::InstalledIdentityEvolutionOutcome>,
}

impl WorthQueryWorkflowStageMaterial {
    pub fn new(output: WorthQueryWorkflowValue) -> Self {
        Self {
            output,
            warnings: Vec::new(),
            result_state: None,
            primary_graph_reads: Vec::new(),
            effects: Vec::new(),
            executed_effects: Vec::new(),
            lineage: Vec::new(),
        }
    }

    pub fn with_primary_graph_read(
        mut self,
        role: impl Into<String>,
        completion: &crate::ordinary::read::WorthQueryReadCompletion,
    ) -> Self {
        self.primary_graph_reads
            .push(WorthQueryWorkflowPrimaryReadEvidence::from_completion(
                role, completion,
            ));
        self
    }

    pub fn projection(
        role: impl Into<String>,
        completion: crate::ordinary::read::WorthQueryReadCompletion,
    ) -> Self {
        let evidence = WorthQueryWorkflowPrimaryReadEvidence::from_completion(role, &completion);
        Self {
            output: WorthQueryWorkflowValue::Projection(Box::new(completion)),
            warnings: Vec::new(),
            result_state: None,
            primary_graph_reads: vec![evidence],
            effects: Vec::new(),
            executed_effects: Vec::new(),
            lineage: Vec::new(),
        }
    }

    pub fn with_warning(mut self, warning: WorthQueryWorkflowStageWarning) -> Self {
        self.warnings.push(warning);
        self
    }

    pub fn with_result_state(
        mut self,
        result_state: crate::domain_installation::WorthQueryOperationResultState,
    ) -> Self {
        self.result_state = Some(result_state);
        self
    }

    pub fn with_lineage_outcomes(
        mut self,
        lineage: Vec<crate::identity_evolution::InstalledIdentityEvolutionOutcome>,
    ) -> Self {
        self.lineage = lineage;
        self
    }

    pub(crate) fn into_parts(self) -> WorthQueryWorkflowStageMaterialParts {
        WorthQueryWorkflowStageMaterialParts {
            output: self.output,
            warnings: self.warnings,
            result_state: self.result_state,
            primary_graph_reads: self.primary_graph_reads,
            effects: self.effects,
            executed_effects: self.executed_effects,
            lineage: self.lineage,
        }
    }

    pub(crate) fn retain_query_executed_effects(
        &mut self,
        effects: Vec<WorthQueryWorkflowEffectEvidence>,
    ) {
        self.effects = effects.clone();
        self.executed_effects = effects;
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct WorthQueryWorkflowStageExecutorFailure {
    class: worth_query_installation::facade::WorthQueryOperationFailureClass,
    detail: String,
    executed_effects: Vec<WorthQueryWorkflowEffectEvidence>,
}

impl WorthQueryWorkflowStageExecutorFailure {
    pub fn new(
        class: worth_query_installation::facade::WorthQueryOperationFailureClass,
        detail: impl Into<String>,
    ) -> Self {
        Self {
            class,
            detail: detail.into(),
            executed_effects: Vec::new(),
        }
    }
    pub fn class(&self) -> &worth_query_installation::facade::WorthQueryOperationFailureClass {
        &self.class
    }
    pub fn detail(&self) -> &str {
        &self.detail
    }

    pub(crate) fn with_executed_effects(
        mut self,
        executed_effects: Vec<WorthQueryWorkflowEffectEvidence>,
    ) -> Self {
        self.executed_effects = executed_effects;
        self
    }

    pub(crate) fn executed_effects(&self) -> &[WorthQueryWorkflowEffectEvidence] {
        &self.executed_effects
    }
}

pub trait WorthQueryDomainWorkflowStageExecutor<D, O, F>: Send + Sync + 'static {
    const LOWERING_FAMILY: &'static str;
    const DETERMINISTIC: bool;
    const IDEMPOTENT_STAGE_RETRY: bool = false;
    const EXECUTION_COST: crate::domain_installation::WorthQueryOperationCostClass;
    const RESULT_WIDTH_COST: crate::domain_installation::WorthQueryOperationCostClass;
    const REPLAY_COMPARATOR_FAMILY: Option<&'static str> = None;

    fn installed_read_declaration(
        &self,
    ) -> Option<&crate::ordinary::read::WorthQueryReadDeclaration> {
        None
    }

    fn execute_stage(
        &self,
        input: WorthQueryWorkflowValue,
        context: &WorthQueryWorkflowStageExecutionContext<'_>,
        workspace: &mut WorthQueryWorkflowStageWorkspace<'_>,
    ) -> Result<WorthQueryWorkflowStageMaterial, WorthQueryWorkflowStageExecutorFailure>;

    fn prepare_aftermath_intent(
        &self,
        _original: &crate::domain_installation::WorthQueryAftermathOriginalEvidence,
    ) -> Option<crate::domain_installation::WorthQueryNormalizedWorkflowIntent> {
        None
    }

    fn verify_aftermath_postcondition(
        &self,
        _original: &crate::domain_installation::WorthQueryAftermathOriginalEvidence,
        _candidate: &crate::domain_installation::WorthQueryWorkflowTraceSemantics,
    ) -> bool {
        false
    }
}

/// Domain-owned semantic comparison for an executor registered on the
/// certification replay lane. Keeping this separate from stage execution makes
/// comparator presence an installation-time fact rather than a post-effect
/// discovery.
pub trait WorthQueryDomainReplaySemanticComparator<D, O, F>: Send + Sync + 'static {
    fn compare_replay_semantics(
        &self,
        original: &crate::domain_installation::WorthQueryWorkflowTraceSemantics,
        replay: &crate::domain_installation::WorthQueryWorkflowTraceSemantics,
        noise: crate::domain_installation::WorthQueryReplayNoiseContract,
    ) -> crate::domain_installation::WorthQueryReplayComparison;
}
