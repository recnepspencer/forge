use crate::evidence_identity::{
    worth_query_evidence_identity, WorthQueryEvidenceIdentity, WorthQueryEvidenceScope,
    WorthQueryEvidenceTag,
};
use crate::query_context::{QueryBasisContextRequest, QueryContextFamily};

use super::identity::basis_lifecycle_digest;
use super::intent::{BasisOperationLaneRequest, RawBasisIntent, RawBasisSourcePath};
use super::normalization::{
    normalize_raw_basis, unsupported_compatibility_family_denial, BasisIntentDenial,
    NormalizedBasisIntent,
};

pub fn try_raw_basis_intent_from_query_context_request(
    request: &QueryBasisContextRequest,
    operation_lane: BasisOperationLaneRequest,
) -> Result<RawBasisIntent, BasisIntentDenial> {
    let intent = match request.family() {
        QueryContextFamily::CurrentBranchHead => RawBasisIntent::current_head(operation_lane),
        QueryContextFamily::BranchHead => {
            RawBasisIntent::branch_head(compatibility_identity(request), operation_lane)
        }
        QueryContextFamily::HistoricalSnapshot => {
            RawBasisIntent::historical_snapshot(compatibility_identity(request), operation_lane)
        }
        QueryContextFamily::HistoricalCommit => {
            RawBasisIntent::historical_commit(compatibility_identity(request), operation_lane)
        }
        QueryContextFamily::PreviewDerivedHistorical => RawBasisIntent::preview_derived_historical(
            compatibility_identity(request),
            operation_lane,
        ),
        QueryContextFamily::DiffComparison => return Err(diff_comparison_denial(operation_lane)),
    };
    Ok(intent.with_source_path(RawBasisSourcePath::QueryContextCompatibility))
}

fn compatibility_identity(request: &QueryBasisContextRequest) -> WorthQueryEvidenceIdentity {
    worth_query_evidence_identity(WorthQueryEvidenceScope::QueryContextCompatibilityBasisLabel)
        .field_shape(
            WorthQueryEvidenceTag::new("family"),
            request.family().as_str(),
        )
        .field_value(
            WorthQueryEvidenceTag::new("declared_basis_label"),
            request.declared_basis_label(),
        )
        .seal()
}

pub fn normalize_query_context_request(
    request: &QueryBasisContextRequest,
    operation_lane: BasisOperationLaneRequest,
) -> Result<NormalizedBasisIntent, BasisIntentDenial> {
    normalize_raw_basis(try_raw_basis_intent_from_query_context_request(
        request,
        operation_lane,
    )?)
}

fn diff_comparison_denial(operation_lane: BasisOperationLaneRequest) -> BasisIntentDenial {
    unsupported_compatibility_family_denial(
        basis_lifecycle_digest(
            "basis_compatibility_diff_comparison_denial_v1",
            [
                ("compatibility_family", "diff_comparison".to_string()),
                ("operation_lane", operation_lane.as_str().to_string()),
                (
                    "source_path",
                    RawBasisSourcePath::QueryContextCompatibility
                        .as_str()
                        .to_string(),
                ),
            ],
        ),
        RawBasisSourcePath::QueryContextCompatibility,
        operation_lane,
        "diff_comparison",
        "worth_query::query_context",
    )
}

#[cfg(test)]
mod tests {
    use super::diff_comparison_denial;
    use crate::query_basis_lifecycle::{
        BasisIntentDenialKind, BasisOperationLaneRequest, RawBasisSourcePath,
    };

    #[test]
    fn diff_comparison_compatibility_denies_without_fabricating_historical_basis() {
        let denial = diff_comparison_denial(BasisOperationLaneRequest::Observation);

        match denial.kind() {
            BasisIntentDenialKind::UnsupportedCompatibilityFamily { family, owner } => {
                assert_eq!(family, &"diff_comparison");
                assert_eq!(owner, &"worth_query::query_context");
            }
            other => panic!("unexpected denial kind: {other:?}"),
        }
        assert_eq!(
            denial.source_path(),
            &RawBasisSourcePath::QueryContextCompatibility
        );
    }
}
