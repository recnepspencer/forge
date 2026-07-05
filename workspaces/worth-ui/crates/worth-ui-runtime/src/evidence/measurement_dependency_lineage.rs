use crate::declaration::stable_text_digest;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum UiMeasurementDependencyLineageKind {
    QueryScrollContentExtent,
    HostFontMetrics,
    HostViewportExtent,
    HostPortalAnchorRect,
    HostScrollContainerViewport,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiMeasurementDependencyLineageEntry {
    kind: UiMeasurementDependencyLineageKind,
    identity_digest: u64,
    generation_digest: u64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiMeasurementDependencyLineage {
    entries: Box<[UiMeasurementDependencyLineageEntry]>,
    identity_digest: u64,
}

impl UiMeasurementDependencyLineageEntry {
    pub const fn new(
        kind: UiMeasurementDependencyLineageKind,
        identity_digest: u64,
        generation_digest: u64,
    ) -> Self {
        Self {
            kind,
            identity_digest,
            generation_digest,
        }
    }

    pub const fn kind(self) -> UiMeasurementDependencyLineageKind {
        self.kind
    }

    pub const fn identity_digest(self) -> u64 {
        self.identity_digest
    }

    pub const fn generation_digest(self) -> u64 {
        self.generation_digest
    }
}

impl UiMeasurementDependencyLineage {
    pub fn new(mut entries: Vec<UiMeasurementDependencyLineageEntry>) -> Self {
        entries.sort_unstable_by_key(|entry| {
            (
                entry.kind as u8,
                entry.identity_digest,
                entry.generation_digest,
            )
        });
        let identity_digest = entries.iter().fold(
            stable_text_digest("worth-ui-measurement-dependency-lineage"),
            |digest, entry| {
                digest
                    ^ stable_text_digest(match entry.kind {
                        UiMeasurementDependencyLineageKind::QueryScrollContentExtent => {
                            "query-scroll-content-extent"
                        }
                        UiMeasurementDependencyLineageKind::HostFontMetrics => "host-font-metrics",
                        UiMeasurementDependencyLineageKind::HostViewportExtent => {
                            "host-viewport-extent"
                        }
                        UiMeasurementDependencyLineageKind::HostPortalAnchorRect => {
                            "host-portal-anchor-rect"
                        }
                        UiMeasurementDependencyLineageKind::HostScrollContainerViewport => {
                            "host-scroll-container-viewport"
                        }
                    })
                    .rotate_left(7)
                    ^ entry.identity_digest.rotate_left(17)
                    ^ entry.generation_digest.rotate_left(29)
            },
        );

        Self {
            entries: entries.into_boxed_slice(),
            identity_digest,
        }
    }

    pub fn entries(&self) -> &[UiMeasurementDependencyLineageEntry] {
        &self.entries
    }

    pub fn identity_digest(&self) -> u64 {
        self.identity_digest
    }
}
