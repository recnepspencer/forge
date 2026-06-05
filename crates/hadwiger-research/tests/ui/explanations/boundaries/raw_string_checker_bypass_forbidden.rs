use hadwiger_research::facade::ExplainRejectionRequest;

fn main() {
    let _ = ExplainRejectionRequest::for_checker_rejection("id", "graph", "checker");
}
