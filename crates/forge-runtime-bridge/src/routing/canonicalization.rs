use std::sync::Arc;

use sha2::{Digest, Sha256};

use crate::input::envelope::BridgeCommittedPatchEnvelope;
use crate::mapping::{FrozenBridgeMappingRegistration, MappingSelector};
use crate::routing::eligibility::EligibleRouteEntry;
use crate::routing::lowering::{BridgeInvalidationTarget, BridgeSubscriptionSlice};
use crate::routing::matching::FineGrainedMatchStatus;
use crate::routing::context::BridgeMappingContext;
use crate::routing::planning::BridgeRouteIdentity;
use crate::snapshot::{canonical_subscription_slice_kind_label, SnapshotReadRequest};

pub(crate) fn canonical_route_entry_order(
    left: &EligibleRouteEntry,
    right: &EligibleRouteEntry,
) -> std::cmp::Ordering {
    left.normalized_surface()
        .entity_identity()
        .cmp(right.normalized_surface().entity_identity())
        .then_with(|| {
            left.normalized_surface()
                .aspect_label()
                .cmp(right.normalized_surface().aspect_label())
        })
        .then_with(|| {
            left.normalized_surface()
                .surface_identity()
                .cmp(right.normalized_surface().surface_identity())
        })
        .then_with(|| canonical_registration_order(left.registration(), right.registration()))
}

pub(crate) fn canonical_target_order<T>(
    left: &T,
    right: &T,
) -> std::cmp::Ordering
where
    T: CanonicalTargetView,
{
    left.signal_scope()
        .cmp(right.signal_scope())
        .then_with(|| left.routing_mode().cmp(&right.routing_mode()))
}

pub(crate) fn canonical_snapshot_request_order(
    left: &SnapshotReadRequest,
    right: &SnapshotReadRequest,
) -> std::cmp::Ordering {
    left.entity_identity()
        .cmp(right.entity_identity())
        .then_with(|| left.aspect_label().cmp(right.aspect_label()))
        .then_with(|| left.slice_kind().cmp(&right.slice_kind()))
        .then_with(|| left.surface_label().cmp(&right.surface_label()))
        .then_with(|| left.request_key().cmp(right.request_key()))
}

pub(crate) fn route_digest_basis(
    envelope: &BridgeCommittedPatchEnvelope,
    mapping_context: &BridgeMappingContext,
    entries: &[EligibleRouteEntry],
) -> String {
    let mut basis = format!(
        "route|commit={}|patch={}|snapshot={}|branch={}|mapping-context={}|entry-count={}",
        envelope.commit_identity().as_str(),
        envelope.patch_identity().as_str(),
        envelope.snapshot_identity().as_str(),
        envelope.branch_identity().as_str(),
        mapping_context.digest(),
        entries.len()
    );

    for entry in entries {
        basis.push_str("|entry=");
        basis.push_str(&canonical_route_entry_key(entry));
    }

    basis
}

pub(crate) fn invalidation_digest_basis(
    route_identity: &BridgeRouteIdentity,
    source_commit: &str,
    source_patch: &str,
    source_snapshot: &str,
    targets: &[BridgeInvalidationTarget],
) -> String {
    let mut basis = format!(
        "invalidation|route={}|commit={}|patch={}|snapshot={}|target-count={}",
        route_identity.as_str(),
        source_commit,
        source_patch,
        source_snapshot,
        targets.len()
    );
    for target in targets {
        basis.push_str("|target=");
        basis.push_str(&canonical_target_key(target));
    }
    basis
}

pub(crate) fn subscription_slice_digest_basis(
    source_snapshot: &str,
    slices: &[BridgeSubscriptionSlice],
) -> String {
    let mut basis = format!(
        "subscription-slices|snapshot={}|slice-count={}",
        source_snapshot,
        slices.len()
    );
    for slice in slices {
        basis.push_str("|slice=");
        basis.push_str(&canonical_subscription_slice_key(slice));
    }
    basis
}

