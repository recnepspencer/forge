use std::collections::{BTreeMap, BTreeSet};

use forge_foundational::facade::{AspectValue, CanonicalF64, CanonicalFieldPath, InternedString};

use crate::declarative_live::DeclarativeLiveQueryRequest;
use crate::memory_workspace::{ForgeQueryCommitIdentity, ForgeQueryEntity};
use crate::runtime::ForgeQueryAspectTouch;
use crate::schema_view::QuerySchemaView;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct ForgeQueryProgramOperationIdentity {
    value: String,
}

impl ForgeQueryProgramOperationIdentity {
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
pub struct ForgeQueryProgramValue {
    value: ForgeQueryProgramValueTree,
}

#[derive(Clone, Debug, PartialEq)]
enum ForgeQueryProgramValueTree {
    Null,
    Bool(bool),
    Number(String),
    String(String),
    Array(Vec<ForgeQueryProgramValueTree>),
    Object(BTreeMap<String, ForgeQueryProgramValueTree>),
}

impl ForgeQueryProgramValue {
    pub fn null() -> Self {
        Self {
            value: ForgeQueryProgramValueTree::Null,
        }
    }

    pub fn bool(value: bool) -> Self {
        Self {
            value: ForgeQueryProgramValueTree::Bool(value),
        }
    }

    pub fn string(value: impl Into<String>) -> Self {
        Self {
            value: ForgeQueryProgramValueTree::String(value.into()),
        }
    }

    pub fn integer(value: i64) -> Self {
        Self {
            value: ForgeQueryProgramValueTree::Number(value.to_string()),
        }
    }

    pub fn unsigned_integer(value: u64) -> Self {
        Self {
            value: ForgeQueryProgramValueTree::Number(value.to_string()),
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
            value: ForgeQueryProgramValueTree::Number(value),
        })
    }

    pub fn array(values: impl IntoIterator<Item = ForgeQueryProgramValue>) -> Self {
        Self {
            value: ForgeQueryProgramValueTree::Array(
                values.into_iter().map(|value| value.value).collect(),
            ),
        }
    }

    pub fn object(
        fields: impl IntoIterator<Item = (impl Into<String>, ForgeQueryProgramValue)>,
    ) -> Self {
        Self {
            value: ForgeQueryProgramValueTree::Object(
                fields
                    .into_iter()
                    .map(|(key, value)| (key.into(), value.value))
                    .collect(),
            ),
        }
    }

    fn from_live_read_entities(rows: impl IntoIterator<Item = ForgeQueryEntity>) -> Self {
        Self {
            value: ForgeQueryProgramValueTree::Array(
                rows.into_iter()
                    .map(|row| program_value_tree_from_live_read_entity(&row))
                    .collect(),
            ),
        }
    }

    pub(crate) fn foundational_scalar_value(&self) -> Result<AspectValue, ForgeQueryProgramError> {
        foundational_scalar_value_from_program_value_tree(&self.value)
    }

    pub fn array_len(&self) -> Option<usize> {
        let ForgeQueryProgramValueTree::Array(values) = &self.value else {
            return None;
        };
        Some(values.len())
    }

