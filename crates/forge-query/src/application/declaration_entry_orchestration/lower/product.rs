use crate::application::{
    checked_route_plan_from_progressed_with_profile, forge_query_checked_declaration_envelope,
    forge_query_checked_declaration_receipt_with_materialized_profile,
    receipt_materialized_profile_for_tier, ForgeQueryAdmittedDeclarationProgression,
    ForgeQueryDeclarationEntryOrchestrationArtifactPolicy,
    ForgeQueryDeclarationEntryOrchestrationExposureLevel,
    ForgeQueryDeclarationEntryOrchestrationInput, ForgeQueryDeclarationEntryOrchestrationPlan,
    ForgeQueryDeclarationEntryOrchestrationProduct, ForgeQueryDeclarationEntryOrchestrationStage,
    ForgeQueryDeclarationEntryOrchestrationStageRecord, ForgeQueryDeclarationEnvelopeChecked,
    ForgeQueryDeclarationEnvelopeInput, ForgeQueryDeclarationInput,
    ForgeQueryDeclarationReceiptChecked, ForgeQueryDeclarationReceiptInput,
    ForgeQueryDeclarationRouteIntent, ForgeQueryDeclarationRoutePlanChecked,
    ForgeQueryDomainEntryMarker, ForgeQueryDomainOperatingContext,
};
use crate::target_binding::{resolve_admitted_progression_target, ForgeQueryBindingTargetWitness};

use super::super::artifacts::canonical_digest_token;
use super::super::materialization::foundational_materialization_tier;

pub(crate) enum ForgeQueryDeclarationEntryProductChecked<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
> {
    RoutePlan(ForgeQueryDeclarationRoutePlanChecked<D, I>),
    Receipt(ForgeQueryDeclarationReceiptChecked<D, I>),
    Envelope(ForgeQueryDeclarationEnvelopeChecked<D, I>),
}

pub(crate) struct ForgeQueryLoweredDeclarationEntryProductOrchestration<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
> {
    pub(crate) plan: ForgeQueryDeclarationEntryOrchestrationPlan<D, I>,
    pub(crate) checked: ForgeQueryDeclarationEntryProductChecked<D, I>,
    pub(crate) step_records: Vec<ForgeQueryDeclarationEntryOrchestrationStageRecord>,
}

pub(crate) fn forge_query_lower_declaration_entry_product_orchestration_from_progressed_on_handle<
    D: ForgeQueryDomainEntryMarker,
    C: ForgeQueryDomainOperatingContext<D>,
    I: ForgeQueryDeclarationInput<D>,
