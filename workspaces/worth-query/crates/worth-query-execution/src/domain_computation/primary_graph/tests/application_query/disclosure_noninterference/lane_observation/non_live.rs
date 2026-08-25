use super::super::super::super::fixture::{
    admit_touch_account_capability, governed_live_account_parameters, IdentityExecutionSchema,
};
use super::{
    buffer_limit, capture_lane, one, ContinuationObservation, GovernedObservationContext,
    GraphWorkOccurrences, LaneObservation,
};
use crate::domain_computation::primary_graph::{
    WorthQueryApplicationPreviewSession, WorthQueryApplicationQueryAccessContext,
    WorthQueryApplicationQueryControls, WorthQueryApplicationQueryResumeControls,
};

pub(super) fn observe_one_shot(
    context: &GovernedObservationContext<'_>,
    occurrences: &mut GraphWorkOccurrences,
) -> LaneObservation {
    let capability =
        admit_touch_account_capability(context.world, context.principal, context.request).unwrap();
    let access = WorthQueryApplicationQueryAccessContext::new(context.principal, context.account);
    let plan = context
        .world
        .application
        .admit_governed_application_query(
            context.query,
            &access,
            capability,
            governed_live_account_parameters("account-1"),
            WorthQueryApplicationQueryControls::current_one_shot(
                one(),
                buffer_limit(),
                context.request,
            ),
        )
        .unwrap();
    let result = context
        .world
        .application
        .execute_application_query_one_shot(plan)
        .unwrap();
    capture_lane(result.rows().to_vec(), result.receipt(), occurrences)
}

pub(super) fn observe_continuation(
    context: &GovernedObservationContext<'_>,
    occurrences: &mut GraphWorkOccurrences,
) -> ContinuationObservation {
    let capability =
        admit_touch_account_capability(context.world, context.principal, context.request).unwrap();
    let access = WorthQueryApplicationQueryAccessContext::new(context.principal, context.account);
    let plan = context
        .world
        .application
        .admit_governed_application_query(
            context.query,
            &access,
            capability,
            governed_live_account_parameters("account-1"),
            WorthQueryApplicationQueryControls::current_continuation_page(
                one(),
                buffer_limit(),
                context.request,
            ),
        )
        .unwrap();
    let page = context
        .world
        .application
        .execute_application_query_continuation_page(plan)
        .unwrap();
    let (rows, continuation, receipt) = page.into_parts();
    let first = capture_lane(rows, &receipt, occurrences);
    let continuation = continuation.expect("the first activity page must continue");
    let first_next_page_ordinal = continuation.page_ordinal();

    let fresh =
        admit_touch_account_capability(context.world, context.principal, context.request).unwrap();
    let plan = context
        .world
        .application
        .readmit_governed_application_query_continuation(
            context.query,
            &access,
            fresh,
            governed_live_account_parameters("account-1"),
            continuation,
            WorthQueryApplicationQueryResumeControls::new(one(), buffer_limit(), context.request),
        )
        .unwrap();
    let page = context
        .world
        .application
        .execute_application_query_continuation_page(plan)
        .unwrap();
    let (rows, continuation, receipt) = page.into_parts();
    ContinuationObservation {
        first,
        first_next_page_ordinal,
        second: capture_lane(rows, &receipt, occurrences),
        second_has_continuation: continuation.is_some(),
    }
}

pub(super) fn observe_historical(
    context: &GovernedObservationContext<'_>,
    _commit: u64,
    occurrences: &mut GraphWorkOccurrences,
) -> LaneObservation {
    let basis = context
        .world
        .application
        .admit_application_historical_basis(
            crate::domain_computation::primary_graph::WorthQueryApplicationHistoricalRead::current_for_test(&context.world.application),
            context.request,
        )
        .unwrap();
    let capability =
        admit_touch_account_capability(context.world, context.principal, context.request).unwrap();
    let access = WorthQueryApplicationQueryAccessContext::new(context.principal, context.account);
    let plan = context
        .world
        .application
        .admit_governed_application_query(
            context.query,
            &access,
            capability,
            governed_live_account_parameters("account-1"),
            WorthQueryApplicationQueryControls::historical(
                basis,
                one(),
                buffer_limit(),
                context.request,
            ),
        )
        .unwrap();
    let result = context
        .world
        .application
        .execute_application_query_historical(plan)
        .unwrap();
    capture_lane(result.rows().to_vec(), result.receipt(), occurrences)
}

pub(super) fn observe_preview(
    context: &GovernedObservationContext<'_>,
    session: &WorthQueryApplicationPreviewSession<IdentityExecutionSchema>,
    occurrences: &mut GraphWorkOccurrences,
) -> LaneObservation {
    let basis = context
        .world
        .application
        .admit_application_preview_basis(session, context.request)
        .unwrap();
    let capability =
        admit_touch_account_capability(context.world, context.principal, context.request).unwrap();
    let access = WorthQueryApplicationQueryAccessContext::new(context.principal, context.account);
    let plan = context
        .world
        .application
        .admit_governed_application_query(
            context.query,
            &access,
            capability,
            governed_live_account_parameters("account-1"),
            WorthQueryApplicationQueryControls::preview(
                basis,
                one(),
                buffer_limit(),
                context.request,
            ),
        )
        .unwrap();
    let result = context
        .world
        .application
        .execute_application_query_preview(plan)
        .unwrap();
    capture_lane(result.rows().to_vec(), result.receipt(), occurrences)
}
