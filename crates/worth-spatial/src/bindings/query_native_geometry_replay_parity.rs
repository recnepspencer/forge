use forge_query::facade::{
    ForgeQueryAdmittedConfiguredDomainHandle, ForgeQueryDeclarationCanonicalEntry,
    ForgeQueryDeclarationInput, ForgeQueryDomainOperatingContext, ForgeQueryOrdinaryOutcome,
};
use worth_primitives::{truth_digest_parts, TruthDigestScope};

use crate::bindings::authority::SpatialBindingKind;
use crate::bindings::query_native_branch_local_geometry_inspection::PrimitiveRebindingBranchLocalInspectionFactReceipt;
use crate::bindings::query_native_geometry_replay_parity_artifact::{
    ensure_equal, sorted_join, PrimitiveRebindingReplayParity, PrimitiveRebindingReplayParityError,
};
use crate::bindings::query_native_historical_geometry_inspection::PrimitiveRebindingHistoricalInspectionFactReceipt;
use crate::bindings::query_native_rebinding::PrimitiveRebindingQueryDomain;
use crate::bindings::query_native_retained_geometry::GeometryReplayParityDeclarationFamily;
use crate::bindings::query_native_retained_view_payload::PrimitiveRebindingRetainedViewPayload;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PrimitiveRebindingReplaySource {
    Historical(PrimitiveRebindingHistoricalInspectionFactReceipt),
    BranchLocal(PrimitiveRebindingBranchLocalInspectionFactReceipt),
}

impl PrimitiveRebindingReplaySource {
    fn payload(&self) -> &PrimitiveRebindingRetainedViewPayload {
        match self {
            Self::Historical(value) => value.payload(),
            Self::BranchLocal(value) => value.payload(),
        }
    }

