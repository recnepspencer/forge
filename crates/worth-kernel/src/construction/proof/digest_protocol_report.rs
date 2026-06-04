use crate::construction::digest::{digest_owned_parts_with_scope, ConstructionDigestScope};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PrimitiveConstructionDigestProtocolReport {
    version_prefix: &'static str,
    algorithm: &'static str,
    scopes: &'static [&'static str],
    report_digest: String,
}

impl PrimitiveConstructionDigestProtocolReport {
    pub fn version_prefix(&self) -> &'static str {
        self.version_prefix
    }

    pub fn report_digest(&self) -> &str {
        &self.report_digest
    }
}

pub fn prepare_primitive_construction_digest_protocol_report(
) -> PrimitiveConstructionDigestProtocolReport {
    let version_prefix = "worth-primitives-digest:v1";
    let algorithm = "sha256";
    let scopes = &[
        "artifact-identity",
        "geometry-identity",
        "witness-identity",
        "contract-identity",
    ];
    let report_digest = digest_owned_parts_with_scope(
        ConstructionDigestScope::ArtifactIdentity,
        &scopes
            .iter()
            .map(|scope| (*scope).to_string())
            .chain([version_prefix.to_string(), algorithm.to_string()])
            .collect::<Vec<_>>(),
    );
    PrimitiveConstructionDigestProtocolReport {
        version_prefix,
        algorithm,
        scopes,
        report_digest,
    }
}
