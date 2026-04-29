use std::collections::BTreeSet;

use serde::Serialize;
use serde_json::{Map, Value};

use super::{ForgeQueryRuntimeError, ForgeQueryWriteCommand};
use crate::memory_workspace::ForgeQueryWorkspaceError;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum ForgeQueryAspectMutationOperationKind {
    Set,
    Clear,
}

impl ForgeQueryAspectMutationOperationKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Set => "set",
            Self::Clear => "clear",
        }
    }
}

impl std::fmt::Display for ForgeQueryAspectMutationOperationKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ForgeQueryAspectMutationOperation {
    aspect_path: String,
    kind: ForgeQueryAspectMutationOperationKind,
}

impl ForgeQueryAspectMutationOperation {
    fn new(aspect_path: impl Into<String>, kind: ForgeQueryAspectMutationOperationKind) -> Self {
        Self {
            aspect_path: aspect_path.into(),
            kind,
        }
    }

    pub fn aspect_path(&self) -> &str {
        &self.aspect_path
    }

    pub fn kind(&self) -> ForgeQueryAspectMutationOperationKind {
        self.kind
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ForgeQueryAspectValue {
    aspect_path: String,
    value: Value,
    clears_existing_value: bool,
}

impl ForgeQueryAspectValue {
    pub fn new<T: Serialize>(
        aspect_path: impl Into<String>,
        value: T,
    ) -> Result<Self, ForgeQueryWorkspaceError> {
        Self::new_set(aspect_path, value)
    }

    pub fn new_set<T: Serialize>(
        aspect_path: impl Into<String>,
        value: T,
    ) -> Result<Self, ForgeQueryWorkspaceError> {
        let aspect_path = aspect_path.into();
        if aspect_path.trim().is_empty() {
            return Err(ForgeQueryWorkspaceError::new(
                "aspect path may not be empty",
            ));
        }
        let value = serde_json::to_value(value).map_err(|error| {
            ForgeQueryWorkspaceError::new(format!(
                "aspect `{aspect_path}` could not serialize into a mutation value: {error}"
            ))
        })?;
        Ok(Self {
            aspect_path,
            value,
            clears_existing_value: false,
        })
    }

    pub fn new_clear(aspect_path: impl Into<String>) -> Result<Self, ForgeQueryWorkspaceError> {
        let aspect_path = aspect_path.into();
        if aspect_path.trim().is_empty() {
            return Err(ForgeQueryWorkspaceError::new(
                "aspect path may not be empty",
            ));
        }
        Ok(Self {
            aspect_path,
            value: Value::Null,
            clears_existing_value: true,
        })
    }

    pub fn aspect_path(&self) -> &str {
        &self.aspect_path
    }

    pub fn value(&self) -> &Value {
        &self.value
    }

    pub fn clears_existing_value(&self) -> bool {
        self.clears_existing_value
    }

    pub fn declared_operation(&self) -> ForgeQueryAspectMutationOperation {
        ForgeQueryAspectMutationOperation::new(
            self.aspect_path.clone(),
            if self.clears_existing_value {
                ForgeQueryAspectMutationOperationKind::Clear
            } else {
                ForgeQueryAspectMutationOperationKind::Set
            },
        )
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct ForgeQueryAspectMutationBuilder {
    aspects: Vec<ForgeQueryAspectValue>,
    seen_aspects: BTreeSet<String>,
    error: Option<String>,
}

impl ForgeQueryAspectMutationBuilder {
    pub(crate) fn new() -> Self {
        Self::default()
    }

    pub fn aspect<T: Serialize>(mut self, aspect_path: impl Into<String>, value: T) -> Self {
        if self.error.is_some() {
            return self;
        }
        match ForgeQueryAspectValue::new_set(aspect_path, value) {
            Ok(aspect) => {
                if !self.seen_aspects.insert(aspect.aspect_path.clone()) {
                    self.error = Some(format!(
                        "aspect `{}` may only be declared once per mutation",
                        aspect.aspect_path()
                    ));
                } else {
                    self.aspects.push(aspect);
                }
            }
            Err(error) => self.error = Some(error.to_string()),
        }
        self
    }

    pub fn clear(mut self, aspect_path: impl Into<String>) -> Self {
        if self.error.is_some() {
            return self;
        }
        match ForgeQueryAspectValue::new_clear(aspect_path) {
            Ok(aspect) => {
                if !self.seen_aspects.insert(aspect.aspect_path.clone()) {
                    self.error = Some(format!(
                        "aspect `{}` may only be declared once per mutation",
                        aspect.aspect_path()
                    ));
                } else {
                    self.aspects.push(aspect);
                }
            }
            Err(error) => self.error = Some(error.to_string()),
        }
        self
    }

    pub(crate) fn build_insert(
        self,
        collection: impl Into<String>,
    ) -> Result<ForgeQueryWriteCommand, ForgeQueryRuntimeError> {
        let collection = collection.into();
        if collection.trim().is_empty() {
            return Err(ForgeQueryRuntimeError::Workspace(
                ForgeQueryWorkspaceError::new("collection may not be empty"),
            ));
        }
        let aspects = self.finish()?;
        Ok(ForgeQueryWriteCommand::InsertAspects {
            collection,
            aspects,
        })
    }

    pub(crate) fn build_update(
        self,
        entity_identity: impl Into<String>,
    ) -> Result<ForgeQueryWriteCommand, ForgeQueryRuntimeError> {
        let entity_identity = entity_identity.into();
        if entity_identity.trim().is_empty() {
            return Err(ForgeQueryRuntimeError::Workspace(
                ForgeQueryWorkspaceError::new("entity identity may not be empty"),
            ));
        }
        let aspects = self.finish()?;
        Ok(ForgeQueryWriteCommand::UpdateAspects {
            entity_identity,
            aspects,
        })
    }

    fn finish(self) -> Result<Vec<ForgeQueryAspectValue>, ForgeQueryRuntimeError> {
        if let Some(error) = self.error {
            return Err(ForgeQueryRuntimeError::Workspace(
                ForgeQueryWorkspaceError::new(error),
            ));
        }
        if self.aspects.is_empty() {
            return Err(ForgeQueryRuntimeError::Workspace(
                ForgeQueryWorkspaceError::new("aspect mutation must declare at least one aspect"),
            ));
        }
        Ok(self.aspects)
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct ForgeQueryMutationBatchBuilder {
    commands: Vec<ForgeQueryWriteCommand>,
    error: Option<String>,
}

impl ForgeQueryMutationBatchBuilder {
    pub(crate) fn new() -> Self {
        Self::default()
    }

    pub fn insert(
        mut self,
        collection: impl Into<String>,
        declaration: impl FnOnce(ForgeQueryAspectMutationBuilder) -> ForgeQueryAspectMutationBuilder,
    ) -> Self {
        if self.error.is_some() {
            return self;
        }
        match declaration(ForgeQueryAspectMutationBuilder::new()).build_insert(collection) {
            Ok(command) => self.commands.push(command),
            Err(error) => self.error = Some(error.to_string()),
        }
        self
    }

    pub fn update(
        mut self,
        entity_identity: impl Into<String>,
        declaration: impl FnOnce(ForgeQueryAspectMutationBuilder) -> ForgeQueryAspectMutationBuilder,
    ) -> Self {
        if self.error.is_some() {
            return self;
        }
        match declaration(ForgeQueryAspectMutationBuilder::new()).build_update(entity_identity) {
            Ok(command) => self.commands.push(command),
            Err(error) => self.error = Some(error.to_string()),
        }
        self
    }

    pub fn delete(mut self, entity_identity: impl Into<String>) -> Self {
        if self.error.is_some() {
            return self;
        }
        let entity_identity = entity_identity.into();
        if entity_identity.trim().is_empty() {
            self.error = Some("entity identity may not be empty".to_string());
            return self;
        }
        self.commands
            .push(ForgeQueryWriteCommand::Delete { entity_identity });
        self
    }

    pub(crate) fn finish(self) -> Result<Vec<ForgeQueryWriteCommand>, ForgeQueryRuntimeError> {
        if let Some(error) = self.error {
            return Err(ForgeQueryRuntimeError::Workspace(
                ForgeQueryWorkspaceError::new(error),
            ));
        }
        if self.commands.is_empty() {
            return Err(ForgeQueryRuntimeError::Workspace(
                ForgeQueryWorkspaceError::new("mutation batch must declare at least one operation"),
            ));
        }
        Ok(self.commands)
    }
}

pub(crate) fn aspect_values_to_payload(
    aspects: &[ForgeQueryAspectValue],
) -> Result<Value, ForgeQueryWorkspaceError> {
    let mut payload = Value::Object(Map::new());
    for aspect in aspects {
        set_json_path(&mut payload, aspect.aspect_path(), aspect.value().clone())?;
    }
    Ok(payload)
}

#[allow(deprecated)]
pub(crate) fn command_declared_aspect_paths(command: &ForgeQueryWriteCommand) -> Vec<String> {
    command_declared_aspect_operations(command)
        .into_iter()
        .map(|operation| operation.aspect_path().to_string())
        .collect()
}

#[allow(deprecated)]
pub(crate) fn command_declared_aspect_operations(
    command: &ForgeQueryWriteCommand,
) -> Vec<ForgeQueryAspectMutationOperation> {
    match command {
        ForgeQueryWriteCommand::Insert { .. } => Vec::new(),
        ForgeQueryWriteCommand::InsertAspects { aspects, .. }
        | ForgeQueryWriteCommand::UpdateAspects { aspects, .. } => aspects
            .iter()
            .map(ForgeQueryAspectValue::declared_operation)
            .collect(),
        ForgeQueryWriteCommand::UpdateAspect { aspect_path, .. } => {
            vec![ForgeQueryAspectMutationOperation::new(
                aspect_path.clone(),
                ForgeQueryAspectMutationOperationKind::Set,
            )]
        }
        ForgeQueryWriteCommand::Delete { .. } => Vec::new(),
    }
}

fn set_json_path(
    target: &mut Value,
    path: &str,
    value: Value,
) -> Result<(), ForgeQueryWorkspaceError> {
    let segments = path
        .split('.')
        .filter(|segment| !segment.is_empty())
        .collect::<Vec<_>>();
    if segments.is_empty() {
        return Err(ForgeQueryWorkspaceError::new(format!(
            "aspect path `{path}` must contain at least one segment"
        )));
    }
    let mut cursor = target;
    for segment in &segments[..segments.len() - 1] {
        match cursor {
            Value::Object(map) => {
                cursor = map
                    .entry((*segment).to_string())
                    .or_insert_with(|| Value::Object(Map::new()));
            }
            _ => {
                return Err(ForgeQueryWorkspaceError::new(format!(
                    "path `{path}` traversed through a non-object segment"
                )));
            }
        }
    }
    let final_segment = segments
        .last()
        .expect("aspect path segments are non-empty after validation");
    match cursor {
        Value::Object(map) => {
            map.insert((*final_segment).to_string(), value);
            Ok(())
        }
        _ => Err(ForgeQueryWorkspaceError::new(format!(
            "path `{path}` terminated on a non-object target"
        ))),
    }
}
