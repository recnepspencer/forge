use forge_store_physical_isolation::{
    CopyOnWritePublicationPlan, PhysicalPublicationReceipt, PhysicalRootPublicationRuntime,
};

use super::{
    LayoutBindingWitness, LayoutEvolutionDenial, LayoutInterruptionBoundary,
    LayoutInterruptionFingerprint, LayoutInterruptionState, LayoutMigrationCounterSnapshot,
    LayoutMigrationPlan, LayoutPlanFingerprint,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LayoutMigrationExecutionFingerprint {
    plan: LayoutPlanFingerprint,
    old_root: forge_store_physical_isolation::CurrentPhysicalRoot,
    new_root: forge_store_physical_isolation::CurrentPhysicalRoot,
    old_root_validation: forge_store_physical_format::RootPublicationValidationWitness,
    new_root_validation: forge_store_physical_format::RootPublicationValidationWitness,
}

impl LayoutMigrationExecutionFingerprint {
    fn from_request(plan: &LayoutMigrationPlan, publication: &CopyOnWritePublicationPlan) -> Self {
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
pub struct LayoutMigrationExecutionRequest {
    plan: LayoutMigrationPlan,
    publication: CopyOnWritePublicationPlan,
    fingerprint: LayoutMigrationExecutionFingerprint,
}

impl LayoutMigrationExecutionRequest {
    pub fn new(plan: LayoutMigrationPlan, publication: CopyOnWritePublicationPlan) -> Self {
        let fingerprint = LayoutMigrationExecutionFingerprint::from_request(&plan, &publication);
        Self {
            plan,
            publication,
            fingerprint,
        }
    }

    pub const fn fingerprint(&self) -> LayoutMigrationExecutionFingerprint {
        self.fingerprint
    }

    pub fn publication_source_root(&self) -> forge_store_physical_isolation::CurrentPhysicalRoot {
        self.publication.binding().old_root()
    }

    pub fn interruption_state(&self) -> LayoutInterruptionState {
        LayoutInterruptionState::new(
            LayoutInterruptionFingerprint::migration_execution(self.fingerprint),
            self.plan.binding().clone(),
            LayoutInterruptionBoundary::SourceBound,
        )
    }

    pub fn resume_or_rollback(
        &self,
        interruption: LayoutInterruptionState,
    ) -> super::LayoutMigrationInterruptionOutcome {
        super::interruption::classify_migration_interruption(
            LayoutInterruptionFingerprint::migration_execution(self.fingerprint),
            self.plan.declaration(),
            interruption,
        )
    }
}

#[derive(Debug, Clone)]
pub struct LayoutMigrationReceipt {
    fingerprint: LayoutMigrationExecutionFingerprint,
    source_binding: LayoutBindingWitness,
    target_binding: LayoutBindingWitness,
    publication: PhysicalPublicationReceipt,
    counters: LayoutMigrationCounterSnapshot,
}

impl LayoutMigrationReceipt {
    pub const fn fingerprint(&self) -> LayoutMigrationExecutionFingerprint {
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

    pub const fn counters(&self) -> LayoutMigrationCounterSnapshot {
        self.counters
    }

    pub fn interruption_state(&self) -> LayoutInterruptionState {
        LayoutInterruptionState::new(
            LayoutInterruptionFingerprint::migration_execution(self.fingerprint),
            self.target_binding.clone(),
            LayoutInterruptionBoundary::TargetPublished,
        )
    }
}

#[derive(Debug, Clone)]
enum LayoutMigrationExecutionCase {
    Published(Box<LayoutMigrationReceipt>),
    Denied(Box<LayoutEvolutionDenial>),
}

#[derive(Debug, Clone)]
pub struct LayoutMigrationExecutionOutcome {
    case: LayoutMigrationExecutionCase,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum LayoutMigrationExecutionCaseId {
    Published,
    Denied(super::super::LayoutEvolutionDenialKind),
}

impl LayoutMigrationExecutionCaseId {
    pub const fn as_str(self) -> &'static str {
        use super::super::LayoutEvolutionDenialKind as Denial;
        match self {
            Self::Published => "layout.migration.execution.published",
            Self::Denied(Denial::PhysicalPublicationStoreAuthorityMismatch) => {
                "layout.migration.execution.denied.store_authority"
            }
            Self::Denied(Denial::PhysicalPublicationSourceMismatch) => {
                "layout.migration.execution.denied.publication_source"
            }
            Self::Denied(Denial::PhysicalPublication) => {
                "layout.migration.execution.denied.physical_publication"
            }
            Self::Denied(_) => "layout.migration.execution.denied.unadvertised",
        }
    }
}

pub fn layout_migration_execution_cases() -> impl Iterator<Item = LayoutMigrationExecutionCaseId> {
    use super::super::LayoutEvolutionDenialKind as Denial;
    [
        LayoutMigrationExecutionCaseId::Published,
        LayoutMigrationExecutionCaseId::Denied(Denial::PhysicalPublicationStoreAuthorityMismatch),
        LayoutMigrationExecutionCaseId::Denied(Denial::PhysicalPublicationSourceMismatch),
        LayoutMigrationExecutionCaseId::Denied(Denial::PhysicalPublication),
    ]
    .into_iter()
}

#[derive(Debug, Clone, Copy)]
pub enum LayoutMigrationExecutionView<'a> {
    Published(&'a LayoutMigrationReceipt),
    Denied(&'a LayoutEvolutionDenial),
}

impl LayoutMigrationExecutionOutcome {
    fn published(receipt: LayoutMigrationReceipt) -> Self {
        Self {
            case: LayoutMigrationExecutionCase::Published(Box::new(receipt)),
        }
    }

    fn denied(denial: LayoutEvolutionDenial) -> Self {
        Self {
            case: LayoutMigrationExecutionCase::Denied(Box::new(denial)),
        }
    }

    pub const fn view(&self) -> LayoutMigrationExecutionView<'_> {
        match &self.case {
            LayoutMigrationExecutionCase::Published(value) => {
                LayoutMigrationExecutionView::Published(value)
            }
            LayoutMigrationExecutionCase::Denied(value) => {
                LayoutMigrationExecutionView::Denied(value)
            }
        }
    }

    pub const fn case_id(&self) -> LayoutMigrationExecutionCaseId {
        match &self.case {
            LayoutMigrationExecutionCase::Published(_) => LayoutMigrationExecutionCaseId::Published,
            LayoutMigrationExecutionCase::Denied(denial) => {
                LayoutMigrationExecutionCaseId::Denied(denial.kind())
            }
        }
    }

    pub fn into_published(self) -> Result<LayoutMigrationReceipt, LayoutEvolutionDenial> {
        match self.case {
            LayoutMigrationExecutionCase::Published(value) => Ok(*value),
            LayoutMigrationExecutionCase::Denied(value) => Err(*value),
        }
    }
}

#[derive(Debug)]
pub struct LayoutMigrationExecution<'a> {
    publication: &'a mut PhysicalRootPublicationRuntime,
}

pub const fn layout_migration_execution(
    publication: &mut PhysicalRootPublicationRuntime,
) -> LayoutMigrationExecution<'_> {
    LayoutMigrationExecution { publication }
}

impl LayoutMigrationExecution<'_> {
    pub fn execute(
        self,
        request: LayoutMigrationExecutionRequest,
    ) -> LayoutMigrationExecutionOutcome {
        let result = validate_migration_publication(&request)
            .and_then(|()| publish_migration_transition(self.publication, request));
        match result {
            Ok(receipt) => LayoutMigrationExecutionOutcome::published(receipt),
            Err(denial) => LayoutMigrationExecutionOutcome::denied(denial),
        }
    }
}

fn validate_migration_publication(
    request: &LayoutMigrationExecutionRequest,
) -> Result<(), LayoutEvolutionDenial> {
    let expected = request
        .plan
        .binding()
        .bound_authority()
        .authority_identity();
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

fn publish_migration_transition(
    publication_runtime: &mut PhysicalRootPublicationRuntime,
    request: LayoutMigrationExecutionRequest,
) -> Result<LayoutMigrationReceipt, LayoutEvolutionDenial> {
    let published = publication_runtime
        .publish(request.publication)
        .map_err(|denial| LayoutEvolutionDenial::PhysicalPublication(Box::new(denial)))?;
    let publication = published.receipt().clone();
    let target_binding = LayoutBindingWitness::issue_transition(
        request.plan.binding(),
        request.plan.target_version(),
        publication.new_root_validation(),
    );
    let counters = LayoutMigrationCounterSnapshot::published(publication.counters());
    Ok(LayoutMigrationReceipt {
        fingerprint: request.fingerprint,
        source_binding: request.plan.binding().clone(),
        target_binding,
        publication,
        counters,
    })
}
