use serde_json::json;

pub(super) fn route_record_json(record: crate::diagnostics::BridgeRouteRecord) -> serde_json::Value {
    json!({
        "route_identity": record.route_identity().as_str(),
        "invalidation_identity": record.invalidation_identity().as_str(),
        "source_commit": record.source_commit().as_str(),
        "source_patch": record.source_patch().as_str(),
        "source_snapshot": record.source_snapshot().as_str(),
        "source_digest": record.source_digest().as_str(),
        "subscription_slice_identity": record.subscription_slice_identity().as_str(),
        "entries": record.entries().iter().map(|entry| {
            json!({
                "entity_identity": entry.entity_identity(),
                "aspect_label": entry.aspect_label(),
                "surface_label": entry.surface_label(),
                "mapping_id": entry.mapping_id().as_str(),
                "signal_scope": entry.signal_scope(),
                "routing_mode": format!("{:?}", entry.routing_mode()),
                "fallback_class": entry.fallback_class().map(|class| format!("{class:?}")),
                "truth_surface_kind": format!("{:?}", entry.truth_surface_kind()),
                "fine_grained_match_status": format!("{:?}", entry.fine_grained_match_status()),
                "aspect_registration_id": entry.aspect_registration_id().map(|id| id.as_str()),
                "subscription_slice_kind": entry.subscription_slice_kind().map(|kind| format!("{kind:?}")),
                "slice_fallback_policy": entry.slice_fallback_policy().map(|policy| format!("{policy:?}")),
            })
        }).collect::<Vec<_>>(),
        "subscription_slices": record.subscription_slices().iter().map(|slice| {
            json!({
                "entity_identity": slice.entity_identity(),
                "aspect_label": slice.aspect_label(),
                "surface_label": slice.surface_label(),
                "slice_kind": format!("{:?}", slice.slice_kind()),
                "match_status": format!("{:?}", slice.match_status()),
            })
        }).collect::<Vec<_>>(),
        "invalidation_targets": record.invalidation_targets().iter().map(|target| {
            json!({
                "signal_scope": target.signal_scope(),
                "routing_mode": format!("{:?}", target.routing_mode()),
            })
        }).collect::<Vec<_>>(),
        "counters": {
            "patch_item_count": record.counters().patch_item_count(),
            "normalized_patch_item_count": record.counters().normalized_patch_item_count(),
            "truth_delta_surface_count": record.counters().truth_delta_surface_count(),
            "normalized_truth_delta_surface_count": record.counters().normalized_truth_delta_surface_count(),
            "planned_slice_match_count": record.counters().planned_slice_match_count(),
            "slice_fallback_count": record.counters().slice_fallback_count(),
            "slice_suppression_count": record.counters().slice_suppression_count(),
            "routing_entry_count": record.counters().routing_entry_count(),
            "invalidation_target_count": record.counters().invalidation_target_count(),
            "mapping_lookup_count": record.counters().mapping_lookup_count(),
            "mapping_fallback_count": record.counters().mapping_fallback_count(),
            "snapshot_read_count": record.counters().snapshot_read_count(),
            "snapshot_read_packet_count": record.counters().snapshot_read_packet_count(),
            "snapshot_identity_mismatch_count": record.counters().snapshot_identity_mismatch_count(),
            "route_replay_mismatch_count": record.counters().route_replay_mismatch_count(),
        }
    })
}
