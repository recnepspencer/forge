use forge_store_recovery_physics::{AdmittedCrashBoundaryLayoutRule, RecoveryLayoutAccess};

fn main() {
    let forged = AdmittedCrashBoundaryLayoutRule { _private: () };
    let _ = RecoveryLayoutAccess::s8().crash_boundary_layout(&forged);
}
