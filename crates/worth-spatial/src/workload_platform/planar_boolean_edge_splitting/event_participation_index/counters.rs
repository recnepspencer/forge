#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PlanarBooleanSplitEventParticipationCounters {
    carriers_indexed: usize,
    point_event_references: usize,
    interval_event_references: usize,
    event_group_references: usize,
    rejected_orphan_references: usize,
    duplicate_references_collapsed: usize,
}

impl PlanarBooleanSplitEventParticipationCounters {
    pub(crate) fn new(
        carriers_indexed: usize,
        point_event_references: usize,
        interval_event_references: usize,
        event_group_references: usize,
        rejected_orphan_references: usize,
        duplicate_references_collapsed: usize,
    ) -> Self {
        Self {
            carriers_indexed,
            point_event_references,
            interval_event_references,
            event_group_references,
            rejected_orphan_references,
            duplicate_references_collapsed,
        }
    }

    pub fn carriers_indexed(self) -> usize {
        self.carriers_indexed
    }

    pub fn point_event_references(self) -> usize {
        self.point_event_references
    }

    pub fn interval_event_references(self) -> usize {
        self.interval_event_references
    }

    pub fn event_group_references(self) -> usize {
        self.event_group_references
    }

    pub fn rejected_orphan_references(self) -> usize {
        self.rejected_orphan_references
    }

    pub fn duplicate_references_collapsed(self) -> usize {
        self.duplicate_references_collapsed
    }
}
