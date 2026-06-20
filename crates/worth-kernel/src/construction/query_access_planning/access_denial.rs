use forge_query::facade::{
    ForgeQueryGraphReadAccessAdmission, ForgeQueryGraphReadAccessAdmissionPosture,
    ForgeQueryGraphReadAccessDenialKind, ForgeQueryRuntimeError,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct PrimitiveConstructionQueryAccessDenial {
    admission: ForgeQueryGraphReadAccessAdmission,
    admission_digest: String,
    admission_posture: ForgeQueryGraphReadAccessAdmissionPosture,
    denial_kind: Option<ForgeQueryGraphReadAccessDenialKind>,
    suggested_posture: Option<ForgeQueryGraphReadAccessAdmissionPosture>,
    executor_entry_count: usize,
    materialized_row_count: usize,
}

#[derive(Debug)]
pub(crate) enum PrimitiveConstructionQueryAccessError {
    Authority(String),
    Lowering(String),
    Runtime(ForgeQueryRuntimeError),
    AccessDenied(PrimitiveConstructionQueryAccessDenial),
    MissingExecutedPlan,
    PlanDigestDrift {
        planned_digest: String,
        executed_digest: String,
    },
    MissingPlanConsumption,
}

impl PrimitiveConstructionQueryAccessDenial {
    pub(crate) fn new(admission: ForgeQueryGraphReadAccessAdmission) -> Self {
        let denial = admission.denial();
        Self {
            admission_digest: admission.digest().to_string(),
            admission_posture: admission.posture().clone(),
            denial_kind: denial.map(|denial| denial.kind().clone()),
            suggested_posture: denial.map(|denial| denial.suggested_posture().clone()),
            admission,
            executor_entry_count: 0,
            materialized_row_count: 0,
        }
    }

    pub(crate) fn admission_digest(&self) -> &str {
        &self.admission_digest
    }

    pub(crate) fn admission_posture(&self) -> &ForgeQueryGraphReadAccessAdmissionPosture {
        &self.admission_posture
    }

    pub(crate) fn denial_kind(&self) -> Option<&ForgeQueryGraphReadAccessDenialKind> {
        self.denial_kind.as_ref()
    }

    pub(crate) fn suggested_posture(&self) -> Option<&ForgeQueryGraphReadAccessAdmissionPosture> {
        self.suggested_posture.as_ref()
    }

    pub(crate) fn executor_entry_count(&self) -> usize {
        self.executor_entry_count
    }

    pub(crate) fn materialized_row_count(&self) -> usize {
        self.materialized_row_count
    }
}

impl From<ForgeQueryRuntimeError> for PrimitiveConstructionQueryAccessError {
    fn from(error: ForgeQueryRuntimeError) -> Self {
        Self::Runtime(error)
    }
}

impl std::fmt::Display for PrimitiveConstructionQueryAccessError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Authority(message) => write!(formatter, "authority denial: {message}"),
            Self::Lowering(message) => write!(formatter, "query access lowering denial: {message}"),
            Self::Runtime(error) => write!(formatter, "query runtime denial: {error}"),
            Self::AccessDenied(denial) => write!(
                formatter,
                "graph-read access denied before execution: admission={}, posture={}, denial={:?}",
                denial.admission_digest(),
                denial.admission_posture().as_str(),
                denial.denial_kind()
            ),
            Self::MissingExecutedPlan => {
                formatter.write_str("executed read receipt did not carry graph access plan")
            }
            Self::PlanDigestDrift {
                planned_digest,
                executed_digest,
            } => write!(
                formatter,
                "executed read consumed graph access plan `{executed_digest}` instead of planned `{planned_digest}`"
            ),
            Self::MissingPlanConsumption => {
                formatter.write_str("executed read receipt did not carry plan consumption proof")
            }
        }
    }
}

impl std::error::Error for PrimitiveConstructionQueryAccessError {}
