use super::S7ColdPlacementState;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColdPosturePermit {
    ImmediatePublication,
    Movement,
    Denied,
}

pub const fn classify_cold_posture_permit(state: S7ColdPlacementState) -> ColdPosturePermit {
    if state.permits_movement() {
        ColdPosturePermit::Movement
    } else if state.permits_immediate_publication() {
        ColdPosturePermit::ImmediatePublication
    } else {
        ColdPosturePermit::Denied
    }
}
