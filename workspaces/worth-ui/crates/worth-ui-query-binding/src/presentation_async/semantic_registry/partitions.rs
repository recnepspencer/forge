use super::super::{
    semantic_transition::RetainedPresentationSemanticState, WorthUiPresentationMechanicBasis,
    WorthUiPresentationPinBasis, WorthUiPresentationRasterKeySetBasis,
    WorthUiPresentationRequestBasis,
};
use std::collections::HashMap;

#[path = "partitions/subscriber_identity.rs"]
mod subscriber_identity;
pub use subscriber_identity::WorthUiPresentationSemanticSubscriberIdentity;

#[path = "partitions/evidence_digest.rs"]
mod evidence_digest;
use evidence_digest::{
    content_digest, foreground_digest, raster_key_set_digest, subscriber_evidence_digests,
};

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub(crate) enum PresentationSemanticPartition {
    Content {
        mounted_frame: worth_ui_host_contract::UiMountedFrameIdentity,
        removal: bool,
        mechanic: Option<worth_ui_host_contract::UiMountedPaintCommandIdentity>,
        generation: Option<worth_ui_host_contract::UiMountedContentGeneration>,
        content: std::sync::Arc<str>,
    },
    Width {
        mounted_frame: worth_ui_host_contract::UiMountedFrameIdentity,
        removal: bool,
        mechanic: Option<worth_ui_host_contract::UiMountedPaintCommandIdentity>,
        width: Option<worth_ui_host_contract::UiQualifiedTextLayoutWidthBasis>,
        request: Option<worth_ui_host_contract::UiQualifiedTextLayoutRequestIdentity>,
        layout: Option<worth_ui_host_contract::UiQualifiedTextLayoutIdentity>,
    },
    PaintValue {
        mounted_frame: worth_ui_host_contract::UiMountedFrameIdentity,
        removal: bool,
        mechanic: Option<worth_ui_host_contract::UiMountedPaintCommandIdentity>,
        spans: Box<[([u8; 32], [u8; 4])]>,
    },
    PaintBoundary {
        mounted_frame: worth_ui_host_contract::UiMountedFrameIdentity,
        removal: bool,
        mechanic: Option<worth_ui_host_contract::UiMountedPaintCommandIdentity>,
        spans: Box<[([u8; 32], u32, u32)]>,
    },
    Dpi {
        mounted_frame: worth_ui_host_contract::UiMountedFrameIdentity,
        removal: bool,
        semantic_surface: worth_ui_host_contract::UiSemanticSurfaceIdentity,
        host_lineage: worth_ui_host_contract::UiHostPresentationLineageIdentity,
        dpi_milli: u32,
        text_scale: Option<worth_ui_host_contract::UiTextScaleGeneration>,
    },
    Upload {
        mounted_frame: worth_ui_host_contract::UiMountedFrameIdentity,
        removal: bool,
        semantic_surface: worth_ui_host_contract::UiSemanticSurfaceIdentity,
        host_lineage: worth_ui_host_contract::UiHostPresentationLineageIdentity,
        keys: WorthUiPresentationRasterKeySetBasis,
    },
    PinRelease {
        mounted_frame: worth_ui_host_contract::UiMountedFrameIdentity,
        semantic_surface: worth_ui_host_contract::UiSemanticSurfaceIdentity,
        host_lineage: worth_ui_host_contract::UiHostPresentationLineageIdentity,
        pins: Box<[WorthUiPresentationPinBasis]>,
    },
    Currentness {
        removal: bool,
        attempt: worth_ui_host_contract::UiMountedPresentationAttemptIdentity,
        semantic_surface: worth_ui_host_contract::UiSemanticSurfaceIdentity,
        host_surface: worth_ui_host_contract::UiHostSurfaceIdentity,
        binding: worth_ui_host_contract::UiSurfaceBindingGeneration,
        lineage: worth_ui_host_contract::UiHostPresentationLineageIdentity,
        mounted_frame: worth_ui_host_contract::UiMountedFrameIdentity,
        predecessor: Option<worth_ui_host_contract::UiMountedFrameIdentity>,
    },
}

pub(super) struct PresentationSemanticInstanceSpecification {
    pub(super) subscriber: WorthUiPresentationSemanticSubscriberIdentity,
    pub(super) partitions: [PresentationSemanticPartition; super::DEPENDENCY_COUNT],
}

pub(crate) struct PresentationPinPartitionIndex {
    binding: HashMap<
        (
            worth_ui_host_contract::UiQualifiedTextLayoutIdentity,
            worth_ui_host_contract::UiGlyphRasterKey,
        ),
        WorthUiPresentationPinBasis,
    >,
    releases: HashMap<
        (
            worth_ui_host_contract::UiQualifiedTextLayoutIdentity,
            worth_ui_host_contract::UiGlyphRasterKey,
        ),
        WorthUiPresentationPinBasis,
    >,
}

