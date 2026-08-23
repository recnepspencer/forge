use super::PresentationSemanticPartition;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub(super) struct PresentationSemanticDetailKey {
    scope: PresentationSemanticScope,
    detail: PresentationSemanticDetail,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub(super) enum PresentationSemanticScope {
    Mechanic {
        aspect: usize,
        removal: bool,
        mechanic: Option<worth_ui_host_contract::UiMountedPaintCommandIdentity>,
    },
    Surface {
        aspect: usize,
        removal: bool,
        semantic_surface: worth_ui_host_contract::UiSemanticSurfaceIdentity,
        host_lineage: worth_ui_host_contract::UiHostPresentationLineageIdentity,
    },
    PinRelease {
        semantic_surface: worth_ui_host_contract::UiSemanticSurfaceIdentity,
        host_lineage: worth_ui_host_contract::UiHostPresentationLineageIdentity,
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

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
enum PresentationSemanticDetail {
    Content {
        mounted_frame: worth_ui_host_contract::UiMountedFrameIdentity,
        generation: Option<worth_ui_host_contract::UiMountedContentGeneration>,
        content: std::sync::Arc<str>,
    },
    Width {
        mounted_frame: worth_ui_host_contract::UiMountedFrameIdentity,
        width: Option<worth_ui_host_contract::UiQualifiedTextLayoutWidthBasis>,
        request: Option<worth_ui_host_contract::UiQualifiedTextLayoutRequestIdentity>,
        layout: Option<worth_ui_host_contract::UiQualifiedTextLayoutIdentity>,
    },
    PaintValue {
        mounted_frame: worth_ui_host_contract::UiMountedFrameIdentity,
        spans: Box<[([u8; 32], [u8; 4])]>,
    },
    PaintBoundary,
    Dpi {
        mounted_frame: worth_ui_host_contract::UiMountedFrameIdentity,
        dpi_milli: u32,
        text_scale: Option<worth_ui_host_contract::UiTextScaleGeneration>,
    },
    Upload {
        mounted_frame: worth_ui_host_contract::UiMountedFrameIdentity,
        keys: super::super::WorthUiPresentationRasterKeySetBasis,
    },
    PinRelease {
        mounted_frame: worth_ui_host_contract::UiMountedFrameIdentity,
        pins: Box<[super::super::WorthUiPresentationPinBasis]>,
    },
    Currentness,
}

impl PresentationSemanticPartition {
    pub(super) const fn aspect_ordinal(&self) -> usize {
        match self {
            Self::Content { .. } => 0,
            Self::Width { .. } => 1,
            Self::PaintValue { .. } => 2,
            Self::PaintBoundary { .. } => 3,
            Self::Dpi { .. } => 4,
            Self::Upload { .. } => 5,
            Self::PinRelease { .. } => 6,
            Self::Currentness { .. } => 7,
        }
    }

    pub(super) fn scope(&self) -> PresentationSemanticScope {
        match self {
            Self::Content {
                removal, mechanic, ..
            }
            | Self::Width {
                removal, mechanic, ..
            }
            | Self::PaintValue {
                removal, mechanic, ..
            }
            | Self::PaintBoundary {
                removal, mechanic, ..
            } => PresentationSemanticScope::Mechanic {
                aspect: self.aspect_ordinal(),
                removal: *removal,
                mechanic: *mechanic,
            },
            Self::Dpi {
                removal,
                semantic_surface,
                host_lineage,
                ..
            }
            | Self::Upload {
                removal,
                semantic_surface,
                host_lineage,
                ..
            } => PresentationSemanticScope::Surface {
                aspect: self.aspect_ordinal(),
                removal: *removal,
                semantic_surface: *semantic_surface,
                host_lineage: *host_lineage,
            },
            Self::PinRelease {
                semantic_surface,
                host_lineage,
                ..
            } => PresentationSemanticScope::PinRelease {
                semantic_surface: *semantic_surface,
                host_lineage: *host_lineage,
            },
            Self::Currentness {
                removal,
                attempt,
                semantic_surface,
                host_surface,
                binding,
                lineage,
                mounted_frame,
                predecessor,
            } => PresentationSemanticScope::Currentness {
                removal: *removal,
                attempt: *attempt,
                semantic_surface: *semantic_surface,
                host_surface: *host_surface,
                binding: *binding,
                lineage: *lineage,
                mounted_frame: *mounted_frame,
                predecessor: *predecessor,
            },
        }
    }

    pub(super) fn detail_key(&self) -> PresentationSemanticDetailKey {
        let detail = match self {
            Self::Content {
                mounted_frame,
                generation,
                content,
                ..
            } => PresentationSemanticDetail::Content {
                mounted_frame: *mounted_frame,
                generation: *generation,
                content: content.clone(),
            },
            Self::Width {
                mounted_frame,
                width,
                request,
                layout,
                ..
            } => PresentationSemanticDetail::Width {
                mounted_frame: *mounted_frame,
                width: *width,
                request: *request,
                layout: *layout,
            },
            Self::PaintValue {
                mounted_frame,
                spans,
                ..
            } => PresentationSemanticDetail::PaintValue {
                mounted_frame: *mounted_frame,
                spans: spans.clone(),
            },
            Self::PaintBoundary { .. } => PresentationSemanticDetail::PaintBoundary,
            Self::Dpi {
                mounted_frame,
                dpi_milli,
                text_scale,
                ..
            } => PresentationSemanticDetail::Dpi {
                mounted_frame: *mounted_frame,
                dpi_milli: *dpi_milli,
                text_scale: *text_scale,
            },
            Self::Upload {
                mounted_frame,
                keys,
                ..
            } => PresentationSemanticDetail::Upload {
                mounted_frame: *mounted_frame,
                keys: keys.clone(),
            },
            Self::PinRelease {
                mounted_frame,
                pins,
                ..
            } => PresentationSemanticDetail::PinRelease {
                mounted_frame: *mounted_frame,
                pins: pins.clone(),
            },
            Self::Currentness { .. } => PresentationSemanticDetail::Currentness,
        };
        PresentationSemanticDetailKey {
            scope: self.scope(),
            detail,
        }
    }
}
