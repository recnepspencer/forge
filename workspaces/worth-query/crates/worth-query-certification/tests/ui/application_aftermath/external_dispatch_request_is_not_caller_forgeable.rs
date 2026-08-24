use worth_query_host::facade::primary_graph::WorthQueryExternalDispatchRequest;
use worth_query_host::facade::domain::WorthQueryExternalEffectCorrelationFamily;
use worth_foundational::facade::{BoundaryProtocolIdentity, BoundaryProtocolVersion};

fn main() {
    let _forged = WorthQueryExternalDispatchRequest {
        correlation_family: &WorthQueryExternalEffectCorrelationFamily::new("replacement-family")
            .unwrap(),
        correlation_token: &[0; 32],
        effect: "ReplacementEffect",
        protocol_identity: &BoundaryProtocolIdentity::new("replacement.protocol"),
        protocol_version: BoundaryProtocolVersion::new(1),
        maximum_payload_bytes: 1,
        payload: &[0],
    };
}
