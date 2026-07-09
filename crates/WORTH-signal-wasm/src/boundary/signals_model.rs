use serde::{Deserialize, Serialize};

use crate::expression::model::{ConditionSpec, Expr, IdentitySpec};
use crate::recipe::model::{RecipeReadSpec, RecipeSpec, WasmAspectId};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct InputOptions {
    #[serde(default)]
    pub produces_aspects: Option<Vec<WasmAspectId>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComputedSpec {
    #[serde(default)]
    pub reads: Vec<RecipeReadSpec>,
    pub expr: Expr,
    #[serde(default)]
    pub when: Option<ConditionSpec>,
    #[serde(default)]
    pub identity: Option<IdentitySpec>,
    #[serde(default)]
    pub produces_aspects: Option<Vec<WasmAspectId>>,
}

impl ComputedSpec {
    pub fn into_recipe(self, id: String) -> RecipeSpec {
        RecipeSpec {
            id,
            reads: self.reads,
            expr: self.expr,
            when: self.when,
            identity: self.identity,
            produces_aspects: self.produces_aspects,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OutputSpec {
    #[serde(default)]
    pub reads: Vec<RecipeReadSpec>,
    pub expr: Expr,
    #[serde(default)]
    pub when: Option<ConditionSpec>,
    #[serde(default)]
    pub identity: Option<IdentitySpec>,
    #[serde(default)]
    pub produces_aspects: Option<Vec<WasmAspectId>>,
}

impl OutputSpec {
    pub fn into_recipe(self, id: String) -> RecipeSpec {
        RecipeSpec {
            id,
            reads: self.reads,
            expr: self.expr,
            when: self.when,
            identity: self.identity.or(Some(IdentitySpec::Exact)),
            produces_aspects: self.produces_aspects,
        }
    }
}
