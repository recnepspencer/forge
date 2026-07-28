pub(super) fn intersection(
    left: worth_ui_inspection::UiClientPhysicalRect,
    right: worth_ui_inspection::UiClientPhysicalRect,
) -> Option<worth_ui_inspection::UiClientPhysicalRect> {
    let bounds = [
        left.left().max(right.left()),
        left.top().max(right.top()),
        left.right().min(right.right()),
        left.bottom().min(right.bottom()),
    ];
    worth_ui_inspection::UiClientPhysicalRect::new(bounds[0], bounds[1], bounds[2], bounds[3]).ok()
}

pub(super) fn subtract_opaque_coverage(
    region: worth_ui_inspection::UiClientPhysicalRect,
    coverage: &[worth_ui_inspection::UiClientPhysicalRect],
) -> Vec<worth_ui_inspection::UiClientPhysicalRect> {
    coverage.iter().fold(vec![region], |fragments, opaque| {
        fragments
            .into_iter()
            .flat_map(|fragment| subtract(fragment, *opaque))
            .collect()
    })
}

fn subtract(
    region: worth_ui_inspection::UiClientPhysicalRect,
    opaque: worth_ui_inspection::UiClientPhysicalRect,
) -> Vec<worth_ui_inspection::UiClientPhysicalRect> {
    let Some(overlap) = intersection(region, opaque) else {
        return vec![region];
    };
    let candidates = [
        rect(region.left(), region.top(), region.right(), overlap.top()),
        rect(
            region.left(),
            overlap.bottom(),
            region.right(),
            region.bottom(),
        ),
        rect(
            region.left(),
            overlap.top(),
            overlap.left(),
            overlap.bottom(),
        ),
        rect(
            overlap.right(),
            overlap.top(),
            region.right(),
            overlap.bottom(),
        ),
    ];
    candidates.into_iter().flatten().collect()
}

fn rect(
    left: u32,
    top: u32,
    right: u32,
    bottom: u32,
) -> Option<worth_ui_inspection::UiClientPhysicalRect> {
    worth_ui_inspection::UiClientPhysicalRect::new(left, top, right, bottom).ok()
}

#[cfg(test)]
mod tests {
    use super::{intersection, subtract_opaque_coverage};

    #[test]
    fn centered_opaque_rect_splits_background_into_four_disjoint_fragments() {
        let background = rect(0, 0, 20, 20);
        let foreground = rect(5, 5, 15, 15);
        let fragments = subtract_opaque_coverage(background, &[foreground]);

        assert_eq!(
            fragments,
            vec![
                rect(0, 0, 20, 5),
                rect(0, 15, 20, 20),
                rect(0, 5, 5, 15),
                rect(15, 5, 20, 15),
            ]
        );
        assert_eq!(intersection(background, foreground), Some(foreground));
    }

    fn rect(
        left: u32,
        top: u32,
        right: u32,
        bottom: u32,
    ) -> worth_ui_inspection::UiClientPhysicalRect {
        worth_ui_inspection::UiClientPhysicalRect::new(left, top, right, bottom).unwrap()
    }
}