    fn source_kind(&self) -> &'static str {
        match self {
            Self::Historical(_) => "historical",
            Self::BranchLocal(_) => "branch_local",
        }
    }

    fn binding_kind(&self) -> SpatialBindingKind {
        match self {
            Self::Historical(value) => value.source().binding_kind(),
            Self::BranchLocal(value) => value.source().binding_kind(),
        }
    }

    fn fact_digest(&self) -> String {
        self.payload().replay_source_fact_digest(self.source_kind())
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct GeometryReplayParityEntry {
    left_source: PrimitiveRebindingReplaySource,
    right_source: PrimitiveRebindingReplaySource,
}

impl GeometryReplayParityEntry {
    pub fn compare<C>(
        &self,
        handle: &ForgeQueryAdmittedConfiguredDomainHandle<PrimitiveRebindingQueryDomain, C>,
    ) -> Result<PrimitiveRebindingReplayParity, PrimitiveRebindingReplayParityError>
    where
        C: ForgeQueryDomainOperatingContext<PrimitiveRebindingQueryDomain>,
    {
        let replay_entry = entry_envelope(self, handle)?;
        let left_payload = self.left_source.payload();
        let right_payload = self.right_source.payload();

        ensure_equal(
            format!("{:?}", self.left_source.binding_kind()),
            format!("{:?}", self.right_source.binding_kind()),
            "replay parity requires retained histories to preserve the same binding family identity",
        )?;
        ensure_equal(
            left_payload.prior_binding_identity(),
            right_payload.prior_binding_identity(),
            "replay parity requires retained histories to preserve the same binding identity",
        )?;
        ensure_equal(
            left_payload.prior_site_identity(),
            right_payload.prior_site_identity(),
            "replay parity requires retained histories to preserve the same anchor identity",
        )?;
        ensure_equal(
            format!("{:?}", left_payload.outcome_class()),
            format!("{:?}", right_payload.outcome_class()),
            "replay parity requires equivalent retained histories to preserve rebinding outcome class",
        )?;
        ensure_equal(
            format!("{:?}", left_payload.continuity_class()),
            format!("{:?}", right_payload.continuity_class()),
            "replay parity requires equivalent retained histories to preserve continuity class",
        )?;
        ensure_equal(
            format!("{:?}", left_payload.motion_posture()),
            format!("{:?}", right_payload.motion_posture()),
            "replay parity requires equivalent retained histories to preserve motion posture",
        )?;
        ensure_equal(
            format!("{:?}", left_payload.neighborhood_family()),
            format!("{:?}", right_payload.neighborhood_family()),
            "replay parity requires equivalent retained histories to preserve binding family",
        )?;
        ensure_equal(
            left_payload.selected_candidate_identity().unwrap_or("none"),
            right_payload.selected_candidate_identity().unwrap_or("none"),
            "replay parity requires equivalent retained histories to preserve selected binding identity",
        )?;
        ensure_equal(
            left_payload.selected_candidate_label().unwrap_or("none"),
            right_payload.selected_candidate_label().unwrap_or("none"),
            "replay parity requires equivalent retained histories to preserve selected binding explanation",
        )?;
        ensure_equal(
            format!("{:?}", left_payload.unsupported_reason()),
            format!("{:?}", right_payload.unsupported_reason()),
            "replay parity requires equivalent retained histories to preserve diagnostics posture",
        )?;
        ensure_equal(
            sorted_join(left_payload.candidate_identities()),
            sorted_join(right_payload.candidate_identities()),
            "replay parity requires equivalent retained histories to preserve candidate identity explanation basis",
        )?;
        ensure_equal(
            sorted_join(left_payload.candidate_labels()),
            sorted_join(right_payload.candidate_labels()),
            "replay parity requires equivalent retained histories to preserve candidate label explanation basis",
        )?;
        ensure_equal(
            sorted_join(left_payload.candidate_site_identities()),
            sorted_join(right_payload.candidate_site_identities()),
            "replay parity requires equivalent retained histories to preserve candidate anchor explanation basis",
        )?;

        let left_ordinary = left_payload.ordinary_shape();
        let right_ordinary = right_payload.ordinary_shape();
        if left_ordinary != right_ordinary {
            return Err(PrimitiveRebindingReplayParityError::ReplayNextStepMismatch {
                reason:
                    "replay parity requires retained histories to preserve the same ordinary next-step truth",
                left_kind: left_ordinary.kind(),
                right_kind: right_ordinary.kind(),
                left_next_step: left_ordinary.next_step(),
                right_next_step: right_ordinary.next_step(),
                left_posture_kind: left_ordinary.posture_kind(),
                right_posture_kind: right_ordinary.posture_kind(),
            });
        }

        Ok(PrimitiveRebindingReplayParity::new(
            truth_digest_parts(
                TruthDigestScope::ArtifactIdentity,
                &[
                    format!("retained_entry:{}", replay_entry.declaration_digest()),
                    format!(
                        "retained_progression:{}",
                        replay_entry.progression_digest().unwrap_or("none")
                    ),
                    format!(
                        "retained_route:{}",
                        replay_entry.route_plan_digest().unwrap_or("none")
                    ),
                    format!("retained_receipt:{:?}", replay_entry.receipt_digest()),
                    format!("retained_envelope:{:?}", replay_entry.envelope_digest()),
                    format!("binding_kind:{:?}", self.left_source.binding_kind()),
                    format!("outcome:{:?}", left_payload.outcome_class()),
                    format!("continuity:{:?}", left_payload.continuity_class()),
                    format!("motion:{:?}", left_payload.motion_posture()),
                    format!("family:{:?}", left_payload.neighborhood_family()),
                    format!("prior:{}", left_payload.prior_binding_identity()),
                    format!("prior_site:{}", left_payload.prior_site_identity()),
                    format!(
                        "selected_identity:{}",
                        left_payload.selected_candidate_identity().unwrap_or("none")
                    ),
                    format!(
                        "selected_label:{}",
                        left_payload.selected_candidate_label().unwrap_or("none")
                    ),
                    format!("unsupported:{:?}", left_payload.unsupported_reason()),
                    format!(
                        "candidate_identities:{}",
                        sorted_join(left_payload.candidate_identities())
                    ),
                    format!(
                        "candidate_labels:{}",
                        sorted_join(left_payload.candidate_labels())
                    ),
                    format!(
                        "candidate_sites:{}",
                        sorted_join(left_payload.candidate_site_identities())
                    ),
                    format!("ordinary_kind:{}", left_ordinary.kind()),
                    format!("ordinary_posture:{:?}", left_ordinary.posture_kind()),
                    format!("next_step:{:?}", left_ordinary.next_step()),
                ],
            ),
            left_payload.prior_binding_identity().to_string(),
            left_payload.prior_site_identity().to_string(),
            left_payload.outcome_class(),
            left_payload.continuity_class(),
            left_payload
                .selected_candidate_identity()
                .map(ToOwned::to_owned),
            left_payload
                .selected_candidate_label()
                .map(ToOwned::to_owned),
            format!("{:?}", left_payload.unsupported_reason()),
            left_ordinary.next_step(),
            left_ordinary.kind(),
            self.left_source.source_kind(),
            self.right_source.source_kind(),
        ))
    }
}

impl ForgeQueryDeclarationInput<PrimitiveRebindingQueryDomain> for GeometryReplayParityEntry {
    type Family = GeometryReplayParityDeclarationFamily;

    fn canonical_declaration_entries(&self) -> Vec<ForgeQueryDeclarationCanonicalEntry> {
        vec![
            ForgeQueryDeclarationCanonicalEntry::text(
                "retained_view.kind",
                "geometry_replay_parity",
            ),
            ForgeQueryDeclarationCanonicalEntry::text("left.kind", self.left_source.source_kind()),
            ForgeQueryDeclarationCanonicalEntry::text(
                "right.kind",
                self.right_source.source_kind(),
            ),
            ForgeQueryDeclarationCanonicalEntry::text(
                "left.fact_digest",
                self.left_source.fact_digest(),
            ),
            ForgeQueryDeclarationCanonicalEntry::text(
                "right.fact_digest",
                self.right_source.fact_digest(),
            ),
        ]
    }
}

pub fn geometry_replay_parity_entry(
    left_source: PrimitiveRebindingReplaySource,
    right_source: PrimitiveRebindingReplaySource,
) -> GeometryReplayParityEntry {
    GeometryReplayParityEntry {
        left_source,
        right_source,
    }
}

fn entry_envelope<C>(
    entry: &GeometryReplayParityEntry,
    handle: &ForgeQueryAdmittedConfiguredDomainHandle<PrimitiveRebindingQueryDomain, C>,
) -> Result<
    forge_query::facade::ForgeQueryDeclarationEnvelope<
        PrimitiveRebindingQueryDomain,
        GeometryReplayParityEntry,
    >,
    PrimitiveRebindingReplayParityError,
>
where
    C: ForgeQueryDomainOperatingContext<PrimitiveRebindingQueryDomain>,
{
    match handle.orchestrate_declaration_entry_outcome(entry.clone()) {
        ForgeQueryOrdinaryOutcome::Bound(envelope) => Ok(envelope),
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
            Err(PrimitiveRebindingReplayParityError::EntryOutcomeNotBound {
                kind: posture.kind(),
                reason: posture.reason().to_string(),
                next_step: posture.next_step(),
            })
        }
    }
}
