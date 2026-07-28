use worth_store_physical_format::{PhysicalCellReuseDomain, PhysicalGenerationOwner};

use super::ProtectedPhysicalReference;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProtectedReferenceRangeSet {
    ranges: Vec<ProtectedReferenceRange>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct ProtectedReferenceRange {
    family: PhysicalReferenceRangeFamily,
    start: u64,
    end: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct ProtectedRangeIntersection {
    protected_ranges: u64,
    candidate_ranges: u64,
    range_comparisons: u64,
    overlapping_ranges: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum PhysicalReferenceRangeFamily {
    Segment {
        segment_id: u64,
        generation: u64,
    },
    Extent {
        segment_id: u64,
        generation: u64,
    },
    RecordExtent {
        generation: u64,
    },
    Page {
        segment_id: u64,
        generation: u64,
    },
    Slot {
        segment_id: u64,
        page_id: u64,
        generation: u64,
    },
    RootPublication {
        generation: u64,
    },
    Singleton,
}

impl ProtectedReferenceRangeSet {
    pub(crate) fn from_references(
        references: &[ProtectedPhysicalReference],
        mut ranges: Vec<ProtectedReferenceRange>,
    ) -> Self {
        ranges.clear();
        for reference in references {
            let Some((family, value)) = range_coordinate(*reference) else {
                ranges.push(ProtectedReferenceRange::singleton());
                continue;
            };
            match ranges.last_mut() {
                Some(range) if range.family == family && range.end.saturating_add(1) == value => {
                    range.end = value;
                }
                _ => ranges.push(ProtectedReferenceRange::new(family, value, value)),
            }
        }
        Self { ranges }
    }

    pub fn ranges(&self) -> &[ProtectedReferenceRange] {
        &self.ranges
    }

    #[cfg(any(test, feature = "certification-authority"))]
    pub(crate) fn for_certification_test() -> Self {
        Self {
            ranges: vec![ProtectedReferenceRange::singleton()],
        }
    }

    pub fn contains_reference(&self, reference: ProtectedPhysicalReference) -> bool {
        let Some((family, value)) = range_coordinate(reference) else {
            return self
                .ranges
                .iter()
                .any(|range| range.family == PhysicalReferenceRangeFamily::Singleton);
        };
        self.ranges
            .iter()
            .any(|range| range.family == family && range.start <= value && value <= range.end)
    }

    pub fn contains_owner(&self, owner: PhysicalGenerationOwner) -> bool {
        let Some((family, value)) = range_coordinate_for_owner(owner) else {
            return self
                .ranges
                .iter()
                .any(|range| range.family == PhysicalReferenceRangeFamily::Singleton);
        };
        self.ranges
            .iter()
            .any(|range| range.family == family && range.start <= value && value <= range.end)
    }

    pub(crate) fn bounded_intersection(
        &self,
        candidates: &[ProtectedReferenceRange],
    ) -> ProtectedRangeIntersection {
        let mut range_comparisons = 0;
        let mut overlapping_ranges = 0;
        for protected in &self.ranges {
            for candidate in candidates {
                range_comparisons += 1;
                if protected.intersects(*candidate) {
                    overlapping_ranges += 1;
                }
            }
        }
        ProtectedRangeIntersection {
            protected_ranges: self.ranges.len() as u64,
            candidate_ranges: candidates.len() as u64,
            range_comparisons,
            overlapping_ranges,
        }
    }
}

impl ProtectedReferenceRange {
    const fn new(family: PhysicalReferenceRangeFamily, start: u64, end: u64) -> Self {
        Self { family, start, end }
    }

    const fn singleton() -> Self {
        Self {
            family: PhysicalReferenceRangeFamily::Singleton,
            start: 0,
            end: 0,
        }
    }

    pub const fn start(self) -> u64 {
        self.start
    }

    pub const fn end(self) -> u64 {
        self.end
    }

    pub(crate) fn digest_component(self) -> u64 {
        let mut digest = 0xcbf29ce484222325_u64;
        mix_u64(&mut digest, self.family.digest_component());
        mix_u64(&mut digest, self.start);
        mix_u64(&mut digest, self.end);
        digest
    }

    pub(crate) fn intersects(self, candidate: Self) -> bool {
        self.family == candidate.family
            && self.start <= candidate.end
            && candidate.start <= self.end
    }
}

impl ProtectedRangeIntersection {
    pub(crate) const fn protected_ranges(self) -> u64 {
        self.protected_ranges
    }

    pub(crate) const fn candidate_ranges(self) -> u64 {
        self.candidate_ranges
    }

    pub(crate) const fn range_comparisons(self) -> u64 {
        self.range_comparisons
    }

    pub(crate) const fn overlapping_ranges(self) -> u64 {
        self.overlapping_ranges
    }
}

pub(crate) fn latch_domain(reference: ProtectedPhysicalReference) -> PhysicalCellReuseDomain {
    reference.owner().domain()
}

fn range_coordinate(
    reference: ProtectedPhysicalReference,
) -> Option<(PhysicalReferenceRangeFamily, u64)> {
    range_coordinate_for_owner(reference.owner())
}

fn range_coordinate_for_owner(
    owner: PhysicalGenerationOwner,
) -> Option<(PhysicalReferenceRangeFamily, u64)> {
    let generation = owner.generation().get();
    match owner.domain() {
        PhysicalCellReuseDomain::Segment => owner.segment_id().map(|segment| {
            (
                PhysicalReferenceRangeFamily::Segment {
                    segment_id: segment.get(),
                    generation,
                },
                segment.get(),
            )
        }),
        PhysicalCellReuseDomain::ExtentAllocation => Some((
            PhysicalReferenceRangeFamily::Extent {
                segment_id: owner.segment_id()?.get(),
                generation,
            },
            owner.extent_id()?.get(),
        )),
        PhysicalCellReuseDomain::RecordExtentAllocation => Some((
            PhysicalReferenceRangeFamily::RecordExtent { generation },
            owner.extent_id()?.get(),
        )),
        PhysicalCellReuseDomain::Page => Some((
            PhysicalReferenceRangeFamily::Page {
                segment_id: owner.segment_id()?.get(),
                generation,
            },
            owner.page_id()?.get(),
        )),
        PhysicalCellReuseDomain::SlotAllocation | PhysicalCellReuseDomain::FreeSpaceReuse => {
            Some((
                PhysicalReferenceRangeFamily::Slot {
                    segment_id: owner.segment_id()?.get(),
                    page_id: owner.page_id()?.get(),
                    generation,
                },
                owner.slot()?.get() as u64,
            ))
        }
        PhysicalCellReuseDomain::RootPublication => owner.root_reference().map(|root| {
            (
                PhysicalReferenceRangeFamily::RootPublication { generation },
                root.get(),
            )
        }),
    }
}

impl PhysicalReferenceRangeFamily {
    fn digest_component(self) -> u64 {
        match self {
            Self::Segment {
                segment_id,
                generation,
            } => segment_id ^ generation.rotate_left(7),
            Self::Extent {
                segment_id,
                generation,
            } => 0x10_0000_0000 | segment_id ^ generation.rotate_left(11),
            Self::RecordExtent { generation } => 0x60_0000_0000 ^ generation.rotate_left(29),
            Self::Page {
                segment_id,
                generation,
            } => 0x20_0000_0000 | segment_id ^ generation.rotate_left(13),
            Self::Slot {
                segment_id,
                page_id,
                generation,
            } => 0x30_0000_0000 | segment_id ^ page_id.rotate_left(17) ^ generation.rotate_left(19),
            Self::RootPublication { generation } => 0x40_0000_0000 ^ generation.rotate_left(23),
            Self::Singleton => 0x50_0000_0000,
        }
    }
}

fn mix_u64(digest: &mut u64, value: u64) {
    for byte in value.to_le_bytes() {
        *digest ^= u64::from(byte);
        *digest = digest.wrapping_mul(0x100000001b3);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use worth_store_physical_format::{
        PhysicalExtentId, PhysicalGeneration, PhysicalGenerationAuthority, PhysicalSegmentId,
    };

    #[test]
    fn top_level_record_extent_range_never_collapses_into_segment_owned_extent_range() {
        let generations = PhysicalGenerationAuthority::for_canonical_physical_format();
        let generation = PhysicalGeneration::from_raw(5).expect("generation");
        let extent = PhysicalExtentId::from_raw(13).expect("extent");
        let record_owner = generations
            .record_extent_cell(extent)
            .with_extent_generation(generation)
            .owner();
        let segment_owner = generations
            .extent_cell(PhysicalSegmentId::from_raw(1).expect("segment"), extent)
            .with_extent_generation(generation)
            .owner();
        let (record_family, coordinate) =
            range_coordinate_for_owner(record_owner).expect("record extent range");
        let (segment_family, _) =
            range_coordinate_for_owner(segment_owner).expect("segment extent range");
        let ranges = ProtectedReferenceRangeSet {
            ranges: vec![ProtectedReferenceRange::new(
                record_family,
                coordinate,
                coordinate,
            )],
        };

        assert_ne!(record_family, segment_family);
        assert!(ranges.contains_owner(record_owner));
        assert!(!ranges.contains_owner(segment_owner));
    }
}
