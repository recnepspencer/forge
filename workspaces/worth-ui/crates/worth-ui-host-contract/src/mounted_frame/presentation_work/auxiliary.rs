use std::collections::{HashMap, HashSet};
use std::sync::Arc;

use super::{UiMountedPaintCommand, UiMountedPaintCommandIdentity};

#[derive(Clone, Debug, PartialEq)]
pub struct UiMountedPresentationAuxiliaryState {
    frame: crate::UiMountedFrameIdentity,
    surface: crate::UiSemanticSurfaceIdentity,
    binding: crate::UiSurfaceBindingGeneration,
    content: crate::UiMountedContentGeneration,
    nodes: Arc<[crate::UiMountedNodeProjectionView]>,
    filled_rects: crate::UiMountedFilledRectTable,
    semantic_text: crate::UiMountedSemanticTextTable,
    clips: crate::UiMountedClipTable,
    layers: crate::UiMountedLayerTable,
    hit_tests: crate::UiMountedHitTestTable,
    paint_batches: crate::UiMountedPaintBatchTable,
    spatial_batches: crate::UiMountedSpatialBatchTable,
    realtime_batches: crate::UiMountedRealtimeBatchTable,
    resources: crate::UiMountedResourceTable,
    authored_commands: Arc<[UiMountedPaintCommandIdentity]>,
    authored_order: Arc<[crate::UiMountedPaintOrderIdentity]>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiMountedPresentationReconstructionDenial {
    DuplicateTableIndex,
    MissingTableIndex,
    CommandMismatch,
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
            nodes: projection.retained_nodes(),
            filled_rects: projection.filled_rects().clone(),
            semantic_text: projection.semantic_text().clone(),
            clips: projection.clips().clone(),
            layers: projection.layers().clone(),
            hit_tests: projection.hit_tests().clone(),
            paint_batches: projection.paint_batches().clone(),
            spatial_batches: projection.spatial_batches().clone(),
            realtime_batches: projection.realtime_batches().clone(),
            resources: projection.resources().clone(),
            authored_commands: projection
                .retained_paint_commands()
                .iter()
                .map(UiMountedPaintCommand::identity)
                .collect(),
            authored_order: projection.retained_paint_order(),
        }
    }

    pub fn reconstruct(
        &self,
        commands: &HashMap<UiMountedPaintCommandIdentity, UiMountedPaintCommand>,
    ) -> Result<crate::UiMountedProjectionView, UiMountedPresentationReconstructionDenial> {
        validate_commands(commands, &self.filled_rects, &self.semantic_text)?;
        Ok(crate::UiMountedProjectionView::new(
            crate::UiMountedProjectionViewInput {
                frame: self.frame,
                surface: self.surface,
                binding: self.binding,
                content_generation: self.content,
                nodes: self.nodes.to_vec(),
                clips: self.clips.clone(),
                layers: self.layers.clone(),
                filled_rects: self.filled_rects.clone(),
                semantic_text: self.semantic_text.clone(),
                hit_tests: self.hit_tests.clone(),
                paint_batches: self.paint_batches.clone(),
                spatial_batches: self.spatial_batches.clone(),
                realtime_batches: self.realtime_batches.clone(),
                resources: self.resources.clone(),
                authored_paint_commands: self
                    .authored_commands
                    .iter()
                    .map(|identity| {
                        commands
                            .get(identity)
                            .expect("validated reconstruction source names a command")
                            .clone()
                    })
                    .collect(),
                authored_paint_order: self.authored_order.to_vec(),
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

    #[doc(hidden)]
    pub fn same_lane_presentation_meaning(&self, other: &Self) -> bool {
        self.paint_batches == other.paint_batches
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

fn validate_commands(
    commands: &HashMap<UiMountedPaintCommandIdentity, UiMountedPaintCommand>,
    filled_rects: &crate::UiMountedFilledRectTable,
    semantic_text: &crate::UiMountedSemanticTextTable,
) -> Result<(), UiMountedPresentationReconstructionDenial> {
    let expected = filled_rects
        .rows()
        .iter()
        .map(UiMountedPaintCommandIdentity::filled_rect)
        .chain(
            semantic_text
                .rows()
                .iter()
                .map(UiMountedPaintCommandIdentity::semantic_text),
        )
        .collect::<HashSet<_>>();
    let observed = commands
        .iter()
        .map(|(identity, command)| {
            let derived = match command {
                UiMountedPaintCommand::FilledRect { mechanic, .. } => {
                    UiMountedPaintCommandIdentity::filled_rect(mechanic)
                }
                UiMountedPaintCommand::SemanticText { mechanic, .. } => {
                    UiMountedPaintCommandIdentity::semantic_text(mechanic)
                }
            };
            (*identity == derived).then_some(*identity)
        })
        .collect::<Option<HashSet<_>>>()
        .ok_or(UiMountedPresentationReconstructionDenial::CommandMismatch)?;
    (expected == observed)
        .then_some(())
        .ok_or(UiMountedPresentationReconstructionDenial::CommandMismatch)
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
