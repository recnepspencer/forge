use super::super::later_milestone_claims::WorthGraphReadAccessLaterMilestoneClaims;
use super::super::{
    WorthGraphReadAccessMilestoneSixCloseout, WorthGraphReadAccessMilestoneSixError,
    WorthGraphReadAccessMilestoneSixErrorKind,
};
use super::current_inventory_closeout;
use crate::graph_read_access_inventory::{
    WorthGraphReadAccessPhaseSixError, WorthGraphReadAccessPhaseSixErrorKind,
};

#[test]
fn milestone_six_closeout_refuses_later_milestone_claims() {
    let closeout = WorthGraphReadAccessMilestoneSixCloseout::from_inventory_closeout(
        current_inventory_closeout(),
    )
    .expect("current inventory should produce final Milestone 6 closeout");

    assert!(!closeout.claims_query_declarations_complete());
    assert!(!closeout.claims_admitted_access_plans_complete());
    assert!(!closeout.claims_graph_read_receipts_complete());
    assert!(!closeout.claims_validator_derivation_complete());
    assert!(!closeout.claims_invalidation_complete());
    assert!(!closeout.claims_replay_complete());
    assert!(!closeout.claims_conflict_complete());
    assert!(!closeout.claims_cache_complete());
    assert!(!closeout.claims_public_diagnostics_complete());

    let error =
        WorthGraphReadAccessMilestoneSixCloseout::from_inventory_closeout_with_later_milestone_claims(
            current_inventory_closeout(),
            WorthGraphReadAccessLaterMilestoneClaims::with_query_declarations_complete(),
        )
        .expect_err("Milestone 6 closeout must reject Milestone 7 declaration completion claims");

    assert_eq!(
        error.kind(),
        WorthGraphReadAccessMilestoneSixErrorKind::LaterMilestoneClaimed
    );
}

#[test]
fn milestone_six_closeout_error_preserves_disposition_cause() {
    let error =
        WorthGraphReadAccessMilestoneSixError::from(WorthGraphReadAccessPhaseSixError::new(
            WorthGraphReadAccessPhaseSixErrorKind::MissingInventoryRowDisposition,
        ));

    assert_eq!(
        error.kind(),
        WorthGraphReadAccessMilestoneSixErrorKind::DispositionCloseoutFailed
    );
    assert_eq!(
        error.disposition_error_kind(),
        Some(WorthGraphReadAccessPhaseSixErrorKind::MissingInventoryRowDisposition)
    );
    assert_eq!(error.inventory_error_kind(), None);
}
