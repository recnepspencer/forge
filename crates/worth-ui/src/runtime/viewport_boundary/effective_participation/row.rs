use crate::runtime::{WorthUiViewportDescendantParticipationReceipt, WorthUiViewportRect};

use super::super::digest::digest_parts;

#[derive(Clone, Debug, PartialEq)]
pub struct WorthUiEffectiveViewportParticipationRow {
    node_id: String,
    visual_frame: WorthUiViewportRect,
    visible: bool,
    hit_participates: bool,
    focus_participates: bool,
    accessibility_participates: bool,
    measurement_participates: bool,
    governing_boundary_count: usize,
    receipt_digest: u64,
}

impl WorthUiEffectiveViewportParticipationRow {
    pub(super) fn from_governing_rows<'a>(
        node_id: &str,
        governing_rows: impl Iterator<Item = &'a WorthUiViewportDescendantParticipationReceipt>,
    ) -> Option<Self> {
        let fold = fold_governing_viewport_rows(governing_rows)?;
        let receipt_digest = digest_parts([
            "effective_viewport_row",
            node_id,
            &fold.visual_frame.x().to_string(),
            &fold.visual_frame.y().to_string(),
            &fold.visual_frame.width().to_string(),
            &fold.visual_frame.height().to_string(),
            if fold.visible { "visible" } else { "clipped" },
            if fold.hit_participates {
                "hit"
            } else {
                "no_hit"
            },
            if fold.focus_participates {
                "focus"
            } else {
                "no_focus"
            },
            if fold.accessibility_participates {
                "a11y"
            } else {
                "no_a11y"
            },
            if fold.measurement_participates {
                "measurement"
            } else {
                "no_measurement"
            },
            &fold.governing_boundary_count.to_string(),
        ]);
        Some(Self {
            node_id: node_id.to_owned(),
            visual_frame: fold.visual_frame,
            visible: fold.visible,
            hit_participates: fold.hit_participates,
            focus_participates: fold.focus_participates,
            accessibility_participates: fold.accessibility_participates,
            measurement_participates: fold.measurement_participates,
            governing_boundary_count: fold.governing_boundary_count,
            receipt_digest,
        })
    }

    pub fn node_id(&self) -> &str {
        &self.node_id
    }

    pub fn visual_frame(&self) -> WorthUiViewportRect {
        self.visual_frame
    }

    pub fn visible(&self) -> bool {
        self.visible
    }

    pub fn hit_participates(&self) -> bool {
        self.hit_participates
    }

    pub fn focus_participates(&self) -> bool {
        self.focus_participates
    }

    pub fn accessibility_participates(&self) -> bool {
        self.accessibility_participates
    }

    pub fn measurement_participates(&self) -> bool {
        self.measurement_participates
    }

    pub fn governing_boundary_count(&self) -> usize {
        self.governing_boundary_count
    }

    pub fn receipt_digest(&self) -> u64 {
        self.receipt_digest
    }
}

struct FoldedGoverningViewportRows {
    visual_frame: WorthUiViewportRect,
    visible: bool,
    hit_participates: bool,
    focus_participates: bool,
    accessibility_participates: bool,
    measurement_participates: bool,
    governing_boundary_count: usize,
}

fn fold_governing_viewport_rows<'a>(
    governing_rows: impl Iterator<Item = &'a WorthUiViewportDescendantParticipationReceipt>,
) -> Option<FoldedGoverningViewportRows> {
    let mut fold = None;
    for row in governing_rows {
        fold = Some(append_governing_viewport_row(fold, row));
    }
    fold
}

fn append_governing_viewport_row(
    fold: Option<FoldedGoverningViewportRows>,
    row: &WorthUiViewportDescendantParticipationReceipt,
) -> FoldedGoverningViewportRows {
    let Some(fold) = fold else {
        return FoldedGoverningViewportRows {
            visual_frame: row.visual_frame(),
            visible: row.visible(),
            hit_participates: row.hit_participates(),
            focus_participates: row.focus_participates(),
            accessibility_participates: row.accessibility_participates(),
            measurement_participates: row.measurement_participates(),
            governing_boundary_count: 1,
        };
    };
    FoldedGoverningViewportRows {
        visual_frame: row.visual_frame(),
        visible: fold.visible && row.visible(),
        hit_participates: fold.hit_participates && row.hit_participates(),
        focus_participates: fold.focus_participates && row.focus_participates(),
        accessibility_participates: fold.accessibility_participates
            && row.accessibility_participates(),
        measurement_participates: fold.measurement_participates && row.measurement_participates(),
        governing_boundary_count: fold.governing_boundary_count + 1,
    }
}
