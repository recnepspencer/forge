use hadwiger_research::facade::{
    HadwigerCheckerCausalEvidence, HadwigerQueryDeclarationReference,
    WholePlaneColoringConstruction, WholePlaneColoringVerification,
};

fn bypass(
    artifact: &WholePlaneColoringConstruction,
    reference: HadwigerQueryDeclarationReference,
    evidence: HadwigerCheckerCausalEvidence,
) {
    let _ = WholePlaneColoringVerification::admitted(artifact, reference, evidence);
}

fn main() {
    let _ = bypass;
}
