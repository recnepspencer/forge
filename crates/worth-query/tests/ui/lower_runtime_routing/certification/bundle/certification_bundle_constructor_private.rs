use worth_query::facade::certification::{WorthQueryLowerRuntimeCertificationBundle, WorthQueryLowerRuntimeCertificationOutputDigest, WorthQueryLowerRuntimeCertificationRow};

fn main() {
    let rows: Vec<WorthQueryLowerRuntimeCertificationRow> = Vec::new();
    let outputs: Vec<WorthQueryLowerRuntimeCertificationOutputDigest> = Vec::new();
    let _ = WorthQueryLowerRuntimeCertificationBundle::new(rows, outputs);
}
