use std::collections::{BTreeMap, BTreeSet};

use worth_foundational::facade::{AspectValue, CanonicalF64, CanonicalFieldPath, InternedString};

use crate::declarative_live::DeclarativeLiveQueryRequest;
use crate::memory_workspace::{WorthQueryCommitIdentity, WorthQueryEntity};
use crate::runtime::WorthQueryAspectTouch;
use crate::schema_view::QuerySchemaView;

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

#[derive(Clone, Debug, PartialEq)]
pub struct WorthQueryProgramValue {
    value: WorthQueryProgramValueTree,
}

#[derive(Clone, Debug, PartialEq)]
enum WorthQueryProgramValueTree {
    Null,
    Bool(bool),
    Number(String),
    String(String),
    Array(Vec<WorthQueryProgramValueTree>),
    Object(BTreeMap<String, WorthQueryProgramValueTree>),
    NativeScalar(AspectValue),
}

impl WorthQueryProgramValue {
    pub fn null() -> Self {
        Self {
            value: WorthQueryProgramValueTree::Null,
        }
    }

    pub fn bool(value: bool) -> Self {
        Self {
            value: WorthQueryProgramValueTree::Bool(value),
        }
    }

    pub fn string(value: impl Into<String>) -> Self {
        Self {
            value: WorthQueryProgramValueTree::String(value.into()),
        }
    }

    pub fn integer(value: i64) -> Self {
        Self {
            value: WorthQueryProgramValueTree::Number(value.to_string()),
        }
    }

    pub fn unsigned_integer(value: u64) -> Self {
        Self {
            value: WorthQueryProgramValueTree::Number(value.to_string()),
        }
    }

    pub fn decimal_text(value: impl Into<String>) -> Result<Self, String> {
        let value = value.into();
        if !is_canonical_number_text(&value) {
            return Err(format!(
                "program value number `{value}` is not valid canonical number text"
            ));
        }
        Ok(Self {
            value: WorthQueryProgramValueTree::Number(value),
        })
    }

    pub fn array(values: impl IntoIterator<Item = WorthQueryProgramValue>) -> Self {
        Self {
            value: WorthQueryProgramValueTree::Array(
                values.into_iter().map(|value| value.value).collect(),
            ),
        }
    }

    pub fn object(
        fields: impl IntoIterator<Item = (impl Into<String>, WorthQueryProgramValue)>,
    ) -> Self {
        Self {
            value: WorthQueryProgramValueTree::Object(
                fields
                    .into_iter()
                    .map(|(key, value)| (key.into(), value.value))
                    .collect(),
            ),
        }
    }

    fn from_live_read_entities(rows: impl IntoIterator<Item = WorthQueryEntity>) -> Self {
        Self {
            value: WorthQueryProgramValueTree::Array(
                rows.into_iter()
                    .map(|row| program_value_tree_from_live_read_entity(&row))
                    .collect(),
            ),
        }
    }

    pub(crate) fn foundational_scalar_value(&self) -> Result<AspectValue, WorthQueryProgramError> {
        foundational_scalar_value_from_program_value_tree(&self.value)
    }

    pub fn array_len(&self) -> Option<usize> {
        let WorthQueryProgramValueTree::Array(values) = &self.value else {
            return None;
        };
        Some(values.len())
    }

