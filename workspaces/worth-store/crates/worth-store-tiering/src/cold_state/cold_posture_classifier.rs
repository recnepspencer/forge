use super::ColdPlacementState;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColdPosturePermit {
    ImmediatePublication,
    Movement,
    Denied,
}

pub const fn classify_cold_posture_permit(state: ColdPlacementState) -> ColdPosturePermit {
    if state.permits_movement() {
        ColdPosturePermit::Movement
    } else if state.permits_immediate_publication() {
        ColdPosturePermit::ImmediatePublication
    } else {
        ColdPosturePermit::Denied
    }
}
