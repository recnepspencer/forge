use hadwiger_research::facade::{
    AdvisoryNoteDeclaration, CandidateGraphDeclaration, ColorabilityDeclaration,
    EmbeddingDeclaration, LowerBoundWitnessDeclaration, PartialAdmissionExplanationDeclaration,
    RejectionExplanationDeclaration, UnitDistanceVerificationDeclaration,
    WholePlaneColoringConstructionDeclaration,
};

fn main() {
    let _candidate = CandidateGraphDeclaration::new("candidate-a").with_graph_version("v1");
    let _embedding = EmbeddingDeclaration::new("candidate-a", "embedding-interval-a");
    let _colorability = ColorabilityDeclaration::new("candidate-a", 5);
    let _witness =
        LowerBoundWitnessDeclaration::new("candidate-a", "embedding-interval-a", 5);
    let _advisory = AdvisoryNoteDeclaration::new("candidate-a", "motif looks reusable");
    let _rejection =
        RejectionExplanationDeclaration::new("candidate-a", "edge-length-failure");
    let _partial = PartialAdmissionExplanationDeclaration::new("candidate-a");
    let _unit_distance =
        UnitDistanceVerificationDeclaration::new("candidate-a", "embedding-interval-a");
    let _whole_plane =
        WholePlaneColoringConstructionDeclaration::new("hexagonal-seven-coloring", 7);
}
