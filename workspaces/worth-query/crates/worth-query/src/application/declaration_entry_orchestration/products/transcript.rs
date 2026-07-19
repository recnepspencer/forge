use crate::application::{
    WorthQueryDeclarationBridgeAuthorityAspectSummary,
    WorthQueryDeclarationEntryOrchestrationCostPosture,
    WorthQueryDeclarationEntryOrchestrationMaterializationPolicy,
    WorthQueryDeclarationEntryOrchestrationPlan,
    WorthQueryDeclarationEntryOrchestrationStageRecord, WorthQueryDeclarationInput,
    WorthQueryDeclarationRelationalAuthorityAspectSummary,
    WorthQueryDeclarationSignalAuthorityAspectSummary, WorthQueryDomainEntryMarker,
};
use crate::identity::hash_parts;

macro_rules! define_product_transcript {
    ($proof:ident, $transcript:ident, $outcome:ty) => {
        pub struct $proof<D: WorthQueryDomainEntryMarker, I: WorthQueryDeclarationInput<D>> {
            plan: WorthQueryDeclarationEntryOrchestrationPlan<D, I>,
            outcome: $outcome,
            step_records: Vec<WorthQueryDeclarationEntryOrchestrationStageRecord>,
            orchestration_digest: String,
        }

        impl<D: WorthQueryDomainEntryMarker, I: WorthQueryDeclarationInput<D>> $proof<D, I> {
            pub(crate) fn new(
                plan: WorthQueryDeclarationEntryOrchestrationPlan<D, I>,
                outcome: $outcome,
                step_records: Vec<WorthQueryDeclarationEntryOrchestrationStageRecord>,
                outcome_identity: String,
            ) -> Self {
                let orchestration_digest = hash_parts(&[
                    format!("plan:{}", plan.orchestration_identity_digest()),
                    format!("outcome:{outcome_identity}"),
                    format!(
                        "steps:{}",
                        step_records
                            .iter()
                            .map(|record| {
                                format!(
                                    "{}:{}:{}:{}",
                                    record.stage().as_str(),
                                    record.disposition().as_str(),
                                    record.retained_digest().unwrap_or("none"),
                                    record.reason().unwrap_or("none")
                                )
                            })
                            .collect::<Vec<_>>()
                            .join("|")
                    ),
                ]);
                Self {
                    plan,
                    outcome,
                    step_records,
                    orchestration_digest,
                }
            }

            pub fn plan(&self) -> &WorthQueryDeclarationEntryOrchestrationPlan<D, I> {
                &self.plan
            }

            pub fn outcome(&self) -> &$outcome {
                &self.outcome
            }

            pub fn step_records(&self) -> &[WorthQueryDeclarationEntryOrchestrationStageRecord] {
                &self.step_records
            }

            pub fn orchestration_digest(&self) -> &str {
                &self.orchestration_digest
            }

            pub fn materialization_policy(
                &self,
            ) -> &WorthQueryDeclarationEntryOrchestrationMaterializationPolicy {
                self.plan.materialization_policy()
            }

            pub fn cost_posture(&self) -> WorthQueryDeclarationEntryOrchestrationCostPosture {
                self.plan.cost_posture()
            }

            pub fn relational_authority_summary(
                &self,
            ) -> &WorthQueryDeclarationRelationalAuthorityAspectSummary {
                self.plan.relational_authority_summary()
            }

            pub fn bridge_authority_summary(
                &self,
            ) -> &WorthQueryDeclarationBridgeAuthorityAspectSummary {
                self.plan.bridge_authority_summary()
            }

            pub fn signal_authority_summary(
                &self,
            ) -> &WorthQueryDeclarationSignalAuthorityAspectSummary {
                self.plan.signal_authority_summary()
            }
        }

        pub type $transcript<D, I> = $proof<D, I>;
    };
}

define_product_transcript!(
    WorthQueryDeclarationRouteOrchestrationProof,
    WorthQueryDeclarationRouteOrchestrationTranscript,
    crate::application::WorthQueryDeclarationRoutePlanChecked<D, I>
);
define_product_transcript!(
    WorthQueryDeclarationReceiptOrchestrationProof,
    WorthQueryDeclarationReceiptOrchestrationTranscript,
    crate::application::WorthQueryDeclarationReceiptChecked<D, I>
);
define_product_transcript!(
    WorthQueryDeclarationEnvelopeOrchestrationProof,
    WorthQueryDeclarationEnvelopeOrchestrationTranscript,
    crate::application::WorthQueryDeclarationEnvelopeChecked<D, I>
);
