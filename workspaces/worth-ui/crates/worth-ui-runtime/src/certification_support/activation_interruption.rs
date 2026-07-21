use std::cell::Cell;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiActivationPrecommitStage {
    AllocationDeltaClosure,
    CatalogActivationPreparation,
    LoweringAuthority,
    TopologyAssembly,
    FrameBoundarySource,
    ExecutionBundleSeal,
    QuerySuccession,
    InvalidationRead,
    ActivationInputValidation,
    PortalBindingValidation,
    GraphPredecessorCheck,
    ScrollBindingCheck,
    CatalogTransitionRead,
    CatalogTransitionPreparation,
    FrameBoundaryCheck,
    LedgerPredecessorCheck,
    CommittedPreflight,
    FrameReplacementCheck,
    InvalidationWrite,
    LedgerCommitPreparation,
    FrameCommitPreparation,
}

impl WorthUiActivationPrecommitStage {
    pub const ALL: [Self; 21] = [
        Self::AllocationDeltaClosure,
        Self::CatalogActivationPreparation,
        Self::LoweringAuthority,
        Self::TopologyAssembly,
        Self::FrameBoundarySource,
        Self::ExecutionBundleSeal,
        Self::QuerySuccession,
        Self::InvalidationRead,
        Self::ActivationInputValidation,
        Self::PortalBindingValidation,
        Self::GraphPredecessorCheck,
        Self::ScrollBindingCheck,
        Self::CatalogTransitionRead,
        Self::CatalogTransitionPreparation,
        Self::FrameBoundaryCheck,
        Self::LedgerPredecessorCheck,
        Self::CommittedPreflight,
        Self::FrameReplacementCheck,
        Self::InvalidationWrite,
        Self::LedgerCommitPreparation,
        Self::FrameCommitPreparation,
    ];

    pub fn label(self) -> &'static str {
        match self {
            Self::AllocationDeltaClosure => "allocation delta closure",
            Self::CatalogActivationPreparation => "catalog activation preparation",
            Self::LoweringAuthority => "lowering authority",
            Self::TopologyAssembly => "topology assembly",
            Self::FrameBoundarySource => "frame boundary source",
            Self::ExecutionBundleSeal => "execution bundle seal",
            Self::QuerySuccession => "Query succession",
            Self::InvalidationRead => "invalidation read",
            Self::ActivationInputValidation => "activation input validation",
            Self::PortalBindingValidation => "portal binding validation",
            Self::GraphPredecessorCheck => "graph predecessor check",
            Self::ScrollBindingCheck => "scroll binding check",
            Self::CatalogTransitionRead => "catalog transition read",
            Self::CatalogTransitionPreparation => "catalog transition preparation",
            Self::FrameBoundaryCheck => "frame boundary check",
            Self::LedgerPredecessorCheck => "ledger predecessor check",
            Self::CommittedPreflight => "committed preflight",
            Self::FrameReplacementCheck => "frame replacement check",
            Self::InvalidationWrite => "invalidation write",
            Self::LedgerCommitPreparation => "ledger commit preparation",
            Self::FrameCommitPreparation => "frame commit preparation",
        }
    }
}

thread_local! {
    static ARMED: Cell<Option<WorthUiActivationPrecommitStage>> = const { Cell::new(None) };
    static OBSERVED: Cell<Option<WorthUiActivationPrecommitStage>> = const { Cell::new(None) };
}

pub fn with_activation_precommit_interruption<Result>(
    stage: WorthUiActivationPrecommitStage,
    action: impl FnOnce() -> Result,
) -> (Result, Option<WorthUiActivationPrecommitStage>) {
    let prior = ARMED.replace(Some(stage));
    assert!(
        prior.is_none(),
        "activation interruption scopes cannot nest"
    );
    OBSERVED.set(None);
    let reset = ActivationInterruptionReset;
    let result = action();
    let observed = OBSERVED.get();
    drop(reset);
    (result, observed)
}

pub(crate) fn interrupt_if_armed(label: &'static str) -> bool {
    let stage = WorthUiActivationPrecommitStage::ALL
        .into_iter()
        .find(|stage| stage.label() == label)
        .unwrap_or_else(|| panic!("unknown activation precommit stage: {label}"));
    if ARMED.get() == Some(stage) {
        OBSERVED.set(Some(stage));
        true
    } else {
        false
    }
}

struct ActivationInterruptionReset;

impl Drop for ActivationInterruptionReset {
    fn drop(&mut self) {
        ARMED.set(None);
        OBSERVED.set(None);
    }
}
