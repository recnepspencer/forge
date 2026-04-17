use serde::{Deserialize, Serialize};

use crate::expression::model::{ConditionSpec, Expr, IdentitySpec};
use crate::recipe::model::{RecipeReadSpec, RecipeSpec};

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
}

impl ComputedSpec {
    pub fn into_recipe(self, id: String) -> RecipeSpec {
        RecipeSpec {
            id,
            reads: self.reads,
            expr: self.expr,
            when: self.when,
            identity: self.identity,
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
}

impl OutputSpec {
    pub fn into_recipe(self, id: String) -> RecipeSpec {
        RecipeSpec {
            id,
            reads: self.reads,
            expr: self.expr,
            when: self.when,
            identity: self.identity.or(Some(IdentitySpec::Exact)),
        }
    }
}
