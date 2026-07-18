use crate::selection::{
    select, ProofProcessModel, ProofProductUnavailable, StoreProofMode, StoreProofRequest,
};

use super::{current_inventory, selection_preflight, workspace_root};

#[test]
fn selection_is_deterministic_and_scenario_filterable() {
    let root = workspace_root();
    let current = current_inventory(&root);
    let smoke = || {
        select(
            &root,
            &current,
            StoreProofRequest::new(StoreProofMode::Smoke, None, None, None, None, true),
            selection_preflight(&root),
        )
        .unwrap()
    };
    let first = smoke();
    let second = smoke();
    assert_eq!(first.plan_digest, second.plan_digest);
    assert_eq!(first.units, second.units);
    assert!(first.units.iter().all(|unit| unit.case_filter.is_some()));
    assert!(first.units.iter().all(|unit| {
        !matches!(
            unit.feature_lane,
            crate::selection::StoreFeatureLane::ProductionEquivalent
        )
    }));
    assert!(first.units.iter().any(|unit| {
        unit.target_name == "durability_recovery"
            && unit.case_filter.as_deref() == Some("wal_durability_ack")
    }));
    assert!(first.units.iter().any(|unit| {
        unit.target_name == "s5_tier_movement_future_chunk_compile_fail"
            && unit.process_model == ProofProcessModel::NestedCargoProcess
            && unit.case_filter.as_deref()
                == Some("future_chunk_placeholder_boundary_misuse_does_not_compile")
    }));

    let proof = current
        .inventory()
        .proofs
        .iter()
        .find(|proof| {
            proof.case.target_identity.as_deref()
                == Some("worth-store-certification::test::durability_recovery")
                && proof.products.contains("store-ci:recovery")
        })
        .unwrap();
    let scenario = format!(
        "{}::{}",
        proof.case.identity.package, proof.case.identity.responsibility
    );
    let partition = proof
        .products
        .iter()
        .find_map(|product| product.strip_prefix("store-ci:"))
        .unwrap()
        .to_owned();
    let selected = select(
        &root,
        &current,
        StoreProofRequest::new(
            StoreProofMode::Ci,
            None,
            Some(partition),
            None,
            Some(scenario),
            true,
        ),
        selection_preflight(&root),
    )
    .unwrap();
    assert_eq!(selected.units.len(), 1);
    assert!(selected.units[0].case_filter.is_some());
}

#[test]
fn explicit_owner_and_hardware_profile_requirements_deny_before_selection() {
    let owner = StoreProofRequest::new(StoreProofMode::Owner, None, None, None, None, true);
    assert!(matches!(
        owner.selected_product_names(&current_inventory(&workspace_root())),
        Err(ProofProductUnavailable::ExplicitOwnerRequired)
    ));

    let foreign_host = if cfg!(windows) {
        "linux-ext4-nvme"
    } else {
        "windows-ntfs-nvme"
    };
    let hardware = StoreProofRequest::new(
        StoreProofMode::Hardware,
        None,
        None,
        Some(foreign_host.to_owned()),
        None,
        true,
    );
    assert!(matches!(
        hardware.validate_host(),
        Err(ProofProductUnavailable::UnsupportedHost { .. })
    ));
}

#[test]
fn soak_and_release_require_registered_execution_inputs() {
    let current = current_inventory(&workspace_root());
    let unknown_soak = StoreProofRequest::new(
        StoreProofMode::Soak,
        None,
        None,
        Some("whatever".to_owned()),
        None,
        true,
    )
    .with_seed(Some(42));
    assert!(matches!(
        unknown_soak.selected_product_names(current),
        Err(ProofProductUnavailable::UnknownProofProfile { .. })
    ));

    let missing_seed = StoreProofRequest::new(
        StoreProofMode::Soak,
        None,
        None,
        Some("checkpoint-heavy".to_owned()),
        None,
        true,
    );
    assert!(matches!(
        missing_seed.selected_product_names(current),
        Err(ProofProductUnavailable::ExplicitSeedRequired(_))
    ));

    let release = StoreProofRequest::new(StoreProofMode::Release, None, None, None, None, true);
    assert!(matches!(
        release.selected_product_names(current),
        Err(ProofProductUnavailable::NamedBackendRequired(_))
    ));
}

#[test]
fn full_ci_denies_when_a_required_partition_loses_reachability() {
    let root = workspace_root();
    let current = current_inventory(&root);
    let mut missing_recovery = current.inventory().clone();
    for proof in &mut missing_recovery.proofs {
        proof.products.remove("store-ci:recovery");
    }
    let missing_recovery = crate::ValidatedProofInventory::from_classified(missing_recovery);
    let denial = select(
        &root,
        &missing_recovery,
        StoreProofRequest::new(StoreProofMode::Ci, None, None, None, None, true),
        selection_preflight(&root),
    )
    .unwrap_err();
    assert!(matches!(
        denial,
        ProofProductUnavailable::MissingRequiredProofProduct(product)
            if product == "store-ci:recovery"
    ));
}

#[test]
fn full_ci_and_owner_plans_keep_their_declared_boundaries() {
    let root = workspace_root();
    let current = current_inventory(&root);
    let ci = select(
        &root,
        current,
        StoreProofRequest::new(StoreProofMode::Ci, None, None, None, None, true),
        selection_preflight(&root),
    )
    .unwrap();
    for required in [
        "store-ci:recovery",
        "store-ci:physical_isolation",
        "store-ci:scheduling",
        "store-ci:layout",
        "store-ci:blobs",
        "store-ci:security",
        "store-ci:certification-owner",
        "store-ci:physical-certification",
        "store-ci:formal-conformance",
        "store-ci:feature-compatibility",
        "store-ci:test-control",
    ] {
        assert!(ci
            .selection
            .included_products
            .iter()
            .any(|product| product == required));
    }

    let owner_name = "worth-store-layout-indexes";
    let owner = select(
        &root,
        current,
        StoreProofRequest::new(
            StoreProofMode::Owner,
            Some(owner_name.to_owned()),
            None,
            None,
            None,
            true,
        ),
        selection_preflight(&root),
    )
    .unwrap();
    assert!(owner.units.iter().all(|unit| unit.package == owner_name));
    assert!(!owner
        .units
        .iter()
        .any(|unit| unit.package == "worth-store-certification"));
}
