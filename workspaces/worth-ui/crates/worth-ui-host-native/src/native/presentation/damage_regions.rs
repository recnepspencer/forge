use worth_ui_host_contract::{
    UiMountedCanonicalBox, UiMountedCanonicalBoxInput, UiMountedLogicalDamage,
};

use super::retained_draw_list::UiNativeRetainedDrawListDenial;

pub(super) fn normalize_damage(
    regions: &[UiMountedLogicalDamage],
) -> Result<Vec<UiMountedLogicalDamage>, UiNativeRetainedDrawListDenial> {
    let mut normalized = Vec::new();
    for region in regions {
        let mut candidate = region.bounds();
        while let Some(index) = normalized
            .iter()
            .position(|retained| overlaps(*retained, candidate))
        {
            candidate = union(normalized.swap_remove(index), candidate)?;
        }
        normalized.push(candidate);
    }
    Ok(normalized
        .into_iter()
        .map(UiMountedLogicalDamage::from_runtime_mounting)
        .collect())
}

fn overlaps(left: UiMountedCanonicalBox, right: UiMountedCanonicalBox) -> bool {
    left.coordinate_space() == right.coordinate_space()
        && left.x() < right.x() + right.width()
        && right.x() < left.x() + left.width()
        && left.y() < right.y() + right.height()
        && right.y() < left.y() + left.height()
}

fn union(
    left: UiMountedCanonicalBox,
    right: UiMountedCanonicalBox,
) -> Result<UiMountedCanonicalBox, UiNativeRetainedDrawListDenial> {
    let x = left.x().min(right.x());
    let y = left.y().min(right.y());
    let right_edge = (left.x() + left.width()).max(right.x() + right.width());
    let bottom_edge = (left.y() + left.height()).max(right.y() + right.height());
    UiMountedCanonicalBox::canonicalize(UiMountedCanonicalBoxInput {
        x,
        y,
        width: right_edge - x,
        height: bottom_edge - y,
        coordinate_space: left.coordinate_space(),
    })
    .map_err(|_| UiNativeRetainedDrawListDenial::DamageMismatch)
}

#[cfg(test)]
mod tests {
    use super::normalize_damage;
    use worth_ui_host_contract::{
        UiMountedCanonicalBox, UiMountedCanonicalBoxInput, UiMountedCoordinateSpace,
        UiMountedLogicalDamage,
    };

    #[test]
    fn overlapping_and_duplicate_damage_become_one_replay_region() {
        let regions = [damage(0.0, 10.0), damage(0.0, 10.0), damage(8.0, 10.0)];
        let normalized = normalize_damage(&regions).unwrap();
        assert_eq!(normalized.len(), 1);
        assert_eq!(normalized[0].bounds(), bounds(0.0, 18.0));
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
