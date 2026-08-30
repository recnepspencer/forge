pub(crate) fn why_scroll_owner(
    owner: Option<&crate::runtime::scroll::UiScrollRuntimeState>,
) -> Option<worth_ui_inspection::UiScrollOwnerInspectionSummary> {
    let record = owner?.last_owner()?;
    Some(worth_ui_inspection::UiScrollOwnerInspectionSummary::new(
        worth_ui_inspection::UiRuntimeServiceInspectionSource::new(
            worth_ui_inspection::UiRuntimeServiceInspectionFamily::Scroll,
            Some(owner_identity(record.owner())),
            record.revision(),
        ),
        record.owners_visited(),
        record.owners_changed(),
        record.remainder_present(),
        worth_ui_inspection::UiRuntimeServiceInspectionCost::latest_record(1, 1),
    ))
}

fn owner_identity(owner: crate::runtime::scroll::UiScrollOwnerIdentity) -> u64 {
    match owner {
        crate::runtime::scroll::UiScrollOwnerIdentity::Region {
            surface,
            region,
            repeated_instance_digest,
            plan_region_index,
        } => {
            surface.diagnostic_value()
                ^ region.digest().rotate_left(13)
                ^ repeated_instance_digest.rotate_left(29)
                ^ u64::from(plan_region_index).rotate_left(41)
        }
        crate::runtime::scroll::UiScrollOwnerIdentity::Surface(surface) => {
            surface.diagnostic_value() ^ 0x5355_5246_4143_4501
        }
        crate::runtime::scroll::UiScrollOwnerIdentity::Viewport(surface) => {
            surface.diagnostic_value() ^ 0x5649_4557_504f_5254
        }
    }
}
