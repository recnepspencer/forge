use forge_query::facade::{
    ForgeQueryAdmittedConfiguredDomainHandle, ForgeQueryDomainOperatingContext,
    ForgeQueryOrdinaryNextStep, ForgeQueryOrdinaryOutcome, ForgeQueryOrdinaryPosture,
    ForgeQueryOrdinaryPostureKind,
};

use crate::bindings::query_native_planar_signed_area::authoring::CertifiedSignedArea2DEntry;
use crate::bindings::query_native_planar_signed_area::domain::CertifiedSignedArea2DQueryDomain;
use crate::planar_contracts::signed_area_2d::{
    certify_signed_area, CertifiedSignedArea2DDenial, CertifiedSignedArea2DPerformanceCounters,
    CertifiedSignedArea2DReceipt,
};

#[derive(Clone, Debug, PartialEq)]
pub enum CertifiedSignedArea2DFactError {
    OutcomeNotBound {
        kind: ForgeQueryOrdinaryPostureKind,
        reason: String,
        next_step: ForgeQueryOrdinaryNextStep,
    },
    AreaBasis {
        denial: CertifiedSignedArea2DDenial,
    },
}

impl CertifiedSignedArea2DFactError {
    fn outcome_not_bound(posture: &ForgeQueryOrdinaryPosture) -> Self {
        Self::OutcomeNotBound {
            kind: posture.kind(),
            reason: posture.reason().to_string(),
            next_step: posture.next_step(),
        }
    }
}

pub fn certified_signed_area_2d_facts<WC>(
    entry: &CertifiedSignedArea2DEntry,
    signed_area_handle: &ForgeQueryAdmittedConfiguredDomainHandle<
        CertifiedSignedArea2DQueryDomain,
        WC,
    >,
) -> Result<CertifiedSignedArea2DReceipt, CertifiedSignedArea2DFactError>
where
    WC: ForgeQueryDomainOperatingContext<CertifiedSignedArea2DQueryDomain>,
{
    match signed_area_handle.orchestrate_declaration_entry_outcome(entry.clone()) {
        ForgeQueryOrdinaryOutcome::Bound(envelope) => {
            let measurement = certify_signed_area(entry.case().basis())
                .map_err(|denial| CertifiedSignedArea2DFactError::AreaBasis { denial })?;
            let certified_basis = entry
                .case()
                .basis()
                .clone()
                .with_measurement(measurement.clone());
            let envelope_digest = format!("{:?}", envelope.envelope_digest());
            let declaration_digest = envelope.declaration_digest().to_string();
            let basis_part_count = CertifiedSignedArea2DReceipt::digest_parts(
                &certified_basis,
                &declaration_digest,
                &envelope_digest,
            )
            .len();
            let fact_digest = CertifiedSignedArea2DReceipt::fact_digest_for(
                &certified_basis,
                &declaration_digest,
                &envelope_digest,
            );
            Ok(CertifiedSignedArea2DReceipt::new(
                certified_basis,
                declaration_digest,
                envelope_digest,
                fact_digest,
                CertifiedSignedArea2DPerformanceCounters::certified(
                    measurement.loop_edges_walked,
                    measurement.area_terms_evaluated,
                    measurement.precision_escalations,
                    measurement.local_scale_comparisons,
                    measurement.degeneracy_localization_breadth,
                    basis_part_count,
                ),
            ))
        }
        ForgeQueryOrdinaryOutcome::Ambiguous(posture)
        | ForgeQueryOrdinaryOutcome::AspectConflict(posture)
        | ForgeQueryOrdinaryOutcome::AuthorityMismatch(posture)
        | ForgeQueryOrdinaryOutcome::BasisMismatch(posture)
        | ForgeQueryOrdinaryOutcome::Deferred(posture)
        | ForgeQueryOrdinaryOutcome::Denied(posture)
        | ForgeQueryOrdinaryOutcome::ExplicitNarrowingRequired(posture)
        | ForgeQueryOrdinaryOutcome::Failed(posture)
        | ForgeQueryOrdinaryOutcome::MissingRequiredAspect(posture)
        | ForgeQueryOrdinaryOutcome::RebindRequired(posture)
        | ForgeQueryOrdinaryOutcome::Refused(posture)
        | ForgeQueryOrdinaryOutcome::Stale(posture)
        | ForgeQueryOrdinaryOutcome::Unavailable(posture)
        | ForgeQueryOrdinaryOutcome::Unsupported(posture)
        | ForgeQueryOrdinaryOutcome::WrongHandle(posture)
        | ForgeQueryOrdinaryOutcome::WrongWorld(posture) => {
            Err(CertifiedSignedArea2DFactError::outcome_not_bound(&posture))
        }
    }
}
