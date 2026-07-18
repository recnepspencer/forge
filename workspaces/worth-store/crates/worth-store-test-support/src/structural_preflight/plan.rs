use serde::{Deserialize, Serialize};

use super::StructuralPredicate;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum StructuralPreflightProfile {
    DeveloperSmoke,
    Ui,
    Complete,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct StructuralPreflightRequest {
    pub profile: StructuralPreflightProfile,
    pub predicates: Vec<StructuralPredicate>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PreflightInputScope {
    pub scope_identity: String,
    pub source_paths: Vec<String>,
    pub included_extensions: Vec<String>,
    pub input_identity: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct StructuralToolDeclaration {
    pub tool_identity: String,
    pub program: String,
    pub arguments: Vec<String>,
    pub source_scope_identity: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct StructuralPredicatePlan {
    pub predicate: StructuralPredicate,
    pub input_scopes: Vec<PreflightInputScope>,
    pub tool: Option<StructuralToolDeclaration>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct StructuralPreflightPlan {
    pub schema_version: u32,
    pub request: StructuralPreflightRequest,
    pub predicates: Vec<StructuralPredicatePlan>,
    pub plan_identity: String,
}

impl StructuralPreflightRequest {
    pub fn new(
        profile: StructuralPreflightProfile,
        mut predicates: Vec<StructuralPredicate>,
    ) -> Result<Self, String> {
        predicates.sort();
        predicates.dedup();
        if predicates.is_empty() {
            return Err("structural preflight requires at least one predicate".to_owned());
        }
        Ok(Self {
            profile,
            predicates,
        })
    }
}
