#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthWorkloadOrdinaryConsumerResidueSurface {
    PlanarBooleanLoopRuntimeRegistrationProof,
    BooleanChainIntegrationHandoff,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthWorkloadOrdinaryConsumerResidueBoundary {
    QueryProofAccompanimentOnly,
    ReplayUndoCloseoutOnly,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthWorkloadOrdinaryConsumerResidueRow {
    surface: WorthWorkloadOrdinaryConsumerResidueSurface,
    owner: &'static str,
    blocker: &'static str,
    removal_trigger: &'static str,
    boundary: WorthWorkloadOrdinaryConsumerResidueBoundary,
}

const ORDINARY_CONSUMER_RESIDUE_ROWS: &[WorthWorkloadOrdinaryConsumerResidueRow] = &[
    WorthWorkloadOrdinaryConsumerResidueRow {
        surface: WorthWorkloadOrdinaryConsumerResidueSurface::PlanarBooleanLoopRuntimeRegistrationProof,
        owner: "worth-kernel",
        blocker: "runtime registration proof remains Query-proof accompaniment and cannot authorize ordinary grouped conflict or batch admission",
        removal_trigger: "phase 12 firewall deletion removes runtime-registration accompaniment from ordinary-consumer accounting",
        boundary: WorthWorkloadOrdinaryConsumerResidueBoundary::QueryProofAccompanimentOnly,
    },
    WorthWorkloadOrdinaryConsumerResidueRow {
        surface: WorthWorkloadOrdinaryConsumerResidueSurface::BooleanChainIntegrationHandoff,
        owner: "worth-kernel",
        blocker: "boolean chain integration remains replay/undo closeout assembly and cannot act as selected-plan or batch-execution authority",
        removal_trigger: "phase 12 firewall deletion replaces chain handoff accounting with proof-only closeout artifacts",
        boundary: WorthWorkloadOrdinaryConsumerResidueBoundary::ReplayUndoCloseoutOnly,
    },
];

pub const fn worth_workload_ordinary_consumer_residue_rows(
) -> &'static [WorthWorkloadOrdinaryConsumerResidueRow] {
    ORDINARY_CONSUMER_RESIDUE_ROWS
}

impl WorthWorkloadOrdinaryConsumerResidueRow {
    pub const fn surface(self) -> WorthWorkloadOrdinaryConsumerResidueSurface {
        self.surface
    }

    pub const fn owner(self) -> &'static str {
        self.owner
    }

    pub const fn blocker(self) -> &'static str {
        self.blocker
    }

    pub const fn removal_trigger(self) -> &'static str {
        self.removal_trigger
    }

    pub const fn boundary(self) -> WorthWorkloadOrdinaryConsumerResidueBoundary {
        self.boundary
    }
}

impl WorthWorkloadOrdinaryConsumerResidueSurface {
    pub const fn surface_name(self) -> &'static str {
        match self {
            Self::PlanarBooleanLoopRuntimeRegistrationProof => {
                "PlanarBooleanLoopRuntimeRegistrationProof"
            }
            Self::BooleanChainIntegrationHandoff => "BooleanChainIntegrationHandoff",
        }
    }
}