    pub fn field_path_value(
        &self,
        field_path: &CanonicalFieldPath,
    ) -> Option<ForgeQueryProgramValueField<'_>> {
        let value = program_value_tree_at_field_path(&self.value, field_path)?;
        Some(ForgeQueryProgramValueField { value })
    }

    pub fn field_path_string_value(&self, field_path: &CanonicalFieldPath) -> Option<&str> {
        let ForgeQueryProgramValueTree::String(value) =
            program_value_tree_at_field_path(&self.value, field_path)?
        else {
            return None;
        };
        Some(value)
    }

    pub fn array_field_path_string_value(
        &self,
        index: usize,
        field_path: &CanonicalFieldPath,
    ) -> Option<&str> {
        let ForgeQueryProgramValueTree::Array(values) = &self.value else {
            return None;
        };
        let ForgeQueryProgramValueTree::String(value) =
            program_value_tree_at_field_path(values.get(index)?, field_path)?
        else {
            return None;
        };
        Some(value)
    }

    pub fn string_value(&self) -> Option<&str> {
        let ForgeQueryProgramValueTree::String(value) = &self.value else {
            return None;
        };
        Some(value)
    }

    pub fn is_string(&self) -> bool {
        matches!(self.value, ForgeQueryProgramValueTree::String(_))
    }

    pub fn is_integer(&self) -> bool {
        match &self.value {
            ForgeQueryProgramValueTree::Number(value) => {
                value.parse::<i64>().is_ok() || value.parse::<u64>().is_ok()
            }
            _ => false,
        }
    }

    pub fn is_boolean(&self) -> bool {
        matches!(self.value, ForgeQueryProgramValueTree::Bool(_))
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum ForgeQueryValueExpr {
    Literal(ForgeQueryProgramValue),
    Input(String),
    Object(BTreeMap<String, ForgeQueryValueExpr>),
    Array(Vec<ForgeQueryValueExpr>),
}

impl ForgeQueryValueExpr {
    pub fn literal(value: ForgeQueryProgramValue) -> Self {
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
        inputs: &BTreeMap<String, ForgeQueryProgramValue>,
    ) -> Result<ForgeQueryProgramValue, ForgeQueryProgramError> {
        match self {
            Self::Literal(value) => Ok(value.clone()),
            Self::Input(name) => inputs.get(name).cloned().ok_or_else(|| {
                ForgeQueryProgramError::new(format!("missing bound input `{name}`"))
            }),
            Self::Object(fields) => fields
                .iter()
                .map(|(key, value)| Ok((key.clone(), value.evaluate(inputs)?)))
                .collect::<Result<Vec<_>, _>>()
                .map(ForgeQueryProgramValue::object),
            Self::Array(items) => items
                .iter()
                .map(|item| item.evaluate(inputs))
                .collect::<Result<Vec<_>, _>>()
                .map(ForgeQueryProgramValue::array),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ForgeQueryAdmittedAspectValueTemplate {
    aspect_touch: ForgeQueryAspectTouch,
    value: ForgeQueryValueExpr,
}

impl ForgeQueryAdmittedAspectValueTemplate {
    pub fn new(aspect_touch: ForgeQueryAspectTouch, value: ForgeQueryValueExpr) -> Self {
        Self {
            aspect_touch,
            value,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum ForgeQueryWriteCommandTemplate {
    InsertAspects {
        collection: String,
        aspects: Vec<ForgeQueryAdmittedAspectValueTemplate>,
    },
    UpdateAspect {
        entity_identity: ForgeQueryValueExpr,
        aspect_touch: ForgeQueryAspectTouch,
        value: ForgeQueryValueExpr,
    },
    Delete {
        entity_identity: ForgeQueryValueExpr,
    },
}

impl ForgeQueryWriteCommandTemplate {
    pub(crate) fn bind(
        &self,
        inputs: &BTreeMap<String, ForgeQueryProgramValue>,
    ) -> Result<crate::runtime::ForgeQueryWriteCommand, ForgeQueryProgramError> {
        match self {
            Self::InsertAspects {
                collection,
                aspects,
            } => Ok(crate::runtime::ForgeQueryWriteCommand::InsertAspects {
                collection: crate::runtime::ForgeQueryMutationTargetCollectionIdentity::new(
                    "write-command-declared",
                    collection,
                ),
                aspects: aspects
                    .iter()
                    .map(|aspect| {
                        crate::runtime::ForgeQueryAdmittedAspectValue::new_set(
                            aspect.aspect_touch.clone(),
                            aspect.value.evaluate(inputs)?.foundational_scalar_value()?,
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
                aspect_touch,
                value,
            } => Ok(crate::runtime::ForgeQueryWriteCommand::UpdateAspect {
                entity_identity: crate::memory_workspace::admit_authored_entity_label(
                    expect_string(entity_identity.evaluate(inputs)?, "entity_identity")?,
                ),
                aspect: crate::runtime::ForgeQueryAdmittedAspectValue::new_set(
                    aspect_touch.clone(),
                    value.evaluate(inputs)?.foundational_scalar_value()?,
                )
                .map_err(|error| ForgeQueryProgramError::new(error.to_string()))?,
            }),
            Self::Delete { entity_identity } => {
                Ok(crate::runtime::ForgeQueryWriteCommand::Delete {
                    entity_identity: crate::memory_workspace::admit_authored_entity_label(
                        expect_string(entity_identity.evaluate(inputs)?, "entity_identity")?,
                    ),
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
    ProgramValue,
    EntityIdentity,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryTypedPort {
    name: String,
    port_type: ForgeQueryPortType,
    optional: bool,
    required_aspects: Vec<ForgeQueryAspectTouch>,
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

    pub fn with_required_aspect(mut self, aspect: ForgeQueryAspectTouch) -> Self {
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
    value: ForgeQueryProgramValue,
}

impl ForgeQueryOperationInput {
    pub fn new(name: impl Into<String>, value: ForgeQueryProgramValue) -> Self {
        Self::from_program_value(name, value)
    }

    pub fn from_program_value(name: impl Into<String>, value: ForgeQueryProgramValue) -> Self {
        Self {
            name: name.into(),
            value,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn value(&self) -> &ForgeQueryProgramValue {
        &self.value
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ForgeQueryOperationOutput {
    name: String,
    value: ForgeQueryProgramValue,
}

impl ForgeQueryOperationOutput {
    pub fn new(name: impl Into<String>, value: ForgeQueryProgramValue) -> Self {
        Self::from_program_value(name, value)
    }

    pub fn from_program_value(name: impl Into<String>, value: ForgeQueryProgramValue) -> Self {
        Self {
            name: name.into(),
            value,
        }
    }

    pub(crate) fn from_live_read_entities(
        name: impl Into<String>,
        rows: impl IntoIterator<Item = ForgeQueryEntity>,
    ) -> Self {
        Self::from_program_value(name, ForgeQueryProgramValue::from_live_read_entities(rows))
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn value(&self) -> &ForgeQueryProgramValue {
        &self.value
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ForgeQueryProgramValueField<'a> {
    value: &'a ForgeQueryProgramValueTree,
}

impl ForgeQueryProgramValueField<'_> {
    pub fn string_value(&self) -> Option<&str> {
        let ForgeQueryProgramValueTree::String(value) = self.value else {
            return None;
        };
        Some(value)
    }

    pub fn foundational_scalar_value(&self) -> Result<AspectValue, ForgeQueryProgramError> {
        foundational_scalar_value_from_program_value_tree(self.value)
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
    dependency_aspects: Vec<ForgeQueryAspectTouch>,
    produced_aspects: Vec<ForgeQueryAspectTouch>,
    upstream_live_views: Vec<String>,
    upstream_derived_views: Vec<String>,
    incremental: bool,
}

impl ForgeQueryDerivedView {
    pub fn new(
        name: impl Into<String>,
        dependency_aspects: impl IntoIterator<Item = ForgeQueryAspectTouch>,
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

    pub fn produces(mut self, aspects: impl IntoIterator<Item = ForgeQueryAspectTouch>) -> Self {
        self.produced_aspects = unique_derived_view_aspects(aspects);
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

    pub fn dependency_aspect_touches(&self) -> &[ForgeQueryAspectTouch] {
        &self.dependency_aspects
    }

    pub fn produced_aspect_touches(&self) -> &[ForgeQueryAspectTouch] {
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
    aspects: impl IntoIterator<Item = ForgeQueryAspectTouch>,
) -> Vec<ForgeQueryAspectTouch> {
    let mut touches = BTreeSet::new();
    for touch in aspects {
        touches.insert(touch);
    }
    touches.into_iter().collect()
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
    operations: BTreeMap<ForgeQueryProgramOperationIdentity, ForgeQueryOperation>,
    workflow_graph: ForgeQueryWorkflowGraph,
}

impl ForgeQueryProgram {
    pub fn new(
        id: impl Into<String>,
        operations: impl IntoIterator<Item = ForgeQueryOperation>,
    ) -> Result<Self, ForgeQueryProgramError> {
        let operations = operations
            .into_iter()
            .map(|operation| {
                (
                    ForgeQueryProgramOperationIdentity::from_operation_id(operation.id.clone()),
                    operation,
                )
            })
            .collect::<BTreeMap<_, _>>();
        if operations.is_empty() {
            return Err(ForgeQueryProgramError::new(
                "program must declare at least one operation",
            ));
        }
        let workflow_graph =
            ForgeQueryWorkflowGraph::linear(operations.keys().map(|key| key.as_str().to_string()));
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
        self.operations
            .get(&ForgeQueryProgramOperationIdentity::from_operation_id(id))
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
    write_receipts: Vec<ForgeQueryCommitIdentity>,
    patch_artifacts: Vec<String>,
    replay_or_parity_metadata: Vec<String>,
}

impl ForgeQueryProgramTrace {
    pub(crate) fn new(
        program_id: impl Into<String>,
        operation_id: impl Into<String>,
        bound_inputs: &BTreeMap<String, ForgeQueryProgramValue>,
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

    pub(crate) fn record_write_receipt(&mut self, receipt: ForgeQueryCommitIdentity) {
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

    pub fn write_receipts(&self) -> &[ForgeQueryCommitIdentity] {
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
) -> Result<BTreeMap<String, ForgeQueryProgramValue>, ForgeQueryProgramError> {
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

fn expect_string(
    value: ForgeQueryProgramValue,
    label: &str,
) -> Result<String, ForgeQueryProgramError> {
    value.string_value().map(ToOwned::to_owned).ok_or_else(|| {
        ForgeQueryProgramError::new(format!("bound `{label}` must evaluate to a string"))
    })
}

fn value_matches_port(value: &ForgeQueryProgramValue, port_type: &ForgeQueryPortType) -> bool {
    match port_type {
        ForgeQueryPortType::String | ForgeQueryPortType::EntityIdentity => value.is_string(),
        ForgeQueryPortType::Integer => value.is_integer(),
        ForgeQueryPortType::Boolean => value.is_boolean(),
        ForgeQueryPortType::ProgramValue => true,
    }
}

fn program_value_tree_from_live_read_entity(row: &ForgeQueryEntity) -> ForgeQueryProgramValueTree {
    let mut fields = BTreeMap::new();
    for (field_path, value) in row.native_field_values() {
        insert_program_field_path(
            &mut fields,
            field_path,
            program_value_tree_from_aspect_value(value),
        );
    }
    ForgeQueryProgramValueTree::Object(fields)
}

fn insert_program_field_path(
    target: &mut BTreeMap<String, ForgeQueryProgramValueTree>,
    field_path: &CanonicalFieldPath,
    value: ForgeQueryProgramValueTree,
) {
    let segments = field_path
        .fields()
        .iter()
        .map(|field| field.as_str().to_owned())
        .collect::<Vec<_>>();
    insert_program_path_segments(target, &segments, value);
}

fn insert_program_path_segments(
    target: &mut BTreeMap<String, ForgeQueryProgramValueTree>,
    segments: &[String],
    value: ForgeQueryProgramValueTree,
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
        .or_insert_with(|| ForgeQueryProgramValueTree::Object(BTreeMap::new()));
    let ForgeQueryProgramValueTree::Object(fields) = entry else {
        *entry = ForgeQueryProgramValueTree::Object(BTreeMap::new());
        let ForgeQueryProgramValueTree::Object(fields) = entry else {
            unreachable!("program path segment was just replaced with an object");
        };
        insert_program_path_segments(fields, tail, value);
        return;
    };
    insert_program_path_segments(fields, tail, value);
}

fn program_value_tree_at_field_path<'a>(
    value: &'a ForgeQueryProgramValueTree,
    field_path: &CanonicalFieldPath,
) -> Option<&'a ForgeQueryProgramValueTree> {
    let mut current = value;
    for field in field_path.fields() {
        let ForgeQueryProgramValueTree::Object(fields) = current else {
            return None;
        };
        current = fields.get(field.as_str())?;
    }
    Some(current)
}

fn program_value_tree_from_aspect_value(value: &AspectValue) -> ForgeQueryProgramValueTree {
    match value {
        AspectValue::Null => ForgeQueryProgramValueTree::Null,
        AspectValue::Bool(value) => ForgeQueryProgramValueTree::Bool(*value),
        AspectValue::Int8(value) => ForgeQueryProgramValueTree::Number(value.to_string()),
        AspectValue::Int16(value) => ForgeQueryProgramValueTree::Number(value.to_string()),
        AspectValue::Int32(value) => ForgeQueryProgramValueTree::Number(value.to_string()),
        AspectValue::Int64(value) => ForgeQueryProgramValueTree::Number(value.to_string()),
        AspectValue::UInt8(value) => ForgeQueryProgramValueTree::Number(value.to_string()),
        AspectValue::UInt16(value) => ForgeQueryProgramValueTree::Number(value.to_string()),
        AspectValue::UInt32(value) => ForgeQueryProgramValueTree::Number(value.to_string()),
        AspectValue::UInt64(value) => ForgeQueryProgramValueTree::Number(value.to_string()),
        AspectValue::Float32(value) => {
            program_number_tree_from_float(f32::from_bits(value.bits()) as f64)
        }
        AspectValue::Float64(value) => program_number_tree_from_float(f64::from_bits(value.bits())),
        AspectValue::String(value) => {
            ForgeQueryProgramValueTree::String(interned_string_text(value))
        }
        other => ForgeQueryProgramValueTree::String(format!("{other:?}")),
    }
}

fn program_number_tree_from_float(value: f64) -> ForgeQueryProgramValueTree {
    if value.is_finite() {
        ForgeQueryProgramValueTree::Number(value.to_string())
    } else {
        ForgeQueryProgramValueTree::Null
    }
}

fn interned_string_text(value: &InternedString) -> String {
    match value {
        InternedString::Raw(value) => value.clone(),
        InternedString::Symbol(symbol) => format!("symbol:{}", symbol.0),
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
    value: &ForgeQueryProgramValueTree,
) -> Result<AspectValue, ForgeQueryProgramError> {
    match value {
        ForgeQueryProgramValueTree::Null => Ok(AspectValue::Null),
        ForgeQueryProgramValueTree::Bool(value) => Ok(AspectValue::Bool(*value)),
        ForgeQueryProgramValueTree::Number(value) => {
            if let Ok(value) = value.parse::<i64>() {
                Ok(AspectValue::Int64(value))
            } else if let Ok(value) = value.parse::<u64>() {
                Ok(AspectValue::UInt64(value))
            } else if let Ok(value) = value.parse::<f64>() {
                if !value.is_finite() {
                    return Err(ForgeQueryProgramError::new(
                        "program scalar aspect value number must be finite",
                    ));
                }
                Ok(AspectValue::Float64(CanonicalF64::from_f64(value)))
            } else {
                Err(ForgeQueryProgramError::new(format!(
                    "program scalar aspect value number `{value}` is invalid"
                )))
            }
        }
        ForgeQueryProgramValueTree::String(value) => {
            Ok(crate::runtime::ForgeQueryAdmittedAspectValue::native_string_value(value.clone()))
        }
        ForgeQueryProgramValueTree::Array(_) => Err(ForgeQueryProgramError::new(
            "program scalar aspect value cannot be an array",
        )),
        ForgeQueryProgramValueTree::Object(_) => Err(ForgeQueryProgramError::new(
            "program scalar aspect value cannot be an object",
        )),
    }
}