>(
    handle: &crate::application::ForgeQueryAdmittedConfiguredDomainHandle<D, C>,
    progressed: ForgeQueryAdmittedDeclarationProgression<D, I>,
    exposure_level: ForgeQueryDeclarationEntryOrchestrationExposureLevel,
    artifact_policy: ForgeQueryDeclarationEntryOrchestrationArtifactPolicy,
    product: ForgeQueryDeclarationEntryOrchestrationProduct,
    route_intent: Option<ForgeQueryDeclarationRouteIntent>,
) -> ForgeQueryLoweredDeclarationEntryProductOrchestration<D, I> {
    let resolved_progression = resolve_admitted_progression_target(progressed);
    let orchestration_input = ForgeQueryDeclarationEntryOrchestrationInput::new(
        handle.handle_identity_digest(),
        handle.operating_context_identity_digest(),
        exposure_level,
        artifact_policy,
    );
    let plan = ForgeQueryDeclarationEntryOrchestrationPlan::from_progressed(
        orchestration_input,
        product,
        route_intent,
    );
    let mut step_records = vec![
        ForgeQueryDeclarationEntryOrchestrationStageRecord::admitted(
            ForgeQueryDeclarationEntryOrchestrationStage::ProgressionResolved,
            Some(resolved_progression.target().binding_digest().to_string()),
        ),
    ];
    let route_checked = checked_route_plan_from_progressed_with_profile(
        handle,
        resolved_progression.into_progressed(),
        route_intent,
        plan.foundational_evidence_profile(),
    );
    step_records.push(foundational_record(&plan, &route_checked));
    let checked = match product {
        ForgeQueryDeclarationEntryOrchestrationProduct::RoutePlan => {
            append_route_stop(&mut step_records, &plan, &route_checked);
            ForgeQueryDeclarationEntryProductChecked::RoutePlan(route_checked)
        }
        ForgeQueryDeclarationEntryOrchestrationProduct::Receipt => {
            append_route_progress(&mut step_records, &plan, &route_checked);
            let receipt_checked = forge_query_checked_declaration_receipt_with_materialized_profile(
                ForgeQueryDeclarationReceiptInput::route_checked(route_checked),
                &receipt_materialized_profile_for_tier(plan.receipt_materialization_tier()),
            );
            append_receipt_stop(&mut step_records, &plan, &receipt_checked);
            ForgeQueryDeclarationEntryProductChecked::Receipt(receipt_checked)
        }
        ForgeQueryDeclarationEntryOrchestrationProduct::Envelope => {
            let receipt_checked = lower_receipt_checked(&mut step_records, &plan, route_checked);
            let envelope_checked =
                lower_envelope_checked(&mut step_records, &plan, receipt_checked);
            ForgeQueryDeclarationEntryProductChecked::Envelope(envelope_checked)
        }
    };
    ForgeQueryLoweredDeclarationEntryProductOrchestration {
        plan,
        checked,
        step_records,
    }
}

fn foundational_record<D, I>(
    plan: &ForgeQueryDeclarationEntryOrchestrationPlan<D, I>,
    checked: &ForgeQueryDeclarationRoutePlanChecked<D, I>,
) -> ForgeQueryDeclarationEntryOrchestrationStageRecord
where
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
{
    let evidence = route_foundational_evidence(checked);
    ForgeQueryDeclarationEntryOrchestrationStageRecord::automated(
        ForgeQueryDeclarationEntryOrchestrationStage::FoundationalDescribed,
        Some(canonical_digest_token(evidence.attachment_bundle_digest())),
    )
    .with_materialization_tier(foundational_materialization_tier(
        plan.foundational_evidence_profile(),
    ))
}

fn route_foundational_evidence<D, I>(
    checked: &ForgeQueryDeclarationRoutePlanChecked<D, I>,
) -> &crate::application::ForgeQueryDeclarationFoundationalEvidence<D, I>
where
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
{
    match checked {
        ForgeQueryDeclarationRoutePlanChecked::Planned(value) => value.foundational_evidence(),
        ForgeQueryDeclarationRoutePlanChecked::Deferred(value) => value.foundational_evidence(),
        ForgeQueryDeclarationRoutePlanChecked::Denied(value) => value.foundational_evidence(),
        ForgeQueryDeclarationRoutePlanChecked::Failed(value) => value.foundational_evidence(),
    }
}

fn lower_receipt_checked<D, I>(
    step_records: &mut Vec<ForgeQueryDeclarationEntryOrchestrationStageRecord>,
    plan: &ForgeQueryDeclarationEntryOrchestrationPlan<D, I>,
    route_checked: ForgeQueryDeclarationRoutePlanChecked<D, I>,
) -> ForgeQueryDeclarationReceiptChecked<D, I>
where
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
{
    append_route_progress(step_records, plan, &route_checked);
    let receipt_checked = forge_query_checked_declaration_receipt_with_materialized_profile(
        ForgeQueryDeclarationReceiptInput::route_checked(route_checked),
        &receipt_materialized_profile_for_tier(plan.receipt_materialization_tier()),
    );
    append_receipt_progress_or_stop(step_records, plan, &receipt_checked);
    receipt_checked
}

