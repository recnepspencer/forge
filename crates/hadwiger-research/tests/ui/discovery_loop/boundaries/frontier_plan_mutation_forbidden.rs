use hadwiger_research::facade::DiscoveryFrontier;

fn mutate(frontier: &mut DiscoveryFrontier) {
    let _ = frontier.experiment_plans_mut();
}

fn main() {}
