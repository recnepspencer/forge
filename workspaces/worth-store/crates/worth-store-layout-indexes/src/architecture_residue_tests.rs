fn source(path: &str) -> String {
    std::fs::read_to_string(std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(path)).unwrap()
}
fn rust_sources_below(path: &std::path::Path, found: &mut Vec<std::path::PathBuf>) {
    for entry in std::fs::read_dir(path).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_dir() {
            rust_sources_below(&path, found);
        } else if path.extension().is_some_and(|extension| extension == "rs") {
            found.push(path);
        }
    }
}
fn source_tree(path: &str) -> String {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(path);
    let mut sources = Vec::new();
    rust_sources_below(&root, &mut sources);
    sources.sort();
    sources
        .into_iter()
        .map(|source| std::fs::read_to_string(source).unwrap())
        .collect::<Vec<_>>()
        .join("\n")
}
mod exact_source_authority_tests;
mod owner_execution_surface_tests;
#[test]
fn btree_lookup_owns_lowering_and_source_bound_readiness() {
    let owner = source("src/access/execution/btree_lookup/readiness.rs");
    let operation = source("src/access/execution/btree_lookup/operation.rs");
    let facade = source("src/read/page_lookup.rs");
    let authority = source("src/access/execution/btree_lookup/authority.rs");
    let shared = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("src/access/execution/transition_authority.rs");
    assert!(owner.contains("pub struct BTreeLookupReady"));
    assert!(owner.contains("pub struct BTreeLookupReadinessOutcome"));
    assert!(owner.contains("enum BTreeLookupReadinessCase"));
    assert!(operation.contains("BTreeLookupReadinessView::Stale"));
    assert!(operation.contains(".into_stale()"));
    assert!(facade.contains("current_btree_materialization_frontier"));
    assert!(facade.contains("request.current_catalog"));
    assert!(facade.contains("request.current_source.as_ref().unwrap_or(&request.source)"));
    assert!(!source_tree("src/access/execution/btree_lookup").contains("StaleBTreeLookup"));
    assert!(authority.contains("BTreeLookupReadinessAuthority"));
    assert!(authority.contains("pub(in crate::access::execution::btree_lookup)"));
    assert!(!shared.exists());
}
#[test]
fn read_operations_keep_worth_proof_issuance_inside_the_exact_owner() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let btree = source("src/access/execution/btree_lookup/authority.rs");
    let degraded = source("src/access/execution/degraded_scan/authority.rs");
    for (owner, source) in [
        ("btree_lookup", btree.as_str()),
        ("degraded_scan", degraded.as_str()),
    ] {
        assert!(
            source.contains(&format!("pub(in crate::access::execution::{owner})")),
            "{owner} issuance is not visibility-scoped to its operation owner"
        );
        assert!(!source.contains("pub(crate) fn"));
        assert!(!source.contains("pub(super) fn"));
    }

    for removed in [
        "src/access/execution/transition_authority.rs",
        "src/access/execution/lowering_facade.rs",
        "src/access/execution/physical_execution.rs",
        "src/access/execution/runtime_owners.rs",
        "src/access/execution/btree_lookup/progression.rs",
        "src/access/execution/degraded_scan/coordinator.rs",
    ] {
        assert!(
            !root.join(removed).exists(),
            "shared read authority lane survived at {removed}"
        );
    }

    for owner_outcome in [
        "src/access/execution/btree_lookup/lowering.rs",
        "src/access/execution/btree_lookup/readiness.rs",
        "src/access/execution/degraded_scan/lowering.rs",
        "src/access/execution/degraded_scan/readiness.rs",
    ] {
        let source = source(owner_outcome);
        for sibling_visible_issuer in [
            "pub(crate) fn issue(",
            "pub(crate) const fn issue(",
            "pub(super) fn issue(",
            "pub(super) const fn issue(",
        ] {
            assert!(
                !source.contains(sibling_visible_issuer),
                "{owner_outcome} exposes sibling-visible raw issuance through {sibling_visible_issuer}",
            );
        }
    }
}

