use forge_signal::facade::SignalBranchBasisArtifact;

fn require_current_branch_basis(_: SignalBranchBasisArtifact) {}

fn main() {
    let digest: &str = "signal-branch-basis-digest";
    require_current_branch_basis(digest);
}
