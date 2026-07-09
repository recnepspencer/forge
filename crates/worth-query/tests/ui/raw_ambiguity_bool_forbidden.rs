use worth_query::facade::CorrespondenceOutcome;

fn main() {
    let _: fn(bool) -> CorrespondenceOutcome = CorrespondenceOutcome::from_ambiguity_bool;
}