#[test]
fn planning_cannot_issue_an_unowned_indexed_operation() {
    let decision = source("src/planning/decision.rs");
    let issuance = source("src/planning/selection_issuance.rs");
    let selected = source("src/planning/selected_plan.rs");

    assert!(!decision.contains("PlanSelectionDecision::Indexed"));
    assert!(!issuance.contains("SelectionIssuedPayload::Indexed"));
    assert!(!selected.contains("SelectedIndexedAccessPlan"));
    let identity = source("src/keyspace/request_identity.rs");
    let selector = source("src/planning/selection_receipt.rs");
    let request = source("src/planning/access_request.rs");
    let identity_basis = source("src/planning/plan_identity.rs");
    let materialization = source("src/materialization/admission.rs");
    let freshness = source("src/materialization/freshness.rs");
    assert!(!identity.contains("scope_only_for_legacy_test"));
    assert!(request.contains("AdmittedPhysicalReadRequest"));
    assert!(request.contains("AdmittedPhysicalRecoveryRequest"));
    assert!(request.contains("AdmittedPhysicalMutationRequest"));
    assert!(!request.contains("struct AdmittedPhysicalAccessRequest"));
    assert!(!selector.contains("select_with_budget"));
    assert!(!selector.contains("concrete_key_for_test"));
    assert!(!selector.contains("admit_catalog_coverage_for_unit_test"));
    assert!(!materialization.contains("admit_catalog_coverage_for_unit_test"));
    assert!(!materialization.contains("require_current_for_unit_test"));
    assert!(!freshness.contains("from_source_for_unit_test"));
    assert!(!identity_basis.contains("family_code("));
    assert!(!identity_basis.contains("detail_code("));
    assert!(!identity_basis.contains("mix_snapshot("));
}

#[test]
fn declaration_facade_cannot_export_intermediate_authority_stages() {
    let public_families = source("src/declarations.rs");
    let declaration_facade = source("src/catalog/declaration_registry.rs");

    for forbidden in [
        "ArtifactFamilyAuthorityWitness",
        "ArtifactFamilyLifecycleAdmission",
        "ArtifactScopePartitionWitness",
        "ArtifactAuthorityRoleWitness",
        "ArtifactDerivedAccuracyWitness",
    ] {
        assert!(
            !public_families.contains(forbidden),
            "cold family vocabulary still exports intermediate authority {forbidden}",
        );
    }
    for forbidden in [
        "pub fn require_production_authority",
        "pub fn require_strategy_lifecycle",
        "pub fn classify_family",
        "pub fn declare_authority_role",
        "pub fn require_scope_partition",
        "pub fn declare_physical_key_domain",
    ] {
        assert!(
            !declaration_facade.contains(forbidden),
            "declaration facade still exposes intermediate authority operation {forbidden}",
        );
    }
}

#[test]
fn lsm_lookup_retains_publication_and_exposes_exhaustive_cases() {
    let lookup_source = source("src/strategy/lsm/execution/lookup_source.rs");
    let execution = source("src/strategy/lsm/execution/lookup_outcome.rs");
    let module = source("src/strategy/lsm/execution/mod.rs");

    assert!(lookup_source.contains("PublishedLsmMembershipReplacement"));
    assert!(!lookup_source.contains("replacement_output: BlobWalRecordIdentity"));
    assert!(execution.contains("enum BaselineLsmLookupObservation"));
    assert!(execution.contains("pub enum BaselineLsmLookupView"));
    assert!(!execution.contains("absence: Option<BaselineLsmLookupAbsence>"));
    assert!(!module.contains("enum BaselineLsmLookupObservation"));
}

#[test]
fn lsm_compaction_runtime_consumes_one_owner_admitted_demand() {
    let demand = source("src/strategy/lsm/execution/compaction_demand.rs");
    let runtime = source("src/strategy/lsm/compaction/preparation.rs");
    let old_request = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("src/strategy/lsm/execution/request.rs");
    let old_binding = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("src/strategy/lsm/execution/binding.rs");

    for retained in ["membership:", "output:", "physical_intent:"] {
        assert!(demand.contains(retained));
    }
    assert!(demand.contains("admit_lsm_replacement_output"));
    assert!(runtime.contains("demand: AdmittedLsmCompactionDemand"));
    assert!(runtime.contains("LsmCompactionPreparationOutcome::issue(prepare(demand))"));
    assert!(!runtime.contains("LsmPhysicalCompactionIntent"));
    assert!(!runtime.contains("LsmCompactionInputs"));
    assert!(!old_request.exists());
    assert!(!old_binding.exists());
}

