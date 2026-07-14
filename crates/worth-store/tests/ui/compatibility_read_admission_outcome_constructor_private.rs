use worth_store::CompatibilityReadAdmissionOutcome;

fn main() {
    let _ = CompatibilityReadAdmissionOutcome::accepted(unreachable!(), unreachable!());
}