fn lower_envelope_checked<D, I>(
    step_records: &mut Vec<ForgeQueryDeclarationEntryOrchestrationStageRecord>,
    plan: &ForgeQueryDeclarationEntryOrchestrationPlan<D, I>,
    receipt_checked: ForgeQueryDeclarationReceiptChecked<D, I>,
) -> ForgeQueryDeclarationEnvelopeChecked<D, I>
where
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
{
    let envelope_checked = forge_query_checked_declaration_envelope(
        ForgeQueryDeclarationEnvelopeInput::receipt_checked(receipt_checked),
    );
    append_envelope_stop(step_records, plan, &envelope_checked);
    envelope_checked
}

fn append_route_stop<D, I>(
    step_records: &mut Vec<ForgeQueryDeclarationEntryOrchestrationStageRecord>,
    plan: &ForgeQueryDeclarationEntryOrchestrationPlan<D, I>,
    checked: &ForgeQueryDeclarationRoutePlanChecked<D, I>,
) where
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
{
    let record = match checked {
        ForgeQueryDeclarationRoutePlanChecked::Planned(value) => {
            ForgeQueryDeclarationEntryOrchestrationStageRecord::terminal_success(
                ForgeQueryDeclarationEntryOrchestrationStage::RoutePlanned,
                Some(value.route_plan_digest().to_string()),
            )
        }
        ForgeQueryDeclarationRoutePlanChecked::Deferred(value) => {
            ForgeQueryDeclarationEntryOrchestrationStageRecord::deferred(
                ForgeQueryDeclarationEntryOrchestrationStage::RoutePlanned,
                None,
                value.reason(),
            )
        }
        ForgeQueryDeclarationRoutePlanChecked::Denied(value) => {
            ForgeQueryDeclarationEntryOrchestrationStageRecord::denied(
                ForgeQueryDeclarationEntryOrchestrationStage::RoutePlanned,
                None,
                value.reason(),
            )
        }
        ForgeQueryDeclarationRoutePlanChecked::Failed(value) => {
            ForgeQueryDeclarationEntryOrchestrationStageRecord::failed(
                ForgeQueryDeclarationEntryOrchestrationStage::RoutePlanned,
                None,
                value.reason(),
            )
        }
    };
    step_records.push(record.with_materialization_tier(plan.materialization_tier()));
}

fn append_route_progress<D, I>(
    step_records: &mut Vec<ForgeQueryDeclarationEntryOrchestrationStageRecord>,
    plan: &ForgeQueryDeclarationEntryOrchestrationPlan<D, I>,
    checked: &ForgeQueryDeclarationRoutePlanChecked<D, I>,
) where
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
{
    let digest = match checked {
        ForgeQueryDeclarationRoutePlanChecked::Planned(value) => {
            Some(value.route_plan_digest().to_string())
        }
        _ => None,
    };
    step_records.push(
        ForgeQueryDeclarationEntryOrchestrationStageRecord::automated(
            ForgeQueryDeclarationEntryOrchestrationStage::RoutePlanned,
            digest,
        )
        .with_materialization_tier(plan.materialization_tier()),
    );
}

fn append_receipt_stop<D, I>(
    step_records: &mut Vec<ForgeQueryDeclarationEntryOrchestrationStageRecord>,
    plan: &ForgeQueryDeclarationEntryOrchestrationPlan<D, I>,
    checked: &ForgeQueryDeclarationReceiptChecked<D, I>,
) where
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
{
    let record = match checked {
        ForgeQueryDeclarationReceiptChecked::Issued(value) => {
            ForgeQueryDeclarationEntryOrchestrationStageRecord::terminal_success(
                ForgeQueryDeclarationEntryOrchestrationStage::ReceiptIssued,
                Some(format!("{:?}", value.receipt_digest())),
            )
        }
        ForgeQueryDeclarationReceiptChecked::Deferred(value) => {
            ForgeQueryDeclarationEntryOrchestrationStageRecord::deferred(
                ForgeQueryDeclarationEntryOrchestrationStage::ReceiptIssued,
                Some(format!("{:?}", value.receipt().receipt_digest())),
                value.reason(),
            )
        }
        ForgeQueryDeclarationReceiptChecked::Denied(value) => {
            ForgeQueryDeclarationEntryOrchestrationStageRecord::denied(
                ForgeQueryDeclarationEntryOrchestrationStage::ReceiptIssued,
                Some(format!("{:?}", value.receipt().receipt_digest())),
                value.reason(),
            )
        }
        ForgeQueryDeclarationReceiptChecked::Failed(value) => {
            ForgeQueryDeclarationEntryOrchestrationStageRecord::failed(
                ForgeQueryDeclarationEntryOrchestrationStage::ReceiptIssued,
                Some(format!("{:?}", value.receipt().receipt_digest())),
                value.reason(),
            )
        }
    };
    step_records.push(record.with_materialization_tier(plan.receipt_materialization_tier()));
}