#[test]
fn freshness_is_bound_to_the_exact_owner_source_not_a_coarse_root_projection() {
    let freshness = source("src/materialization/freshness.rs");
    let materialization_source = source("src/materialization/source.rs");
    let materialization_admission = source("src/materialization/admission.rs");
    let runtime = source_tree("src/read");
    let lsm_admission = source_tree("src/strategy/lsm/execution/admission");
    let btree_witness = source("src/strategy/btree/execution/witness.rs");

    assert!(freshness.contains("if materialization.source() != frontier.source()"));
    assert!(!freshness.contains("source.root_owner() != frontier.source.root_owner()"));
    assert!(materialization_source.contains(
        "std::sync::Arc<crate::strategy::btree::execution::BaselineBTreeReadSourceReceipt>"
    ));
    assert!(materialization_source.contains(
        "BTreePublication(worth_store_physical_format::RootPublicationValidationWitness)"
    ));
    assert!(materialization_source
        .contains("LayoutMaterializationSourceAuthority::BTreePublication(publication)"));
    assert!(materialization_admission
        .contains("source.store_authority_identity() != family.authority_identity()"));
    assert!(materialization_admission
        .contains("publication: worth_store_physical_format::RootPublicationValidationWitness"));
    assert!(runtime.contains("current_btree_materialization_frontier"));
    assert!(runtime.contains("admit_btree_lookup_materialization"));
    assert!(!runtime.contains("admit_btree_root_materialization"));
    assert!(runtime.contains("current_lsm_materialization_frontier"));
    assert!(!lsm_admission.contains("BaselineLsmLookupAdmissionCase::Denied"));
    assert!(btree_witness.contains("replay_artifact.store_identity().clone()"));
}

#[test]
fn ordinary_operation_declarations_do_not_transport_coverage() {
    let contract = source("src/access/shape/contract.rs");
    let mut shape_sources = Vec::new();
    rust_sources_below(
        &std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src/access/shape"),
        &mut shape_sources,
    );
    let reads = source_tree("src/read");
    let btree_replay = source_tree("src/recovery/btree_replay");
    let lsm_replay = source_tree("src/maintenance/lsm");

    assert!(!contract.contains("\n    coverage: Option<LayoutCoverageWitness>"));
    assert!(!contract.contains("legacy_test_coverage"));
    assert!(!contract.contains("fn coverage(self)"));
    for path in shape_sources {
        let text = std::fs::read_to_string(&path).unwrap();
        assert!(
            !text.contains("LayoutCoverageWitness"),
            "shape declaration imports materialization authority in {}",
            path.display()
        );
    }
    assert!(!reads.contains("materialization.coverage()"));
    assert!(!btree_replay.contains("materialization.coverage()"));
    assert!(!lsm_replay.contains("materialization.coverage()"));
    assert!(reads.contains("point_access()"));
    assert!(btree_replay.contains("rebuild_access"));
    assert!(lsm_replay.contains("rebuild_access"));
}

#[test]
fn coverage_authority_always_retains_an_owner_source() {
    let coverage = source("src/materialization/coverage.rs");
    let planning_facade = source("src/planning/admission.rs");
    let bootstrap_admission = source("src/bootstrap/catalog_read_outcome.rs");

    assert!(coverage.contains("source: LayoutMaterializationSourceIdentity"));
    assert!(!coverage.contains("source: Option<LayoutMaterializationSourceIdentity>"));
    assert!(!coverage.contains("Self::new(state, watermark, watermark, None, None)"));
    for fixture_only_method in [
        "exact_root_epoch_coverage",
        "exact_wal_lsn_coverage",
        "exact_blob_generation_coverage",
        "exact_checkpoint_coverage",
        "stale_root_epoch_coverage",
        "lagged_wal_lsn_coverage",
        "partial_wal_lsn_coverage",
        "quarantined_wal_lsn_coverage",
    ] {
        assert!(
            !planning_facade.contains(fixture_only_method),
            "planning facade still authors fixture coverage through {fixture_only_method}",
        );
    }
    assert!(!bootstrap_admission.contains("exact_materialization_for"));
}

#[test]
fn materialization_rejects_scalar_sources_and_lsm_reopen_retains_store_lineage() {
    let facade = source("src/planning/admission.rs");
    let admission = source("src/materialization/admission.rs");
    let lsm_key = std::fs::read_to_string(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../worth-store-lsm-authority/src/membership/model.rs"),
    )
    .unwrap();
    let lsm_reopen_owner = std::fs::read_to_string(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../worth-store-lsm-authority/src/membership/runtime/reopen/operation.rs"),
    )
    .unwrap();

    for removed in [
        "admit_catalog_root_epoch_materialization",
        "admit_catalog_wal_materialization",
    ] {
        assert!(
            !facade.contains(removed),
            "scalar source lane {removed} remains"
        );
    }
    assert!(admission.contains("fn require_lsm_source_authority"));
    assert!(admission.contains("key.authority_identity() != family.authority_identity()"));
    assert!(admission.contains("key.security_identity() != family.security_identity()"));
    assert!(lsm_key
        .contains("authority_identity: worth_store_authority::StoreCurrentAuthorityIdentity"));
    assert!(lsm_key.contains("security_identity: worth_store_security::StoreSecurityScopeIdentity"));
    assert!(lsm_reopen_owner
        .contains("current_scope: &worth_store_security::StoreCurrentSecurityScopeWitnessSet"));
}