pub(crate) fn planning_provenance_digest_basis(
    route_identity: &BridgeRouteIdentity,
    envelope: &BridgeCommittedPatchEnvelope,
    mapping_context: &BridgeMappingContext,
    entries: &[EligibleRouteEntry],
    read_packet: &SnapshotReadRequestSetView<'_>,
) -> String {
    let mut basis = format!(
        "planning-provenance|route={}|commit={}|patch={}|snapshot={}|branch={}|patch-digest={}|mapping-context={}|entry-count={}|read-count={}",
        route_identity.as_str(),
        envelope.commit_identity().as_str(),
        envelope.patch_identity().as_str(),
        envelope.snapshot_identity().as_str(),
        envelope.branch_identity().as_str(),
        envelope.digest().as_str(),
        mapping_context.digest(),
        entries.len(),
        read_packet.len(),
    );
    for entry in entries {
        basis.push_str("|entry=");
        basis.push_str(&canonical_route_entry_key(entry));
    }
    for read in read_packet.reads() {
        basis.push_str("|read=");
        basis.push_str(read.request_key());
    }
    basis
}

pub(crate) fn planning_summary_digest_basis(
    route_identity: &BridgeRouteIdentity,
    routing_entry_count: usize,
    invalidation_target_count: usize,
    subscription_slice_count: usize,
    snapshot_read_count: usize,
) -> String {
    format!(
        "planning-summary|route={}|routing-entry-count={}|invalidation-target-count={}|subscription-slice-count={}|snapshot-read-count={}",
        route_identity.as_str(),
        routing_entry_count,
        invalidation_target_count,
        subscription_slice_count,
        snapshot_read_count,
    )
}

pub(crate) fn lowering_provenance_digest_basis(
    route_identity: &BridgeRouteIdentity,
    planning_provenance_digest: &str,
    source_commit: &str,
    source_patch: &str,
    source_snapshot: &str,
) -> String {
    format!(
        "lowering-provenance|route={}|planning={}|commit={}|patch={}|snapshot={}",
        route_identity.as_str(),
        planning_provenance_digest,
        source_commit,
        source_patch,
        source_snapshot,
    )
}

pub(crate) fn lowering_summary_digest_basis(
    route_identity: &BridgeRouteIdentity,
    invalidation_targets: &[BridgeInvalidationTarget],
    subscription_slices: &[BridgeSubscriptionSlice],
    planned_read_count: usize,
) -> String {
    let mut basis = format!(
        "lowering-summary|route={}|target-count={}|slice-count={}|planned-read-count={}",
        route_identity.as_str(),
        invalidation_targets.len(),
        subscription_slices.len(),
        planned_read_count,
    );
    for target in invalidation_targets {
        basis.push_str("|target=");
        basis.push_str(&canonical_target_key(target));
    }
    for slice in subscription_slices {
        basis.push_str("|slice=");
        basis.push_str(&canonical_subscription_slice_key(slice));
    }
    basis
}

pub(crate) fn digest_string(kind: &str, basis: &str) -> Arc<str> {
    digest_value(kind, basis)
}

pub(crate) struct SnapshotReadRequestSetView<'a> {
    reads: &'a [SnapshotReadRequest],
}

impl<'a> SnapshotReadRequestSetView<'a> {
    pub(crate) fn new(reads: &'a [SnapshotReadRequest]) -> Self {
        Self { reads }
    }

    pub(crate) fn len(&self) -> usize {
        self.reads.len()
    }

    pub(crate) fn reads(&self) -> &'a [SnapshotReadRequest] {
        self.reads
    }
}

fn canonical_route_entry_key(entry: &EligibleRouteEntry) -> String {
    format!(
        "{}:{}:{}:{}:{}:{}:{}",
        entry.normalized_surface().entity_identity(),
        entry.normalized_surface().aspect_label(),
        entry.normalized_surface().surface_label(),
        entry.normalized_surface().surface_identity().as_str(),
        entry.registration().mapping_id().as_str(),
        entry.registration().signal_scope().as_str(),
        routing_mode_label(entry.registration().routing_mode())
    )
}

