use worth_query::facade::{AdvisoryIdentityCandidateSet, SingularIdentityContinuityResult};

fn main() {
    let _: fn(AdvisoryIdentityCandidateSet) -> SingularIdentityContinuityResult =
        AdvisoryIdentityCandidateSet::promote_to_authoritative_continuity;
}
