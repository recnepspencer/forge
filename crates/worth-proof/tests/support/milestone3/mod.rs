use std::any::type_name;

use worth_proof::{
    Admitted, AssumptionBasis, AuthorityRevalidationRequiredBasis,
    BoundaryBridgedAuthorityRevalidationRequiredBasis, BoundaryBridgedRebindRequiredBasis,
    BoundaryBridgedStaleReadableBasis, CurrentValidity, FreshnessScopedBasis, Lowered,
    RebindRequiredBasis, Recipe, Resolved, StaleReadableBasis,
};

use super::compile_fail::{CompileFailBundle, CompileFailCase};
use super::compile_pass::{CompilePassBundle, CompilePassCase};
use super::proof_shapes::{BasisDigest, FailureDigest, TransitionDigest};
use super::type_shapes::DebtItem;
use super::type_shapes::ResidualDebtReport;

pub fn compile_fail_bundle() -> CompileFailBundle {
    CompileFailBundle::new(
        "assumption_freshness_readmission_and_downgrade",
        vec![
            CompileFailCase::new(
                "unresolved_misuse",
                "tests/ui/milestone3/unresolved_recipe_cannot_bridge_trust_boundary.rs",
            ),
            CompileFailCase::new(
                "stale_misuse",
                "tests/ui/milestone3/stale_readable_recipe_rejects_strong_basis_api.rs",
            ),
            CompileFailCase::new(
                "rebind_misuse",
                "tests/ui/milestone3/rebind_required_recipe_cannot_lower.rs",
            ),
            CompileFailCase::new(
                "pre_readmission_misuse",
                "tests/ui/milestone3/boundary_bridged_recipe_rejects_strong_basis_api.rs",
            ),
            CompileFailCase::new(
                "basis_drift_misuse",
                "tests/ui/milestone3/shifted_basis_readmission_cannot_be_treated_as_original_basis.rs",
            ),
        ],
    )
}

pub fn compile_pass_bundle() -> CompilePassBundle {
    CompilePassBundle::new(
        "assumption_freshness_readmission_and_downgrade",
        vec![
            CompilePassCase::new(
                "control_progression",
                "tests/ui/milestone3/explicit_current_validity_progression_compiles.rs",
            ),
            CompilePassCase::new(
                "same_basis_readmission_progression",
                "tests/ui/milestone3/explicit_same_basis_readmission_progression_compiles.rs",
            ),
            CompilePassCase::new(
                "shifted_basis_readmission_progression",
                "tests/ui/milestone3/explicit_readmission_progression_compiles.rs",
            ),
        ],
    )
}

pub fn basis_digest() -> BasisDigest {
    BasisDigest::new(
        "assumption_freshness_readmission_and_downgrade",
        vec![
            type_name::<FreshnessScopedBasis<CurrentValidity, AssumptionBasis<u8>>>(),
            type_name::<FreshnessScopedBasis<CurrentValidity, AssumptionBasis<u16>>>(),
            type_name::<StaleReadableBasis<u8>>(),
            type_name::<RebindRequiredBasis<u8>>(),
            type_name::<AuthorityRevalidationRequiredBasis<u8>>(),
            type_name::<BoundaryBridgedAuthorityRevalidationRequiredBasis<u8>>(),
        ],
    )
}

pub fn failure_digest() -> FailureDigest {
    FailureDigest::new(
        "assumption_freshness_readmission_and_downgrade",
        vec![
            "unresolved_misuse::tests/ui/milestone3/unresolved_recipe_cannot_bridge_trust_boundary.rs",
            "stale_misuse::tests/ui/milestone3/stale_readable_recipe_rejects_strong_basis_api.rs",
            "rebind_misuse::tests/ui/milestone3/rebind_required_recipe_cannot_lower.rs",
            "pre_readmission_misuse::tests/ui/milestone3/boundary_bridged_recipe_rejects_strong_basis_api.rs",
            "basis_drift_misuse::tests/ui/milestone3/shifted_basis_readmission_cannot_be_treated_as_original_basis.rs",
        ],
    )
}

pub fn transition_digest() -> TransitionDigest {
    TransitionDigest::new(
        "assumption_freshness_readmission_and_downgrade",
        vec![
            type_name::<
                Recipe<Resolved, u64, FreshnessScopedBasis<CurrentValidity, AssumptionBasis<u8>>>,
            >(),
            type_name::<Recipe<Resolved, u64, RebindRequiredBasis<u8>>>(),
            type_name::<Recipe<Resolved, u64, BoundaryBridgedRebindRequiredBasis<u8>>>(),
            type_name::<
                Recipe<Lowered, u64, FreshnessScopedBasis<CurrentValidity, AssumptionBasis<u8>>>,
            >(),
            type_name::<Recipe<Lowered, u64, StaleReadableBasis<u8>>>(),
            type_name::<Recipe<Lowered, u64, BoundaryBridgedStaleReadableBasis<u8>>>(),
            type_name::<
                Recipe<Admitted, u64, FreshnessScopedBasis<CurrentValidity, AssumptionBasis<u8>>>,
            >(),
            type_name::<
                Recipe<Admitted, u64, FreshnessScopedBasis<CurrentValidity, AssumptionBasis<u16>>>,
            >(),
            type_name::<Recipe<Admitted, u64, AuthorityRevalidationRequiredBasis<u8>>>(),
            type_name::<Recipe<Admitted, u64, BoundaryBridgedAuthorityRevalidationRequiredBasis<u8>>>(
            ),
        ],
    )
}

pub fn residual_debt_report() -> ResidualDebtReport {
    ResidualDebtReport::new(
        "assumption_freshness_readmission_and_downgrade",
        vec![DebtItem::new(
            "representative_domain_basis_catalog",
            "Milestone 3 closes the shared substrate with representative same-basis, shifted-basis, stale, rebind, and trust-shift lanes, but it does not enumerate every domain-specific basis taxonomy; later domain crates still choose their own semantic basis families on top of this law.",
        )],
    )
}
