use forge_store_layout_indexes::layout_closeout::{
    LegacyAccessPathBypassInventory, LegacySurfaceDisposition,
    LegacySurfaceDispositionAndDedicatedWorkspaceBoundary,
};

const LEGACY_LIB_RS: &str = include_str!("../../../../../crates/forge-store/src/lib.rs");
const LEGACY_LAYOUT_READS_RS: &str =
    include_str!("../../../../../crates/forge-store/src/facade/layout_reads.rs");
const LEGACY_LAYOUT_SUPPORT_RS: &str =
    include_str!("../../../../../crates/forge-store/src/facade/layout_support.rs");
const LEGACY_PUBLIC_EXPORT_FRAGMENTS: [&str; 8] = [
    "pub use facade::{ForgeStore, ForgeStoreBuilder};",
    "\n    CompatibilityRegistry,\n",
    "\n    MaintenanceDeclaration,\n",
    "\n    SubscriptionSupportAccessStructure,\n",
    "\n    AspectLayoutReadRequest,\n",
    "\n    AspectLayoutReadPlanDecision,\n",
    "\n    AspectLayoutReadExecutionResult,\n",
    "\n    Milestone6ChunkModelExport,\n",
];
const LEGACY_FACADE_METHODS: [(&str, &str); 17] = [
    (
        "pub fn plan_aspect_layout_read(",
        "ForgeStore::plan_aspect_layout_read",
    ),
    (
        "pub fn admit_structural_block_reuse(",
        "ForgeStore::admit_structural_block_reuse",
    ),
    (
        "pub fn freeze_chunk_model(",
        "ForgeStore::freeze_chunk_model",
    ),
    (
        "pub fn admit_milestone_7_independent_layout_reference(",
        "ForgeStore::admit_milestone_7_independent_layout_reference",
    ),
    (
        "pub fn admit_milestone_9_physical_chunk_reference(",
        "ForgeStore::admit_milestone_9_physical_chunk_reference",
    ),
    (
        "pub fn prepare_milestone_6_layout_support(",
        "ForgeStore::prepare_milestone_6_layout_support",
    ),
    (
        "pub fn prepare_milestone_6_layout_support_with_policy(",
        "ForgeStore::prepare_milestone_6_layout_support_with_policy",
    ),
    (
        "pub fn materialize_milestone_6_layout_support(",
        "ForgeStore::materialize_milestone_6_layout_support",
    ),
    (
        "pub fn fetch_milestone_6_layout_support(",
        "ForgeStore::fetch_milestone_6_layout_support",
    ),
    (
        "pub fn execute_aspect_layout_read(",
        "ForgeStore::execute_aspect_layout_read",
    ),
    (
        "pub fn read_aspect_layout_control_truth(",
        "ForgeStore::read_aspect_layout_control_truth",
    ),
    (
        "pub fn execute_dedup_backed_read(",
        "ForgeStore::execute_dedup_backed_read",
    ),
    (
        "pub fn structural_block_lookup(",
        "ForgeStore::structural_block_lookup",
    ),
    (
        "pub fn export_milestone_6_chunk_model(",
        "ForgeStore::export_milestone_6_chunk_model",
    ),
    (
        "pub fn export_milestone_6_chunk_model_in_lane(",
        "ForgeStore::export_milestone_6_chunk_model_in_lane",
    ),
    (
        "pub fn rebuild_milestone_6_derived_artifacts_from_materializations(",
        "ForgeStore::rebuild_milestone_6_derived_artifacts_from_materializations",
    ),
    (
        "pub fn rebuild_milestone_6_derived_artifacts_from_authority(",
        "ForgeStore::rebuild_milestone_6_derived_artifacts_from_authority",
    ),
];

