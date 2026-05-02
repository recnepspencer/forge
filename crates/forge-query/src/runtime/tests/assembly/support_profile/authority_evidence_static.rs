use super::super::super::support::*;

#[test]
fn static_authority_evidence_closeout_matches_runtime_method_for_support_profile() {
    let support_profile = ForgeQueryRuntimeSupportProfile::bridge_backed(
        "static-support-live",
        "static-support-preview",
        "static-support-inspect",
    );
    let runtime = bridge_runtime_with_support(support_profile.clone());
    let workspace = runtime
        .workspace("task.authority-evidence-static")
        .expect("task runtime should open a named workspace");

    let static_support =
        ForgeQueryRuntime::public_authoritative_mutation_evidence_support_for_posture(
            support_profile.posture(),
        );
    let static_closeout =
        ForgeQueryRuntime::public_authoritative_mutation_evidence_closeout_for_support_profile(
            &support_profile,
        );

    assert_eq!(
        static_support.support_digest(),
        workspace
            .public_authoritative_mutation_evidence_support()
            .support_digest()
    );
    assert_eq!(
        static_closeout.closeout_digest(),
        workspace
            .public_authoritative_mutation_evidence_closeout()
            .closeout_digest()
    );
}
