use super::{
    WorthQueryWorkflowEffectEvidence, WorthQueryWorkflowPredecessorReceipt,
    WorthQueryWorkflowPrimaryReadEvidence, WorthQueryWorkflowStageReceipt,
    WorthQueryWorkflowStageWorkspace,
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
}

pub(crate) struct WorthQueryWorkflowStageMaterialParts {
    pub(crate) output: WorthQueryWorkflowValue,
    pub(crate) warnings: Vec<WorthQueryWorkflowStageWarning>,
    pub(crate) result_state: Option<crate::domain_installation::WorthQueryOperationResultState>,
    pub(crate) primary_graph_reads: Vec<WorthQueryWorkflowPrimaryReadEvidence>,
    pub(crate) effects: Vec<WorthQueryWorkflowEffectEvidence>,
    pub(crate) executed_effects: Vec<WorthQueryWorkflowEffectEvidence>,
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

    pub(crate) fn into_parts(self) -> WorthQueryWorkflowStageMaterialParts {
        WorthQueryWorkflowStageMaterialParts {
            output: self.output,
            warnings: self.warnings,
            result_state: self.result_state,
            primary_graph_reads: self.primary_graph_reads,
            effects: self.effects,
            executed_effects: self.executed_effects,
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

pub struct WorthQueryWorkflowStageExecutionContext<'a> {
    operation_identity: &'a str,
    run_identity: &'a str,
    stage: &'a worth_query_installation::facade::WorthQueryPortableWorkflowStage,
    predecessor_receipts: Vec<WorthQueryWorkflowPredecessorReceipt<'a>>,
    effect_workflow_binding: crate::workflow::WorkflowContextBinding,
    basis: crate::basis_lifecycle::BasisFamily,
    installed_read: Option<&'a crate::ordinary::read::WorthQueryReadDeclaration>,
    operation_graph_reads:
        &'a [worth_query_installation::facade::WorthQueryOperationGraphReadRole],
    graph_receipts: &'a [super::WorthQueryBoundGraphExecutionReceipt],
}

pub(crate) struct WorthQueryWorkflowStageExecutionAuthority<'a> {
    pub(crate) effect_workflow_binding: crate::workflow::WorkflowContextBinding,
    pub(crate) basis: crate::basis_lifecycle::BasisFamily,
    pub(crate) installed_read: Option<&'a crate::ordinary::read::WorthQueryReadDeclaration>,
    pub(crate) operation_graph_reads:
        &'a [worth_query_installation::facade::WorthQueryOperationGraphReadRole],
    pub(crate) graph_receipts: &'a [super::WorthQueryBoundGraphExecutionReceipt],
}

impl<'a> WorthQueryWorkflowStageExecutionContext<'a> {
    pub(crate) fn new(
        operation_identity: &'a str,
        run_identity: &'a str,
        stage: &'a worth_query_installation::facade::WorthQueryPortableWorkflowStage,
        predecessor_receipts: &'a [&'a WorthQueryWorkflowStageReceipt],
        authority: WorthQueryWorkflowStageExecutionAuthority<'a>,
    ) -> Self {
        Self {
            operation_identity,
            run_identity,
            stage,
            predecessor_receipts: predecessor_receipts
                .iter()
                .map(|receipt| WorthQueryWorkflowPredecessorReceipt::new(receipt))
                .collect(),
            effect_workflow_binding: authority.effect_workflow_binding,
            basis: authority.basis,
            installed_read: authority.installed_read,
            operation_graph_reads: authority.operation_graph_reads,
            graph_receipts: authority.graph_receipts,
        }
    }

    pub fn operation_identity(&self) -> &str {
        self.operation_identity
    }
    pub fn run_identity(&self) -> &str {
        self.run_identity
    }
    pub fn stage(&self) -> &worth_query_installation::facade::WorthQueryPortableWorkflowStage {
        self.stage
    }
    pub fn predecessor_receipts(&self) -> &[WorthQueryWorkflowPredecessorReceipt<'a>] {
        &self.predecessor_receipts
    }
    pub(crate) fn effect_workflow_binding(&self) -> &crate::workflow::WorkflowContextBinding {
        &self.effect_workflow_binding
    }

