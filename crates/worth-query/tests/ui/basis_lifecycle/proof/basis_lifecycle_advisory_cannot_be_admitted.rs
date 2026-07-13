use worth_query::facade::foundation::{BasisOperationLane, InspectionLaneWitness};
use worth_query::facade::{admit_basis_capability, evaluate_basis_inspection_advisory_eligibility, normalize_raw_basis_intent, RawBasisIntent};

fn main() {
    let normalized = normalize_raw_basis_intent(
        RawBasisIntent::PreviewDerived {
            preview_identity: "preview".to_string(),
            source_basis_identity: "branch".to_string(),
        },
        <InspectionLaneWitness as BasisOperationLane>::lane_name(),
    )
    .unwrap();

    let advisory = evaluate_basis_inspection_advisory_eligibility(normalized).unwrap();

    let _ = admit_basis_capability(advisory);
}
