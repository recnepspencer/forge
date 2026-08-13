use std::marker::PhantomData;

use worth_query_admission::facade::authenticated_principal::WorthQueryRequestScope;
use worth_query_declaration::facade::application_schema::ApplicationSchema;
#[cfg(test)]
use worth_relational::facade::bridge::RelationalBridgeTruthViewBasisDenial;
use worth_relational::facade::runtime::{
    RelationalApplicationCommitBasisDenial, RelationalExecutionBasisLease,
    RelationalRetainedCommitSnapshotDenialKind,
};

use super::super::{
    historical_authority::WorthQueryApplicationHistoricalReadSource,
    WorthQueryApplicationHistoricalBasis, WorthQueryApplicationHistoricalRead,
};
use super::validation::{denial, validate_truth_view_request};
use crate::domain_computation::primary_graph::{
    application_query::{
        WorthQueryApplicationQueryAdmissionDenial, WorthQueryApplicationQueryAdmissionDenialKind,
    },
    WorthQueryPrimaryGraphApplicationRuntime,
};

impl<Schema> WorthQueryPrimaryGraphApplicationRuntime<Schema>
where
    Schema: ApplicationSchema,
{
    pub fn admit_application_historical_basis(
        &self,
        read: WorthQueryApplicationHistoricalRead,
        request: &WorthQueryRequestScope,
    ) -> Result<
        WorthQueryApplicationHistoricalBasis<Schema>,
        WorthQueryApplicationQueryAdmissionDenial,
    > {
        validate_truth_view_request(request)?;
        let lease = self.admit_historical_execution_lease(read.into_source())?;
        self.ensure_truth_view_indexes(lease.version_id())?;
        let lease = self.basis_leases.register(lease);
        validate_truth_view_request(request)?;
        Ok(WorthQueryApplicationHistoricalBasis {
            runtime_authority: self.runtime.authority_identity(),
            schema_binding: self.installed_schema.binding_identity(),
            graph_authority_identity: self
                .primary_graph_authority
                .authority_identity()
                .to_string(),
            provider_identity: self.primary_graph_authority.provider_identity().to_string(),
            expires_at: request.deadline(),
            lease,
            _schema: PhantomData,
        })
    }

    fn admit_historical_execution_lease(
        &self,
        source: WorthQueryApplicationHistoricalReadSource,
    ) -> Result<RelationalExecutionBasisLease, WorthQueryApplicationQueryAdmissionDenial> {
        match source {
            WorthQueryApplicationHistoricalReadSource::ApplicationCommit {
                provider_runtime_instance_id,
                commit,
            } => self
                .execution_basis_source
                .admit_application_commit(provider_runtime_instance_id, &commit)
                .map_err(map_application_commit_basis_denial),
            #[cfg(test)]
            source @ WorthQueryApplicationHistoricalReadSource::BridgeSelector(_) => {
                let evaluation = self
                    .bridge
                    .evaluate(source.into_evaluation_request())
                    .map_err(|error| {
                        denial(
                            WorthQueryApplicationQueryAdmissionDenialKind::TruthViewUnavailable,
                            error.to_string(),
                        )
                    })?;
                self.relational_source
                    .admit_truth_view_execution_basis(&evaluation)
                    .map_err(map_basis_unavailable)
            }
        }
    }
}

fn map_application_commit_basis_denial(
    error: RelationalApplicationCommitBasisDenial,
) -> WorthQueryApplicationQueryAdmissionDenial {
    match error {
        RelationalApplicationCommitBasisDenial::RetainedCommit(retained)
            if retained.kind() == RelationalRetainedCommitSnapshotDenialKind::ForeignRuntime =>
        {
            denial(
                WorthQueryApplicationQueryAdmissionDenialKind::ForeignHistoricalReceipt,
                "application commit receipt",
            )
        }
        error => denial(
            WorthQueryApplicationQueryAdmissionDenialKind::BasisUnavailable,
            format!("{error:?}"),
        ),
    }
}

#[cfg(test)]
fn map_basis_unavailable(
    error: RelationalBridgeTruthViewBasisDenial,
) -> WorthQueryApplicationQueryAdmissionDenial {
    denial(
        WorthQueryApplicationQueryAdmissionDenialKind::BasisUnavailable,
        format!("{error:?}"),
    )
}
