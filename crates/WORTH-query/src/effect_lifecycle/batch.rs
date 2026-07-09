use crate::basis_lifecycle::BasisFamily;
use crate::workflow::{
    lower_mutation_intent_declaration, LoweredMutationIntentDeclaration, WorkflowBasisFamily,
};
use crate::{WorthQueryEvidenceIdentity, WorthQueryEvidenceScope, WorthQueryEvidenceTag};

use super::batch_admission::AdmittedEffectBatch;
use super::batch_execution::{EffectBatchExecutionDenial, ExecutedEffectBatchPlan};
use super::counters::EffectLifecycleCounters;
use super::eligibility::AdmittedEffectIntent;
use super::execution::{
    EffectExecutionAuthority, EffectExecutionDenialKind, ExecutedEffectAuthorityArtifact,
    ExecutedEffectPlan,
};
use super::execution_relational_batch::execute_lowered_mutation_batch;
use super::lowering::{assemble_lowered_batch_mutation_component, EffectLoweringDenial};
use super::normalized::EffectOperationInput;
use super::planning::EffectAuthorityOwner;
use super::receipt::EffectExecutionReceipt;
use super::taxonomy::EffectAuthorityLane;

impl AdmittedEffectBatch {
    pub fn lower(self) -> Result<LoweredEffectBatchExecutionPlan, EffectLoweringDenial> {
        let declarations = self
            .admitted()
            .iter()
            .map(lower_batch_mutation_component)
            .collect::<Result<Vec<_>, _>>()?;
        let workflow_basis_lane = self.admitted()[0]
            .workflow_declaration()
            .binding()
            .basis_family()
            .clone();
        Ok(LoweredEffectBatchExecutionPlan::new(
            self.authority_lane(),
            self.basis_family(),
            EffectAuthorityOwner::WORTHRelational,
            LoweredEffectBatchExecutionArtifact::RelationalMutation(
                LoweredRelationalMutationBatchExecutionArtifact::new(
                    workflow_basis_lane,
                    declarations,
                ),
            ),
            self.admitted().to_vec(),
        ))
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum LoweredEffectBatchExecutionArtifact {
    RelationalMutation(LoweredRelationalMutationBatchExecutionArtifact),
}

#[derive(Clone, Debug, PartialEq)]
pub struct LoweredRelationalMutationBatchExecutionArtifact {
    workflow_basis_lane: WorkflowBasisFamily,
    declarations: Vec<LoweredMutationIntentDeclaration>,
    batch_mutation_identity: WorthQueryEvidenceIdentity,
}

impl LoweredRelationalMutationBatchExecutionArtifact {
    fn new(
        workflow_basis_lane: WorkflowBasisFamily,
        declarations: Vec<LoweredMutationIntentDeclaration>,
    ) -> Self {
        let batch_mutation_identity =
            WorthQueryEvidenceIdentity::compose(WorthQueryEvidenceScope::WorkflowMutationLowering)
                .field_shape(
                    WorthQueryEvidenceTag::new("identity_family"),
                    "lowered_relational_mutation_batch_v1",
                )
                .field_shape(
                    WorthQueryEvidenceTag::new("workflow_basis"),
                    workflow_basis_lane.as_str(),
                )
                .field_evidence_identity_sequence(
                    WorthQueryEvidenceTag::new("declaration"),
                    declarations
                        .iter()
                        .map(LoweredMutationIntentDeclaration::lowering_identity),
                )
                .seal();
        Self {
            workflow_basis_lane,
            declarations,
            batch_mutation_identity,
        }
    }

    pub fn workflow_basis_lane(&self) -> &WorkflowBasisFamily {
        &self.workflow_basis_lane
    }

    pub fn declarations(&self) -> &[LoweredMutationIntentDeclaration] {
        &self.declarations
    }

    pub fn batch_mutation_for_reporting(&self) -> &str {
        self.batch_mutation_identity.as_str()
    }

    pub fn batch_mutation_digest(&self) -> &str {
        self.batch_mutation_for_reporting()
    }

    pub fn batch_mutation_identity(&self) -> &WorthQueryEvidenceIdentity {
        &self.batch_mutation_identity
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct LoweredEffectBatchExecutionPlan {
    authority_lane: EffectAuthorityLane,
    basis_family: BasisFamily,
    authority_owner: EffectAuthorityOwner,
    artifact: LoweredEffectBatchExecutionArtifact,
    admitted_components: Vec<super::AdmittedEffectIntent>,
    admitted_batch_identity: WorthQueryEvidenceIdentity,
    batch_identity: WorthQueryEvidenceIdentity,
    counters: EffectLifecycleCounters,
}

impl LoweredEffectBatchExecutionPlan {
    fn new(
        authority_lane: EffectAuthorityLane,
        basis_family: BasisFamily,
        authority_owner: EffectAuthorityOwner,
        artifact: LoweredEffectBatchExecutionArtifact,
        admitted_components: Vec<super::AdmittedEffectIntent>,
    ) -> Self {
        let admitted_batch_identity =
            WorthQueryEvidenceIdentity::compose(WorthQueryEvidenceScope::WorkflowMutationLowering)
                .field_shape(
                    WorthQueryEvidenceTag::new("identity_family"),
                    "admitted_effect_batch_v1",
                )
                .field_evidence_identity_sequence(
                    WorthQueryEvidenceTag::new("admitted"),
                    admitted_components
                        .iter()
                        .map(AdmittedEffectIntent::admitted_identity),
                )
                .seal();
        let counters = EffectLifecycleCounters::lowered_batch(
            admitted_components.len(),
            lowered_batch_artifact_width(&artifact),
            lowered_batch_artifact_executor_rediscovery_count(&artifact),
        );
        let batch_identity =
            WorthQueryEvidenceIdentity::compose(WorthQueryEvidenceScope::WorkflowMutationLowering)
                .field_shape(
                    WorthQueryEvidenceTag::new("identity_family"),
                    "lowered_effect_batch_execution_plan_v2",
                )
                .field_shape(
                    WorthQueryEvidenceTag::new("authority"),
                    authority_lane.as_str(),
                )
                .field_shape(WorthQueryEvidenceTag::new("basis"), basis_family.as_str())
                .field_shape(
                    WorthQueryEvidenceTag::new("owner"),
                    authority_owner.as_str(),
                )
                .field_evidence_identity(
                    WorthQueryEvidenceTag::new("artifact"),
                    lowered_batch_artifact_identity(&artifact),
                )
                .field_evidence_identity(
                    WorthQueryEvidenceTag::new("counters"),
                    &counters.evidence_identity(),
                )
                .seal();
        Self {
            authority_lane,
            basis_family,
            authority_owner,
            artifact,
            admitted_components,
            admitted_batch_identity,
            batch_identity,
            counters,
        }
    }

    pub fn authority_lane(&self) -> EffectAuthorityLane {
        self.authority_lane
    }

    pub fn basis_family(&self) -> BasisFamily {
        self.basis_family
    }

    pub fn authority_owner(&self) -> EffectAuthorityOwner {
        self.authority_owner
    }

    pub fn workflow_basis_lane(&self) -> &WorkflowBasisFamily {
        match &self.artifact {
            LoweredEffectBatchExecutionArtifact::RelationalMutation(batch) => {
                batch.workflow_basis_lane()
            }
        }
    }

    pub fn artifact(&self) -> &LoweredEffectBatchExecutionArtifact {
        &self.artifact
    }

    pub fn as_relational_mutation_batch(
        &self,
    ) -> Option<&LoweredRelationalMutationBatchExecutionArtifact> {
        match &self.artifact {
            LoweredEffectBatchExecutionArtifact::RelationalMutation(batch) => Some(batch),
        }
    }

    pub fn batch_for_reporting(&self) -> &str {
        self.batch_identity.as_str()
    }

    pub fn batch_identity(&self) -> &WorthQueryEvidenceIdentity {
        &self.batch_identity
    }

    pub fn admitted_batch_for_reporting(&self) -> &str {
        self.admitted_batch_identity.as_str()
    }

    pub fn admitted_batch_identity(&self) -> &WorthQueryEvidenceIdentity {
        &self.admitted_batch_identity
    }

    pub fn counters(&self) -> &EffectLifecycleCounters {
        &self.counters
    }

    pub fn execute_with(
        self,
        mut authority: EffectExecutionAuthority<'_>,
    ) -> Result<ExecutedEffectBatchPlan, EffectBatchExecutionDenial> {
        let lowered = self.clone();
        let Self {
            authority_lane,
            basis_family,
            authority_owner,
            artifact,
            admitted_components,
            ..
        } = self;
        match artifact {
            LoweredEffectBatchExecutionArtifact::RelationalMutation(batch) => {
                if !authority.has_relational_authority() && authority.has_bridge_authority() {
                    return Err(EffectBatchExecutionDenial::aggregate(
                        EffectExecutionDenialKind::AuthorityOverrideRejected,
                        "lowered relational mutation batch execution rejected bridge host override; the admitted lowered batch requires relational authority",
                    ));
                }
                let runtime = authority.relational_runtime().ok_or_else(|| {
                    EffectBatchExecutionDenial::aggregate(
                        EffectExecutionDenialKind::MissingRelationalAuthority,
                        "lowered relational mutation batch execution requires a relational runtime authority",
                    )
                })?;
                let aggregate_commit =
                    execute_lowered_mutation_batch(runtime, batch.declarations()).map_err(
                        |(kind, message)| EffectBatchExecutionDenial::aggregate(kind, message),
                    )?;
                let components = batch
                    .declarations
                    .into_iter()
                    .zip(admitted_components)
                    .map(|(declaration, admitted)| {
                        ExecutedEffectPlan::new(
                            assemble_lowered_batch_mutation_component(admitted, declaration),
                            ExecutedEffectAuthorityArtifact::Mutation(aggregate_commit.clone()),
                            1,
                        )
                    })
                    .collect::<Vec<_>>();
                Ok(ExecutedEffectBatchPlan::new(
                    lowered,
                    authority_lane,
                    basis_family,
                    authority_owner,
                    ExecutedEffectAuthorityArtifact::Mutation(aggregate_commit),
                    components,
                ))
            }
        }
    }

    pub fn execute_receipt_with(
        self,
        authority: EffectExecutionAuthority<'_>,
    ) -> Result<EffectExecutionReceipt, EffectBatchExecutionDenial> {
        self.execute_with(authority)
            .map(|executed| executed.receipt())
    }
}

fn lower_batch_mutation_component(
    admitted: &super::AdmittedEffectIntent,
) -> Result<LoweredMutationIntentDeclaration, EffectLoweringDenial> {
    let input = match admitted.normalized().operation_input() {
        EffectOperationInput::Mutation(input) => input.clone(),
        _ => unreachable!("batch admission preserves mutation-only components"),
    };
    lower_mutation_intent_declaration(
        admitted.workflow_declaration(),
        admitted
            .normalized()
            .expected_lower_runtime_binding_identity()
            .expect("admitted mutation effects must preserve a lower-runtime binding identity"),
        input,
    )
    .map_err(|error| {
        EffectLoweringDenial::from_workflow_error_for_batch(
            admitted
                .workflow_declaration()
                .report()
                .declaration_identity(),
            admitted.normalized().counters().effect_support_row_count(),
            error,
        )
    })
}

fn lowered_batch_artifact_identity(
    artifact: &LoweredEffectBatchExecutionArtifact,
) -> &WorthQueryEvidenceIdentity {
    match artifact {
        LoweredEffectBatchExecutionArtifact::RelationalMutation(batch) => {
            batch.batch_mutation_identity()
        }
    }
}

fn lowered_batch_artifact_width(artifact: &LoweredEffectBatchExecutionArtifact) -> usize {
    match artifact {
        LoweredEffectBatchExecutionArtifact::RelationalMutation(batch) => batch
            .declarations()
            .iter()
            .map(|declaration| declaration.counters().workflow_lowering_width())
            .sum(),
    }
}

fn lowered_batch_artifact_executor_rediscovery_count(
    artifact: &LoweredEffectBatchExecutionArtifact,
) -> usize {
    match artifact {
        LoweredEffectBatchExecutionArtifact::RelationalMutation(batch) => batch
            .declarations()
            .iter()
            .map(|declaration| declaration.counters().workflow_executor_rediscovery_count())
            .sum(),
    }
}