#[test]
fn phase29_inventory_is_bound_to_the_real_legacy_root_surface() {
    let boundary = LegacySurfaceDispositionAndDedicatedWorkspaceBoundary::current();
    let inventory: LegacyAccessPathBypassInventory = boundary.inventory();

    assert_public_family_disposition_parity(
        &inventory,
        collect_reexported_names(LEGACY_LIB_RS, "pub use compatibility::{")
            .into_iter()
            .filter(|surface| surface.contains("CompatibilityRegistry")),
        LegacySurfaceDisposition::ConsumedAsInputOnly,
    );
    assert_public_family_disposition_parity(
        &inventory,
        collect_reexported_names(LEGACY_LIB_RS, "pub use maintenance::{")
            .into_iter()
            .filter(|surface| is_phase29_legacy_maintenance_surface(surface)),
        LegacySurfaceDisposition::ConsumedAsInputOnly,
    );
    assert_public_family_disposition_parity(
        &inventory,
        collect_reexported_names(LEGACY_LIB_RS, "pub use subscription_support::{")
            .into_iter()
            .filter(|surface| is_phase29_legacy_support_structure_surface(surface)),
        LegacySurfaceDisposition::ConsumedAsInputOnly,
    );
    assert_public_family_disposition_parity(
        &inventory,
        collect_reexported_names(LEGACY_LIB_RS, "pub use subscription_support::{")
            .into_iter()
            .filter(|surface| surface == "SupportTrustAccessPath"),
        LegacySurfaceDisposition::ForbiddenAsAuthority,
    );
    assert_public_family_disposition_parity(
        &inventory,
        collect_reexported_names(LEGACY_LIB_RS, "pub use delta::{")
            .into_iter()
            .filter(|surface| surface.contains("FallbackClass")),
        LegacySurfaceDisposition::SupersededAndForbidden,
    );
    assert_public_family_disposition_parity(
        &inventory,
        collect_reexported_names(LEGACY_LIB_RS, "pub use delta::{")
            .into_iter()
            .filter(|surface| surface == "Milestone7IndependentReference"),
        LegacySurfaceDisposition::ForbiddenAsAuthority,
    );
    assert_public_family_disposition_parity(
        &inventory,
        collect_reexported_names(LEGACY_LIB_RS, "pub use evidence::{")
            .into_iter()
            .filter(|surface| is_phase29_legacy_evidence_surface(surface)),
        LegacySurfaceDisposition::CertificationOnly,
    );

    assert!(
        !LEGACY_LIB_RS.contains("pub use facade::{ForgeStore, ForgeStoreBuilder};"),
        "legacy root still re-exports the displaced facade"
    );
    for fragment in LEGACY_PUBLIC_EXPORT_FRAGMENTS {
        assert!(
            !LEGACY_LIB_RS.contains(fragment),
            "legacy root still publicly exports displaced fragment {fragment}"
        );
    }

    for (needle, surface) in LEGACY_FACADE_METHODS {
        let found_in_reads = LEGACY_LAYOUT_READS_RS.contains(needle);
        let found_in_support = LEGACY_LAYOUT_SUPPORT_RS.contains(needle);
        assert!(
            found_in_reads || found_in_support,
            "legacy residue audit is stale; missing {needle}"
        );
        assert_eq!(
            inventory.disposition_for(surface),
            LegacySurfaceDisposition::SupersededAndForbidden,
            "legacy facade method {surface} must be classified as displaced authority"
        );
    }
}

fn assert_public_family_disposition_parity<I>(
    inventory: &LegacyAccessPathBypassInventory,
    surfaces: I,
    expected_disposition: LegacySurfaceDisposition,
) where
    I: IntoIterator<Item = String>,
{
    let mut seen_any = false;
    for surface in surfaces {
        seen_any = true;
        assert_eq!(
            inventory.disposition_for(&surface),
            expected_disposition,
            "legacy inventory lost parity for {surface}"
        );
        assert_eq!(
            inventory
                .rows()
                .iter()
                .filter(|row| row.surface() == surface.as_str())
                .count(),
            1,
            "legacy inventory must classify {surface} exactly once"
        );
    }
    assert!(
        seen_any,
        "legacy residue audit did not discover any family surfaces"
    );
}

fn collect_reexported_names(source: &str, anchor: &str) -> Vec<String> {
    let mut names = Vec::new();
    let mut in_block = false;

    for line in source.lines() {
        if !in_block {
            if let Some((_, remainder)) = line.split_once(anchor) {
                in_block = true;
                push_names(remainder, &mut names);
                if line.contains("};") {
                    in_block = false;
                }
            }
            continue;
        }

        push_names(line, &mut names);
        if line.contains("};") {
            in_block = false;
        }
    }

    names
}

fn push_names(fragment: &str, names: &mut Vec<String>) {
    let trimmed = fragment.replace("};", "").replace('{', "").replace('}', "");
    for candidate in trimmed.split(',') {
        let name = candidate.trim();
        if !name.is_empty() {
            names.push(name.to_owned());
        }
    }
}

fn is_phase29_legacy_evidence_surface(surface: &str) -> bool {
    surface == "Milestone5ReadPathReport"
        || matches!(
            surface,
            "Milestone6AccessStructureClaim"
                | "Milestone6AccessStructureContract"
                | "Milestone6AccessStructureVerification"
                | "Milestone6AccessStructureVerificationPath"
                | "Milestone6CertificationBundle"
                | "Milestone6CertificationOrigin"
                | "Milestone6CertificationSummary"
                | "Milestone6ComplexityPathStatus"
                | "Milestone6ComplexitySurface"
                | "Milestone6CounterContract"
                | "Milestone6LayoutMaterializationReport"
                | "Milestone6LayoutReadReport"
                | "Milestone6PhysicalLayoutReport"
                | "Milestone7AccessStructureClaim"
                | "Milestone7AccessStructureContract"
                | "Milestone7AccessStructureVerification"
                | "Milestone7AccessStructureVerificationPath"
                | "Milestone7CertificationBundle"
                | "Milestone7ComplexityPathStatus"
                | "Milestone7ComplexitySurface"
                | "Milestone7CounterContract"
        )
}

fn is_phase29_legacy_maintenance_surface(surface: &str) -> bool {
    surface.ends_with("MaintenanceDeclaration")
        || matches!(
            surface,
            "MaintenanceDeclarationClass" | "MaintenanceDeclarationId"
        )
}

fn is_phase29_legacy_support_structure_surface(surface: &str) -> bool {
    surface.contains("SubscriptionSupportAccessStructure")
        || surface.contains("SupportTrustAccessStructure")
        || surface == "SupportTrustAccessIndexKind"
}
