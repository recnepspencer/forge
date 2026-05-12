use super::{
    admit_mutation_preparation_basis_intent, admit_observation_basis_intent,
    query_basis_lifecycle_support_report, BasisCapabilityAdmission, BasisLaneSupportStatus,
    BasisOperationLaneRequest, BasisScopedAdmissionDenial, NormalizedBasisFamily, RawBasisIntent,
};
use std::collections::BTreeSet;

#[test]
fn support_report_covers_every_family_with_supported_and_denied_rows() {
    let report = query_basis_lifecycle_support_report();

    for family in [
        NormalizedBasisFamily::CurrentHead,
        NormalizedBasisFamily::BranchHead,
        NormalizedBasisFamily::BranchSnapshot,
        NormalizedBasisFamily::RuntimeSnapshot,
        NormalizedBasisFamily::HistoricalSnapshot,
        NormalizedBasisFamily::HistoricalCommit,
        NormalizedBasisFamily::Preview,
        NormalizedBasisFamily::PreviewDerivedHistorical,
    ] {
        assert!(
            report.rows().iter().any(|row| {
                row.family() == &family
                    && matches!(
                        row.status(),
                        BasisLaneSupportStatus::Admitted | BasisLaneSupportStatus::Advisory
                    )
            }),
            "expected supported row for {family:?}"
        );
        assert!(
            report.rows().iter().any(
                |row| row.family() == &family && row.status() == BasisLaneSupportStatus::Denied
            ),
            "expected denied row for {family:?}"
        );
    }
}

#[test]
fn support_report_covers_each_family_lane_pair_exactly_once() {
    let report = query_basis_lifecycle_support_report();
    let mut seen = BTreeSet::new();

    for row in report.rows() {
        let key = format!(
            "{}::{}",
            row.family().as_str(),
            row.operation_lane().as_str()
        );
        assert!(seen.insert(key), "duplicate support row detected");
    }

    assert_eq!(report.rows().len(), 8 * 9);
}

#[test]
fn support_report_matches_public_common_path_execution_for_every_row() {
    let report = query_basis_lifecycle_support_report();

    for row in report.rows() {
        let intent = raw_intent_for(row.family(), row.operation_lane().clone());
        let actual_status = match execute_public_common_path(intent) {
            Ok(BasisCapabilityAdmission::Admitted(_)) => BasisLaneSupportStatus::Admitted,
            Ok(BasisCapabilityAdmission::Advisory(_)) => BasisLaneSupportStatus::Advisory,
            Err(_) => BasisLaneSupportStatus::Denied,
        };

        assert_eq!(
            row.status(),
            actual_status,
            "support row drifted from public common path for {}:{}",
            row.family().as_str(),
            row.operation_lane().as_str()
        );
    }
}

#[test]
fn support_report_marks_preview_mutation_preparation_as_preview_drifted() {
    let report = query_basis_lifecycle_support_report();
    let row = report
        .rows()
        .iter()
        .find(|row| {
            row.family() == &NormalizedBasisFamily::Preview
                && row.operation_lane() == &BasisOperationLaneRequest::MutationPreparation
        })
        .expect("preview mutation-preparation row should exist");

    assert_eq!(row.status(), BasisLaneSupportStatus::Denied);
    assert_eq!(row.denial_label(), Some("preview_drifted"));
}

fn raw_intent_for(
    family: &NormalizedBasisFamily,
    lane: BasisOperationLaneRequest,
) -> RawBasisIntent {
    match family {
        NormalizedBasisFamily::CurrentHead => RawBasisIntent::current_head(lane),
        NormalizedBasisFamily::BranchHead => RawBasisIntent::branch_head("branch:main", lane),
        NormalizedBasisFamily::BranchSnapshot => {
            RawBasisIntent::branch_snapshot("branch:main", "snapshot:1", lane)
        }
        NormalizedBasisFamily::RuntimeSnapshot => {
            RawBasisIntent::runtime_snapshot("runtime:snapshot:1", lane)
        }
        NormalizedBasisFamily::HistoricalSnapshot => {
            RawBasisIntent::historical_snapshot("history:snapshot:1", lane)
        }
        NormalizedBasisFamily::HistoricalCommit => {
            RawBasisIntent::historical_commit("commit:1", lane)
        }
        NormalizedBasisFamily::Preview => RawBasisIntent::preview("preview:session-1", lane),
        NormalizedBasisFamily::PreviewDerivedHistorical => {
            RawBasisIntent::preview_derived_historical("preview:session-1", lane)
        }
    }
}

fn execute_public_common_path(
    intent: RawBasisIntent,
) -> Result<BasisCapabilityAdmission, BasisScopedAdmissionDenial> {
    match intent.operation_lane() {
        BasisOperationLaneRequest::Observation => {
            admit_observation_basis_intent(intent).map(|capability| capability.admission().clone())
        }
        BasisOperationLaneRequest::MutationPreparation => {
            admit_mutation_preparation_basis_intent(intent)
                .map(|capability| capability.admission().clone())
        }
        BasisOperationLaneRequest::Replay => super::admit_replay_basis_intent(intent)
            .map(|capability| capability.admission().clone()),
        BasisOperationLaneRequest::Inspection => super::admit_inspection_basis_intent(intent)
            .map(|capability| capability.admission().clone()),
        BasisOperationLaneRequest::Materialization => {
            super::admit_materialization_basis_intent(intent)
                .map(|capability| capability.admission().clone())
        }
        BasisOperationLaneRequest::SubscriptionDeclaration => {
            super::admit_subscription_declaration_basis_intent(intent)
                .map(|capability| capability.admission().clone())
        }
        BasisOperationLaneRequest::SubscriptionActivation => {
            super::admit_subscription_activation_basis_intent(intent)
                .map(|capability| capability.admission().clone())
        }
        BasisOperationLaneRequest::PreviewCloseout => {
            super::admit_preview_closeout_basis_intent(intent)
                .map(|capability| capability.admission().clone())
        }
        BasisOperationLaneRequest::Certification => super::admit_certification_basis_intent(intent)
            .map(|capability| capability.admission().clone()),
    }
}

#[test]
fn common_path_observation_admits_without_manual_phase_assembly() {
    let capability = admit_observation_basis_intent(RawBasisIntent::current_head(
        BasisOperationLaneRequest::Observation,
    ))
    .expect("observation common path should admit current head");

    match capability.admission() {
        BasisCapabilityAdmission::Admitted(admitted) => {
            assert_eq!(
                admitted.operation_lane(),
                &BasisOperationLaneRequest::Observation
            );
        }
        other => panic!("unexpected capability admission: {other:?}"),
    }
}

#[test]
fn common_path_mutation_preparation_denies_preview_before_capability_construction() {
    let denial = admit_mutation_preparation_basis_intent(RawBasisIntent::preview(
        "preview:session-1",
        BasisOperationLaneRequest::MutationPreparation,
    ))
    .expect_err("preview mutation preparation should deny at the common path");

    match denial {
        BasisScopedAdmissionDenial::Eligibility(denied) => {
            assert_eq!(denied.trace().rule_label(), "preview_authority_lane_denied");
        }
        other => panic!("unexpected denial path: {other:?}"),
    }
}