    pub fn field_path_value(
        &self,
        field_path: &CanonicalFieldPath,
    ) -> Option<WorthQueryProgramValueField<'_>> {
        let value = program_value_tree_at_field_path(&self.value, field_path)?;
        Some(WorthQueryProgramValueField { value })
    }

    pub fn field_path_string_value(&self, field_path: &CanonicalFieldPath) -> Option<&str> {
        program_tree_string_value(program_value_tree_at_field_path(&self.value, field_path)?)
    }

    pub fn array_field_path_string_value(
        &self,
        index: usize,
        field_path: &CanonicalFieldPath,
    ) -> Option<&str> {
        let WorthQueryProgramValueTree::Array(values) = &self.value else {
            return None;
        };
        program_tree_string_value(program_value_tree_at_field_path(
            values.get(index)?,
            field_path,
        )?)
    }

    pub fn string_value(&self) -> Option<&str> {
        let WorthQueryProgramValueTree::String(value) = &self.value else {
            return None;
        };
        Some(value)
    }

    pub fn is_string(&self) -> bool {
        matches!(self.value, WorthQueryProgramValueTree::String(_))
    }

    pub fn is_integer(&self) -> bool {
        match &self.value {
            WorthQueryProgramValueTree::Number(value) => {
                value.parse::<i64>().is_ok() || value.parse::<u64>().is_ok()
            }
            _ => false,
        }
    }

    pub fn is_boolean(&self) -> bool {
        matches!(self.value, WorthQueryProgramValueTree::Bool(_))
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum WorthQueryValueExpr {
    Literal(WorthQueryProgramValue),
    Input(String),
    Object(BTreeMap<String, WorthQueryValueExpr>),
    Array(Vec<WorthQueryValueExpr>),
}

impl WorthQueryValueExpr {
    pub fn literal(value: WorthQueryProgramValue) -> Self {
        Self::Literal(value)
    }

    pub fn input(name: impl Into<String>) -> Self {
        Self::Input(name.into())
    }

    pub fn object(fields: impl IntoIterator<Item = (String, WorthQueryValueExpr)>) -> Self {
        Self::Object(fields.into_iter().collect())
    }

    pub fn array(items: impl IntoIterator<Item = WorthQueryValueExpr>) -> Self {
        Self::Array(items.into_iter().collect())
    }

    pub(crate) fn evaluate(
        &self,
        inputs: &BTreeMap<String, WorthQueryProgramValue>,
    ) -> Result<WorthQueryProgramValue, WorthQueryProgramError> {
        match self {
            Self::Literal(value) => Ok(value.clone()),
            Self::Input(name) => inputs.get(name).cloned().ok_or_else(|| {
                WorthQueryProgramError::new(format!("missing bound input `{name}`"))
            }),
            Self::Object(fields) => fields
                .iter()
                .map(|(key, value)| Ok((key.clone(), value.evaluate(inputs)?)))
                .collect::<Result<Vec<_>, _>>()
                .map(WorthQueryProgramValue::object),
            Self::Array(items) => items
                .iter()
                .map(|item| item.evaluate(inputs))
                .collect::<Result<Vec<_>, _>>()
                .map(WorthQueryProgramValue::array),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct WorthQueryAdmittedAspectValueTemplate {
    aspect_touch: WorthQueryAspectTouch,
    value: WorthQueryValueExpr,
}

impl WorthQueryAdmittedAspectValueTemplate {
    pub fn new(aspect_touch: WorthQueryAspectTouch, value: WorthQueryValueExpr) -> Self {
        Self {
            aspect_touch,
            value,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum WorthQueryWriteCommandTemplate {
    InsertAspects {
        collection: String,
        aspects: Vec<WorthQueryAdmittedAspectValueTemplate>,
    },
    UpdateAspect {
        entity_identity: WorthQueryValueExpr,
        aspect_touch: WorthQueryAspectTouch,
        value: WorthQueryValueExpr,
    },
    Delete {
        entity_identity: WorthQueryValueExpr,
    },
}

impl WorthQueryWriteCommandTemplate {
    pub(crate) fn bind(
        &self,
        inputs: &BTreeMap<String, WorthQueryProgramValue>,
    ) -> Result<crate::runtime::WorthQueryWriteCommand, WorthQueryProgramError> {
        match self {
            Self::InsertAspects {
                collection,
                aspects,
            } => Ok(crate::runtime::WorthQueryWriteCommand::InsertAspects {
                collection: crate::runtime::WorthQueryMutationTargetCollectionIdentity::new(
                    "write-command-declared",
                    collection,
                ),
                aspects: aspects
                    .iter()
                    .map(|aspect| {
                        crate::runtime::WorthQueryAuthoredAspectMutation::new_set(
                            aspect.aspect_touch.clone(),
                            aspect.value.evaluate(inputs)?.foundational_scalar_value()?,
                        )
                        .map_err(|error| WorthQueryProgramError::new(error.to_string()))
                    })
                    .collect::<Result<Vec<_>, _>>()?,
                symbolic_aspect_references: Vec::new(),
                metadata: crate::runtime::WorthQueryMutationMetadata::default(),
                naming_intent: None,
                continuity_intent: None,
                symbolic_target_reference: None,
            }),
            Self::UpdateAspect {
                entity_identity,
                aspect_touch,
                value,
            } => Ok(crate::runtime::WorthQueryWriteCommand::UpdateAspect {
                entity_identity: crate::memory_workspace::admit_authored_entity_label(
                    expect_string(entity_identity.evaluate(inputs)?, "entity_identity")?,
                ),
                aspect: crate::runtime::WorthQueryAuthoredAspectMutation::new_set(
                    aspect_touch.clone(),
                    value.evaluate(inputs)?.foundational_scalar_value()?,
                )
                .map_err(|error| WorthQueryProgramError::new(error.to_string()))?,
            }),
            Self::Delete { entity_identity } => {
                Ok(crate::runtime::WorthQueryWriteCommand::Delete {
                    entity_identity: crate::memory_workspace::admit_authored_entity_label(
                        expect_string(entity_identity.evaluate(inputs)?, "entity_identity")?,
                    ),
                })
            }
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthQueryPortType {
    String,
    Integer,
    Boolean,
    ProgramValue,
    EntityIdentity,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryTypedPort {
    name: String,
    port_type: WorthQueryPortType,
    optional: bool,
    required_aspects: Vec<WorthQueryAspectTouch>,
    binding_slot: Option<String>,
    result_shape: Option<String>,
}

impl WorthQueryTypedPort {
    pub fn new(name: impl Into<String>, port_type: WorthQueryPortType) -> Self {
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

    pub fn with_required_aspect(mut self, aspect: WorthQueryAspectTouch) -> Self {
        self.required_aspects.push(aspect);
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

    pub fn port_type(&self) -> &WorthQueryPortType {
        &self.port_type
    }

    pub fn optionality(&self) -> bool {
        self.optional
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct WorthQueryOperationInput {
    name: String,
    value: WorthQueryProgramValue,
}

impl WorthQueryOperationInput {
    pub fn new(name: impl Into<String>, value: WorthQueryProgramValue) -> Self {
        Self::from_program_value(name, value)
    }

    pub fn from_program_value(name: impl Into<String>, value: WorthQueryProgramValue) -> Self {
        Self {
            name: name.into(),
            value,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn value(&self) -> &WorthQueryProgramValue {
        &self.value
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct WorthQueryOperationOutput {
    name: String,
    value: WorthQueryProgramValue,
}

impl WorthQueryOperationOutput {
    pub fn new(name: impl Into<String>, value: WorthQueryProgramValue) -> Self {
        Self::from_program_value(name, value)
    }

    pub fn from_program_value(name: impl Into<String>, value: WorthQueryProgramValue) -> Self {
        Self {
            name: name.into(),
            value,
        }
    }

    pub(crate) fn from_live_read_entities(
        name: impl Into<String>,
        rows: impl IntoIterator<Item = WorthQueryEntity>,
    ) -> Self {
        Self::from_program_value(name, WorthQueryProgramValue::from_live_read_entities(rows))
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn value(&self) -> &WorthQueryProgramValue {
        &self.value
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WorthQueryProgramValueField<'a> {
    value: &'a WorthQueryProgramValueTree,
}

impl WorthQueryProgramValueField<'_> {
    pub fn string_value(&self) -> Option<&str> {
        program_tree_string_value(self.value)
    }

    pub fn foundational_scalar_value(&self) -> Result<AspectValue, WorthQueryProgramError> {
        foundational_scalar_value_from_program_value_tree(self.value)
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

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryDerivedView {
    name: String,
    dependency_aspects: Vec<WorthQueryAspectTouch>,
    produced_aspects: Vec<WorthQueryAspectTouch>,
    upstream_live_views: Vec<String>,
    upstream_derived_views: Vec<String>,
    incremental: bool,
}

impl WorthQueryDerivedView {
    pub fn new(
        name: impl Into<String>,
        dependency_aspects: impl IntoIterator<Item = WorthQueryAspectTouch>,
    ) -> Self {
        Self {
            name: name.into(),
            dependency_aspects: unique_derived_view_aspects(dependency_aspects),
            produced_aspects: Vec::new(),
            upstream_live_views: Vec::new(),
            upstream_derived_views: Vec::new(),
            incremental: true,
        }
    }

    pub fn produces(mut self, aspects: impl IntoIterator<Item = WorthQueryAspectTouch>) -> Self {
        self.produced_aspects = unique_derived_view_aspects(aspects);
        self
    }

    pub fn depends_on_live<T>(mut self, view: &crate::runtime::WorthQueryLiveView<T>) -> Self {
        self.upstream_live_views.push(view.name().to_string());
        self
    }

    pub fn depends_on_derived<T>(
        mut self,
        view: &crate::runtime::WorthQueryDerivedViewHandle<T>,
    ) -> Self {
        self.upstream_derived_views.push(view.name().to_string());
        self
    }

    pub(crate) fn depends_on_live_name_from_workspace_declaration(
        mut self,
        name: impl Into<String>,
    ) -> Self {
        self.upstream_live_views.push(name.into());
        self
    }

    pub(crate) fn depends_on_derived_name_from_workspace_declaration(
        mut self,
        name: impl Into<String>,
    ) -> Self {
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

    pub fn dependency_aspect_touches(&self) -> &[WorthQueryAspectTouch] {
        &self.dependency_aspects
    }

    pub fn produced_aspect_touches(&self) -> &[WorthQueryAspectTouch] {
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

fn unique_derived_view_aspects(
    aspects: impl IntoIterator<Item = WorthQueryAspectTouch>,
) -> Vec<WorthQueryAspectTouch> {
    let mut touches = BTreeSet::new();
    for touch in aspects {
        touches.insert(touch);
    }
    touches.into_iter().collect()
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

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryProgramTrace {
    program_id: String,
    operation_id: String,
    bound_inputs: Vec<String>,
    authority_requirements: Vec<WorthQueryAuthorityRequirement>,
    generated_declarations: Vec<String>,
    write_receipts: Vec<WorthQueryCommitIdentity>,
    patch_artifacts: Vec<String>,
    replay_or_parity_metadata: Vec<String>,
}

impl WorthQueryProgramTrace {
    pub(crate) fn new(
        program_id: impl Into<String>,
        operation_id: impl Into<String>,
        bound_inputs: &BTreeMap<String, WorthQueryProgramValue>,
        authority_requirements: Vec<WorthQueryAuthorityRequirement>,
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

    pub(crate) fn record_write_receipt(&mut self, receipt: WorthQueryCommitIdentity) {
        self.write_receipts.push(receipt);
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

    pub fn write_receipts(&self) -> &[WorthQueryCommitIdentity] {
        &self.write_receipts
    }

    pub fn patch_artifacts(&self) -> &[String] {
        &self.patch_artifacts
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryProgramError {
    message: String,
}

impl WorthQueryProgramError {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl std::fmt::Display for WorthQueryProgramError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.message)
    }
}

impl std::error::Error for WorthQueryProgramError {}

pub(crate) fn validate_inputs(
    operation: &WorthQueryOperation,
    inputs: &[WorthQueryOperationInput],
) -> Result<BTreeMap<String, WorthQueryProgramValue>, WorthQueryProgramError> {
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
            return Err(WorthQueryProgramError::new(format!(
                "missing required input `{}`",
                port.name()
            )));
        };
        if !value_matches_port(value, port.port_type()) {
            return Err(WorthQueryProgramError::new(format!(
                "input `{}` does not match required type {:?}",
                port.name(),
                port.port_type()
            )));
        }
        bound.insert(port.name().to_string(), (*value).clone());
    }
    Ok(bound)
}

fn expect_string(
    value: WorthQueryProgramValue,
    label: &str,
) -> Result<String, WorthQueryProgramError> {
    value.string_value().map(ToOwned::to_owned).ok_or_else(|| {
        WorthQueryProgramError::new(format!("bound `{label}` must evaluate to a string"))
    })
}

fn value_matches_port(value: &WorthQueryProgramValue, port_type: &WorthQueryPortType) -> bool {
    match port_type {
        WorthQueryPortType::String | WorthQueryPortType::EntityIdentity => value.is_string(),
        WorthQueryPortType::Integer => value.is_integer(),
        WorthQueryPortType::Boolean => value.is_boolean(),
        WorthQueryPortType::ProgramValue => true,
    }
}

fn program_value_tree_from_live_read_entity(row: &WorthQueryEntity) -> WorthQueryProgramValueTree {
    let mut fields = BTreeMap::new();
    for (field_path, value) in row.native_field_values() {
        insert_program_field_path(
            &mut fields,
            field_path,
            program_value_tree_from_aspect_value(value),
        );
    }
    WorthQueryProgramValueTree::Object(fields)
}

fn insert_program_field_path(
    target: &mut BTreeMap<String, WorthQueryProgramValueTree>,
    field_path: &CanonicalFieldPath,
    value: WorthQueryProgramValueTree,
) {
    let segments = field_path
        .fields()
        .iter()
        .map(|field| field.as_str().to_owned())
        .collect::<Vec<_>>();
    insert_program_path_segments(target, &segments, value);
}

fn insert_program_path_segments(
    target: &mut BTreeMap<String, WorthQueryProgramValueTree>,
    segments: &[String],
    value: WorthQueryProgramValueTree,
) {
    let Some((head, tail)) = segments.split_first() else {
        return;
    };
    if tail.is_empty() {
        target.insert(head.clone(), value);
        return;
    }

    let entry = target
        .entry(head.clone())
        .or_insert_with(|| WorthQueryProgramValueTree::Object(BTreeMap::new()));
    let WorthQueryProgramValueTree::Object(fields) = entry else {
        *entry = WorthQueryProgramValueTree::Object(BTreeMap::new());
        let WorthQueryProgramValueTree::Object(fields) = entry else {
            unreachable!("program path segment was just replaced with an object");
        };
        insert_program_path_segments(fields, tail, value);
        return;
    };
    insert_program_path_segments(fields, tail, value);
}

fn program_value_tree_at_field_path<'a>(
    value: &'a WorthQueryProgramValueTree,
    field_path: &CanonicalFieldPath,
) -> Option<&'a WorthQueryProgramValueTree> {
    let mut current = value;
    for field in field_path.fields() {
        let WorthQueryProgramValueTree::Object(fields) = current else {
            return None;
        };
        current = fields.get(field.as_str())?;
    }
    Some(current)
}

fn program_value_tree_from_aspect_value(value: &AspectValue) -> WorthQueryProgramValueTree {
    WorthQueryProgramValueTree::NativeScalar(value.clone())
}

fn program_tree_string_value(value: &WorthQueryProgramValueTree) -> Option<&str> {
    match value {
        WorthQueryProgramValueTree::String(value) => Some(value),
        WorthQueryProgramValueTree::NativeScalar(AspectValue::String(InternedString::Raw(
            value,
        ))) => Some(value),
        _ => None,
    }
}

fn is_canonical_number_text(value: &str) -> bool {
    let bytes = value.as_bytes();
    let mut index = 0;
    if bytes.is_empty() {
        return false;
    }
    if bytes[index] == b'-' {
        index += 1;
        if index == bytes.len() {
            return false;
        }
    }
    match bytes[index] {
        b'0' => {
            index += 1;
            if index < bytes.len() && bytes[index].is_ascii_digit() {
                return false;
            }
        }
        b'1'..=b'9' => {
            index += 1;
            while index < bytes.len() && bytes[index].is_ascii_digit() {
                index += 1;
            }
        }
        _ => return false,
    }
    if index < bytes.len() && bytes[index] == b'.' {
        index += 1;
        if index == bytes.len() || !bytes[index].is_ascii_digit() {
            return false;
        }
        while index < bytes.len() && bytes[index].is_ascii_digit() {
            index += 1;
        }
    }
    if index < bytes.len() && matches!(bytes[index], b'e' | b'E') {
        index += 1;
        if index < bytes.len() && matches!(bytes[index], b'+' | b'-') {
            index += 1;
        }
        if index == bytes.len() || !bytes[index].is_ascii_digit() {
            return false;
        }
        while index < bytes.len() && bytes[index].is_ascii_digit() {
            index += 1;
        }
    }
    index == bytes.len()
}

fn foundational_scalar_value_from_program_value_tree(
    value: &WorthQueryProgramValueTree,
) -> Result<AspectValue, WorthQueryProgramError> {
    match value {
        WorthQueryProgramValueTree::Null => Ok(AspectValue::Null),
        WorthQueryProgramValueTree::Bool(value) => Ok(AspectValue::Bool(*value)),
        WorthQueryProgramValueTree::Number(value) => {
            if let Ok(value) = value.parse::<i64>() {
                Ok(AspectValue::Int64(value))
            } else if let Ok(value) = value.parse::<u64>() {
                Ok(AspectValue::UInt64(value))
            } else if let Ok(value) = value.parse::<f64>() {
                if !value.is_finite() {
                    return Err(WorthQueryProgramError::new(
                        "program scalar aspect value number must be finite",
                    ));
                }
                Ok(AspectValue::Float64(CanonicalF64::from_f64(value)))
            } else {
                Err(WorthQueryProgramError::new(format!(
                    "program scalar aspect value number `{value}` is invalid"
                )))
            }
        }
        WorthQueryProgramValueTree::String(value) => Ok(
            crate::runtime::WorthQueryAuthoredAspectMutation::native_string_value(value.clone()),
        ),
        WorthQueryProgramValueTree::NativeScalar(value) => Ok(value.clone()),
        WorthQueryProgramValueTree::Array(_) => Err(WorthQueryProgramError::new(
            "program scalar aspect value cannot be an array",
        )),
        WorthQueryProgramValueTree::Object(_) => Err(WorthQueryProgramError::new(
            "program scalar aspect value cannot be an object",
        )),
    }
}
