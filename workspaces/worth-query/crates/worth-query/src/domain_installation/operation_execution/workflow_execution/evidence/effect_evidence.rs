use crate::basis_lifecycle::BasisFamily;
use crate::runtime::WorthQueryWriteReceipt;
use crate::WorthQueryEvidenceIdentity;
use worth_query_installation::facade::WorthQueryOperationEffectFamily;

#[derive(Clone, Debug, PartialEq)]
enum WorthQueryWorkflowEffectSource {
    RuntimeMutation(Box<WorthQueryWorkflowRuntimeMutationSource>),
}

#[derive(Clone, Debug, PartialEq)]
struct WorthQueryWorkflowRuntimeMutationSource {
    receipt: WorthQueryWriteReceipt,
    workflow_binding_identity: WorthQueryEvidenceIdentity,
    basis: BasisFamily,
}

#[derive(Clone, Debug, PartialEq)]
pub struct WorthQueryWorkflowEffectEvidence {
    family: WorthQueryOperationEffectFamily,
    source: WorthQueryWorkflowEffectSource,
}

impl WorthQueryWorkflowEffectEvidence {
    pub(crate) fn runtime_mutation(
        receipt: WorthQueryWriteReceipt,
        binding: &crate::workflow::WorkflowContextBinding,
        basis: BasisFamily,
    ) -> Self {
        Self {
            family: WorthQueryOperationEffectFamily::Mutation,
            source: WorthQueryWorkflowEffectSource::RuntimeMutation(Box::new(
                WorthQueryWorkflowRuntimeMutationSource {
                    receipt,
                    workflow_binding_identity: binding.binding_identity().clone(),
                    basis,
                },
            )),
        }
    }

    pub fn family(&self) -> WorthQueryOperationEffectFamily {
        self.family
    }

    pub fn mutation_receipt(&self) -> Option<&WorthQueryWriteReceipt> {
        match &self.source {
            WorthQueryWorkflowEffectSource::RuntimeMutation(source) => Some(&source.receipt),
        }
    }

    pub fn receipt_identity(&self) -> &str {
        match &self.source {
            WorthQueryWorkflowEffectSource::RuntimeMutation(source) => {
                source.receipt.commit_evidence_identity().as_str()
            }
        }
    }

    pub(crate) fn semantic_replay_eq(&self, candidate: &Self) -> bool {
        let candidate_family = candidate.family;
        let (Some(subject), Some(candidate_receipt)) =
            (self.mutation_receipt(), candidate.mutation_receipt())
        else {
            return false;
        };
        self.family == candidate_family
            && subject.mutation_family() == candidate_receipt.mutation_family()
            && subject.target_collection_identity()
                == candidate_receipt.target_collection_identity()
            && same_current_target_entity(subject, candidate_receipt)
            && subject.declared_aspect_operations()
                == candidate_receipt.declared_aspect_operations()
            && subject.declared_aspect_value_digest()
                == candidate_receipt.declared_aspect_value_digest()
            && subject.deltas() == candidate_receipt.deltas()
    }

    pub(crate) fn binds_workflow(
        &self,
        binding: &crate::workflow::WorkflowContextBinding,
        basis: BasisFamily,
    ) -> bool {
        match &self.source {
            WorthQueryWorkflowEffectSource::RuntimeMutation(source) => {
                source.workflow_binding_identity == *binding.binding_identity()
                    && source.basis == basis
            }
        }
    }
}

fn same_current_target_entity(
    subject: &crate::runtime::WorthQueryWriteReceipt,
    candidate: &crate::runtime::WorthQueryWriteReceipt,
) -> bool {
    match (
        subject.target_entity_identity(),
        candidate.target_entity_identity(),
    ) {
        (Some(subject), Some(candidate)) => subject.is_same_current_identity_as(candidate),
        (None, None) => true,
        _ => false,
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryWorkflowInvariantOutcome {
    invariant_role: String,
    installed_invariant_identity: String,
    effect_receipt_identities: Vec<String>,
}

impl WorthQueryWorkflowInvariantOutcome {
    pub(crate) fn from_query_commits(
        invariant_role: impl Into<String>,
        installed_invariant_identity: impl Into<String>,
        effects: &[WorthQueryWorkflowEffectEvidence],
    ) -> Self {
        let mut effect_receipt_identities = effects
            .iter()
            .map(|effect| effect.receipt_identity().to_string())
            .collect::<Vec<_>>();
        effect_receipt_identities.sort();
        Self {
            invariant_role: invariant_role.into(),
            installed_invariant_identity: installed_invariant_identity.into(),
            effect_receipt_identities,
        }
    }

    pub fn invariant_role(&self) -> &str {
        &self.invariant_role
    }

    pub fn effect_receipt_identities(&self) -> &[String] {
        &self.effect_receipt_identities
    }
    pub fn installed_invariant_identity(&self) -> &str {
        &self.installed_invariant_identity
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthQueryWorkflowStageEffectDenial {
    UndeclaredEffectFamily,
    Runtime(String),
}
