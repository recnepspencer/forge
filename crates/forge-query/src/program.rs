use std::collections::{BTreeMap, BTreeSet};

use serde_json::Value;

use crate::declarative_live::DeclarativeLiveQueryRequest;
use crate::schema_view::QuerySchemaView;

#[derive(Clone, Debug, PartialEq)]
pub enum ForgeQueryValueExpr {
    Literal(Value),
    Input(String),
    Object(BTreeMap<String, ForgeQueryValueExpr>),
    Array(Vec<ForgeQueryValueExpr>),
}

impl ForgeQueryValueExpr {
    pub fn literal(value: Value) -> Self {
        Self::Literal(value)
    }

    pub fn input(name: impl Into<String>) -> Self {
        Self::Input(name.into())
    }

    pub fn object(fields: impl IntoIterator<Item = (String, ForgeQueryValueExpr)>) -> Self {
        Self::Object(fields.into_iter().collect())
    }

    pub fn array(items: impl IntoIterator<Item = ForgeQueryValueExpr>) -> Self {
        Self::Array(items.into_iter().collect())
    }

    pub(crate) fn evaluate(
        &self,
        inputs: &BTreeMap<String, Value>,
    ) -> Result<Value, ForgeQueryProgramError> {
        match self {
            Self::Literal(value) => Ok(value.clone()),
            Self::Input(name) => inputs.get(name).cloned().ok_or_else(|| {
                ForgeQueryProgramError::new(format!("missing bound input `{name}`"))
            }),
            Self::Object(fields) => fields
                .iter()
                .map(|(key, value)| Ok((key.clone(), value.evaluate(inputs)?)))
                .collect::<Result<serde_json::Map<_, _>, _>>()
                .map(Value::Object),
            Self::Array(items) => items
                .iter()
                .map(|item| item.evaluate(inputs))
                .collect::<Result<Vec<_>, _>>()
                .map(Value::Array),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ForgeQueryAspectValueTemplate {
    aspect_path: String,
    value: ForgeQueryValueExpr,
}

impl ForgeQueryAspectValueTemplate {
    pub fn new(aspect_path: impl Into<String>, value: ForgeQueryValueExpr) -> Self {
        Self {
            aspect_path: aspect_path.into(),
            value,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum ForgeQueryWriteCommandTemplate {
    InsertAspects {
        collection: String,
        aspects: Vec<ForgeQueryAspectValueTemplate>,
    },
    UpdateAspect {
        entity_identity: ForgeQueryValueExpr,
        aspect_path: String,
        value: ForgeQueryValueExpr,
    },
    Delete {
        entity_identity: ForgeQueryValueExpr,
    },
}

impl ForgeQueryWriteCommandTemplate {
    pub(crate) fn bind(
        &self,
        inputs: &BTreeMap<String, Value>,
    ) -> Result<crate::runtime::ForgeQueryWriteCommand, ForgeQueryProgramError> {
        match self {
            Self::InsertAspects {
                collection,
                aspects,
            } => Ok(crate::runtime::ForgeQueryWriteCommand::InsertAspects {
                collection: collection.clone(),
                aspects: aspects
                    .iter()
                    .map(|aspect| {
                        crate::runtime::ForgeQueryAspectValue::new_set(
                            aspect.aspect_path.clone(),
                            aspect.value.evaluate(inputs)?,
                        )
                        .map_err(|error| ForgeQueryProgramError::new(error.to_string()))
                    })
                    .collect::<Result<Vec<_>, _>>()?,
                symbolic_aspect_references: Vec::new(),
                metadata: crate::runtime::ForgeQueryMutationMetadata::default(),
                naming_intent: None,
                continuity_intent: None,
                symbolic_target_reference: None,
            }),
            Self::UpdateAspect {
                entity_identity,
                aspect_path,
                value,
            } => Ok(crate::runtime::ForgeQueryWriteCommand::UpdateAspect {
                entity_identity: expect_string(
                    entity_identity.evaluate(inputs)?,
                    "entity_identity",
                )?,
                aspect_path: aspect_path.clone(),
                value: value.evaluate(inputs)?,
            }),
            Self::Delete { entity_identity } => {
                Ok(crate::runtime::ForgeQueryWriteCommand::Delete {
                    entity_identity: expect_string(
                        entity_identity.evaluate(inputs)?,
                        "entity_identity",
                    )?,
                })
            }
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ForgeQueryPortType {
    String,
    Integer,
    Boolean,
    Json,
    EntityIdentity,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryTypedPort {
    name: String,
    port_type: ForgeQueryPortType,
    optional: bool,
    required_aspects: Vec<String>,
    binding_slot: Option<String>,
    result_shape: Option<String>,
}

impl ForgeQueryTypedPort {
    pub fn new(name: impl Into<String>, port_type: ForgeQueryPortType) -> Self {
        Self {
            name: name.into(),
            port_type,
            optional: false,
            required_aspects: Vec::new(),
            binding_slot: None,
            result_shape: None,
        }
    }

    pub fn optional(mut self) -> Self {
        self.optional = true;
        self
    }

    pub fn with_required_aspect(mut self, aspect: impl Into<String>) -> Self {
        self.required_aspects.push(aspect.into());
        self
    }

    pub fn with_binding_slot(mut self, slot: impl Into<String>) -> Self {
        self.binding_slot = Some(slot.into());
        self
    }

    pub fn with_result_shape(mut self, shape: impl Into<String>) -> Self {
        self.result_shape = Some(shape.into());
        self
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn port_type(&self) -> &ForgeQueryPortType {
        &self.port_type
    }

    pub fn optionality(&self) -> bool {
        self.optional
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ForgeQueryOperationInput {
    name: String,
    value: Value,
}

impl ForgeQueryOperationInput {
    pub fn new(name: impl Into<String>, value: Value) -> Self {
        Self {
            name: name.into(),
            value,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn value(&self) -> &Value {
        &self.value
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ForgeQueryOperationOutput {
    name: String,
    value: Value,
}

impl ForgeQueryOperationOutput {
    pub fn new(name: impl Into<String>, value: Value) -> Self {
        Self {
            name: name.into(),
            value,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn value(&self) -> &Value {
        &self.value
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum ForgeQueryAuthorityRequirement {
    ReadOnly,
    Live,
    BranchLocal,
    Previewable,
    Writeback,
    Merge,
    Destructive,
    ReplayRequired,
}

impl ForgeQueryAuthorityRequirement {
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

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryDerivedView {
    name: String,
    dependency_aspects: Vec<String>,
    produced_aspects: Vec<String>,
    upstream_live_views: Vec<String>,
    upstream_derived_views: Vec<String>,
    incremental: bool,
}

impl ForgeQueryDerivedView {
    pub fn new(
        name: impl Into<String>,
        dependency_aspects: impl IntoIterator<Item = String>,
    ) -> Self {
        Self {
            name: name.into(),
            dependency_aspects: dependency_aspects.into_iter().collect(),
            produced_aspects: Vec::new(),
            upstream_live_views: Vec::new(),
            upstream_derived_views: Vec::new(),
            incremental: true,
        }
    }

    pub fn produces(mut self, aspects: impl IntoIterator<Item = String>) -> Self {
        self.produced_aspects = aspects.into_iter().collect();
        self
    }

    pub fn depends_on_live<T>(mut self, view: &crate::runtime::ForgeQueryLiveView<T>) -> Self {
        self.upstream_live_views.push(view.name().to_string());
        self
    }

    pub fn depends_on_derived<T>(
        mut self,
        view: &crate::runtime::ForgeQueryDerivedViewHandle<T>,
    ) -> Self {
        self.upstream_derived_views.push(view.name().to_string());
        self
    }

    pub fn depends_on_live_name(mut self, name: impl Into<String>) -> Self {
        self.upstream_live_views.push(name.into());
        self
    }

    pub fn depends_on_derived_name(mut self, name: impl Into<String>) -> Self {
        self.upstream_derived_views.push(name.into());
        self
    }

    pub fn whole_refresh_fallback(mut self) -> Self {
        self.incremental = false;
        self
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn dependency_aspects(&self) -> &[String] {
        &self.dependency_aspects
    }

    pub fn produced_aspects(&self) -> &[String] {
        &self.produced_aspects
    }

    pub fn upstream_live_views(&self) -> &[String] {
        &self.upstream_live_views
    }

    pub fn upstream_derived_views(&self) -> &[String] {
        &self.upstream_derived_views
    }

    pub fn incremental(&self) -> bool {
        self.incremental
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum ForgeQueryProgramEffect {
    DeclareLiveView {
        name: String,
        request: DeclarativeLiveQueryRequest,
        schema_view: QuerySchemaView,
    },
    DeclareDerivedView(ForgeQueryDerivedView),
    Write(crate::runtime::ForgeQueryWriteCommand),
    WriteTemplate(ForgeQueryWriteCommandTemplate),
    ReadLive {
        view_name: String,
    },
    DrainPatches {
        view_name: String,
    },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryWorkflowGraph {
    operation_order: Vec<String>,
}

impl ForgeQueryWorkflowGraph {
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
pub struct ForgeQueryOperation {
    id: String,
    inputs: Vec<ForgeQueryTypedPort>,
    outputs: Vec<ForgeQueryTypedPort>,
    authority_requirements: BTreeSet<ForgeQueryAuthorityRequirement>,
    effects: Vec<ForgeQueryProgramEffect>,
}

impl ForgeQueryOperation {
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            inputs: Vec::new(),
            outputs: Vec::new(),
            authority_requirements: BTreeSet::new(),
            effects: Vec::new(),
        }
    }

    pub fn with_input(mut self, input: ForgeQueryTypedPort) -> Self {
        self.inputs.push(input);
        self
    }

    pub fn with_output(mut self, output: ForgeQueryTypedPort) -> Self {
        self.outputs.push(output);
        self
    }

    pub fn requires(mut self, requirement: ForgeQueryAuthorityRequirement) -> Self {
        self.authority_requirements.insert(requirement);
        self
    }

    pub fn with_effect(mut self, effect: ForgeQueryProgramEffect) -> Self {
        self.effects.push(effect);
        self
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn inputs(&self) -> &[ForgeQueryTypedPort] {
        &self.inputs
    }

    pub fn outputs(&self) -> &[ForgeQueryTypedPort] {
        &self.outputs
    }

    pub fn authority_requirements(&self) -> &BTreeSet<ForgeQueryAuthorityRequirement> {
        &self.authority_requirements
    }

    pub fn effects(&self) -> &[ForgeQueryProgramEffect] {
        &self.effects
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ForgeQueryProgram {
    id: String,
    operations: BTreeMap<String, ForgeQueryOperation>,
    workflow_graph: ForgeQueryWorkflowGraph,
}

impl ForgeQueryProgram {
    pub fn new(
        id: impl Into<String>,
        operations: impl IntoIterator<Item = ForgeQueryOperation>,
    ) -> Result<Self, ForgeQueryProgramError> {
        let operations = operations
            .into_iter()
            .map(|operation| (operation.id.clone(), operation))
            .collect::<BTreeMap<_, _>>();
        if operations.is_empty() {
            return Err(ForgeQueryProgramError::new(
                "program must declare at least one operation",
            ));
        }
        let workflow_graph = ForgeQueryWorkflowGraph::linear(operations.keys().cloned());
        Ok(Self {
            id: id.into(),
            operations,
            workflow_graph,
        })
    }

    pub fn compile<S, A>(domain_ir: S, schema_adapter: &A) -> Result<Self, ForgeQueryProgramError>
    where
        S: ForgeQueryProgramSource,
        A: ForgeQuerySchemaAdapter + ?Sized,
    {
        domain_ir.compile_program(schema_adapter)
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn operation(&self, id: &str) -> Option<&ForgeQueryOperation> {
        self.operations.get(id)
    }

    pub fn operations(&self) -> impl Iterator<Item = &ForgeQueryOperation> {
        self.operations.values()
    }

    pub fn workflow_graph(&self) -> &ForgeQueryWorkflowGraph {
        &self.workflow_graph
    }
}

pub trait ForgeQuerySchemaAdapter {
    fn schema_view(&self, operation_id: &str) -> Option<QuerySchemaView>;
}

pub trait ForgeQueryProgramSource {
    fn compile_program<A>(
        self,
        schema_adapter: &A,
    ) -> Result<ForgeQueryProgram, ForgeQueryProgramError>
    where
        A: ForgeQuerySchemaAdapter + ?Sized;
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryProgramTrace {
    program_id: String,
    operation_id: String,
    bound_inputs: Vec<String>,
    authority_requirements: Vec<ForgeQueryAuthorityRequirement>,
    generated_declarations: Vec<String>,
    write_receipts: Vec<String>,
    patch_artifacts: Vec<String>,
    replay_or_parity_metadata: Vec<String>,
}

impl ForgeQueryProgramTrace {
    pub(crate) fn new(
        program_id: impl Into<String>,
        operation_id: impl Into<String>,
        bound_inputs: &BTreeMap<String, Value>,
        authority_requirements: Vec<ForgeQueryAuthorityRequirement>,
    ) -> Self {
        Self {
            program_id: program_id.into(),
            operation_id: operation_id.into(),
            bound_inputs: bound_inputs.keys().cloned().collect(),
            authority_requirements,
            generated_declarations: Vec::new(),
            write_receipts: Vec::new(),
            patch_artifacts: Vec::new(),
            replay_or_parity_metadata: Vec::new(),
        }
    }

    pub(crate) fn record_declaration(&mut self, declaration: impl Into<String>) {
        self.generated_declarations.push(declaration.into());
    }

    pub(crate) fn record_write_receipt(&mut self, receipt: impl Into<String>) {
        self.write_receipts.push(receipt.into());
    }

    pub(crate) fn record_patch_artifact(&mut self, artifact: impl Into<String>) {
        self.patch_artifacts.push(artifact.into());
    }

    pub(crate) fn record_replay_or_parity(&mut self, metadata: impl Into<String>) {
        self.replay_or_parity_metadata.push(metadata.into());
    }

    pub fn program_id(&self) -> &str {
        &self.program_id
    }

    pub fn operation_id(&self) -> &str {
        &self.operation_id
    }

    pub fn generated_declarations(&self) -> &[String] {
        &self.generated_declarations
    }

    pub fn write_receipts(&self) -> &[String] {
        &self.write_receipts
    }

    pub fn patch_artifacts(&self) -> &[String] {
        &self.patch_artifacts
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryProgramError {
    message: String,
}

impl ForgeQueryProgramError {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl std::fmt::Display for ForgeQueryProgramError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.message)
    }
}

impl std::error::Error for ForgeQueryProgramError {}

pub(crate) fn validate_inputs(
    operation: &ForgeQueryOperation,
    inputs: &[ForgeQueryOperationInput],
) -> Result<BTreeMap<String, Value>, ForgeQueryProgramError> {
    let provided = inputs
        .iter()
        .map(|input| (input.name(), input.value()))
        .collect::<BTreeMap<_, _>>();
    let mut bound = BTreeMap::new();
    for port in operation.inputs() {
        let Some(value) = provided.get(port.name()) else {
            if port.optionality() {
                continue;
            }
            return Err(ForgeQueryProgramError::new(format!(
                "missing required input `{}`",
                port.name()
            )));
        };
        if !value_matches_port(value, port.port_type()) {
            return Err(ForgeQueryProgramError::new(format!(
                "input `{}` does not match required type {:?}",
                port.name(),
                port.port_type()
            )));
        }
        bound.insert(port.name().to_string(), (*value).clone());
    }
    Ok(bound)
}

fn expect_string(value: Value, label: &str) -> Result<String, ForgeQueryProgramError> {
    value.as_str().map(ToOwned::to_owned).ok_or_else(|| {
        ForgeQueryProgramError::new(format!("bound `{label}` must evaluate to a string"))
    })
}

fn value_matches_port(value: &Value, port_type: &ForgeQueryPortType) -> bool {
    match port_type {
        ForgeQueryPortType::String | ForgeQueryPortType::EntityIdentity => value.is_string(),
        ForgeQueryPortType::Integer => value.is_i64() || value.is_u64(),
        ForgeQueryPortType::Boolean => value.is_boolean(),
        ForgeQueryPortType::Json => true,
    }
}
