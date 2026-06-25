use forge_query::facade::consumer_kit::ForgeQueryGraphReadBypassClass;

pub(crate) fn graph_read_bypass_residue_cap_inventory() -> &'static [WorthGraphReadBypassResidueCap]
{
    GRAPH_READ_BYPASS_RESIDUE_CAPS
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct WorthGraphReadBypassResidueCap {
    class: ForgeQueryGraphReadBypassClass,
    owner: &'static str,
    introduced_in: &'static str,
    must_not_exceed_count: usize,
    blocker: &'static str,
    removal_trigger: &'static str,
}

impl WorthGraphReadBypassResidueCap {
    pub fn class(self) -> ForgeQueryGraphReadBypassClass {
        self.class
    }

    pub const fn owner(self) -> &'static str {
        self.owner
    }

    pub const fn introduced_in(self) -> &'static str {
        self.introduced_in
    }

    pub const fn must_not_exceed_count(self) -> usize {
        self.must_not_exceed_count
    }

    pub const fn blocker(self) -> &'static str {
        self.blocker
    }

    pub const fn removal_trigger(self) -> &'static str {
        self.removal_trigger
    }
}

const GRAPH_READ_BYPASS_RESIDUE_CAPS: &[WorthGraphReadBypassResidueCap] = &[
    residue_cap(
        ForgeQueryGraphReadBypassClass::ManualRelationRowLoop,
        5,
        "Milestone 7 declaration seeds must replace relation-row loops with Query access plans",
    ),
    residue_cap(
        ForgeQueryGraphReadBypassClass::PerNodeNeighborLookup,
        2,
        "Milestone 8 access-plan adoption must remove per-node neighbor lookups",
    ),
    residue_cap(
        ForgeQueryGraphReadBypassClass::AdHocAdjacencyMap,
        4,
        "Milestone 8 persistent access structures must replace local adjacency materialization",
    ),
    residue_cap(
        ForgeQueryGraphReadBypassClass::ManualVisitedSetTraversal,
        4,
        "Milestone 8 streaming frontier plans must replace local visited-set traversal",
    ),
    residue_cap(
        ForgeQueryGraphReadBypassClass::BroadBooleanGraphScan,
        3,
        "Milestone 8 boolean selectivity plans must replace broad relation filtering",
    ),
];

const fn residue_cap(
    class: ForgeQueryGraphReadBypassClass,
    must_not_exceed_count: usize,
    blocker: &'static str,
) -> WorthGraphReadBypassResidueCap {
    WorthGraphReadBypassResidueCap {
        class,
        owner: "worth-kernel",
        introduced_in: "Milestone 6 Phase 5 graph-read inventory bypass audit",
        must_not_exceed_count,
        blocker,
        removal_trigger:
            "the responsible milestone migrates this class into Query-owned graph-read access",
    }
}
