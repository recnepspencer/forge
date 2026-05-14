use forge_foundational::FoundationalDiagnosticExplanationBundle;
use forge_foundational::FoundationalDiagnosticRow;

fn needs_explanation_bundle(_bundle: FoundationalDiagnosticExplanationBundle) {}

fn main() {
    let rows: Vec<FoundationalDiagnosticRow> = Vec::new();
    needs_explanation_bundle(rows);
}