fn append_receipt_progress_or_stop<D, I>(
    step_records: &mut Vec<ForgeQueryDeclarationEntryOrchestrationStageRecord>,
    plan: &ForgeQueryDeclarationEntryOrchestrationPlan<D, I>,
    checked: &ForgeQueryDeclarationReceiptChecked<D, I>,
) where
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
{
    let digest = match checked {
        ForgeQueryDeclarationReceiptChecked::Issued(value) => {
            Some(format!("{:?}", value.receipt_digest()))
        }
        ForgeQueryDeclarationReceiptChecked::Deferred(value) => {
            Some(format!("{:?}", value.receipt().receipt_digest()))
        }
        ForgeQueryDeclarationReceiptChecked::Denied(value) => {
            Some(format!("{:?}", value.receipt().receipt_digest()))
        }
        ForgeQueryDeclarationReceiptChecked::Failed(value) => {
            Some(format!("{:?}", value.receipt().receipt_digest()))
        }
    };
    step_records.push(
        ForgeQueryDeclarationEntryOrchestrationStageRecord::automated(
            ForgeQueryDeclarationEntryOrchestrationStage::ReceiptIssued,
            digest,
        )
        .with_materialization_tier(plan.receipt_materialization_tier()),
    );
}

fn append_envelope_stop<D, I>(
    step_records: &mut Vec<ForgeQueryDeclarationEntryOrchestrationStageRecord>,
    plan: &ForgeQueryDeclarationEntryOrchestrationPlan<D, I>,
    checked: &ForgeQueryDeclarationEnvelopeChecked<D, I>,
) where
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
{
    let record = match checked {
        ForgeQueryDeclarationEnvelopeChecked::Enveloped(value) => {
            ForgeQueryDeclarationEntryOrchestrationStageRecord::terminal_success(
                ForgeQueryDeclarationEntryOrchestrationStage::EnvelopeConstructed,
                Some(format!("{:?}", value.envelope_digest())),
            )
        }
        ForgeQueryDeclarationEnvelopeChecked::Deferred(value) => {
            ForgeQueryDeclarationEntryOrchestrationStageRecord::deferred(
                ForgeQueryDeclarationEntryOrchestrationStage::EnvelopeConstructed,
                Some(format!("{:?}", value.envelope().envelope_digest())),
                value.reason(),
            )
        }
        ForgeQueryDeclarationEnvelopeChecked::Denied(value) => {
            ForgeQueryDeclarationEntryOrchestrationStageRecord::denied(
                ForgeQueryDeclarationEntryOrchestrationStage::EnvelopeConstructed,
                Some(format!("{:?}", value.envelope().envelope_digest())),
                value.reason(),
            )
        }
        ForgeQueryDeclarationEnvelopeChecked::Failed(value) => {
            ForgeQueryDeclarationEntryOrchestrationStageRecord::failed(
                ForgeQueryDeclarationEntryOrchestrationStage::EnvelopeConstructed,
                Some(format!("{:?}", value.envelope().envelope_digest())),
                value.reason(),
            )
        }
    };
    step_records.push(record.with_materialization_tier(plan.envelope_materialization_tier()));
}
