use crate::failure::{StoreError, StoreErrorKind};
use worth_relational::facade::history::{BranchId, CommitId};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SingleEntityAspectScope {
    entity_id: String,
}
impl SingleEntityAspectScope {
    pub fn new(entity_id: impl Into<String>) -> Self {
        Self {
            entity_id: entity_id.into(),
        }
    }
    pub fn entity_id(&self) -> &str {
        &self.entity_id
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EntitySetUniformAspectScope {
    entity_ids: Vec<String>,
}
impl EntitySetUniformAspectScope {
    pub fn new(entity_ids: Vec<String>) -> Self {
        Self { entity_ids }
    }
    pub fn entity_ids(&self) -> &[String] {
        &self.entity_ids
    }
    pub(crate) fn canonical_entity_ids(&self) -> Vec<String> {
        canonicalize_strings(&self.entity_ids)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CdcTouchedAspectScope {
    cdc_token: String,
    touched_entity_ids: Vec<String>,
}
impl CdcTouchedAspectScope {
    pub fn new(cdc_token: impl Into<String>, touched_entity_ids: Vec<String>) -> Self {
        Self {
            cdc_token: cdc_token.into(),
            touched_entity_ids,
        }
    }
    pub fn cdc_token(&self) -> &str {
        &self.cdc_token
    }
    pub fn touched_entity_ids(&self) -> &[String] {
        &self.touched_entity_ids
    }
    pub(crate) fn canonical_touched_entity_ids(&self) -> Vec<String> {
        canonicalize_strings(&self.touched_entity_ids)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AspectScopeClass {
    SingleEntity(SingleEntityAspectScope),
    EntitySetUniform(EntitySetUniformAspectScope),
    CdcTouched(CdcTouchedAspectScope),
    Generalized { descriptor: String },
}
impl AspectScopeClass {
    pub fn label(&self) -> &'static str {
        match self {
            Self::SingleEntity(_) => "single_entity",
            Self::EntitySetUniform(_) => "entity_set_uniform",
            Self::CdcTouched(_) => "cdc_touched",
            Self::Generalized { .. } => "generalized",
        }
    }

    pub(crate) fn canonical_scope_key(&self) -> Result<CanonicalScopeKey, StoreError> {
        match self {
            Self::SingleEntity(scope) => {
                let entity_id = scope.entity_id.trim();
                if entity_id.is_empty() {
                    return Err(StoreError::new(
                        StoreErrorKind::AspectScopeAmbiguous,
                        "single-entity aspect scope requires a non-empty entity id",
                    ));
                }
                Ok(CanonicalScopeKey {
                    scope_label: self.label().to_string(),
                    members: vec![entity_id.to_string()],
                    cdc_token: None,
                })
            }
            Self::EntitySetUniform(scope) => {
                let members = scope.canonical_entity_ids();
                if members.is_empty() {
                    return Err(StoreError::new(
                        StoreErrorKind::AspectScopeAmbiguous,
                        "entity-set uniform aspect scope requires at least one entity id",
                    ));
                }
                Ok(CanonicalScopeKey {
                    scope_label: self.label().to_string(),
                    members,
                    cdc_token: None,
                })
            }
            Self::CdcTouched(scope) => {
                let cdc_token = scope.cdc_token.trim();
                if cdc_token.is_empty() {
                    return Err(StoreError::new(
                        StoreErrorKind::AspectScopeAmbiguous,
                        "cdc-touched aspect scope requires a non-empty CDC token",
                    ));
                }
                let members = scope.canonical_touched_entity_ids();
                if members.is_empty() {
                    return Err(StoreError::new(
                        StoreErrorKind::AspectScopeAmbiguous,
                        "cdc-touched aspect scope requires at least one touched entity id",
                    ));
                }
                Ok(CanonicalScopeKey {
                    scope_label: self.label().to_string(),
                    members,
                    cdc_token: Some(cdc_token.to_string()),
                })
            }
            Self::Generalized { descriptor } => {
                if descriptor.trim().is_empty() {
                    return Err(StoreError::new(
                        StoreErrorKind::AspectScopeAmbiguous,
                        "generalized aspect scope requires a non-empty descriptor",
                    ));
                }
                Ok(CanonicalScopeKey {
                    scope_label: self.label().to_string(),
                    members: vec![descriptor.trim().to_string()],
                    cdc_token: None,
                })
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AspectLayoutTarget {
    branch_id: BranchId,
    frontier_commit_id: CommitId,
}
impl AspectLayoutTarget {
    pub fn new(branch_id: BranchId, frontier_commit_id: CommitId) -> Self {
        Self {
            branch_id,
            frontier_commit_id,
        }
    }
    pub fn branch_id(&self) -> &BranchId {
        &self.branch_id
    }
    pub fn frontier_commit_id(&self) -> CommitId {
        self.frontier_commit_id
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AspectProjectionSet {
    aspect_names: Vec<String>,
}
impl AspectProjectionSet {
    pub fn new(aspect_names: Vec<String>) -> Self {
        Self { aspect_names }
    }
    pub fn aspect_names(&self) -> &[String] {
        &self.aspect_names
    }
    pub(crate) fn canonical_aspects(&self) -> Result<Vec<String>, StoreError> {
        let aspects = canonicalize_strings(&self.aspect_names);
        if aspects.is_empty() {
            return Err(StoreError::new(
                StoreErrorKind::AspectScopeAmbiguous,
                "aspect projection set requires at least one aspect name",
            ));
        }
        Ok(aspects)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AspectLayoutReadRequest {
    target: AspectLayoutTarget,
    scope_class: AspectScopeClass,
    projection_set: AspectProjectionSet,
}
impl AspectLayoutReadRequest {
    pub fn new(
        target: AspectLayoutTarget,
        scope_class: AspectScopeClass,
        projection_set: AspectProjectionSet,
    ) -> Self {
        Self {
            target,
            scope_class,
            projection_set,
        }
    }
    pub fn target(&self) -> &AspectLayoutTarget {
        &self.target
    }
    pub fn scope_class(&self) -> &AspectScopeClass {
        &self.scope_class
    }
    pub fn projection_set(&self) -> &AspectProjectionSet {
        &self.projection_set
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AspectReadRegime {
    DirectLayoutSlice,
    BlockReuseBacked,
    ControlReplay,
    ExplicitBroadFallback,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub(crate) struct CanonicalScopeKey {
    pub(crate) scope_label: String,
    pub(crate) members: Vec<String>,
    pub(crate) cdc_token: Option<String>,
}

fn canonicalize_strings(values: &[String]) -> Vec<String> {
    let mut canonical = values
        .iter()
        .map(|value| value.trim())
        .filter(|value| !value.is_empty())
        .map(ToOwned::to_owned)
        .collect::<Vec<_>>();
    canonical.sort();
    canonical.dedup();
    canonical
}
