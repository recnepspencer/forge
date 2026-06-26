use super::inventory_record::QuerySelectionSurfaceClassification;

pub(super) fn spatial_support_projection_residue_cap(
    surface: &'static str,
    classification: QuerySelectionSurfaceClassification,
) -> Option<&'static str> {
    if !is_capped_spatial_support_projection_residue(classification) {
        return None;
    }

    match surface {
        "current_spatial_workload_support_pin_rows" => {
            Some("7 support projection rows max until public facade projection is deleted")
        }
        _ => Some("residue surface must stay capped"),
    }
}

pub(super) fn spatial_support_projection_residue_blocker(
    surface: &'static str,
    classification: QuerySelectionSurfaceClassification,
) -> Option<&'static str> {
    if !is_capped_spatial_support_projection_residue(classification) {
        return None;
    }

    match surface {
        "current_spatial_workload_support_pin_rows" => {
            Some("closeout consumers still inspect the support projection facade")
        }
        _ => Some("parallel selected-obligation status has not replaced this support projection"),
    }
}

pub(super) fn spatial_support_projection_residue_trigger(
    surface: &'static str,
    classification: QuerySelectionSurfaceClassification,
) -> Option<&'static str> {
    if !is_capped_spatial_support_projection_residue(classification) {
        return None;
    }

    match surface {
        "current_spatial_workload_support_pin_rows" => {
            Some("Query-owned selected-obligation status replaces the support projection facade")
        }
        _ => Some("parallel selected-obligation lane deletes this support projection"),
    }
}

fn is_capped_spatial_support_projection_residue(
    classification: QuerySelectionSurfaceClassification,
) -> bool {
    matches!(
        classification,
        QuerySelectionSurfaceClassification::CappedResidue
    )
}