fn canonical_target_key<T>(target: &T) -> String
where
    T: CanonicalTargetView,
{
    format!(
        "{}:{}",
        target.signal_scope(),
        routing_mode_label(target.routing_mode())
    )
}

fn canonical_subscription_slice_key(slice: &BridgeSubscriptionSlice) -> String {
    format!(
        "{}:{}:{}:{}:{}",
        slice.entity_identity(),
        slice.aspect_label(),
        slice.surface_label(),
        canonical_subscription_slice_kind_label(slice.slice_kind()),
        canonical_match_status_label(slice.match_status())
    )
}

fn canonical_match_status_label(status: FineGrainedMatchStatus) -> &'static str {
    match status {
        FineGrainedMatchStatus::Matched => "matched",
        FineGrainedMatchStatus::FallbackAdmitted => "fallback-admitted",
        FineGrainedMatchStatus::SuppressedByRegistrationPolicy => "suppressed-by-registration-policy",
        FineGrainedMatchStatus::UnsupportedSurfaceCategory => "unsupported-surface-category",
        FineGrainedMatchStatus::AmbiguousRegistration => "ambiguous-registration",
    }
}

pub(crate) trait CanonicalTargetView {
    fn signal_scope(&self) -> &str;
    fn routing_mode(&self) -> crate::mapping::CoarseRoutingMode;
}

fn canonical_registration_order(
    left: &FrozenBridgeMappingRegistration,
    right: &FrozenBridgeMappingRegistration,
) -> std::cmp::Ordering {
    left.mapping_id()
        .as_str()
        .cmp(right.mapping_id().as_str())
        .then_with(|| {
            selector_order(
                left.truth_scope().entity_selector(),
                right.truth_scope().entity_selector(),
            )
        })
        .then_with(|| {
            selector_order(
                left.truth_scope().aspect_selector(),
                right.truth_scope().aspect_selector(),
            )
        })
        .then_with(|| {
            selector_order(
                left.truth_scope().surface_selector(),
                right.truth_scope().surface_selector(),
            )
        })
        .then_with(|| left.signal_scope().as_str().cmp(right.signal_scope().as_str()))
        .then_with(|| {
            left.truth_scope()
                .specificity_rank()
                .cmp(&right.truth_scope().specificity_rank())
        })
        .then_with(|| left.routing_mode().cmp(&right.routing_mode()))
}

impl CanonicalTargetView for BridgeInvalidationTarget {
    fn signal_scope(&self) -> &str {
        self.signal_scope()
    }

    fn routing_mode(&self) -> crate::mapping::CoarseRoutingMode {
        self.routing_mode()
    }
}

impl CanonicalTargetView for (Arc<str>, crate::mapping::CoarseRoutingMode) {
    fn signal_scope(&self) -> &str {
        self.0.as_ref()
    }

    fn routing_mode(&self) -> crate::mapping::CoarseRoutingMode {
        self.1
    }
}

fn selector_order(left: &MappingSelector, right: &MappingSelector) -> std::cmp::Ordering {
    match (left, right) {
        (MappingSelector::Any, MappingSelector::Any) => std::cmp::Ordering::Equal,
        (MappingSelector::Any, MappingSelector::Exact(_)) => std::cmp::Ordering::Less,
        (MappingSelector::Exact(_), MappingSelector::Any) => std::cmp::Ordering::Greater,
        (MappingSelector::Exact(left), MappingSelector::Exact(right)) => left.as_ref().cmp(right.as_ref()),
    }
}

fn routing_mode_label(mode: crate::mapping::CoarseRoutingMode) -> &'static str {
    match mode {
        crate::mapping::CoarseRoutingMode::Direct => "direct",
    }
}

fn digest_value(kind: &str, basis: &str) -> Arc<str> {
    let digest = Sha256::digest(basis.as_bytes());
    format!("{kind}:sha256:{digest:x}").into()
}
