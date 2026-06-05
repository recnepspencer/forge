use hadwiger_research::facade::CandidateGraphDeclaration;

fn main() {
    let request = CandidateGraphDeclaration::new("candidate-a");
    let _ = request.progress_declaration();
}
