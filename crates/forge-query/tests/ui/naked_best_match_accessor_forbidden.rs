use forge_query::facade::CorrespondenceOutcome;

fn main() {
    let _: fn(&CorrespondenceOutcome) -> &str = CorrespondenceOutcome::best_match;
}
