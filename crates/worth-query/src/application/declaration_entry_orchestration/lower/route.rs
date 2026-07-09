use crate::application::{
    receipt_materialized_profile_for_tier,
    worth_query_checked_declaration_receipt_with_materialized_profile,
    WorthQueryDeclarationEntryOrchestrationOutcome, WorthQueryDeclarationEntryOrchestrationPlan,
    WorthQueryDeclarationEntryOrchestrationRefusal, WorthQueryDeclarationEntryOrchestrationStage,
    WorthQueryDeclarationEntryOrchestrationStageRecord, WorthQueryDeclarationInput,
    WorthQueryDeclarationReceiptInput, WorthQueryDeclarationRoutePlanChecked,
    WorthQueryDeclarationRoutePlanDenialCause, WorthQueryDomainEntryMarker,
};

use super::super::sequencing::{
    WorthQueryDeclarationEntryOrchestrationAutomationContext,
    WorthQueryDeclarationEntryOrchestrationAutomationRefusal,
    WorthQueryDeclarationEntryOrchestrationAutomationRefusalClass,
};
use super::receipt::lower_from_receipt_checked;

pub(super) fn lower_from_route_checked<
    D: WorthQueryDomainEntryMarker,
    C: crate::application::WorthQueryDomainOperatingContext<D>,
    I: WorthQueryDeclarationInput<D>,
>(
    handle: &crate::application::WorthQueryAdmittedConfiguredDomainHandle<D, C>,
    orchestration_plan: &WorthQueryDeclarationEntryOrchestrationPlan<D, I>,
    automation_context: &WorthQueryDeclarationEntryOrchestrationAutomationContext<'_>,
    step_records: &mut Vec<WorthQueryDeclarationEntryOrchestrationStageRecord>,
    checked: WorthQueryDeclarationRoutePlanChecked<D, I>,
) -> WorthQueryDeclarationEntryOrchestrationOutcome<D, I> {
    match checked {
        WorthQueryDeclarationRoutePlanChecked::Planned(plan) => {
            if plan.automation_requires_explicit_handoff() {
                let route_stop = WorthQueryDeclarationEntryOrchestrationStage::RoutePlanned;
                let digest = Some(plan.route_plan_digest().to_string());
                step_records.push(
                    WorthQueryDeclarationEntryOrchestrationStageRecord::explicit_for_caller(
                        route_stop,
                        digest.clone(),
                        plan.explain().route_contract_reason(),
                    ),
                );
                return WorthQueryDeclarationEntryOrchestrationOutcome::Refused(
                    WorthQueryDeclarationEntryOrchestrationRefusal::from_automation(
                        WorthQueryDeclarationEntryOrchestrationAutomationRefusal::new(
                            WorthQueryDeclarationEntryOrchestrationAutomationRefusalClass::ExpensiveAutomationForbidden,
                            route_stop,
                            plan.explain().route_contract_reason(),
                            plan.declaration_family_key(),
                            digest,
                            automation_context.orchestration_identity_digest(),
                            automation_context.automation_boundary(),
                        ),
                        route_stop,
                    ),
                );
            }

            step_records.push(
                WorthQueryDeclarationEntryOrchestrationStageRecord::automated(
                    WorthQueryDeclarationEntryOrchestrationStage::RoutePlanned,
                    Some(plan.route_plan_digest().to_string()),
                ),
            );
            lower_from_receipt_checked(
                handle,
                orchestration_plan,
                automation_context,
                step_records,
                worth_query_checked_declaration_receipt_with_materialized_profile(
                    WorthQueryDeclarationReceiptInput::planned(plan),
                    &receipt_materialized_profile_for_tier(
                        orchestration_plan.receipt_materialization_tier(),
                    ),
                    orchestration_plan.receipt_materialization_tier(),
                ),
            )
        }
        WorthQueryDeclarationRoutePlanChecked::Deferred(plan) => {
            step_records.push(
                WorthQueryDeclarationEntryOrchestrationStageRecord::automated(
                    WorthQueryDeclarationEntryOrchestrationStage::RoutePlanned,
                    None,
                ),
            );
            lower_from_receipt_checked(
                handle,
                orchestration_plan,
                automation_context,
                step_records,
                worth_query_checked_declaration_receipt_with_materialized_profile(
                    WorthQueryDeclarationReceiptInput::deferred(plan),
                    &receipt_materialized_profile_for_tier(
                        orchestration_plan.receipt_materialization_tier(),
                    ),
                    orchestration_plan.receipt_materialization_tier(),
                ),
            )
        }
        WorthQueryDeclarationRoutePlanChecked::Denied(plan) => {
            let route_stop = WorthQueryDeclarationEntryOrchestrationStage::RoutePlanned;
            let automation_refusal_class = match plan.cause() {
                WorthQueryDeclarationRoutePlanDenialCause::IntentRequired => {
                    Some(WorthQueryDeclarationEntryOrchestrationAutomationRefusalClass::ExplicitIntentRequired)
                }
                _ => None,
            };

            if let Some(refusal_class) = automation_refusal_class {
                step_records.push(WorthQueryDeclarationEntryOrchestrationStageRecord::refused(
                    route_stop,
                    None,
                    plan.reason(),
                ));
                return WorthQueryDeclarationEntryOrchestrationOutcome::Refused(
                    WorthQueryDeclarationEntryOrchestrationRefusal::from_automation(
                        WorthQueryDeclarationEntryOrchestrationAutomationRefusal::new(
                            refusal_class,
                            route_stop,
                            plan.reason(),
                            plan.declaration_family_key(),
                            None,
                            automation_context.orchestration_identity_digest(),
                            automation_context.automation_boundary(),
                        ),
                        route_stop,
                    ),
                );
            }

            step_records.push(
                WorthQueryDeclarationEntryOrchestrationStageRecord::automated(route_stop, None),
            );
            lower_from_receipt_checked(
                handle,
                orchestration_plan,
                automation_context,
                step_records,
                worth_query_checked_declaration_receipt_with_materialized_profile(
                    WorthQueryDeclarationReceiptInput::denied(plan),
                    &receipt_materialized_profile_for_tier(
                        orchestration_plan.receipt_materialization_tier(),
                    ),
                    orchestration_plan.receipt_materialization_tier(),
                ),
            )
        }
        WorthQueryDeclarationRoutePlanChecked::Failed(plan) => {
            step_records.push(
                WorthQueryDeclarationEntryOrchestrationStageRecord::automated(
                    WorthQueryDeclarationEntryOrchestrationStage::RoutePlanned,
                    None,
                ),
            );
            lower_from_receipt_checked(
                handle,
                orchestration_plan,
                automation_context,
                step_records,
                worth_query_checked_declaration_receipt_with_materialized_profile(
                    WorthQueryDeclarationReceiptInput::failed(plan),
                    &receipt_materialized_profile_for_tier(
                        orchestration_plan.receipt_materialization_tier(),
                    ),
                    orchestration_plan.receipt_materialization_tier(),
                ),
            )
        }
    }
}
