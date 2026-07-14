use crate::integrity::LayoutCorruptionOutcome;
use crate::strategy::{admit_strategy_from_basis, AdmittedLayoutStrategy, StrategyAuthorityBasis};

use super::outcome::RebuildAdmissionDenial;
use super::source::admission::{
    admit_source_authority, classify_corruption, validate_rebuild_shape,
};
use super::source::DerivedIndexAuthoritySource;
use super::{
    DerivedIndexParityBasis, DerivedIndexRebuildAdmissionOutcome,
    DerivedIndexRebuildCounterSnapshot, DerivedIndexRebuildDenied, DerivedIndexRebuildOutcome,
    DerivedIndexRebuildRequest, DerivedIndexRebuildScope, DerivedIndexResultIdentity,
};

#[derive(Debug, PartialEq, Eq)]
pub struct DerivedIndexRebuildPlan {
    request: DerivedIndexRebuildRequest,
    admitted_strategy: AdmittedLayoutStrategy,
    source_authority: DerivedIndexAuthoritySource,
    rebuild_scope: DerivedIndexRebuildScope,
    corruption: LayoutCorruptionOutcome,
    result_identity: DerivedIndexResultIdentity,
}

impl DerivedIndexRebuildPlan {
    fn issue(
        request: DerivedIndexRebuildRequest,
        admitted_strategy: AdmittedLayoutStrategy,
        source_authority: DerivedIndexAuthoritySource,
        rebuild_scope: DerivedIndexRebuildScope,
        corruption: LayoutCorruptionOutcome,
    ) -> Self {
        let result_identity = source_authority.result_identity();
        Self {
            request,
            admitted_strategy,
            source_authority,
            rebuild_scope,
            corruption,
            result_identity,
        }
    }

    pub const fn request(&self) -> &DerivedIndexRebuildRequest {
        &self.request
    }
    pub(super) const fn source_authority(&self) -> &DerivedIndexAuthoritySource {
        &self.source_authority
    }
    pub const fn admitted_strategy(&self) -> AdmittedLayoutStrategy {
        self.admitted_strategy
    }
    pub const fn rebuild_scope(&self) -> &DerivedIndexRebuildScope {
        &self.rebuild_scope
    }
    pub const fn corruption(&self) -> &LayoutCorruptionOutcome {
        &self.corruption
    }
    pub const fn result_identity(&self) -> DerivedIndexResultIdentity {
        self.result_identity
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct DerivedIndexRebuildReceipt {
    plan: DerivedIndexRebuildPlan,
    admitted_strategy: AdmittedLayoutStrategy,
    rebuilt_basis: DerivedIndexParityBasis,
    counters: DerivedIndexRebuildCounterSnapshot,
}

pub(super) struct RebuildOutcomeIssuer(());

impl RebuildOutcomeIssuer {
    const fn owner() -> Self {
        Self(())
    }
}

impl DerivedIndexRebuildReceipt {
    fn issue(
        plan: DerivedIndexRebuildPlan,
        admitted_strategy: AdmittedLayoutStrategy,
        rebuilt_basis: DerivedIndexParityBasis,
        counters: DerivedIndexRebuildCounterSnapshot,
    ) -> Self {
        Self {
            plan,
            admitted_strategy,
            rebuilt_basis,
            counters,
        }
    }

    pub const fn plan(&self) -> &DerivedIndexRebuildPlan {
        &self.plan
    }

    pub const fn admitted_strategy(&self) -> AdmittedLayoutStrategy {
        self.admitted_strategy
    }

    pub const fn rebuilt_basis(&self) -> &DerivedIndexParityBasis {
        &self.rebuilt_basis
    }

    pub fn candidate_declaration(&self) -> super::DerivedIndexCandidateDeclaration {
        super::DerivedIndexCandidateDeclaration::from_canonical_basis(self.rebuilt_basis.clone())
    }

    pub const fn counters(&self) -> DerivedIndexRebuildCounterSnapshot {
        self.counters
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LayoutRebuildAdmission;

impl LayoutRebuildAdmission {
    pub fn admit_plan(
        &self,
        request: DerivedIndexRebuildRequest,
    ) -> DerivedIndexRebuildAdmissionOutcome {
        DerivedIndexRebuildAdmissionOutcome::from_result(
            RebuildOutcomeIssuer::owner(),
            self.admit_plan_inner(request),
        )
    }

    fn admit_plan_inner(
        &self,
        request: DerivedIndexRebuildRequest,
    ) -> Result<DerivedIndexRebuildPlan, RebuildAdmissionDenial> {
        let admitted_strategy = admit_strategy_from_basis(
            StrategyAuthorityBasis::admitted(
                request.admitted_family(),
                request.admitted_key_domain(),
            ),
            request.strategy_family(),
        )
        .map_err(|denial| {
            RebuildAdmissionDenial::strategy(DerivedIndexRebuildDenied::StrategyDenied { denial })
        })?;
        validate_rebuild_shape(&request)?;
        let shape_coverage = request.materialization().coverage().clone();
        let source_authority =
            admit_source_authority(&request, admitted_strategy, shape_coverage.clone())?;
        let corruption = classify_corruption(&source_authority);

        Ok(DerivedIndexRebuildPlan::issue(
            request,
            admitted_strategy,
            source_authority,
            DerivedIndexRebuildScope::from_coverage(shape_coverage),
            corruption,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LayoutRebuildExecution;

impl LayoutRebuildExecution {
    pub fn execute(&self, plan: DerivedIndexRebuildPlan) -> DerivedIndexRebuildOutcome {
        let rebuilt_basis = plan
            .source_authority()
            .rebuild_candidate(plan.request().key_domain());
        self.issue_candidate(plan, rebuilt_basis)
    }

    fn issue_candidate(
        &self,
        plan: DerivedIndexRebuildPlan,
        rebuilt_basis: DerivedIndexParityBasis,
    ) -> DerivedIndexRebuildOutcome {
        let admitted_strategy = plan.admitted_strategy();
        let counters = DerivedIndexRebuildCounterSnapshot::from_candidate(
            plan.source_authority().source_artifact_count(),
            &rebuilt_basis,
        );
        DerivedIndexRebuildOutcome::rebuilt(
            RebuildOutcomeIssuer::owner(),
            DerivedIndexRebuildReceipt::issue(plan, admitted_strategy, rebuilt_basis, counters),
        )
    }
}

pub const fn layout_rebuild_admission() -> LayoutRebuildAdmission {
    LayoutRebuildAdmission
}

pub const fn layout_rebuild_execution() -> LayoutRebuildExecution {
    LayoutRebuildExecution
}
