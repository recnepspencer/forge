use worth_ui_host_contract::UiMountedLogicalDamage;

use super::retained_draw_list::UiNativeRetainedDrawListDenial;

pub(super) fn normalize_damage(
    regions: &[UiMountedLogicalDamage],
) -> Result<Vec<UiMountedLogicalDamage>, UiNativeRetainedDrawListDenial> {
    let mut normalized = regions.to_vec();
    normalized.sort_by(|left, right| compare_bounds(left.bounds(), right.bounds()));
    normalized.dedup();
    Ok(normalized)
}

fn compare_bounds(
    left: worth_ui_host_contract::UiMountedCanonicalBox,
    right: worth_ui_host_contract::UiMountedCanonicalBox,
) -> std::cmp::Ordering {
    left.x()
        .total_cmp(&right.x())
        .then_with(|| left.y().total_cmp(&right.y()))
        .then_with(|| left.width().total_cmp(&right.width()))
        .then_with(|| left.height().total_cmp(&right.height()))
}

#[cfg(test)]
mod tests {
    use super::normalize_damage;
    use worth_ui_host_contract::{
        UiMountedCanonicalBox, UiMountedCanonicalBoxInput, UiMountedCoordinateSpace,
        UiMountedLogicalDamage,
    };

    #[test]
    fn exact_duplicate_regions_collapse_without_quadratic_search() {
        let regions = [damage(0.0, 10.0), damage(0.0, 10.0), damage(8.0, 10.0)];
        let normalized = normalize_damage(&regions).unwrap();
        assert_eq!(normalized, [damage(0.0, 10.0), damage(8.0, 10.0)]);
    }

    fn damage(x: f32, width: f32) -> UiMountedLogicalDamage {
        UiMountedLogicalDamage::from_runtime_mounting(bounds(x, width))
    }

    fn bounds(x: f32, width: f32) -> UiMountedCanonicalBox {
        UiMountedCanonicalBox::canonicalize(UiMountedCanonicalBoxInput {
            x,
            y: 0.0,
            width,
            height: 10.0,
            coordinate_space: UiMountedCoordinateSpace::HostSurface,
        })
        .unwrap()
    }
}
