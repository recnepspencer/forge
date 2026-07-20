use worth_signal::facade::adapters::FrontierRouteEvidenceReceipt;

#[derive(Debug)]
pub struct WorthQueryWorkflowParallelAdmissionCall {
    operation_identity: String,
    binding_identity: String,
    run_identity: String,
    basis_identity: String,
    frontier: Vec<WorthQueryWorkflowParallelFrontierStage>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryWorkflowParallelFrontierStage {
    stage_identity: String,
    predecessor_receipt_identities: Vec<String>,
    graph_read_roles: Vec<String>,
    touch_roles: Vec<String>,
    effect_roles: Vec<worth_query_installation::facade::WorthQueryOperationEffectFamily>,
}

impl WorthQueryWorkflowParallelFrontierStage {
    pub(super) fn new(
        stage_identity: String,
        predecessor_receipt_identities: Vec<String>,
        graph_read_roles: Vec<String>,
        touch_roles: Vec<String>,
        effect_roles: Vec<worth_query_installation::facade::WorthQueryOperationEffectFamily>,
    ) -> Self {
        Self {
            stage_identity,
            predecessor_receipt_identities,
            graph_read_roles,
            touch_roles,
            effect_roles,
        }
    }

    pub fn stage_identity(&self) -> &str {
        &self.stage_identity
    }

    pub fn predecessor_receipt_identities(&self) -> &[String] {
        &self.predecessor_receipt_identities
    }

    pub fn graph_read_roles(&self) -> &[String] {
        &self.graph_read_roles
    }

    pub fn touch_roles(&self) -> &[String] {
        &self.touch_roles
    }

    pub fn effect_roles(
        &self,
    ) -> &[worth_query_installation::facade::WorthQueryOperationEffectFamily] {
        &self.effect_roles
    }
}

impl WorthQueryWorkflowParallelAdmissionCall {
    pub(super) fn new(
        operation_identity: &str,
        binding_identity: &str,
        run_identity: &str,
        basis_identity: &str,
        frontier: Vec<WorthQueryWorkflowParallelFrontierStage>,
    ) -> Self {
        Self {
            operation_identity: operation_identity.into(),
            binding_identity: binding_identity.into(),
            run_identity: run_identity.into(),
            basis_identity: basis_identity.into(),
            frontier,
        }
    }

    pub fn operation_identity(&self) -> &str {
        &self.operation_identity
    }

    pub fn binding_identity(&self) -> &str {
        &self.binding_identity
    }

    pub fn run_identity(&self) -> &str {
        &self.run_identity
    }

    pub fn basis_identity(&self) -> &str {
        &self.basis_identity
    }

    pub fn frontier(&self) -> &[WorthQueryWorkflowParallelFrontierStage] {
        &self.frontier
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryWorkflowParallelAdmissionFailure {
    detail: String,
}

impl WorthQueryWorkflowParallelAdmissionFailure {
    pub fn new(detail: impl Into<String>) -> Self {
        Self {
            detail: detail.into(),
        }
    }

    pub fn detail(&self) -> &str {
        &self.detail
    }
}

pub trait WorthQueryWorkflowParallelAdmissionProvider<D, O, F>: Send + Sync + 'static {
    fn admit_parallel_frontier(
        &self,
        call: &WorthQueryWorkflowParallelAdmissionCall,
    ) -> Result<FrontierRouteEvidenceReceipt, WorthQueryWorkflowParallelAdmissionFailure>;
}

#[derive(Debug)]
pub struct WorthQueryWorkflowParallelAdmissionReceipt {
    identity: String,
    run_identity: String,
    frontier: Vec<WorthQueryWorkflowParallelFrontierStage>,
    lower_receipt: FrontierRouteEvidenceReceipt,
}

impl WorthQueryWorkflowParallelAdmissionReceipt {
    pub(super) fn mint(
        call: &WorthQueryWorkflowParallelAdmissionCall,
        lower_receipt: FrontierRouteEvidenceReceipt,
    ) -> Self {
        let identity = crate::identity::hash_parts(&[
            "worth_query_workflow_parallel_admission_v1".into(),
            format!("operation:{}", call.operation_identity),
            format!("binding:{}", call.binding_identity),
            format!("run:{}", call.run_identity),
            format!("basis:{}", call.basis_identity),
            format!(
                "frontier:{}",
                call.frontier
                    .iter()
                    .map(|stage| format!(
                        "{}:[{}]:reads[{}]:touches[{}]:effects[{}]",
                        stage.stage_identity,
                        stage.predecessor_receipt_identities.join(","),
                        stage.graph_read_roles.join(","),
                        stage.touch_roles.join(","),
                        stage
                            .effect_roles
                            .iter()
                            .map(|effect| effect.as_str())
                            .collect::<Vec<_>>()
                            .join(",")
                    ))
                    .collect::<Vec<_>>()
                    .join("|")
            ),
            format!("lower_reason:{:?}", lower_receipt.reason()),
        ]);
        Self {
            identity,
            run_identity: call.run_identity.clone(),
            frontier: call.frontier.clone(),
            lower_receipt,
        }
    }

    pub fn identity(&self) -> &str {
        &self.identity
    }

    pub fn run_identity(&self) -> &str {
        &self.run_identity
    }

    pub fn frontier(&self) -> &[WorthQueryWorkflowParallelFrontierStage] {
        &self.frontier
    }

    pub fn lower_receipt(&self) -> FrontierRouteEvidenceReceipt {
        self.lower_receipt
    }
}
