use crate::domain_computation::authorization::WorthQueryOperationGraphWorkSession;
use crate::domain_computation::primary_graph::WorthQueryAdmittedApplicationOperation;
use crate::domain_computation::provider_session::{
    WorthQueryGraphWorkDecisionReadSetDenial, WorthQueryManagedMutationGraphWorkProgression,
    WorthQueryManagedMutationTerminalDenial, WorthQueryMutationGraphWorkProgression,
};

pub(in crate::domain_computation::primary_graph) struct WorthQueryProgressingApplicationOperation<
    Schema,
    Operation,
    Input,
    Scope,
> {
    admission: WorthQueryAdmittedApplicationOperation<Schema, Operation, Input, Scope>,
    graph_work: WorthQueryMutationGraphWorkProgression<
        crate::domain_computation::primary_graph::WorthQueryApplicationSnapshotLease,
    >,
}

pub(super) struct WorthQueryManagedApplicationOperation<Schema, Operation, Input, Scope> {
    admission: WorthQueryAdmittedApplicationOperation<Schema, Operation, Input, Scope>,
    graph_work: WorthQueryManagedMutationGraphWorkProgression<
        crate::domain_computation::primary_graph::WorthQueryApplicationSnapshotLease,
    >,
}

pub(super) struct WorthQueryApplicationManagedAdmissionFailure<Schema, Operation, Input, Scope> {
    progressing: WorthQueryProgressingApplicationOperation<Schema, Operation, Input, Scope>,
}

impl<Schema, Operation, Input, Scope>
    WorthQueryApplicationManagedAdmissionFailure<Schema, Operation, Input, Scope>
{
    pub(super) fn into_progressing(
        self,
    ) -> WorthQueryProgressingApplicationOperation<Schema, Operation, Input, Scope> {
        self.progressing
    }
}

impl<Schema, Operation, Input, Scope>
    WorthQueryProgressingApplicationOperation<Schema, Operation, Input, Scope>
{
    pub(super) fn advance(
        mut admission: WorthQueryAdmittedApplicationOperation<Schema, Operation, Input, Scope>,
    ) -> Result<Self, WorthQueryGraphWorkDecisionReadSetDenial> {
        let session: WorthQueryOperationGraphWorkSession = admission.take_graph_work_session();
        let graph_work = session
            .complete_decision_read_set()?
            .into_mutation_progression();
        Ok(Self {
            admission,
            graph_work,
        })
    }

    pub(super) const fn admission(
        &self,
    ) -> &WorthQueryAdmittedApplicationOperation<Schema, Operation, Input, Scope> {
        &self.admission
    }

    pub(super) fn admission_mut(
        &mut self,
    ) -> &mut WorthQueryAdmittedApplicationOperation<Schema, Operation, Input, Scope> {
        &mut self.admission
    }

    pub(super) const fn graph_work(
        &self,
    ) -> &WorthQueryMutationGraphWorkProgression<
        crate::domain_computation::primary_graph::WorthQueryApplicationSnapshotLease,
    > {
        &self.graph_work
    }

    pub(super) fn authorization_revalidation_parts(
        &mut self,
    ) -> (
        &mut WorthQueryAdmittedApplicationOperation<Schema, Operation, Input, Scope>,
        &WorthQueryOperationGraphWorkSession,
    ) {
        (&mut self.admission, self.graph_work.session())
    }

    pub(super) fn abort(
        self,
    ) -> crate::domain_computation::provider_session::WorthQueryGraphWorkSessionReleaseReceipt {
        self.graph_work.abort()
    }
}

impl<Schema, Operation, Input, Scope>
    WorthQueryProgressingApplicationOperation<Schema, Operation, Input, Scope>
{
    pub(super) fn enter_managed_run(
        self,
        managed_admission: &crate::domain_computation::managed_run::WorthQueryManagedRunAdmission<
            '_,
        >,
        request: crate::domain_computation::managed_run::WorthQueryManagedTruthReadRequest,
    ) -> Result<
        (
            WorthQueryManagedApplicationOperation<Schema, Operation, Input, Scope>,
            crate::domain_computation::managed_run::WorthQueryAdmittedDirectRun,
        ),
        WorthQueryApplicationManagedAdmissionFailure<Schema, Operation, Input, Scope>,
    > {
        let WorthQueryProgressingApplicationOperation {
            admission,
            graph_work,
        } = self;
        match graph_work.admit_managed_run(managed_admission, request) {
            Ok((run, graph_work)) => Ok((
                WorthQueryManagedApplicationOperation {
                    admission,
                    graph_work,
                },
                run,
            )),
            Err(failure) => Err(WorthQueryApplicationManagedAdmissionFailure {
                progressing: WorthQueryProgressingApplicationOperation {
                    admission,
                    graph_work: failure.into_progression(),
                },
            }),
        }
    }
}

impl<Schema, Operation, Input, Scope>
    WorthQueryManagedApplicationOperation<Schema, Operation, Input, Scope>
{
    pub(super) fn parts_mut(
        &mut self,
    ) -> (
        &WorthQueryAdmittedApplicationOperation<Schema, Operation, Input, Scope>,
        &mut WorthQueryManagedMutationGraphWorkProgression<
            crate::domain_computation::primary_graph::WorthQueryApplicationSnapshotLease,
        >,
    ) {
        (&self.admission, &mut self.graph_work)
    }

    pub(super) fn finish_mutation(
        self,
        cleanup: &crate::domain_computation::managed_run::WorthQueryDirectRunCleanupReceipt,
    ) -> Result<
        crate::domain_computation::provider_session::WorthQueryGraphWorkSessionReleaseReceipt,
        WorthQueryManagedMutationTerminalDenial,
    > {
        self.graph_work.finish_mutation(cleanup)
    }

    pub(super) fn abort(
        self,
        cleanup: &crate::domain_computation::managed_run::WorthQueryDirectRunCleanupReceipt,
    ) -> Result<
        crate::domain_computation::provider_session::WorthQueryGraphWorkSessionReleaseReceipt,
        WorthQueryManagedMutationTerminalDenial,
    > {
        self.graph_work.abort(cleanup)
    }
}
