use super::assert_sources_exclude;

const PHYSICAL_RUNTIME: &str = "src/physical_runtime";

#[test]
fn legacy_signal_resource_construction_is_forbidden() {
    assert_sources_exclude(
        PHYSICAL_RUNTIME,
        "legacy-resource-node",
        &["ResourceNodeDeclaration"],
    );
}

#[test]
fn raw_signal_slots_cannot_become_semantic_authority() {
    assert_sources_exclude(
        PHYSICAL_RUNTIME,
        "raw-signal-slot-authority",
        &["RawSignalSlotSemanticAuthority"],
    );
}

#[test]
fn foundational_masks_cannot_substitute_for_native_bindings() {
    assert_sources_exclude(
        PHYSICAL_RUNTIME,
        "foundational-mask-substitution",
        &["FoundationalMaskSubstitution"],
    );
}

#[test]
fn callers_cannot_broaden_aspect_or_partition_scope() {
    assert_sources_exclude(
        PHYSICAL_RUNTIME,
        "aspect-partition-broadening",
        &["PhysicalAspectPartitionBroadening"],
    );
}
