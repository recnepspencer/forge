use super::{
    deny_query_descriptor_digest_as_spatial_evidence_lookup_authority,
    SpatialEvidenceLookupDenialKind,
};

#[test]
fn query_descriptor_digest_is_rejected_as_spatial_lookup_authority() {
    let denial =
        deny_query_descriptor_digest_as_spatial_evidence_lookup_authority("forge-query-digest");

    assert_eq!(
        denial.kind(),
        SpatialEvidenceLookupDenialKind::QueryDescriptorDigestSubstitution
    );
    assert!(
        denial
            .detail()
            .contains("cannot construct spatial evidence lookup authority"),
        "denial must name the authority boundary"
    );
}
