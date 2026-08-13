const PROJECTION: &str = "worth-store-physical-format/recovery-projection";
const ROOT_STATE: &str = "worth-store-physical-format/recovery-projection/root-state";
const BASIS: &str = "progression/planned/basis";

pub(super) const PHASE_FOUR_PROJECTION_SURFACES: &[(&str, &str, &str)] = &[
    (
        "PersistedPhysicalRecoveryRootState::successor_manifest_capacity",
        ROOT_STATE,
        "phase-4",
    ),
    ("PersistedInlineSegmentAllocation", ROOT_STATE, "phase-4"),
    (
        "PersistedInlineSegmentAllocation::new",
        ROOT_STATE,
        "phase-4",
    ),
    (
        "PersistedInlineSegmentAllocation::page_capacity",
        ROOT_STATE,
        "phase-4",
    ),
    (
        "PersistedInlineSegmentAllocation::segment",
        ROOT_STATE,
        "phase-4",
    ),
    (
        "PersistedInlineSegmentAllocation::used_pages",
        ROOT_STATE,
        "phase-4",
    ),
    ("PersistedPhysicalRecoveryFrame", PROJECTION, "phase-4"),
    (
        "PersistedPhysicalRecoveryFrame::bytes",
        PROJECTION,
        "phase-4",
    ),
    (
        "PersistedPhysicalRecoveryFrame::coordinate",
        PROJECTION,
        "phase-4",
    ),
    ("PersistedPhysicalRecoveryFrame::new", PROJECTION, "phase-4"),
    (
        "PersistedPhysicalRecoveryFrame::subject",
        PROJECTION,
        "phase-4",
    ),
    ("PersistedPhysicalRecoveryManifest", PROJECTION, "phase-4"),
    (
        "PersistedPhysicalRecoveryManifest::artifact",
        PROJECTION,
        "phase-4",
    ),
    (
        "PersistedPhysicalRecoveryManifest::bytes",
        PROJECTION,
        "phase-4",
    ),
    (
        "PersistedPhysicalRecoveryManifest::new",
        PROJECTION,
        "phase-4",
    ),
    ("PersistedPhysicalRecoveryProjection", PROJECTION, "phase-4"),
    (
        "PersistedPhysicalRecoveryProjection::decode",
        "worth-store-physical-format/recovery-projection/codec",
        "phase-4",
    ),
    (
        "PersistedPhysicalRecoveryProjection::encode",
        "worth-store-physical-format/recovery-projection/codec",
        "phase-4",
    ),
    (
        "PersistedPhysicalRecoveryProjection::frames",
        PROJECTION,
        "phase-4",
    ),
    (
        "PersistedPhysicalRecoveryProjection::manifests",
        PROJECTION,
        "phase-4",
    ),
    (
        "PersistedPhysicalRecoveryProjection::new",
        PROJECTION,
        "phase-4",
    ),
    (
        "PersistedPhysicalRecoveryProjection::placements",
        PROJECTION,
        "phase-4",
    ),
    (
        "PersistedPhysicalRecoveryProjection::record_identities",
        PROJECTION,
        "phase-4",
    ),
    (
        "PersistedPhysicalRecoveryProjection::root_state",
        PROJECTION,
        "phase-4",
    ),
    (
        "PersistedPhysicalRecoveryProjection::segment_updates",
        PROJECTION,
        "phase-4",
    ),
    (
        "PersistedPhysicalRecoveryProjection::source_root_generation",
        PROJECTION,
        "phase-4",
    ),
    ("PersistedPhysicalRecoveryRootState", ROOT_STATE, "phase-4"),
    (
        "PersistedPhysicalRecoveryRootState::inline_allocations",
        ROOT_STATE,
        "phase-4",
    ),
    (
        "PersistedPhysicalRecoveryRootState::last_inline_record",
        ROOT_STATE,
        "phase-4",
    ),
    (
        "PersistedPhysicalRecoveryRootState::last_inline_segment",
        ROOT_STATE,
        "phase-4",
    ),
    (
        "PersistedPhysicalRecoveryRootState::manifest_capacity_transition",
        ROOT_STATE,
        "phase-4",
    ),
    (
        "PersistedPhysicalRecoveryRootState::new",
        ROOT_STATE,
        "phase-4",
    ),
    (
        "PersistedPhysicalRecoveryRootState::root_publication_allocation_bytes",
        ROOT_STATE,
        "phase-4",
    ),
    (
        "PhysicalRecoveryProjectionDecodeLimits",
        PROJECTION,
        "phase-4",
    ),
    ("PhysicalRecoveryProjectionDenial", PROJECTION, "phase-4"),
    ("RecoveryBaseImageAction::is_projected", BASIS, "phase-4"),
    ("RecoveryBaseImagePlan::manifests", BASIS, "phase-4"),
    ("RecoveryBaseImagePlan::root_states", BASIS, "phase-4"),
    ("RecoveryBaseImagePlan::segment_updates", BASIS, "phase-4"),
    ("RecoveryPayloadManifestAction", BASIS, "phase-4"),
    ("RecoveryPayloadManifestAction::artifact", BASIS, "phase-4"),
    ("RecoveryPayloadManifestAction::ordinal", BASIS, "phase-4"),
    ("RecoverySegmentRoutingAction", BASIS, "phase-4"),
    ("RecoverySegmentRoutingAction::ordinal", BASIS, "phase-4"),
    ("RecoverySegmentRoutingAction::update", BASIS, "phase-4"),
];
