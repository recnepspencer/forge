use crate::application::{
    ForgeQueryDeclarationEntryOrchestrationCostPosture,
    ForgeQueryDeclarationEntryOrchestrationMaterializationPolicy,
    ForgeQueryDeclarationEntryOrchestrationPlan,
    ForgeQueryDeclarationEntryOrchestrationStageRecord, ForgeQueryDeclarationInput,
    ForgeQueryDomainEntryMarker,
};
use crate::identity::hash_parts;

macro_rules! define_product_transcript {
    ($proof:ident, $transcript:ident, $outcome:ty) => {
        pub struct $proof<D: ForgeQueryDomainEntryMarker, I: ForgeQueryDeclarationInput<D>> {
            plan: ForgeQueryDeclarationEntryOrchestrationPlan<D, I>,
            outcome: $outcome,
            step_records: Vec<ForgeQueryDeclarationEntryOrchestrationStageRecord>,
            orchestration_digest: String,
        }

        impl<D: ForgeQueryDomainEntryMarker, I: ForgeQueryDeclarationInput<D>> $proof<D, I> {
            pub(crate) fn new(
                plan: ForgeQueryDeclarationEntryOrchestrationPlan<D, I>,
                outcome: $outcome,
                step_records: Vec<ForgeQueryDeclarationEntryOrchestrationStageRecord>,
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

            pub fn plan(&self) -> &ForgeQueryDeclarationEntryOrchestrationPlan<D, I> {
                &self.plan
            }

            pub fn outcome(&self) -> &$outcome {
                &self.outcome
            }

            pub fn step_records(&self) -> &[ForgeQueryDeclarationEntryOrchestrationStageRecord] {
                &self.step_records
            }

            pub fn orchestration_digest(&self) -> &str {
                &self.orchestration_digest
            }

            pub fn materialization_policy(
                &self,
            ) -> &ForgeQueryDeclarationEntryOrchestrationMaterializationPolicy {
                self.plan.materialization_policy()
            }

            pub fn cost_posture(&self) -> ForgeQueryDeclarationEntryOrchestrationCostPosture {
                self.plan.cost_posture()
            }
        }

        pub type $transcript<D, I> = $proof<D, I>;
    };
}

define_product_transcript!(
    ForgeQueryDeclarationRouteOrchestrationProof,
    ForgeQueryDeclarationRouteOrchestrationTranscript,
    crate::application::ForgeQueryDeclarationRoutePlanChecked<D, I>
);
define_product_transcript!(
    ForgeQueryDeclarationReceiptOrchestrationProof,
    ForgeQueryDeclarationReceiptOrchestrationTranscript,
    crate::application::ForgeQueryDeclarationReceiptChecked<D, I>
);
define_product_transcript!(
    ForgeQueryDeclarationEnvelopeOrchestrationProof,
    ForgeQueryDeclarationEnvelopeOrchestrationTranscript,
    crate::application::ForgeQueryDeclarationEnvelopeChecked<D, I>
);
