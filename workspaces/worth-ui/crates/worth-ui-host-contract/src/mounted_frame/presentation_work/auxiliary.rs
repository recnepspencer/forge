use std::collections::{BTreeMap, HashMap};

use super::{UiMountedPaintCommand, UiMountedPaintCommandIdentity};

#[derive(Clone, Debug, PartialEq)]
pub struct UiMountedPresentationAuxiliaryState {
    frame: crate::UiMountedFrameIdentity,
    surface: crate::UiSemanticSurfaceIdentity,
    binding: crate::UiSurfaceBindingGeneration,
    content: crate::UiMountedContentGeneration,
    nodes: Box<[crate::UiMountedNodeProjectionView]>,
    clips: crate::UiMountedClipTable,
    layers: crate::UiMountedLayerTable,
    hit_tests: crate::UiMountedHitTestTable,
    paint_batches: crate::UiMountedPaintBatchTable,
    spatial_batches: crate::UiMountedSpatialBatchTable,
    realtime_batches: crate::UiMountedRealtimeBatchTable,
    resources: crate::UiMountedResourceTable,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiMountedPresentationReconstructionDenial {
    DuplicateTableIndex,
    MissingTableIndex,
    CapacityExceeded,
}

impl UiMountedPresentationAuxiliaryState {
    #[doc(hidden)]
    pub fn from_runtime_mounting(projection: &crate::UiMountedProjectionView) -> Self {
        Self {
            frame: projection.frame(),
            surface: projection.surface(),
            binding: projection.binding(),
            content: projection.content_generation(),
            nodes: projection.nodes().to_vec().into_boxed_slice(),
            clips: projection.clips().clone(),
            layers: projection.layers().clone(),
            hit_tests: projection.hit_tests().clone(),
            paint_batches: projection.paint_batches().clone(),
            spatial_batches: projection.spatial_batches().clone(),
            realtime_batches: projection.realtime_batches().clone(),
            resources: projection.resources().clone(),
        }
    }

    pub fn reconstruct(
        &self,
        commands: &HashMap<UiMountedPaintCommandIdentity, UiMountedPaintCommand>,
    ) -> Result<crate::UiMountedProjectionView, UiMountedPresentationReconstructionDenial> {
        let mut filled_rects = BTreeMap::new();
        let mut semantic_text = BTreeMap::new();
        for command in commands.values() {
            match command {
                UiMountedPaintCommand::FilledRect {
                    table_index,
                    mechanic,
                    ..
                } => insert(&mut filled_rects, *table_index, *mechanic)?,
                UiMountedPaintCommand::SemanticText {
                    table_index,
                    mechanic,
                    ..
                } => insert(&mut semantic_text, *table_index, mechanic.clone())?,
            }
        }
        let filled_rects = contiguous(filled_rects)?;
        let semantic_text = contiguous(semantic_text)?;
        Ok(crate::UiMountedProjectionView::new(
            crate::UiMountedProjectionViewInput {
                frame: self.frame,
                surface: self.surface,
                binding: self.binding,
                content_generation: self.content,
                nodes: self.nodes.to_vec(),
                clips: self.clips.clone(),
                layers: self.layers.clone(),
                filled_rects: crate::UiMountedFilledRectTable::from_runtime_mounting(filled_rects)
                    .map_err(|_| UiMountedPresentationReconstructionDenial::CapacityExceeded)?,
                semantic_text: crate::UiMountedSemanticTextTable::from_runtime_mounting(
                    semantic_text,
                )
                .map_err(|_| UiMountedPresentationReconstructionDenial::CapacityExceeded)?,
                hit_tests: self.hit_tests.clone(),
                paint_batches: self.paint_batches.clone(),
                spatial_batches: self.spatial_batches.clone(),
                realtime_batches: self.realtime_batches.clone(),
                resources: self.resources.clone(),
            },
        ))
    }

    #[doc(hidden)]
    pub fn same_presentation_meaning(&self, other: &Self) -> bool {
        self.surface == other.surface
            && self.binding == other.binding
            && same_nodes(&self.nodes, &other.nodes)
            && self.clips == other.clips
            && self.layers == other.layers
            && same_hit_tests(&self.hit_tests, &other.hit_tests)
            && self.paint_batches == other.paint_batches
            && self.spatial_batches == other.spatial_batches
            && self.realtime_batches == other.realtime_batches
            && self.resources == other.resources
    }

    pub const fn frame(&self) -> crate::UiMountedFrameIdentity {
        self.frame
    }

    pub const fn surface(&self) -> crate::UiSemanticSurfaceIdentity {
        self.surface
    }

    pub const fn binding(&self) -> crate::UiSurfaceBindingGeneration {
        self.binding
    }

    pub const fn content(&self) -> crate::UiMountedContentGeneration {
        self.content
    }
}

fn same_nodes(
    left: &[crate::UiMountedNodeProjectionView],
    right: &[crate::UiMountedNodeProjectionView],
) -> bool {
    left.len() == right.len()
        && left.iter().zip(right).all(|(left, right)| {
            left.mounted_instance() == right.mounted_instance()
                && left.role() == right.role()
                && left.participation() == right.participation()
                && left.allocation() == right.allocation()
                && left.preview() == right.preview()
                && left.paint() == right.paint()
                && left.hit_test() == right.hit_test()
                && left.accessibility() == right.accessibility()
                && left.motion() == right.motion()
                && left.diagnostic() == right.diagnostic()
                && left.drawables() == right.drawables()
                && left.semantic_text() == right.semantic_text()
        })
}

fn same_hit_tests(
    left: &crate::UiMountedHitTestTable,
    right: &crate::UiMountedHitTestTable,
) -> bool {
    left.rows().len() == right.rows().len()
        && left.rows().iter().zip(right.rows()).all(|(left, right)| {
            left.surface() == right.surface()
                && left.binding() == right.binding()
                && left.mounted_instance() == right.mounted_instance()
                && left.bounds() == right.bounds()
                && left.clip_bounds() == right.clip_bounds()
                && left.order() == right.order()
        })
}

fn insert<T>(
    rows: &mut BTreeMap<u16, T>,
    index: u16,
    row: T,
) -> Result<(), UiMountedPresentationReconstructionDenial> {
    if rows.insert(index, row).is_some() {
        return Err(UiMountedPresentationReconstructionDenial::DuplicateTableIndex);
    }
    Ok(())
}

fn contiguous<T>(
    rows: BTreeMap<u16, T>,
) -> Result<Vec<T>, UiMountedPresentationReconstructionDenial> {
    rows.into_iter()
        .enumerate()
        .map(|(expected, (observed, row))| {
            (usize::from(observed) == expected)
                .then_some(row)
                .ok_or(UiMountedPresentationReconstructionDenial::MissingTableIndex)
        })
        .collect()
}
