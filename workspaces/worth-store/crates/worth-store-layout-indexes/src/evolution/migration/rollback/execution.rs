use worth_store_physical_isolation::{
    CopyOnWritePublicationPlan, PhysicalPublicationReceipt, PhysicalRootPublicationRuntime,
};

use super::{
    LayoutBindingWitness, LayoutEvolutionDenial, LayoutRollbackCounterSnapshot, LayoutRollbackPlan,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LayoutRollbackExecutionFingerprint {
    plan: super::LayoutPlanFingerprint,
    old_root: worth_store_physical_isolation::CurrentPhysicalRoot,
    new_root: worth_store_physical_isolation::CurrentPhysicalRoot,
    old_root_validation: worth_store_physical_format::RootPublicationValidationWitness,
    new_root_validation: worth_store_physical_format::RootPublicationValidationWitness,
}

impl LayoutRollbackExecutionFingerprint {
    fn from_request(plan: &LayoutRollbackPlan, publication: &CopyOnWritePublicationPlan) -> Self {
        let binding = publication.binding();
        Self {
            plan: plan.fingerprint(),
            old_root: binding.old_root(),
            new_root: binding.new_root(),
            old_root_validation: binding.old_root_validation(),
            new_root_validation: binding.new_root_validation(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct LayoutRollbackExecutionRequest {
    plan: LayoutRollbackPlan,
    publication: CopyOnWritePublicationPlan,
    fingerprint: LayoutRollbackExecutionFingerprint,
}

impl LayoutRollbackExecutionRequest {
    pub fn new(plan: LayoutRollbackPlan, publication: CopyOnWritePublicationPlan) -> Self {
        let fingerprint = LayoutRollbackExecutionFingerprint::from_request(&plan, &publication);
        Self {
            plan,
            publication,
            fingerprint,
        }
    }

    pub const fn fingerprint(&self) -> LayoutRollbackExecutionFingerprint {
        self.fingerprint
    }

    pub fn publication_source_root(&self) -> worth_store_physical_isolation::CurrentPhysicalRoot {
        self.publication.binding().old_root()
    }

    pub fn interruption_state(&self) -> super::LayoutRollbackInterruptionState {
        super::LayoutRollbackInterruptionState::source_bound(
            self.fingerprint,
            self.plan.binding().clone(),
        )
    }

    pub fn classify_interruption(
        &self,
        interruption: super::LayoutRollbackInterruptionState,
    ) -> super::LayoutRollbackInterruptionOutcome {
        super::interruption::classify_rollback_interruption(self.fingerprint, interruption)
    }
}

#[derive(Debug, Clone)]
pub struct LayoutRollbackReceipt {
    fingerprint: LayoutRollbackExecutionFingerprint,
    source_binding: LayoutBindingWitness,
    target_binding: LayoutBindingWitness,
    publication: PhysicalPublicationReceipt,
    counters: LayoutRollbackCounterSnapshot,
}

impl LayoutRollbackReceipt {
    pub const fn fingerprint(&self) -> LayoutRollbackExecutionFingerprint {
        self.fingerprint
    }

    pub const fn source_binding(&self) -> &LayoutBindingWitness {
        &self.source_binding
    }

    pub const fn target_binding(&self) -> &LayoutBindingWitness {
        &self.target_binding
    }

    pub const fn publication(&self) -> &PhysicalPublicationReceipt {
        &self.publication
    }

    pub const fn counters(&self) -> LayoutRollbackCounterSnapshot {
        self.counters
    }

    pub fn interruption_state(&self) -> super::LayoutRollbackInterruptionState {
        super::LayoutRollbackInterruptionState::target_published(
            self.fingerprint,
            self.target_binding.clone(),
        )
    }
}

#[derive(Debug, Clone)]
enum LayoutRollbackExecutionCase {
    Published(Box<LayoutRollbackReceipt>),
    Denied(Box<LayoutEvolutionDenial>),
}

#[derive(Debug, Clone)]
pub struct LayoutRollbackExecutionOutcome {
    case: LayoutRollbackExecutionCase,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum LayoutRollbackExecutionCaseId {
    Published,
    Denied(super::super::LayoutEvolutionDenialKind),
}

impl LayoutRollbackExecutionCaseId {
    pub const fn as_str(self) -> &'static str {
        use super::super::LayoutEvolutionDenialKind as Denial;
        match self {
            Self::Published => "layout.rollback.execution.published",
            Self::Denied(Denial::PhysicalPublicationStoreAuthorityMismatch) => {
                "layout.rollback.execution.denied.store_authority"
            }
            Self::Denied(Denial::PhysicalPublicationSourceMismatch) => {
                "layout.rollback.execution.denied.publication_source"
            }
            Self::Denied(Denial::PhysicalPublication) => {
                "layout.rollback.execution.denied.physical_publication"
            }
            Self::Denied(_) => "layout.rollback.execution.denied.unadvertised",
        }
    }
}

pub fn layout_rollback_execution_cases() -> impl Iterator<Item = LayoutRollbackExecutionCaseId> {
    use super::super::LayoutEvolutionDenialKind as Denial;
    [
        LayoutRollbackExecutionCaseId::Published,
        LayoutRollbackExecutionCaseId::Denied(Denial::PhysicalPublicationStoreAuthorityMismatch),
        LayoutRollbackExecutionCaseId::Denied(Denial::PhysicalPublicationSourceMismatch),
        LayoutRollbackExecutionCaseId::Denied(Denial::PhysicalPublication),
    ]
    .into_iter()
}

#[derive(Debug, Clone, Copy)]
pub enum LayoutRollbackExecutionView<'a> {
    Published(&'a LayoutRollbackReceipt),
    Denied(&'a LayoutEvolutionDenial),
}

impl LayoutRollbackExecutionOutcome {
    fn published(receipt: LayoutRollbackReceipt) -> Self {
        Self {
            case: LayoutRollbackExecutionCase::Published(Box::new(receipt)),
        }
    }

    fn denied(denial: LayoutEvolutionDenial) -> Self {
        Self {
            case: LayoutRollbackExecutionCase::Denied(Box::new(denial)),
        }
    }

    pub const fn view(&self) -> LayoutRollbackExecutionView<'_> {
        match &self.case {
            LayoutRollbackExecutionCase::Published(value) => {
                LayoutRollbackExecutionView::Published(value)
            }
            LayoutRollbackExecutionCase::Denied(value) => {
                LayoutRollbackExecutionView::Denied(value)
            }
        }
    }

    pub const fn case_id(&self) -> LayoutRollbackExecutionCaseId {
        match &self.case {
            LayoutRollbackExecutionCase::Published(_) => LayoutRollbackExecutionCaseId::Published,
            LayoutRollbackExecutionCase::Denied(denial) => {
                LayoutRollbackExecutionCaseId::Denied(denial.kind())
            }
        }
    }

    pub fn into_published(self) -> Result<LayoutRollbackReceipt, LayoutEvolutionDenial> {
        match self.case {
            LayoutRollbackExecutionCase::Published(value) => Ok(*value),
            LayoutRollbackExecutionCase::Denied(value) => Err(*value),
        }
    }
}

#[derive(Debug)]
pub struct LayoutRollbackExecution<'a> {
    publication: &'a mut PhysicalRootPublicationRuntime,
}

pub const fn layout_rollback_execution(
    publication: &mut PhysicalRootPublicationRuntime,
) -> LayoutRollbackExecution<'_> {
    LayoutRollbackExecution { publication }
}

impl LayoutRollbackExecution<'_> {
    pub fn execute(
        self,
        request: LayoutRollbackExecutionRequest,
    ) -> LayoutRollbackExecutionOutcome {
        let result = validate_rollback_publication(&request)
            .and_then(|()| publish_rollback_transition(self.publication, request));
        match result {
            Ok(receipt) => LayoutRollbackExecutionOutcome::published(receipt),
            Err(denial) => LayoutRollbackExecutionOutcome::denied(denial),
        }
    }
}

fn validate_rollback_publication(
    request: &LayoutRollbackExecutionRequest,
) -> Result<(), LayoutEvolutionDenial> {
    let expected = request.plan.authority().authority_identity();
    let actual = request.publication.binding().store_authority_identity();
    if expected != actual {
        return Err(
            LayoutEvolutionDenial::PhysicalPublicationStoreAuthorityMismatch {
                binding: expected,
                publication: actual,
            },
        );
    }
    let publication_source = request.publication.binding().old_root_validation();
    if !request
        .plan
        .binding()
        .accepts_publication_source(publication_source)
    {
        return Err(LayoutEvolutionDenial::PhysicalPublicationSourceMismatch {
            expected: request.plan.binding().source_identity(),
            actual: publication_source.reference(),
        });
    }
    Ok(())
}

fn publish_rollback_transition(
    publication_runtime: &mut PhysicalRootPublicationRuntime,
    request: LayoutRollbackExecutionRequest,
) -> Result<LayoutRollbackReceipt, LayoutEvolutionDenial> {
    let published = publication_runtime
        .publish(request.publication)
        .map_err(|denial| LayoutEvolutionDenial::PhysicalPublication(Box::new(denial)))?;
    let publication = published.receipt().clone();
    let source_binding = request.plan.binding().clone();
    let target_binding = LayoutBindingWitness::issue_transition(
        request.plan.binding(),
        request.plan.rollback_target(),
        publication.new_root_validation(),
    );
    let counters = LayoutRollbackCounterSnapshot::published(publication.counters());
    Ok(LayoutRollbackReceipt {
        fingerprint: request.fingerprint,
        source_binding,
        target_binding,
        publication,
        counters,
    })
}
