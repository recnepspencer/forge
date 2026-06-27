use forge_proof::TransitionOutcome;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StoreJsonFixtureBoundaryDenial {
    TerminalProjectionJsonRequiresTerminalProjectionSuite,
    HostileReadmissionJsonRequiresHostileReadmissionSuite,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StoreTerminalProjectionJsonFixtureBoundaryWitness {
    _sealed: (),
}

impl StoreTerminalProjectionJsonFixtureBoundaryWitness {
    const fn new() -> Self {
        Self { _sealed: () }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StoreHostileReadmissionJsonFixtureBoundaryWitness {
    _sealed: (),
}

impl StoreHostileReadmissionJsonFixtureBoundaryWitness {
    const fn new() -> Self {
        Self { _sealed: () }
    }
}

pub type StoreTerminalProjectionJsonFixtureBoundaryOutcome = TransitionOutcome<
    StoreTerminalProjectionJsonFixtureBoundaryWitness,
    StoreJsonFixtureBoundaryDenial,
>;
pub type StoreHostileReadmissionJsonFixtureBoundaryOutcome = TransitionOutcome<
    StoreHostileReadmissionJsonFixtureBoundaryWitness,
    StoreJsonFixtureBoundaryDenial,
>;

#[track_caller]
pub(crate) fn require_terminal_projection_boundary(
) -> StoreTerminalProjectionJsonFixtureBoundaryOutcome {
    let caller = std::panic::Location::caller();
    if path_names_terminal_projection_boundary(caller.file()) {
        TransitionOutcome::success(StoreTerminalProjectionJsonFixtureBoundaryWitness::new())
    } else {
        TransitionOutcome::denied(
            StoreJsonFixtureBoundaryDenial::TerminalProjectionJsonRequiresTerminalProjectionSuite,
        )
    }
}

#[track_caller]
pub(crate) fn require_hostile_readmission_boundary(
) -> StoreHostileReadmissionJsonFixtureBoundaryOutcome {
    let caller = std::panic::Location::caller();
    if path_names_hostile_readmission_boundary(caller.file()) {
        TransitionOutcome::success(StoreHostileReadmissionJsonFixtureBoundaryWitness::new())
    } else {
        TransitionOutcome::denied(
            StoreJsonFixtureBoundaryDenial::HostileReadmissionJsonRequiresHostileReadmissionSuite,
        )
    }
}

fn path_names_terminal_projection_boundary(path: &str) -> bool {
    path.contains("terminal_projection")
}

fn path_names_hostile_readmission_boundary(path: &str) -> bool {
    path.contains("hostile_readmission")
}
