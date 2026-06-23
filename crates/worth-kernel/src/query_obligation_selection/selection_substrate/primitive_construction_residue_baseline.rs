use super::primitive_construction_contract::query_primitive_construction_family_cardinality_closeout;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QueryPrimitiveConstructionResidueBaseline {
    version: &'static str,
    rows: Vec<QueryPrimitiveConstructionResidueBaselineRow>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QueryPrimitiveConstructionResidueBaselineRow {
    class: &'static str,
    owner: &'static str,
    introduced_in: &'static str,
    current_count: usize,
    must_not_exceed_count: usize,
    blocker: &'static str,
    removal_trigger: &'static str,
}

pub fn query_primitive_construction_residue_baseline_v1(
) -> QueryPrimitiveConstructionResidueBaseline {
    let family_count_gap =
        query_primitive_construction_family_cardinality_closeout().missing_family_count();
    QueryPrimitiveConstructionResidueBaseline {
        version: "touched-graph-milestone-5-primitive-residue-v1",
        rows: vec![
            QueryPrimitiveConstructionResidueBaselineRow {
                class: "kernel-handoff-only-result-helper",
                owner: "worth-kernel primitive construction result surface",
                introduced_in: "forge-query-9.9-phase-18",
                current_count: 1,
                must_not_exceed_count: 1,
                blocker: "legacy tests still exercise handoff-only prepared results without a workspace-backed compose execution",
                removal_trigger: "all construction result helpers require a workspace-backed executed compose artifact or move behind a compatibility-only test helper",
            },
            QueryPrimitiveConstructionResidueBaselineRow {
                class: "kernel-motion-preflight-sequencing",
                owner: "worth-kernel motion construction support",
                introduced_in: "forge-query-9.9-phase-18",
                current_count: 1,
                must_not_exceed_count: 1,
                blocker: "motion compound-lowering support now returns typed spatial denials but is not yet represented as its own graph-obligation preflight registration",
                removal_trigger: "motion branch-preview sequencing is represented as a typed preflight graph obligation with denial evidence",
            },
            QueryPrimitiveConstructionResidueBaselineRow {
                class: "kernel-primitive-family-cardinality-gap",
                owner: "worth-kernel primitive construction family inventory",
                introduced_in: "forge-query-9.9-phase-18",
                current_count: family_count_gap,
                must_not_exceed_count: family_count_gap,
                blocker: "phase 18 certification language names seven primitive birth families while the current kernel request enum exposes six",
                removal_trigger: "the spec and kernel primitive family inventory agree, or a seventh family is added and covered by compose execution",
            },
        ],
    }
}

impl QueryPrimitiveConstructionResidueBaseline {
    pub const fn version(&self) -> &'static str {
        self.version
    }

    pub fn rows(&self) -> &[QueryPrimitiveConstructionResidueBaselineRow] {
        &self.rows
    }
}

impl QueryPrimitiveConstructionResidueBaselineRow {
    pub const fn class(&self) -> &'static str {
        self.class
    }

    pub const fn owner(&self) -> &'static str {
        self.owner
    }

    pub const fn introduced_in(&self) -> &'static str {
        self.introduced_in
    }

    pub const fn current_count(&self) -> usize {
        self.current_count
    }

    pub const fn must_not_exceed_count(&self) -> usize {
        self.must_not_exceed_count
    }

    pub const fn blocker(&self) -> &'static str {
        self.blocker
    }

    pub const fn removal_trigger(&self) -> &'static str {
        self.removal_trigger
    }
}
