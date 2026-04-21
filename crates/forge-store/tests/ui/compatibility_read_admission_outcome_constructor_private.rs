use forge_store::CompatibilityReadAdmissionOutcome;

fn main() {
    let _ = CompatibilityReadAdmissionOutcome::accepted(unreachable!(), unreachable!());
}
