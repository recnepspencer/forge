use std::collections::BTreeSet;
use std::sync::Arc;

use forge_foundational::facade::AspectKey;

use crate::diagnostics::BridgeRouteRecordEntry;
use crate::error::{BridgeRouteError, BridgeRouteErrorKind};
use crate::routing::canonicalization::{canonical_snapshot_request_order, canonical_target_order};
use crate::routing::eligibility::EligibleRouteEntry;
use crate::routing::lowering::BridgeSubscriptionSlice;
use crate::routing::matching::FineGrainedMatchStatus;
use crate::snapshot::{SnapshotReadPacket, SnapshotReadRequest};

pub(super) fn canonical_invalidation_targets(
    entries: &[EligibleRouteEntry],
) -> Vec<(Arc<str>, crate::mapping::CoarseRoutingMode)> {
    let mut deduped = BTreeSet::new();
    for entry in entries {
        deduped.insert((
            Arc::<str>::from(entry.registration().signal_scope().as_str()),
            entry.registration().routing_mode(),
        ));
    }

    let mut targets = deduped.into_iter().collect::<Vec<_>>();
    targets.sort_by(|left, right| canonical_target_order(left, right));
    targets
}

pub(super) fn canonical_read_packet(
    subscription_slices: &[BridgeSubscriptionSlice],
    entries: &[EligibleRouteEntry],
) -> Result<SnapshotReadPacket, BridgeRouteError> {
    if subscription_slices.is_empty() {
        return canonical_coarse_read_packet(entries);
    }

    let mut deduped = BTreeSet::new();
    for slice in subscription_slices {
        deduped.insert((
            slice.entity_identity().to_owned(),
            slice.aspect_label().to_owned(),
            slice.surface_label().to_owned(),
            slice.slice_kind().clone(),
        ));
    }

    let mut reads = deduped
        .into_iter()
        .map(
            |(entity_identity, aspect_label, surface_label, slice_kind)| {
                Ok(SnapshotReadRequest::for_subscription_slice(
                    entity_identity,
                    read_request_aspect_key(&aspect_label)?,
                    surface_label,
                    slice_kind,
                ))
            },
        )
        .collect::<Result<Vec<_>, BridgeRouteError>>()?;
    reads.sort_by(canonical_snapshot_request_order);
    Ok(SnapshotReadPacket::new(reads))
}

fn canonical_coarse_read_packet(
    entries: &[EligibleRouteEntry],
) -> Result<SnapshotReadPacket, BridgeRouteError> {
    let mut deduped = BTreeSet::new();
    for entry in entries {
        deduped.insert((
            entry.item().entity_identity().to_owned(),
            entry.item().aspect_label().to_owned(),
        ));
    }

    let mut reads = deduped
        .into_iter()
        .map(|(entity_identity, aspect_label)| {
            Ok(SnapshotReadRequest::for_coarse(
                entity_identity,
                read_request_aspect_key(&aspect_label)?,
            ))
        })
        .collect::<Result<Vec<_>, BridgeRouteError>>()?;
    reads.sort_by(canonical_snapshot_request_order);
    Ok(SnapshotReadPacket::new(reads))
}

fn read_request_aspect_key(aspect_label: &str) -> Result<AspectKey, BridgeRouteError> {
    AspectKey::new(aspect_label).ok_or_else(|| {
        BridgeRouteError::new(
            BridgeRouteErrorKind::SliceReadPacketConstructionFailure,
            format!(
                "Snapshot read packet construction rejected invalid aspect key `{aspect_label}`."
            ),
        )
    })
}

pub(super) fn canonical_subscription_slices(
    entries: &[EligibleRouteEntry],
) -> Vec<BridgeSubscriptionSlice> {
    let mut deduped = BTreeSet::new();
    for entry in entries {
        match entry.fine_grained_match().status() {
            FineGrainedMatchStatus::Matched | FineGrainedMatchStatus::FallbackAdmitted => {
                let Some(slice_kind) = entry.fine_grained_match().subscription_slice_kind() else {
                    continue;
                };

                deduped.insert(BridgeSubscriptionSlice::new(
                    entry.normalized_surface().entity_identity(),
                    entry.normalized_surface().aspect_label(),
                    entry.normalized_surface().surface_label(),
                    slice_kind.clone(),
                    entry.fine_grained_match().status(),
                ));
            }
            FineGrainedMatchStatus::SuppressedByRegistrationPolicy
            | FineGrainedMatchStatus::UnsupportedSurfaceCategory
            | FineGrainedMatchStatus::AmbiguousRegistration => {}
        }
    }

    let mut slices = deduped.into_iter().collect::<Vec<_>>();
    slices.sort();
    slices
}

pub(super) fn canonical_route_record_entries(
    entries: &[EligibleRouteEntry],
) -> Arc<[BridgeRouteRecordEntry]> {
    Arc::from(
        entries
            .iter()
            .map(|entry| {
                BridgeRouteRecordEntry::new(
                    entry.normalized_surface().entity_identity(),
                    entry.normalized_surface().aspect_label(),
                    entry.normalized_surface().surface_label(),
                    entry.item().surface_label(),
                    entry.normalized_surface().surface_identity().as_str(),
                    entry.registration().mapping_id().clone(),
                    entry.registration().signal_scope().as_str(),
                    entry.registration().routing_mode(),
                    entry.fallback_class(),
                    entry.fine_grained_match().clone(),
                )
            })
            .collect::<Vec<_>>(),
    )
}
