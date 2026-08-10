use std::collections::{BTreeMap, BTreeSet};

use crate::declarative_live::DeclarativeLiveQueryRequest;
use crate::schema_view::QuerySchemaView;

use super::derived_views::WorthQueryDerivedView;
use super::error::WorthQueryProgramError;
use super::ports::WorthQueryTypedPort;
use super::write_commands::WorthQueryWriteCommandTemplate;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct WorthQueryProgramOperationIdentity {
    value: String,
}

impl WorthQueryProgramOperationIdentity {
    pub(crate) fn from_operation_id(operation_id: impl Into<String>) -> Self {
        Self {
            value: operation_id.into(),
        }
    }

    pub fn as_str(&self) -> &str {
        &self.value
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum WorthQueryAuthorityRequirement {
    ReadOnly,
    Live,
    BranchLocal,
    Previewable,
    Writeback,
    Merge,
    Destructive,
    ReplayRequired,
}

impl WorthQueryAuthorityRequirement {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::ReadOnly => "read_only",
            Self::Live => "live",
            Self::BranchLocal => "branch_local",
            Self::Previewable => "previewable",
            Self::Writeback => "writeback",
            Self::Merge => "merge",
            Self::Destructive => "destructive",
            Self::ReplayRequired => "replay_required",
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum WorthQueryProgramEffect {
    DeclareLiveView {
        name: String,
        request: DeclarativeLiveQueryRequest,
        schema_view: QuerySchemaView,
    },
    DeclareDerivedView(WorthQueryDerivedView),
    Write(crate::runtime::WorthQueryWriteCommand),
    WriteTemplate(WorthQueryWriteCommandTemplate),
    ReadLive {
        view_name: String,
    },
    DrainPatches {
        view_name: String,
    },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryWorkflowGraph {
    operation_order: Vec<String>,
}

impl WorthQueryWorkflowGraph {
    pub fn linear(operation_order: impl IntoIterator<Item = String>) -> Self {
        Self {
            operation_order: operation_order.into_iter().collect(),
        }
    }

    pub fn operation_order(&self) -> &[String] {
        &self.operation_order
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct WorthQueryOperation {
    id: String,
    inputs: Vec<WorthQueryTypedPort>,
    outputs: Vec<WorthQueryTypedPort>,
    authority_requirements: BTreeSet<WorthQueryAuthorityRequirement>,
    effects: Vec<WorthQueryProgramEffect>,
}

impl WorthQueryOperation {
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            inputs: Vec::new(),
            outputs: Vec::new(),
            authority_requirements: BTreeSet::new(),
            effects: Vec::new(),
        }
    }

    pub fn with_input(mut self, input: WorthQueryTypedPort) -> Self {
        self.inputs.push(input);
        self
    }

    pub fn with_output(mut self, output: WorthQueryTypedPort) -> Self {
        self.outputs.push(output);
        self
    }

    pub fn requires(mut self, requirement: WorthQueryAuthorityRequirement) -> Self {
        self.authority_requirements.insert(requirement);
        self
    }

    pub fn with_effect(mut self, effect: WorthQueryProgramEffect) -> Self {
        self.effects.push(effect);
        self
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn inputs(&self) -> &[WorthQueryTypedPort] {
        &self.inputs
    }

    pub fn outputs(&self) -> &[WorthQueryTypedPort] {
        &self.outputs
    }

    pub fn authority_requirements(&self) -> &BTreeSet<WorthQueryAuthorityRequirement> {
        &self.authority_requirements
    }

    pub fn effects(&self) -> &[WorthQueryProgramEffect] {
        &self.effects
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct WorthQueryProgram {
    id: String,
    operations: BTreeMap<WorthQueryProgramOperationIdentity, WorthQueryOperation>,
    workflow_graph: WorthQueryWorkflowGraph,
}

impl WorthQueryProgram {
    pub fn new(
        id: impl Into<String>,
        operations: impl IntoIterator<Item = WorthQueryOperation>,
    ) -> Result<Self, WorthQueryProgramError> {
        let operations = operations
            .into_iter()
            .map(|operation| {
                (
                    WorthQueryProgramOperationIdentity::from_operation_id(operation.id.clone()),
                    operation,
                )
            })
            .collect::<BTreeMap<_, _>>();
        if operations.is_empty() {
            return Err(WorthQueryProgramError::new(
                "program must declare at least one operation",
            ));
        }
        let workflow_graph =
            WorthQueryWorkflowGraph::linear(operations.keys().map(|key| key.as_str().to_string()));
        Ok(Self {
            id: id.into(),
            operations,
            workflow_graph,
        })
    }

    pub fn compile<S, A>(domain_ir: S, schema_adapter: &A) -> Result<Self, WorthQueryProgramError>
    where
        S: WorthQueryProgramSource,
        A: WorthQuerySchemaAdapter + ?Sized,
    {
        domain_ir.compile_program(schema_adapter)
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn operation(&self, id: &str) -> Option<&WorthQueryOperation> {
        self.operations
            .get(&WorthQueryProgramOperationIdentity::from_operation_id(id))
    }

    pub fn operations(&self) -> impl Iterator<Item = &WorthQueryOperation> {
        self.operations.values()
    }

    pub fn workflow_graph(&self) -> &WorthQueryWorkflowGraph {
        &self.workflow_graph
    }
}

pub trait WorthQuerySchemaAdapter {
    fn schema_view(&self, operation_id: &str) -> Option<QuerySchemaView>;
}

pub trait WorthQueryProgramSource {
    fn compile_program<A>(
        self,
        schema_adapter: &A,
    ) -> Result<WorthQueryProgram, WorthQueryProgramError>
    where
        A: WorthQuerySchemaAdapter + ?Sized;
}
