use hadwiger_research::facade::{
    ResearchGraphInvariantFamily, ResearchGraphInvariantObligation,
};

fn main() {
    let _ = ResearchGraphInvariantObligation::unchecked(
        ResearchGraphInvariantFamily::FailureResidency,
        "hadwiger.research_graph.failure",
    );
}
