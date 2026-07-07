use schema::facade::platform::authority::touched_graph_parity_closeout::TouchedGraphParityFamilyKind;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum LookupConsumedVerticalSliceDisplacedSurfaceDisposition {
    DeletedNow,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct LookupConsumedVerticalSliceDisplacedSurfaceRow {
    current_surface: &'static str,
    current_path: &'static str,
    family_kind: TouchedGraphParityFamilyKind,
    owner: &'static str,
    blocker: &'static str,
    removal_trigger: &'static str,
    disposition: LookupConsumedVerticalSliceDisplacedSurfaceDisposition,
}

const DISPLACED_SURFACES: &[LookupConsumedVerticalSliceDisplacedSurfaceRow] = &[
    LookupConsumedVerticalSliceDisplacedSurfaceRow {
        current_surface: "current_worth_workload_ordinary_consumer_batch_execution_receipt direct evidence-lookup conflict-input lowering",
        current_path: "crates/worth-kernel/src/workload_composition/worth_workload/ordinary_consumer_sweep/current_cutover_proof.rs",
        family_kind: TouchedGraphParityFamilyKind::CompiledProductReuse,
        owner: "worth-kernel",
        blocker: "the direct ordinary route reopened conflict input from handoff plus receipt without first resolving the typed compiled-product reuse posture",
        removal_trigger: "phase 10 vertical slice caller now imports the packet-backed lookup-consumed cutover lane instead of the direct with_evidence_lookup route",
        disposition: LookupConsumedVerticalSliceDisplacedSurfaceDisposition::DeletedNow,
    },
];

pub(crate) const fn current_lookup_consumed_vertical_slice_displaced_surfaces(
) -> &'static [LookupConsumedVerticalSliceDisplacedSurfaceRow] {
    DISPLACED_SURFACES
}

impl LookupConsumedVerticalSliceDisplacedSurfaceRow {
    pub(crate) const fn current_surface(self) -> &'static str {
        self.current_surface
    }

    pub(crate) const fn current_path(self) -> &'static str {
        self.current_path
    }

    pub(crate) const fn family_kind(self) -> TouchedGraphParityFamilyKind {
        self.family_kind
    }

    pub(crate) const fn owner(self) -> &'static str {
        self.owner
    }

    pub(crate) const fn blocker(self) -> &'static str {
        self.blocker
    }

    pub(crate) const fn removal_trigger(self) -> &'static str {
        self.removal_trigger
    }

    pub(crate) const fn disposition(
        self,
    ) -> LookupConsumedVerticalSliceDisplacedSurfaceDisposition {
        self.disposition
    }
}
