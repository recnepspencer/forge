use forge_query::facade::{QueryContextExecutionArtifact, QueryDiffChangeSetArtifact};

fn expects_query_diff_artifact(_artifact: QueryDiffChangeSetArtifact) {}

fn main() {
    let execution =
        unsafe { std::mem::MaybeUninit::<QueryContextExecutionArtifact>::zeroed().assume_init() };
    expects_query_diff_artifact(execution);
}
