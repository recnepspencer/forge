use forge_foundational::facade::CanonicalDerivedDigest;
use forge_query::facade::{
    ForgeQueryAdmittedConfiguredDomainHandle, ForgeQueryDeclarationEnvelopeChecked,
    ForgeQueryDomainOperatingContext,
};

use crate::bindings::query_native_planar_projection_consumption::authoring::ProjectionConsumedPlanarFactsEntry;
use crate::bindings::query_native_planar_projection_consumption::domain::ProjectionConsumedPlanarFactsQueryDomain;
use crate::bindings::query_native_planar_projection_consumption::inspection::ProjectionConsumedPlanarFactsInspectionRow;
use crate::planar_contracts::projection_consumed_facts::{
    projection_consumed_planar_fact_digest, ProjectionConsumedPlanarFactsCounters,
    ProjectionConsumedPlanarFactsReceipt,
};

#[derive(Clone, Debug, PartialEq)]
pub enum ProjectionConsumedPlanarFactsFactError {
    TruncatedProjectionConsumptionBasis { reason: &'static str },
}

impl ProjectionConsumedPlanarFactsFactError {
    pub fn reason(&self) -> &str {
        match self {
            Self::TruncatedProjectionConsumptionBasis { reason } => reason,
        }
    }
}

pub fn projection_consumed_planar_facts<WC>(
    entry: &ProjectionConsumedPlanarFactsEntry,
    handle: &ForgeQueryAdmittedConfiguredDomainHandle<ProjectionConsumedPlanarFactsQueryDomain, WC>,
) -> Result<ProjectionConsumedPlanarFactsReceipt, ProjectionConsumedPlanarFactsFactError>
where
    WC: ForgeQueryDomainOperatingContext<ProjectionConsumedPlanarFactsQueryDomain>,
{
    let query_artifacts = projection_consumed_query_artifacts(entry, handle)?;
    Ok(projection_consumed_receipt_from_query_artifacts(
        query_artifacts,
    ))
}

struct ProjectionConsumedQueryArtifacts {
    basis: crate::planar_contracts::projection_consumed_facts::ProjectionConsumedPlanarFactsBasis,
    declaration_digest: String,
    progression_digest: String,
    route_plan_digest: String,
    query_receipt_digest: String,
    envelope_digest: String,
    materialization_digest: String,
    projection_consumption_digest: String,
    inspection_row_count: usize,
}

fn projection_consumed_query_artifacts<WC>(
    entry: &ProjectionConsumedPlanarFactsEntry,
    handle: &ForgeQueryAdmittedConfiguredDomainHandle<ProjectionConsumedPlanarFactsQueryDomain, WC>,
) -> Result<ProjectionConsumedQueryArtifacts, ProjectionConsumedPlanarFactsFactError>
where
    WC: ForgeQueryDomainOperatingContext<ProjectionConsumedPlanarFactsQueryDomain>,
{
    match handle
        .declare_review_and_progress(entry.clone())
        .map(|progressed| handle.orchestrate_envelope_from_progressed_checked(progressed))
    {
        Ok(ForgeQueryDeclarationEnvelopeChecked::Enveloped(envelope)) => {
            let basis = entry.case().basis().clone();
            let declaration_digest = envelope.declaration_digest().to_string();
            let progression_digest =
                envelope
                    .progression_digest()
                    .ok_or(ProjectionConsumedPlanarFactsFactError::TruncatedProjectionConsumptionBasis {
                        reason: "projection-consumed planar facts require Query progression digest before downstream consumption",
                    })?
                    .to_string();
            let route_plan_digest =
                envelope
                    .route_plan_digest()
                    .ok_or(ProjectionConsumedPlanarFactsFactError::TruncatedProjectionConsumptionBasis {
                        reason: "projection-consumed planar facts require Query route-plan digest before downstream consumption",
                    })?
                    .to_string();
            let query_receipt_digest = canonical_digest_token(envelope.receipt_digest());
            let envelope_digest = canonical_digest_token(envelope.envelope_digest());
            let materialization_digest = materialization_digest_for(&basis);
            let projection_consumption_digest =
                ProjectionConsumedPlanarFactsReceipt::projection_consumption_digest_for(
                    &basis,
                    &declaration_digest,
                    &progression_digest,
                    &route_plan_digest,
                    &query_receipt_digest,
                    &envelope_digest,
                    &materialization_digest,
                );
            let inspection_rows = ProjectionConsumedPlanarFactsInspectionRow::from_basis(&basis);
            Ok(ProjectionConsumedQueryArtifacts {
                basis,
                declaration_digest,
                progression_digest,
                route_plan_digest,
                query_receipt_digest,
                envelope_digest,
                materialization_digest,
                projection_consumption_digest,
                inspection_row_count: inspection_rows.len(),
            })
        }
        Ok(
            ForgeQueryDeclarationEnvelopeChecked::Deferred(_)
            | ForgeQueryDeclarationEnvelopeChecked::Denied(_)
            | ForgeQueryDeclarationEnvelopeChecked::Failed(_),
        ) => Err(
            ProjectionConsumedPlanarFactsFactError::TruncatedProjectionConsumptionBasis {
                reason:
                    "projection-consumed planar facts require an enveloped Query declaration entry",
            },
        ),
        Err(_) => Err(
            ProjectionConsumedPlanarFactsFactError::TruncatedProjectionConsumptionBasis {
                reason:
                    "projection-consumed planar facts require a progressed Query declaration entry",
            },
        ),
    }
}

fn projection_consumed_receipt_from_query_artifacts(
    query_artifacts: ProjectionConsumedQueryArtifacts,
) -> ProjectionConsumedPlanarFactsReceipt {
    let counters = ProjectionConsumedPlanarFactsCounters::consumed(
        retained_source_row_count(&query_artifacts.basis),
        query_artifacts.basis.projection_receipts().len(),
        1,
        query_artifacts.inspection_row_count,
    );
    ProjectionConsumedPlanarFactsReceipt::new(
        query_artifacts.basis,
        query_artifacts.declaration_digest,
        query_artifacts.progression_digest,
        query_artifacts.route_plan_digest,
        query_artifacts.query_receipt_digest,
        query_artifacts.envelope_digest,
        query_artifacts.materialization_digest,
        query_artifacts.projection_consumption_digest,
        counters,
    )
}

fn retained_source_row_count(
    basis: &crate::planar_contracts::projection_consumed_facts::ProjectionConsumedPlanarFactsBasis,
) -> usize {
    basis
        .retained_planar_facts_receipt()
        .basis()
        .boolean_readiness_receipt()
        .basis()
        .family_rows()
        .len()
}

fn materialization_digest_for(
    basis: &crate::planar_contracts::projection_consumed_facts::ProjectionConsumedPlanarFactsBasis,
) -> String {
    let mut parts = vec![
        format!("basis:{}", basis.materialization_basis_identity()),
        format!("retained:{}", basis.retained_planar_fact_digest()),
    ];
    parts.extend(
        basis
            .projection_receipts()
            .iter()
            .map(|receipt| format!("projection:{}", receipt.fact_digest())),
    );
    projection_consumed_planar_fact_digest(&parts)
}

fn canonical_digest_token(digest: &CanonicalDerivedDigest) -> String {
    let hex = digest
        .value()
        .bytes()
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect::<String>();
    format!("{}:{hex}", digest.metadata().algorithm().id().as_str())
}
