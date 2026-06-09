#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PrimitiveConstructionDigestProtocolReport {
    version_prefix: &'static str,
    algorithm: &'static str,
    scopes: &'static [&'static str],
}

impl PrimitiveConstructionDigestProtocolReport {
    pub fn version_prefix(&self) -> &'static str {
        self.version_prefix
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
    PrimitiveConstructionDigestProtocolReport {
        version_prefix,
        algorithm,
        scopes,
    }
}
