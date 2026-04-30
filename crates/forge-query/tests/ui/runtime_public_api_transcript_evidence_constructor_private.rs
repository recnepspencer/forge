use forge_query::facade::ForgeQueryRuntimePublicApiTranscriptEvidence;

fn main() {
    let _evidence = ForgeQueryRuntimePublicApiTranscriptEvidence {
        transcript_family: String::new(),
        support_contract_digest: String::new(),
        state_digest: String::new(),
        live_surface_digest: String::new(),
        computed_surface_digest: String::new(),
        effect_surface_digest: String::new(),
        intent_receipt_digest: String::new(),
        inspection_digest: String::new(),
        unsupported_neighbor_denial_digests: Vec::new(),
        delivery_residue_count: 0,
        authority_lane_digest: String::new(),
        meaningful_assertion_count: 0,
        transcript_digest: String::new(),
    };
}
