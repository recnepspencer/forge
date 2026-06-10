use super::super::support::*;
use super::proof_support::*;

#[test]
fn public_api_contract_transcript_and_support_report_emit_canonical_evidence_tokens() {
    let runtime = stateful_bridge_task_runtime();
    let contract = runtime.public_api_contract();
    assert_canonical_evidence_identity_token(contract.contract_digest());
    assert_eq!(
        contract.contract_digest(),
        compose_public_api_contract_identity(&contract).as_str()
    );
    for family in contract.families() {
        assert_canonical_evidence_identity_token(family.contract_digest());
        assert_eq!(
            family.contract_digest(),
            compose_public_api_family_contract_identity(family).as_str()
        );
    }

    let transcript = ForgeQueryRuntimePublicApiTranscriptEvidence::new(
        "workflow|editor",
        contract.contract_digest(),
        "state|digest",
        "live:digest",
        "computed|digest",
        "effect:digest",
        "intent|receipt",
        "inspection:digest",
        ["denial|one", "denial:two"],
        3,
        "lane|digest",
        7,
    );
    assert_canonical_evidence_identity_token(transcript.transcript_digest());
    assert_eq!(
        transcript.transcript_digest(),
        compose_runtime_public_api_transcript_identity(&transcript).as_str()
    );

    let report =
        crate::application::ForgeQueryApplicationFacade::runtime_backed_default().support_report();
    assert_canonical_evidence_identity_token(report.report_digest());
    assert_eq!(
        report.report_digest(),
        compose_support_report_identity(&report).as_str()
    );

    let query_disabled_report = crate::application::ForgeQueryApplicationFacade::new(
        crate::application::ForgeQueryConfig::runtime_backed_default()
            .with_query(crate::application::ForgeQueryQueryConfig::disabled())
            .with_signal(crate::application::ForgeQuerySignalConfig::disabled())
            .with_runtime_bridge(crate::application::ForgeQueryRuntimeBridgeConfig::disabled())
            .with_relational(crate::application::ForgeQueryRelationalConfig::disabled()),
    )
    .expect("query-disabled facade config should remain valid")
    .support_report();
    assert_canonical_evidence_identity_token(query_disabled_report.report_digest());
    assert_eq!(
        query_disabled_report.report_digest(),
        compose_support_report_identity(&query_disabled_report).as_str()
    );
    assert_ne!(
        report.report_digest(),
        query_disabled_report.report_digest()
    );
}

#[test]
fn phase_two_covered_surfaces_have_no_hash_parts_residue() {
    assert_phase_two_surface_has_no_hash_parts(include_str!("../../public_api.rs"));
    assert_phase_two_surface_has_no_hash_parts(include_str!("../../public_api_transcript.rs"));
    assert_phase_two_surface_has_no_hash_parts(include_str!("../../support/profile.rs"));
    assert_phase_two_surface_has_no_hash_parts(include_str!(
        "../../../application/support/report.rs"
    ));
    assert_phase_two_surface_has_no_hash_parts(include_str!("../../support_matrix.rs"));
    assert_phase_two_surface_has_no_hash_parts(include_str!("../../state_snapshot.rs"));
}
