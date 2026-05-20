use forge_query::facade::{
    ForgeQueryLowerRuntimeCertificationBundle, ForgeQueryLowerRuntimeCertificationOutputDigest,
    ForgeQueryLowerRuntimeCertificationRow,
};

fn main() {
    let rows: Vec<ForgeQueryLowerRuntimeCertificationRow> = Vec::new();
    let outputs: Vec<ForgeQueryLowerRuntimeCertificationOutputDigest> = Vec::new();
    let _ = ForgeQueryLowerRuntimeCertificationBundle::new(rows, outputs);
}
