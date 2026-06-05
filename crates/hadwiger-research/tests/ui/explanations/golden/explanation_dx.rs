use hadwiger_research::facade::{
    explain_partial_admission, explain_query_recovery_brief, explain_rejection,
    ExplainPartialAdmissionRequest, ExplainRejectionRequest, GraphVersion,
    HadwigerCanonicalArtifact, HadwigerExplanationError, HadwigerProofClaimAdmissionError,
    HadwigerQueryRecoveryExplanationRequest, HadwigerResearchHandle,
    UnitDistanceVerificationChecked,
};

fn explanation_dx(
    handle: &HadwigerResearchHandle,
    graph_version: &GraphVersion,
    unit_checked: &UnitDistanceVerificationChecked,
    blocked: HadwigerProofClaimAdmissionError,
) -> Result<(), HadwigerExplanationError> {
    let rejection = explain_rejection(
        handle,
        ExplainRejectionRequest::for_checker_rejection(
            "bad-unit-distance-edge",
            graph_version,
            unit_checked.verification(),
        )
        .with_rejected_aspect(unit_checked.unit_distance_aspect())
        .with_repair_obligation("repair exact unit-distance coordinates")?,
    )?;
    assert!(!rejection.admits_theorem_authority());

    if let HadwigerProofClaimAdmissionError::Blocked(blocked_claim) = blocked {
        let partial = explain_partial_admission(
            handle,
            ExplainPartialAdmissionRequest::from_blocked_proof_claim(
                "partial",
                graph_version,
                &blocked_claim,
            )
            .with_surviving_artifact(graph_version.reference())
            .with_repair_obligation("supply admitted checker evidence")?,
        )?;
        assert!(!partial.admits_theorem_authority());
    }

    let outcome = hadwiger_research::facade::orchestrate_research_request_entry(
        handle,
        hadwiger_research::facade::RejectionExplanationDeclaration::new("candidate-a", "bad"),
    );
    if let Some(recovery) =
        hadwiger_research::facade::recover_research_stop_from_outcome(handle, &outcome)
    {
        let query_explanation = explain_query_recovery_brief(
            handle,
            HadwigerQueryRecoveryExplanationRequest::new("query", recovery),
        )?;
        assert!(query_explanation.is_query_owned());
    }
    Ok(())
}

fn main() {}
