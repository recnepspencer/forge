use crate::authenticity::authenticity_counters::StoreAuthenticityCheckCounterRecorder;
use crate::authenticity::authenticity_witness::StoreAuthenticityWitnessPosture;
use crate::{
    StoreAuthenticityCheckDenial, StoreAuthenticityCheckDenialKind, StoreAuthenticityRequirement,
    StoreAuthenticityResult, StoreAuthenticityWitnessBinding, StoreAuthenticityWitnessInput,
    StoreCurrentAuthenticityScopeWitness, StoreSecurityScopeIdentity,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StoreAuthenticityCheck {
    requirement: StoreAuthenticityRequirement,
}

impl StoreAuthenticityCheck {
    pub const fn for_requirement(requirement: StoreAuthenticityRequirement) -> Self {
        Self { requirement }
    }

    pub const fn with_security_scope(
        self,
        scope: &StoreCurrentAuthenticityScopeWitness,
    ) -> StoreScopedAuthenticityCheck {
        StoreScopedAuthenticityCheck {
            requirement: self.requirement,
            scope_identity: scope.identity(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StoreScopedAuthenticityCheck {
    requirement: StoreAuthenticityRequirement,
    scope_identity: StoreSecurityScopeIdentity,
}

impl StoreScopedAuthenticityCheck {
    pub const fn with_physical_identity<I>(
        self,
        physical_identity: I,
    ) -> StorePhysicalAuthenticityCheck<I> {
        StorePhysicalAuthenticityCheck {
            requirement: self.requirement,
            scope_identity: self.scope_identity,
            physical_identity,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StorePhysicalAuthenticityCheck<I> {
    requirement: StoreAuthenticityRequirement,
    scope_identity: StoreSecurityScopeIdentity,
    physical_identity: I,
}

impl<I> StorePhysicalAuthenticityCheck<I> {
    pub fn with_witness(
        self,
        witness: StoreAuthenticityWitnessInput<I>,
    ) -> StoreAuthenticityCheckInput<I> {
        StoreAuthenticityCheckInput {
            requirement: self.requirement,
            scope_identity: self.scope_identity,
            physical_identity: self.physical_identity,
            witness,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StoreAuthenticityCheckInput<I> {
    requirement: StoreAuthenticityRequirement,
    scope_identity: StoreSecurityScopeIdentity,
    physical_identity: I,
    witness: StoreAuthenticityWitnessInput<I>,
}

impl<I: Copy + Eq> StoreAuthenticityCheckInput<I> {
    pub fn admit(self) -> Result<StoreAuthenticityResult<I>, StoreAuthenticityCheckDenial> {
        let mut counters = StoreAuthenticityCheckCounterRecorder::new();
        counters.record_requirement_check();
        if !self.requirement.requires_admission_before_result() {
            return Err(denial(
                StoreAuthenticityCheckDenialKind::ResultNotRequired,
                self.requirement,
                self.scope_identity,
                counters,
            ));
        }
        counters.record_witness_observation();
        match self.witness.posture() {
            StoreAuthenticityWitnessPosture::Absent => {
                counters.record_missing_witness_denial();
                Err(denial(
                    StoreAuthenticityCheckDenialKind::MissingWitness,
                    self.requirement,
                    self.scope_identity,
                    counters,
                ))
            }
            StoreAuthenticityWitnessPosture::Unavailable => {
                counters.record_unavailable_denial();
                Err(denial(
                    StoreAuthenticityCheckDenialKind::Unavailable,
                    self.requirement,
                    self.scope_identity,
                    counters,
                ))
            }
            StoreAuthenticityWitnessPosture::Unsupported => {
                counters.record_unsupported_denial();
                Err(denial(
                    StoreAuthenticityCheckDenialKind::Unsupported,
                    self.requirement,
                    self.scope_identity,
                    counters,
                ))
            }
            StoreAuthenticityWitnessPosture::Stale(binding) => {
                reject_wrong_physical_identity(
                    binding,
                    self.physical_identity,
                    self.requirement,
                    self.scope_identity,
                    &mut counters,
                )?;
                counters.record_stale_witness_denial();
                Err(denial_for_binding(
                    StoreAuthenticityCheckDenialKind::StaleWitness,
                    self.requirement,
                    self.scope_identity,
                    binding,
                    counters,
                ))
            }
            StoreAuthenticityWitnessPosture::Failed(binding) => {
                reject_wrong_physical_identity(
                    binding,
                    self.physical_identity,
                    self.requirement,
                    self.scope_identity,
                    &mut counters,
                )?;
                reject_wrong_scope(
                    binding,
                    self.scope_identity,
                    self.requirement,
                    &mut counters,
                )?;
                counters.record_failed_denial();
                Err(denial(
                    StoreAuthenticityCheckDenialKind::Failed,
                    self.requirement,
                    self.scope_identity,
                    counters,
                ))
            }
            StoreAuthenticityWitnessPosture::Verified(binding) => {
                reject_wrong_physical_identity(
                    binding,
                    self.physical_identity,
                    self.requirement,
                    self.scope_identity,
                    &mut counters,
                )?;
                reject_wrong_scope(
                    binding,
                    self.scope_identity,
                    self.requirement,
                    &mut counters,
                )?;
                counters.record_verified_result();
                Ok(StoreAuthenticityResult::verified(
                    self.requirement,
                    self.scope_identity,
                    self.physical_identity,
                    counters.snapshot(),
                ))
            }
        }
    }
}

fn reject_wrong_physical_identity<I: Copy + Eq>(
    binding: StoreAuthenticityWitnessBinding<I>,
    expected_physical_identity: I,
    requirement: StoreAuthenticityRequirement,
    expected_scope: StoreSecurityScopeIdentity,
    counters: &mut StoreAuthenticityCheckCounterRecorder,
) -> Result<(), StoreAuthenticityCheckDenial> {
    if binding.physical_identity() == expected_physical_identity {
        return Ok(());
    }
    counters.record_wrong_physical_identity_denial();
    Err(denial(
        StoreAuthenticityCheckDenialKind::WrongPhysicalIdentity,
        requirement,
        expected_scope,
        *counters,
    ))
}

fn reject_wrong_scope<I>(
    binding: StoreAuthenticityWitnessBinding<I>,
    expected_scope: StoreSecurityScopeIdentity,
    requirement: StoreAuthenticityRequirement,
    counters: &mut StoreAuthenticityCheckCounterRecorder,
) -> Result<(), StoreAuthenticityCheckDenial> {
    if binding.scope_identity() == expected_scope {
        return Ok(());
    }
    counters.record_wrong_scope_denial();
    Err(denial(
        StoreAuthenticityCheckDenialKind::WrongScope,
        requirement,
        expected_scope,
        *counters,
    ))
}

fn denial_for_binding<I>(
    kind: StoreAuthenticityCheckDenialKind,
    requirement: StoreAuthenticityRequirement,
    scope_identity: StoreSecurityScopeIdentity,
    _binding: StoreAuthenticityWitnessBinding<I>,
    counters: StoreAuthenticityCheckCounterRecorder,
) -> StoreAuthenticityCheckDenial {
    denial(kind, requirement, scope_identity, counters)
}

fn denial(
    kind: StoreAuthenticityCheckDenialKind,
    requirement: StoreAuthenticityRequirement,
    scope_identity: StoreSecurityScopeIdentity,
    counters: StoreAuthenticityCheckCounterRecorder,
) -> StoreAuthenticityCheckDenial {
    StoreAuthenticityCheckDenial::new(kind, requirement, scope_identity, counters.snapshot())
}
