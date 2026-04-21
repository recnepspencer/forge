use forge_store::CompatibilityWriteAdmissionOutcome;

fn main() {
    let _ = CompatibilityWriteAdmissionOutcome::accepted(unreachable!(), unreachable!());
}
