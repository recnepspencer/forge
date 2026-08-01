use worth_query_installation::facade::WorthQueryInstalledGraphObligationKind;

use super::{
    basis_release::WorthQueryManagedGraphWorkBasisHandoff,
    WorthQueryCompleteGraphWorkDecisionReadSet, WorthQueryGraphWorkSessionReleaseReceipt,
    WorthQueryGraphWorkSessionTerminalDenial, WorthQueryManagedGraphWorkSession,
    WorthQueryMutationGraphWorkLane,
};

#[cfg(test)]
use super::basis_release::WorthQueryGraphWorkBasisRelease;

pub(in crate::domain_computation) struct WorthQueryMutationGraphWorkProgression<Basis> {
    session: WorthQueryManagedGraphWorkSession<WorthQueryMutationGraphWorkLane, Basis>,
}

pub(in crate::domain_computation) struct WorthQueryManagedMutationGraphWorkProgression<Basis> {
    session: WorthQueryManagedGraphWorkSession<WorthQueryMutationGraphWorkLane, Basis>,
}

pub(in crate::domain_computation) struct WorthQueryMutationManagedRunAdmissionFailure<Basis> {
    progression: WorthQueryMutationGraphWorkProgression<Basis>,
}

impl<Basis> WorthQueryMutationManagedRunAdmissionFailure<Basis> {
    pub(in crate::domain_computation) fn into_progression(
        self,
    ) -> WorthQueryMutationGraphWorkProgression<Basis> {
        self.progression
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(in crate::domain_computation) enum WorthQueryManagedMutationTerminalDenial {
    IncompleteOwnerProgression,
    Cleanup(WorthQueryGraphWorkSessionTerminalDenial),
}

#[cfg(test)]
impl<Basis> WorthQueryManagedGraphWorkSession<WorthQueryMutationGraphWorkLane, Basis>
where
    Basis: WorthQueryGraphWorkBasisRelease,
{
    pub(in crate::domain_computation) fn abort_mutation(
        self,
    ) -> WorthQueryGraphWorkSessionReleaseReceipt {
        self.release()
    }
}

impl<Basis> WorthQueryCompleteGraphWorkDecisionReadSet<WorthQueryMutationGraphWorkLane, Basis> {
    pub(in crate::domain_computation) fn into_mutation_progression(
        self,
    ) -> WorthQueryMutationGraphWorkProgression<Basis> {
        WorthQueryMutationGraphWorkProgression {
            session: self.session,
        }
    }
}

impl<Basis> WorthQueryMutationGraphWorkProgression<Basis>
where
    Basis: WorthQueryManagedGraphWorkBasisHandoff,
{
    pub(in crate::domain_computation) const fn session(
        &self,
    ) -> &WorthQueryManagedGraphWorkSession<WorthQueryMutationGraphWorkLane, Basis> {
        &self.session
    }

    pub(in crate::domain_computation) fn admit_managed_run(
        mut self,
        admission: &crate::domain_computation::managed_run::WorthQueryManagedRunAdmission<'_>,
        request: crate::domain_computation::managed_run::WorthQueryManagedTruthReadRequest,
    ) -> Result<
        (
            crate::domain_computation::managed_run::WorthQueryAdmittedDirectRun,
            WorthQueryManagedMutationGraphWorkProgression<Basis>,
        ),
        WorthQueryMutationManagedRunAdmissionFailure<Basis>,
    > {
        let attempt = self
            .session
            .direct_attempt
            .take()
            .expect("a pre-managed mutation progression owns one direct attempt");
        let relational_basis = self
            .session
            .basis
            .as_mut()
            .expect("a pre-managed mutation progression owns one basis shell")
            .take_managed_relational_basis()
            .expect("a pre-managed mutation progression owns one Relational basis");
        match admission.admit_direct_with_retained_basis(attempt, request, relational_basis) {
            Ok(admitted) => Ok((
                admitted,
                WorthQueryManagedMutationGraphWorkProgression {
                    session: self.session,
                },
            )),
            Err(failure) => {
                let (attempt, relational_basis) = failure
                    .into_retained_resources()
                    .expect("retained-basis admission returns both move-only resources");
                self.session.direct_attempt = Some(attempt);
                let restored = self
                    .session
                    .basis
                    .as_mut()
                    .expect("a denied managed admission retains its basis shell")
                    .restore_managed_relational_basis(relational_basis);
                assert!(
                    restored.is_ok(),
                    "a denied managed admission restores into an empty basis shell"
                );
                Err(WorthQueryMutationManagedRunAdmissionFailure { progression: self })
            }
        }
    }

    pub(in crate::domain_computation) fn abort(self) -> WorthQueryGraphWorkSessionReleaseReceipt {
        self.session.release()
    }
}

impl<Basis> WorthQueryManagedMutationGraphWorkProgression<Basis>
where
    Basis: WorthQueryManagedGraphWorkBasisHandoff,
{
    pub(in crate::domain_computation) const fn session(
        &self,
    ) -> &WorthQueryManagedGraphWorkSession<WorthQueryMutationGraphWorkLane, Basis> {
        &self.session
    }

    pub(in crate::domain_computation) fn session_mut(
        &mut self,
    ) -> &mut WorthQueryManagedGraphWorkSession<WorthQueryMutationGraphWorkLane, Basis> {
        &mut self.session
    }

    pub(in crate::domain_computation) fn finish_mutation(
        self,
        cleanup: &crate::domain_computation::managed_run::WorthQueryDirectRunCleanupReceipt,
    ) -> Result<WorthQueryGraphWorkSessionReleaseReceipt, WorthQueryManagedMutationTerminalDenial>
    {
        if !owner_progression_is_complete(&self.session) {
            return Err(WorthQueryManagedMutationTerminalDenial::IncompleteOwnerProgression);
        }
        self.session
            .release_after_managed_cleanup(cleanup)
            .map_err(WorthQueryManagedMutationTerminalDenial::Cleanup)
    }

    pub(in crate::domain_computation) fn abort(
        self,
        cleanup: &crate::domain_computation::managed_run::WorthQueryDirectRunCleanupReceipt,
    ) -> Result<WorthQueryGraphWorkSessionReleaseReceipt, WorthQueryManagedMutationTerminalDenial>
    {
        self.session
            .release_after_managed_cleanup(cleanup)
            .map_err(WorthQueryManagedMutationTerminalDenial::Cleanup)
    }
}

fn owner_progression_is_complete<Basis>(
    session: &WorthQueryManagedGraphWorkSession<WorthQueryMutationGraphWorkLane, Basis>,
) -> bool {
    session
        .plan()
        .required_obligations()
        .iter()
        .filter(|row| {
            matches!(
                row.kind(),
                WorthQueryInstalledGraphObligationKind::EffectApplication
                    | WorthQueryInstalledGraphObligationKind::InvariantExecution
            )
        })
        .all(|row| {
            row.owner_progression()
                .iter()
                .enumerate()
                .all(|(ordinal, _)| {
                    session
                        .completed_owner_steps
                        .contains(&(row.identity().slot(), ordinal))
                })
        })
}
