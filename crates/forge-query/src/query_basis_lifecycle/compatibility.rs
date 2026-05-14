use crate::identity::hash_parts;
use crate::query_context::{QueryBasisContextRequest, QueryContextFamily};

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
            RawBasisIntent::branch_head(request.declared_basis_label(), operation_lane)
        }
        QueryContextFamily::HistoricalSnapshot => {
            RawBasisIntent::historical_snapshot(request.declared_basis_label(), operation_lane)
        }
        QueryContextFamily::HistoricalCommit => {
            RawBasisIntent::historical_commit(request.declared_basis_label(), operation_lane)
        }
        QueryContextFamily::PreviewDerivedHistorical => RawBasisIntent::preview_derived_historical(
            request.declared_basis_label(),
            operation_lane,
        ),
        QueryContextFamily::DiffComparison => return Err(diff_comparison_denial(operation_lane)),
    };
    Ok(intent.with_source_path(RawBasisSourcePath::QueryContextCompatibility))
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
        hash_parts(&[
            "compatibility_family:diff_comparison".to_string(),
            format!("operation_lane:{}", operation_lane.as_str()),
            format!(
                "source_path:{}",
                RawBasisSourcePath::QueryContextCompatibility.as_str()
            ),
        ]),
        RawBasisSourcePath::QueryContextCompatibility,
        operation_lane,
        "diff_comparison",
        "forge_query::query_context",
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
                assert_eq!(owner, &"forge_query::query_context");
            }
            other => panic!("unexpected denial kind: {other:?}"),
        }
        assert_eq!(
            denial.source_path(),
            &RawBasisSourcePath::QueryContextCompatibility
        );
    }
}