    pub fn graph_projection(&self, role: &str) -> Option<&crate::runtime::WorthQueryReadResult> {
        if !self
            .stage
            .semantics()
            .graph_read_roles
            .iter()
            .any(|declared| declared == role)
        {
            return None;
        }
        self.graph_receipts
            .iter()
            .find(|receipt| {
                receipt.role() == role
                    && receipt.kind()
                        == crate::domain_installation::WorthQueryGraphProviderCallKind::Project
            })
            .and_then(super::WorthQueryBoundGraphExecutionReceipt::projection)
    }

    pub fn execute_mutation(
        &self,
        command: crate::runtime::WorthQueryWriteCommand,
        workspace: &mut WorthQueryWorkflowStageWorkspace<'_>,
    ) -> Result<WorthQueryWorkflowEffectEvidence, super::WorthQueryWorkflowStageEffectDenial> {
        if !self
            .stage
            .semantics()
            .effect_roles
            .contains(&worth_query_installation::facade::WorthQueryOperationEffectFamily::Mutation)
        {
            return Err(super::WorthQueryWorkflowStageEffectDenial::UndeclaredEffectFamily);
        }
        let execution = workspace
            .workspace
            .execute_ordinary_authoritative_mutation(command, false)
            .map_err(|error| {
                super::WorthQueryWorkflowStageEffectDenial::Runtime(format!("{error:?}"))
            })?;
        let evidence = WorthQueryWorkflowEffectEvidence::runtime_mutation(
            execution.into_receipt(),
            &self.effect_workflow_binding,
            self.basis,
        );
        workspace.executed_effects.push(evidence.clone());
        Ok(evidence)
    }

    pub fn execute_installed_read(
        &self,
        role: &str,
        workspace: &mut WorthQueryWorkflowStageWorkspace<'_>,
    ) -> Result<
        crate::ordinary::read::WorthQueryReadCompletion,
        WorthQueryWorkflowStageExecutorFailure,
    > {
        let role_is_admitted = self
            .stage
            .semantics()
            .graph_read_roles
            .iter()
            .any(|declared| declared == role)
            && self.operation_graph_reads.iter().any(|declared| {
                declared.role == role
                    && declared.participation
                        == worth_query_installation::facade::WorthQueryOperationGraphParticipation::PrimaryLogicalGraph
            });
        if !role_is_admitted {
            return Err(WorthQueryWorkflowStageExecutorFailure::new(
                crate::domain_installation::WorthQueryOperationFailureClass::Indeterminate,
                "workflow stage lacks the installed primary read role",
            ));
        }
        let declaration = self.installed_read.ok_or_else(|| {
            WorthQueryWorkflowStageExecutorFailure::new(
                crate::domain_installation::WorthQueryOperationFailureClass::Indeterminate,
                "workflow operation has no Query-installed read declaration",
            )
        })?;
        if workspace.installed_read_executions != 0 {
            return Err(WorthQueryWorkflowStageExecutorFailure::new(
                crate::domain_installation::WorthQueryOperationFailureClass::Indeterminate,
                "installed canonical read may execute only once per workflow stage",
            ));
        }
        workspace.installed_read_executions += 1;
        declaration
            .clone_for_installed_execution()
            .using(crate::ordinary::read::current())
            .run(workspace.workspace)
            .into_result()
            .map_err(|stop| {
                WorthQueryWorkflowStageExecutorFailure::new(
                    crate::domain_installation::WorthQueryOperationFailureClass::Dependency,
                    format!("{stop:?}"),
                )
            })
    }

    pub(crate) fn requires_primary_read(&self) -> bool {
        self.operation_graph_reads.iter().any(|declared| {
            self.stage
                .semantics()
                .graph_read_roles
                .contains(&declared.role)
                && declared.participation
                    == worth_query_installation::facade::WorthQueryOperationGraphParticipation::PrimaryLogicalGraph
        })
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
    const EXECUTION_COST: crate::domain_installation::WorthQueryOperationCostClass;
    const RESULT_WIDTH_COST: crate::domain_installation::WorthQueryOperationCostClass;

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
}
