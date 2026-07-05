use crate::declaration::{stable_text_digest, UiDeclaredMeasurementBasisRequirementSet};

use super::{
    UiMeasurementDependencyLineage, UiMeasurementDependencyLineageEntry,
    UiMeasurementDependencyLineageKind, UiMeasurementNeighborhoodClassHint,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiMeasurementDependencyMapEntry {
    lineage: UiMeasurementDependencyLineageEntry,
    neighborhood_class_hint: UiMeasurementNeighborhoodClassHint,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiMeasurementDependencyMap {
    entries: Box<[UiMeasurementDependencyMapEntry]>,
    identity_digest: u64,
}

impl UiMeasurementDependencyMapEntry {
    pub(crate) const fn new(
        lineage: UiMeasurementDependencyLineageEntry,
        neighborhood_class_hint: UiMeasurementNeighborhoodClassHint,
    ) -> Self {
        Self {
            lineage,
            neighborhood_class_hint,
        }
    }

    pub const fn lineage(self) -> UiMeasurementDependencyLineageEntry {
        self.lineage
    }

    pub const fn neighborhood_class_hint(self) -> UiMeasurementNeighborhoodClassHint {
        self.neighborhood_class_hint
    }
}

impl UiMeasurementDependencyMap {
    pub(crate) fn new(mut entries: Vec<UiMeasurementDependencyMapEntry>) -> Self {
        entries.sort_unstable_by_key(|entry| {
            (
                entry.neighborhood_class_hint as u8,
                entry.lineage.kind() as u8,
                entry.lineage.identity_digest(),
                entry.lineage.generation_digest(),
            )
        });
        let identity_digest = entries.iter().fold(
            stable_text_digest("worth-ui-measurement-dependency-map"),
            |digest, entry| {
                digest
                    ^ (entry.neighborhood_class_hint as u64).rotate_left(7)
                    ^ (entry.lineage.kind() as u64).rotate_left(13)
                    ^ entry.lineage.identity_digest().rotate_left(19)
                    ^ entry.lineage.generation_digest().rotate_left(23)
            },
        );

        Self {
            entries: entries.into_boxed_slice(),
            identity_digest,
        }
    }

    pub fn entries(&self) -> &[UiMeasurementDependencyMapEntry] {
        &self.entries
    }

    pub fn identity_digest(&self) -> u64 {
        self.identity_digest
    }

    pub fn dominant_neighborhood_class_hint(&self) -> UiMeasurementNeighborhoodClassHint {
        if self.entries.iter().any(|entry| {
            entry.neighborhood_class_hint
                == UiMeasurementNeighborhoodClassHint::PortalAnchorDependency
        }) {
            UiMeasurementNeighborhoodClassHint::PortalAnchorDependency
        } else if self.entries.iter().any(|entry| {
            entry.neighborhood_class_hint
                == UiMeasurementNeighborhoodClassHint::ScrollContainerDependency
        }) {
            UiMeasurementNeighborhoodClassHint::ScrollContainerDependency
        } else if self.entries.iter().any(|entry| {
            entry.neighborhood_class_hint == UiMeasurementNeighborhoodClassHint::ViewportDependency
        }) {
            UiMeasurementNeighborhoodClassHint::ViewportDependency
        } else if self.entries.iter().any(|entry| {
            entry.neighborhood_class_hint
                == UiMeasurementNeighborhoodClassHint::LocalIntrinsicContentDependency
        }) {
            UiMeasurementNeighborhoodClassHint::LocalIntrinsicContentDependency
        } else {
            UiMeasurementNeighborhoodClassHint::ContainerAvailableSpaceDependency
        }
    }
}

pub fn derive_measurement_dependency_map(
    dependency_lineage: &UiMeasurementDependencyLineage,
) -> UiMeasurementDependencyMap {
    let entries = dependency_lineage
        .entries()
        .iter()
        .copied()
        .map(|lineage| UiMeasurementDependencyMapEntry::new(lineage, classify_lineage(lineage)))
        .collect::<Vec<_>>();
    UiMeasurementDependencyMap::new(entries)
}

pub fn derive_measurement_neighborhood_class_hint(
    requirements: &UiDeclaredMeasurementBasisRequirementSet,
    dependency_map: &UiMeasurementDependencyMap,
) -> UiMeasurementNeighborhoodClassHint {
    if dependency_map.entries().is_empty() {
        fallback_neighborhood_class_hint(requirements)
    } else {
        dependency_map.dominant_neighborhood_class_hint()
    }
}

fn classify_lineage(
    lineage: UiMeasurementDependencyLineageEntry,
) -> UiMeasurementNeighborhoodClassHint {
    match lineage.kind() {
        UiMeasurementDependencyLineageKind::QueryScrollContentExtent
        | UiMeasurementDependencyLineageKind::HostFontMetrics => {
            UiMeasurementNeighborhoodClassHint::LocalIntrinsicContentDependency
        }
        UiMeasurementDependencyLineageKind::HostViewportExtent => {
            UiMeasurementNeighborhoodClassHint::ViewportDependency
        }
        UiMeasurementDependencyLineageKind::HostPortalAnchorRect => {
            UiMeasurementNeighborhoodClassHint::PortalAnchorDependency
        }
        UiMeasurementDependencyLineageKind::HostScrollContainerViewport => {
            UiMeasurementNeighborhoodClassHint::ScrollContainerDependency
        }
    }
}

fn fallback_neighborhood_class_hint(
    requirements: &UiDeclaredMeasurementBasisRequirementSet,
) -> UiMeasurementNeighborhoodClassHint {
    if requirements.requires_portal_anchor_metrics() {
        UiMeasurementNeighborhoodClassHint::PortalAnchorDependency
    } else if requirements.requires_scroll_container_viewport() {
        UiMeasurementNeighborhoodClassHint::ScrollContainerDependency
    } else if requirements.requires_viewport_extent() {
        UiMeasurementNeighborhoodClassHint::ViewportDependency
    } else {
        UiMeasurementNeighborhoodClassHint::ContainerAvailableSpaceDependency
    }
}