impl PresentationPinPartitionIndex {
    pub(crate) fn from_basis(basis: &WorthUiPresentationRequestBasis) -> Self {
        Self {
            binding: index_pins(basis.binding_pins()),
            releases: index_pins(basis.pin_releases()),
        }
    }

    fn pins_for(
        &self,
        mechanic: &WorthUiPresentationMechanicBasis,
        removal: bool,
    ) -> Box<[WorthUiPresentationPinBasis]> {
        let index = if removal {
            &self.releases
        } else {
            &self.binding
        };
        mechanic
            .raster_key_set()
            .keys()
            .iter()
            .filter_map(|key| index.get(&(mechanic.layout(), *key)).copied())
            .collect::<Vec<_>>()
            .into_boxed_slice()
    }
}

pub(super) fn semantic_instance_specifications(
    basis: &WorthUiPresentationRequestBasis,
    state: &RetainedPresentationSemanticState,
    removed: &[WorthUiPresentationMechanicBasis],
) -> Vec<PresentationSemanticInstanceSpecification> {
    let pin_index = PresentationPinPartitionIndex::from_basis(basis);
    let mut mechanics = state.mechanics().values().collect::<Vec<_>>();
    mechanics.sort_by_key(|mechanic| {
        let (slot, row) = mechanic
            .mechanic()
            .semantic_text_identity_parts()
            .expect("retained mechanic remains semantic text");
        (mechanic.mounted_instance().diagnostic_value(), slot, row)
    });
    let mut specifications = if mechanics.is_empty() {
        vec![specification(
            basis,
            None,
            state.dpi_milli(),
            false,
            &pin_index,
        )]
    } else {
        mechanics
            .into_iter()
            .map(|mechanic| {
                specification(basis, Some(mechanic), state.dpi_milli(), false, &pin_index)
            })
            .collect::<Vec<_>>()
    };
    specifications.extend(
        removed.iter().map(|mechanic| {
            specification(basis, Some(mechanic), state.dpi_milli(), true, &pin_index)
        }),
    );
    specifications
}

pub(crate) fn partition_for_mechanic(
    basis: &WorthUiPresentationRequestBasis,
    mechanic: &WorthUiPresentationMechanicBasis,
    change: super::WorthUiPresentationSemanticChange,
    pin_index: &PresentationPinPartitionIndex,
) -> PresentationSemanticPartition {
    partitions(
        basis,
        Some(mechanic),
        basis.dpi_milli(),
        false,
        Some(pin_index),
    )[change.ordinal()]
    .clone()
}

pub(crate) fn partition_for_removed_mechanic(
    basis: &WorthUiPresentationRequestBasis,
    mechanic: &WorthUiPresentationMechanicBasis,
    change: super::WorthUiPresentationSemanticChange,
    pin_index: &PresentationPinPartitionIndex,
) -> PresentationSemanticPartition {
    partitions(
        basis,
        Some(mechanic),
        basis.dpi_milli(),
        true,
        Some(pin_index),
    )[change.ordinal()]
    .clone()
}

pub(crate) fn partition_for_empty(
    basis: &WorthUiPresentationRequestBasis,
    change: super::WorthUiPresentationSemanticChange,
) -> PresentationSemanticPartition {
    partitions(basis, None, basis.dpi_milli(), false, None)[change.ordinal()].clone()
}

pub(crate) fn currentness_partition(
    basis: &WorthUiPresentationRequestBasis,
) -> PresentationSemanticPartition {
    partitions(
        basis,
        basis.mechanics().first(),
        basis.dpi_milli(),
        false,
        None,
    )[7]
    .clone()
}

fn specification(
    basis: &WorthUiPresentationRequestBasis,
    mechanic: Option<&WorthUiPresentationMechanicBasis>,
    dpi_milli: u32,
    removal: bool,
    pin_index: &PresentationPinPartitionIndex,
) -> PresentationSemanticInstanceSpecification {
    let partitions = partitions(basis, mechanic, dpi_milli, removal, Some(pin_index));
    let (source_digest, dependency_digests) = subscriber_evidence_digests(basis, mechanic, removal);
    PresentationSemanticInstanceSpecification {
        subscriber: WorthUiPresentationSemanticSubscriberIdentity {
            mounted_instance: mechanic.map(WorthUiPresentationMechanicBasis::mounted_instance),
            mechanic: mechanic.map(WorthUiPresentationMechanicBasis::mechanic),
            mounted_frame: basis.mounted_frame(),
            removal,
            content_digest: mechanic.map(content_digest).unwrap_or([0; 32]),
            layout_digest: mechanic
                .map(|mechanic| mechanic.layout().digest())
                .unwrap_or([0; 32]),
            foreground_digest: mechanic.map(foreground_digest).unwrap_or([0; 32]),
            raster_key_set_digest: mechanic.map(raster_key_set_digest).unwrap_or([0; 32]),
            source_digest,
            dependency_digests,
            attempt: basis.attempt(),
            semantic_surface: basis.semantic_surface(),
            host_surface: basis.host_surface(),
            binding: basis.binding(),
            host_lineage: basis.host_lineage(),
        },
        partitions,
    }
}

