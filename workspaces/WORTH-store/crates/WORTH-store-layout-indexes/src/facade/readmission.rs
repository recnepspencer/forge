pub use crate::execution::{S8ExecutionReadmissionWitness, S8ExecutionRebindWitness};
use crate::{
    execution::{
        S8AccessLoweringDenied, S8RebindRequiredAccessReceipt, S8StaleLoweredAccessReceipt,
    },
    ArtifactFamilyLifecycleAdmission, PhysicalKeyDomainWitness, S8LayoutCoverageWitness,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct ExecutionFreshnessFacade;

impl ExecutionFreshnessFacade {
    pub fn admit_current_for_stale(
        &self,
        stale: &S8StaleLoweredAccessReceipt,
        lifecycle: ArtifactFamilyLifecycleAdmission,
        key_domain: PhysicalKeyDomainWitness,
        coverage: S8LayoutCoverageWitness,
    ) -> Result<S8ExecutionReadmissionWitness, S8AccessLoweringDenied> {
        let exact_coverage = coverage
            .require_exact()
            .map_err(|denial| S8AccessLoweringDenied::CoverageDenied {
                basis: stale.basis(),
                denial,
            })?;
        let expected_coverage = stale
            .selected()
            .access_shape()
            .coverage()
            .expect("stale lowered access retains declared coverage")
            .require_exact()
            .expect("stale lowered access retains exact declared coverage");
        let expected_family = expected_coverage.family();
        let actual_family = lifecycle.declaration().family();
        if actual_family != expected_family || exact_coverage.family() != expected_family {
            return Err(S8AccessLoweringDenied::LifecycleFamilyMismatch {
                basis: stale.basis(),
                expected: expected_family,
                actual: actual_family,
            });
        }
        let expected_domain = stale.selected().fingerprint().key_domain().domain();
        let actual_domain = key_domain.domain();
        if actual_domain != expected_domain {
            return Err(S8AccessLoweringDenied::KeyDomainMismatch {
                basis: stale.basis(),
                expected: expected_domain,
                actual: actual_domain,
            });
        }
        if exact_coverage != expected_coverage {
            return Err(S8AccessLoweringDenied::CurrentCoverageMismatch {
                basis: stale.basis(),
                expected: expected_coverage,
                actual: exact_coverage,
            });
        }

        Ok(S8ExecutionReadmissionWitness::new(
            stale.basis(),
            stale.selected().planned_counter_envelope().lookup(),
            exact_coverage,
        ))
    }

    pub fn admit_rebind_for_execution(
        &self,
        rebind: &S8RebindRequiredAccessReceipt,
        lifecycle: ArtifactFamilyLifecycleAdmission,
        key_domain: PhysicalKeyDomainWitness,
        coverage: S8LayoutCoverageWitness,
    ) -> Result<S8ExecutionRebindWitness, S8AccessLoweringDenied> {
        let exact_coverage = coverage
            .require_exact()
            .map_err(|denial| S8AccessLoweringDenied::CoverageDenied {
                basis: rebind.basis(),
                denial,
            })?;
        let expected_coverage = rebind
            .selected()
            .access_shape()
            .coverage()
            .expect("rebind-required lowered access retains declared coverage")
            .require_exact()
            .expect("rebind-required lowered access retains exact declared coverage");
        let expected_family = expected_coverage.family();
        let actual_family = lifecycle.declaration().family();
        if actual_family != expected_family || exact_coverage.family() != expected_family {
            return Err(S8AccessLoweringDenied::LifecycleFamilyMismatch {
                basis: rebind.basis(),
                expected: expected_family,
                actual: actual_family,
            });
        }
        let expected_domain = rebind.selected().fingerprint().key_domain().domain();
        let actual_domain = key_domain.domain();
        if actual_domain != expected_domain {
            return Err(S8AccessLoweringDenied::KeyDomainMismatch {
                basis: rebind.basis(),
                expected: expected_domain,
                actual: actual_domain,
            });
        }
        if exact_coverage != expected_coverage {
            return Err(S8AccessLoweringDenied::CurrentCoverageMismatch {
                basis: rebind.basis(),
                expected: expected_coverage,
                actual: exact_coverage,
            });
        }

        Ok(S8ExecutionRebindWitness::new(
            rebind.basis(),
            exact_coverage,
        ))
    }
}

pub(crate) const fn layout_execution_freshness() -> ExecutionFreshnessFacade {
    ExecutionFreshnessFacade
}
