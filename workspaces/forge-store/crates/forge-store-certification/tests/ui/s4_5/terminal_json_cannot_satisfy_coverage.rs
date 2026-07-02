use forge_store_physical_certification::GeneratedCoverageMatrix;

fn requires_generated_coverage(_: GeneratedCoverageMatrix) {}

fn main() {
    requires_generated_coverage(serde_json::json!({
        "surface": "terminal-only"
    }));
}
