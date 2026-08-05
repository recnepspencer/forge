use bank_domain::reads::EstateLegalAuthorityView;
use worth_query_host::facade::{
    domain::{WorthQueryCanonicalWorkEvidence, WorthQueryCanonicalWorkPhases},
    primary_graph::WorthQueryApplicationAuthorizationWorkEvidence,
};

use super::fixture::{request_scope, warm_locality_world, WarmLocalityAxis, ESTATE};
use crate::{queries, BankReadControls};

const EXPANDED_COUNT: usize = 8;

#[test]
fn every_named_population_axis_stays_out_of_warm_authorization_work() {
    let baseline = observe("warm-locality-baseline", WarmLocalityAxis::Fields, 0);
    for (scenario, axis) in [
        ("warm-locality-grants", WarmLocalityAxis::Grants),
        (
            "warm-locality-relationships",
            WarmLocalityAxis::Relationships,
        ),
        ("warm-locality-fields", WarmLocalityAxis::Fields),
        ("warm-locality-cases", WarmLocalityAxis::Cases),
    ] {
        let expanded = observe(scenario, axis, EXPANDED_COUNT);
        assert_eq!(baseline.authorization, expanded.authorization);
        assert_eq!(baseline.rows, expanded.rows);
        expanded.assert_exact_zero_warm_work();
    }

    let result_rows = observe(
        "warm-locality-result-rows",
        WarmLocalityAxis::ResultRows,
        EXPANDED_COUNT,
    );
    assert_eq!(baseline.authorization, result_rows.authorization);
    assert_eq!(result_rows.rows.len(), baseline.rows.len() + EXPANDED_COUNT);
    baseline.assert_exact_zero_warm_work();
    result_rows.assert_exact_zero_warm_work();
}

struct WarmLocalityObservation {
    authorization: WorthQueryApplicationAuthorizationWorkEvidence,
    canonical: WorthQueryCanonicalWorkPhases,
    fallback_count: usize,
    rows: Vec<EstateLegalAuthorityView>,
}

impl WarmLocalityObservation {
    fn assert_exact_zero_warm_work(&self) {
        assert_eq!(self.fallback_count, 0);
        assert_zero(self.authorization.canonical_work());
        for work in [
            self.canonical.execution(),
            self.canonical.provider_commit(),
            self.canonical.projection(),
            self.canonical.live_delivery(),
            self.canonical.retry_resolution(),
            self.canonical.recovery_inspection(),
            self.canonical.publication(),
        ] {
            assert_zero(work);
        }
    }
}

fn observe(scenario: &str, axis: WarmLocalityAxis, count: usize) -> WarmLocalityObservation {
    let fixture = warm_locality_world(scenario, axis, count);
    let principal = fixture.authenticate_reviewer();
    let result = fixture
        .runtime
        .query(queries::estate_legal_compliance(ESTATE))
        .as_principal(&principal)
        .controls(BankReadControls::current(request_scope(), 1, 50_000).unwrap())
        .execute()
        .expect("the legal-compliance query should remain authorized");
    let receipt = result.receipt();
    WarmLocalityObservation {
        authorization: receipt.authorization_work(),
        canonical: receipt.canonical_work(),
        fallback_count: receipt.fallback_count(),
        rows: result.rows()[0].authorities().to_vec(),
    }
}

fn assert_zero(work: WorthQueryCanonicalWorkEvidence) {
    assert_eq!(work.basis_preparations(), 0);
    assert_eq!(work.digest_derivations(), 0);
    assert_eq!(work.canonical_entries(), 0);
    assert_eq!(work.canonical_encoded_bytes(), 0);
    assert_eq!(work.canonical_material_allocation_bytes(), 0);
    assert_eq!(work.sha256_input_bytes(), 0);
    assert_eq!(work.sha256_compression_blocks(), 0);
    assert_eq!(work.digest_text_materializations(), 0);
}