fn partitions(
    basis: &WorthUiPresentationRequestBasis,
    mechanic: Option<&WorthUiPresentationMechanicBasis>,
    dpi_milli: u32,
    removal: bool,
    pin_index: Option<&PresentationPinPartitionIndex>,
) -> [PresentationSemanticPartition; super::DEPENDENCY_COUNT] {
    let paint_values = mechanic
        .into_iter()
        .flat_map(WorthUiPresentationMechanicBasis::paint_spans)
        .map(|span| (span.identity(), span.foreground()))
        .collect::<Vec<_>>()
        .into_boxed_slice();
    let paint_boundaries = mechanic
        .into_iter()
        .flat_map(WorthUiPresentationMechanicBasis::paint_spans)
        .map(|span| {
            let range = span.original_range();
            (span.identity(), range.start(), range.end())
        })
        .collect::<Vec<_>>()
        .into_boxed_slice();
    let raster_keys = mechanic
        .map(|mechanic| mechanic.raster_key_set().clone())
        .unwrap_or_else(|| WorthUiPresentationRasterKeySetBasis::from_runtime(Vec::new()));
    let pins = mechanic
        .map(|mechanic| {
            if let Some(index) = pin_index {
                return index.pins_for(mechanic, removal);
            }
            let pins = if removal {
                basis.pin_releases()
            } else {
                basis.binding_pins()
            };
            pins.iter()
                .copied()
                .filter(|pin| {
                    pin.layout() == mechanic.layout()
                        && mechanic.raster_key_set().contains(pin.key())
                })
                .collect::<Vec<_>>()
                .into_boxed_slice()
        })
        .unwrap_or_else(|| basis.pin_releases().into());
    [
        PresentationSemanticPartition::Content {
            mounted_frame: basis.mounted_frame(),
            removal,
            mechanic: mechanic.map(WorthUiPresentationMechanicBasis::mechanic),
            generation: mechanic.map(WorthUiPresentationMechanicBasis::content_generation),
            content: mechanic
                .map(|mechanic| std::sync::Arc::from(mechanic.content()))
                .unwrap_or_else(|| std::sync::Arc::from("")),
        },
        PresentationSemanticPartition::Width {
            mounted_frame: basis.mounted_frame(),
            removal,
            mechanic: mechanic.map(WorthUiPresentationMechanicBasis::mechanic),
            width: mechanic.map(WorthUiPresentationMechanicBasis::layout_width),
            request: mechanic.map(WorthUiPresentationMechanicBasis::layout_request),
            layout: mechanic.map(WorthUiPresentationMechanicBasis::layout),
        },
        PresentationSemanticPartition::PaintValue {
            mounted_frame: basis.mounted_frame(),
            removal,
            mechanic: mechanic.map(WorthUiPresentationMechanicBasis::mechanic),
            spans: paint_values,
        },
        PresentationSemanticPartition::PaintBoundary {
            mounted_frame: basis.mounted_frame(),
            removal,
            mechanic: mechanic.map(WorthUiPresentationMechanicBasis::mechanic),
            spans: paint_boundaries,
        },
        PresentationSemanticPartition::Dpi {
            mounted_frame: basis.mounted_frame(),
            removal,
            semantic_surface: basis.semantic_surface(),
            host_lineage: basis.host_lineage(),
            dpi_milli,
            text_scale: mechanic.map(WorthUiPresentationMechanicBasis::text_scale),
        },
        PresentationSemanticPartition::Upload {
            mounted_frame: basis.mounted_frame(),
            removal,
            semantic_surface: basis.semantic_surface(),
            host_lineage: basis.host_lineage(),
            keys: raster_keys,
        },
        PresentationSemanticPartition::PinRelease {
            mounted_frame: basis.mounted_frame(),
            semantic_surface: basis.semantic_surface(),
            host_lineage: basis.host_lineage(),
            pins,
        },
        PresentationSemanticPartition::Currentness {
            removal,
            attempt: basis.attempt(),
            semantic_surface: basis.semantic_surface(),
            host_surface: basis.host_surface(),
            binding: basis.binding(),
            lineage: basis.host_lineage(),
            mounted_frame: basis.mounted_frame(),
            predecessor: basis.predecessor(),
        },
    ]
}

fn index_pins(
    pins: &[WorthUiPresentationPinBasis],
) -> HashMap<
    (
        worth_ui_host_contract::UiQualifiedTextLayoutIdentity,
        worth_ui_host_contract::UiGlyphRasterKey,
    ),
    WorthUiPresentationPinBasis,
> {
    pins.iter()
        .copied()
        .map(|pin| ((pin.layout(), pin.key()), pin))
        .collect()
}
