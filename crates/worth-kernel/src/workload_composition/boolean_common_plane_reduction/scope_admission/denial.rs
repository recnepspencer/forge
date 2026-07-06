use crate::workload_composition::WorkloadCatalogRecipeKind;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PlanarBooleanCommonPlaneScopeAdmissionError {
    UnsupportedOperandPairRecipe {
        actual_recipe: WorkloadCatalogRecipeKind,
        admitted_scope: &'static str,
        request_identity: String,
        operand_pair_identity: String,
    },
}

impl PlanarBooleanCommonPlaneScopeAdmissionError {
    pub fn human_reason(&self) -> &'static str {
        match self {
            Self::UnsupportedOperandPairRecipe { .. } => {
                "Common-plane scope admission only admits closed planar body pair families before plane agreement begins."
            }
        }
    }

    pub fn actual_recipe(&self) -> WorkloadCatalogRecipeKind {
        match self {
            Self::UnsupportedOperandPairRecipe { actual_recipe, .. } => *actual_recipe,
        }
    }

    pub fn admitted_scope(&self) -> &'static str {
        match self {
            Self::UnsupportedOperandPairRecipe { admitted_scope, .. } => admitted_scope,
        }
    }

    pub fn request_identity(&self) -> &str {
        match self {
            Self::UnsupportedOperandPairRecipe {
                request_identity, ..
            } => request_identity,
        }
    }

    pub fn operand_pair_identity(&self) -> &str {
        match self {
            Self::UnsupportedOperandPairRecipe {
                operand_pair_identity,
                ..
            } => operand_pair_identity,
        }
    }
}
