use worth_primitives::{truth_digest_parts, TruthDigestScope};

use super::error::WorkloadCatalogError;
use super::recipe_kind::{WorkloadCatalogRecipeKind, WorkloadCatalogSupportPosture};
use super::support_receipt::{WorkloadCatalogDeclarationReceipt, WorkloadCatalogSupportReceipt};

pub(super) fn ensure_member_support(
    recipe: super::catalog::WorkloadCatalogRecipe,
) -> Result<(), WorkloadCatalogError> {
    let support = recipe.inspect_support()?;
    if support.posture() == WorkloadCatalogSupportPosture::Admitted {
        Ok(())
    } else {
        Err(WorkloadCatalogError::UnsupportedRecipe {
            recipe: support.recipe(),
            reason: support.human_reason().to_string(),
        })
    }
}

pub(super) fn ensure_clean_fail_member_support(
    recipe: super::catalog::WorkloadCatalogRecipe,
) -> Result<(), WorkloadCatalogError> {
    recipe.inspect_clean_fail_support().map(|_| ())
}

pub(super) fn require_admitted_pair_support(
    support: &WorkloadCatalogSupportReceipt,
) -> Result<(), WorkloadCatalogError> {
    if support.posture() == WorkloadCatalogSupportPosture::Admitted {
        Ok(())
    } else {
        Err(WorkloadCatalogError::UnsupportedRecipe {
            recipe: support.recipe(),
            reason: support.human_reason().to_string(),
        })
    }
}

pub(super) fn operand_pair_identity(
    recipe: WorkloadCatalogRecipeKind,
    left: &WorkloadCatalogDeclarationReceipt,
    right: &WorkloadCatalogDeclarationReceipt,
) -> String {
    truth_digest_parts(
        TruthDigestScope::ArtifactIdentity,
        &[
            "workload-catalog-boolean-operand-pair".to_string(),
            format!("recipe:{}", recipe.query_key()),
            format!("left:{}", left.query_declaration_digest()),
            format!("right:{}", right.query_declaration_digest()),
        ],
    )
}
