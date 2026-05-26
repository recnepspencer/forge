use crate::application::{
    ForgeQueryDeclarationEntryOrchestrationOutcome, ForgeQueryDeclarationEntryOrchestrationRefusal,
    ForgeQueryDeclarationEntryOrchestrationStage,
    ForgeQueryDeclarationEntryOrchestrationStageRecord, ForgeQueryDeclarationInput,
    ForgeQueryDeclarationReceiptInput, ForgeQueryDeclarationRoutePlanChecked,
    ForgeQueryDeclarationRoutePlanDenialCause, ForgeQueryDomainEntryMarker,
};

use super::super::sequencing::{
    ForgeQueryDeclarationEntryOrchestrationAutomationContext,
    ForgeQueryDeclarationEntryOrchestrationAutomationRefusal,
    ForgeQueryDeclarationEntryOrchestrationAutomationRefusalClass,
};
use super::receipt::lower_from_receipt_checked;

pub(super) fn lower_from_route_checked<
    D: ForgeQueryDomainEntryMarker,
    C: crate::application::ForgeQueryDomainOperatingContext<D>,
    I: ForgeQueryDeclarationInput<D>,
>(
    handle: &crate::application::ForgeQueryAdmittedConfiguredDomainHandle<D, C>,
    automation_context: &ForgeQueryDeclarationEntryOrchestrationAutomationContext<'_>,
    step_records: &mut Vec<ForgeQueryDeclarationEntryOrchestrationStageRecord>,
    checked: ForgeQueryDeclarationRoutePlanChecked<D, I>,
) -> ForgeQueryDeclarationEntryOrchestrationOutcome<D, I> {
    match checked {
        ForgeQueryDeclarationRoutePlanChecked::Planned(plan) => {
            if plan.automation_requires_explicit_handoff() {
                let route_stop = ForgeQueryDeclarationEntryOrchestrationStage::RoutePlanned;
                let digest = Some(plan.route_plan_digest().to_string());
                step_records.push(
                    ForgeQueryDeclarationEntryOrchestrationStageRecord::explicit_for_caller(
                        route_stop,
                        digest.clone(),
                        plan.explain().route_contract_reason(),
                    ),
                );
                return ForgeQueryDeclarationEntryOrchestrationOutcome::Refused(
                    ForgeQueryDeclarationEntryOrchestrationRefusal::from_automation(
                        ForgeQueryDeclarationEntryOrchestrationAutomationRefusal::new(
                            ForgeQueryDeclarationEntryOrchestrationAutomationRefusalClass::ExpensiveAutomationForbidden,
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
                ForgeQueryDeclarationEntryOrchestrationStageRecord::automated(
                    ForgeQueryDeclarationEntryOrchestrationStage::RoutePlanned,
                    Some(plan.route_plan_digest().to_string()),
                ),
            );
            lower_from_receipt_checked(
                handle,
                automation_context,
                step_records,
                handle.receipt_routes_checked(ForgeQueryDeclarationReceiptInput::planned(plan)),
            )
        }
        ForgeQueryDeclarationRoutePlanChecked::Deferred(plan) => {
            step_records.push(
                ForgeQueryDeclarationEntryOrchestrationStageRecord::automated(
                    ForgeQueryDeclarationEntryOrchestrationStage::RoutePlanned,
                    None,
                ),
            );
            lower_from_receipt_checked(
                handle,
                automation_context,
                step_records,
                handle.receipt_routes_checked(ForgeQueryDeclarationReceiptInput::deferred(plan)),
            )
        }
        ForgeQueryDeclarationRoutePlanChecked::Denied(plan) => {
            let route_stop = ForgeQueryDeclarationEntryOrchestrationStage::RoutePlanned;
            let automation_refusal_class = match plan.cause() {
                ForgeQueryDeclarationRoutePlanDenialCause::IntentRequired => {
                    Some(ForgeQueryDeclarationEntryOrchestrationAutomationRefusalClass::ExplicitIntentRequired)
                }
                _ => None,
            };

            if let Some(refusal_class) = automation_refusal_class {
                step_records.push(ForgeQueryDeclarationEntryOrchestrationStageRecord::refused(
                    route_stop,
                    None,
                    plan.reason(),
                ));
                return ForgeQueryDeclarationEntryOrchestrationOutcome::Refused(
                    ForgeQueryDeclarationEntryOrchestrationRefusal::from_automation(
                        ForgeQueryDeclarationEntryOrchestrationAutomationRefusal::new(
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
                ForgeQueryDeclarationEntryOrchestrationStageRecord::automated(route_stop, None),
            );
            lower_from_receipt_checked(
                handle,
                automation_context,
                step_records,
                handle.receipt_routes_checked(ForgeQueryDeclarationReceiptInput::denied(plan)),
            )
        }
        ForgeQueryDeclarationRoutePlanChecked::Failed(plan) => {
            step_records.push(
                ForgeQueryDeclarationEntryOrchestrationStageRecord::automated(
                    ForgeQueryDeclarationEntryOrchestrationStage::RoutePlanned,
                    None,
                ),
            );
            lower_from_receipt_checked(
                handle,
                automation_context,
                step_records,
                handle.receipt_routes_checked(ForgeQueryDeclarationReceiptInput::failed(plan)),
            )
        }
    }
}
