use worth_query::facade::foundation::CorrespondenceOutcome;

fn main() {
    let _: fn(&CorrespondenceOutcome) -> &str = CorrespondenceOutcome::best_match;
}
