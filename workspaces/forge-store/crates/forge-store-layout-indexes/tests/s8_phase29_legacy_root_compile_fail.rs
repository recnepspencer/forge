const LEGACY_LIB_RS: &str = include_str!("../../../../../crates/forge-store/src/lib.rs");

const FORBIDDEN_PUBLIC_FRAGMENTS: [&str; 8] = [
    "pub use facade::{ForgeStore, ForgeStoreBuilder};",
    "\n    CompatibilityRegistry,\n",
    "\n    MaintenanceDeclaration,\n",
    "\n    SubscriptionSupportAccessStructure,\n",
    "\n    AspectLayoutReadRequest,\n",
    "\n    AspectLayoutReadPlanDecision,\n",
    "\n    AspectLayoutReadExecutionResult,\n",
    "\n    Milestone6ChunkModelExport,\n",
];

#[test]
fn phase29_legacy_root_default_surface_is_sealed() {
    assert!(
        LEGACY_LIB_RS.contains("mod facade;"),
        "legacy root must keep facade topology private"
    );
    assert!(
        LEGACY_LIB_RS.contains("mod layout;"),
        "legacy root must keep layout topology private"
    );
    assert!(
        !LEGACY_LIB_RS.contains("pub mod facade;"),
        "legacy root reopened the displaced facade module"
    );
    assert!(
        !LEGACY_LIB_RS.contains("pub mod layout;"),
        "legacy root reopened the displaced layout module"
    );

    for fragment in FORBIDDEN_PUBLIC_FRAGMENTS {
        assert!(
            !LEGACY_LIB_RS.contains(fragment),
            "legacy root still publishes displaced authority fragment {fragment}"
        );
    }
}
