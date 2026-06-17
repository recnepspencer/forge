use forge_query::facade::{
    admit_identity_evolution_query, execute_admitted_identity_evolution_query, BasisDigest,
    CanonicalQueryDigest, IdentityEvolutionExecutionArtifact, IdentityEvolutionOutcomeFamily,
    IdentityEvolutionQueryContext, LineageTraversalDescriptor,
};

use super::counters::PlanarBooleanSplitPersistentNamingCounters;
use super::denial::{
    PlanarBooleanSplitPersistentNamingDenial, PlanarBooleanSplitPersistentNamingDenialKind,
};
use super::input::PlanarBooleanSplitPersistentNamingQueryBasis;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlanarBooleanSplitIdentityEvolutionOutcomeKind {
    PluralSplitSuccessors,
    SingularContinuity,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanSplitIdentityEvolutionRow {
    source_edge_identity: String,
    query_digest: String,
    basis_digest: String,
    lineage_digest: String,
    result_digest: String,
    outcome_kind: PlanarBooleanSplitIdentityEvolutionOutcomeKind,
    successor_identities: Vec<String>,
}

impl PlanarBooleanSplitIdentityEvolutionRow {
    pub(crate) fn from_query_artifact(
        source_edge_identity: &str,
        artifact: IdentityEvolutionExecutionArtifact,
    ) -> Result<Self, PlanarBooleanSplitPersistentNamingDenial> {
        let result_bundle = artifact.result_bundle();
        let (outcome_kind, successor_identities) = match result_bundle.outcome_family() {
            IdentityEvolutionOutcomeFamily::PluralIdentitySuccessorSet => {
                let successors = result_bundle
                    .as_plural_identity_successor_set()
                    .expect("plural outcome family must expose plural successors")
                    .successor_identities()
                    .to_vec();
                (
                    PlanarBooleanSplitIdentityEvolutionOutcomeKind::PluralSplitSuccessors,
                    successors,
                )
            }
            IdentityEvolutionOutcomeFamily::SingularIdentityContinuity => {
                let continuity = result_bundle
                    .as_singular_identity_continuity()
                    .expect("singular outcome family must expose singular continuity");
                (
                    PlanarBooleanSplitIdentityEvolutionOutcomeKind::SingularContinuity,
                    vec![continuity.authoritative_identity().to_string()],
                )
            }
            IdentityEvolutionOutcomeFamily::Ambiguity => {
                return Err(PlanarBooleanSplitPersistentNamingDenial::new(
                    PlanarBooleanSplitPersistentNamingDenialKind::AmbiguousIdentityEvolution,
                    source_edge_identity,
                    "split persistent naming must not auto-pick ambiguous identity evolution",
                ));
            }
            IdentityEvolutionOutcomeFamily::IdentityBreak => {
                return Err(PlanarBooleanSplitPersistentNamingDenial::new(
                    PlanarBooleanSplitPersistentNamingDenialKind::IdentityEvolutionBreak,
                    source_edge_identity,
                    "split persistent naming requires lineage continuity, not a broken identity",
                ));
            }
            IdentityEvolutionOutcomeFamily::Denied => {
                return Err(PlanarBooleanSplitPersistentNamingDenial::new(
                    PlanarBooleanSplitPersistentNamingDenialKind::DeniedIdentityEvolution,
                    source_edge_identity,
                    "split persistent naming requires an admitted identity evolution result",
                ));
            }
            IdentityEvolutionOutcomeFamily::AdvisoryIdentityCandidateSet => {
                return Err(PlanarBooleanSplitPersistentNamingDenial::new(
                    PlanarBooleanSplitPersistentNamingDenialKind::AdvisoryIdentityEvolutionNotAuthoritative,
                    source_edge_identity,
                    "advisory correspondence candidates cannot authorize persistent split names",
                ));
            }
        };

        Ok(Self {
            source_edge_identity: source_edge_identity.to_string(),
            query_digest: artifact.query_digest().to_string(),
            basis_digest: artifact.basis_digest().to_string(),
            lineage_digest: artifact.lineage_digest().to_string(),
            result_digest: artifact.result_digest().to_string(),
            outcome_kind,
            successor_identities,
        })
    }

    pub fn source_edge_identity(&self) -> &str {
        &self.source_edge_identity
    }
    pub fn query_digest(&self) -> &str {
        &self.query_digest
    }
    pub fn basis_digest(&self) -> &str {
        &self.basis_digest
    }
    pub fn lineage_digest(&self) -> &str {
        &self.lineage_digest
    }
    pub fn result_digest(&self) -> &str {
        &self.result_digest
    }
    pub fn outcome_kind(&self) -> PlanarBooleanSplitIdentityEvolutionOutcomeKind {
        self.outcome_kind
    }
    pub fn successor_identities(&self) -> &[String] {
        &self.successor_identities
    }
}

pub(crate) fn execute_split_identity_evolution(
    source_edge_identity: &str,
    query_basis: &PlanarBooleanSplitPersistentNamingQueryBasis,
    validation_receipt_identity: &str,
    counters: &mut PlanarBooleanSplitPersistentNamingCounters,
) -> Result<PlanarBooleanSplitIdentityEvolutionRow, PlanarBooleanSplitPersistentNamingDenial> {
    counters.inspected_source_identity();
    let query_digest = CanonicalQueryDigest::from_domain_parts(&[
        "worth.spatial.planar_boolean.split_persistent_naming.identity_evolution".to_string(),
        format!("source-edge:{source_edge_identity}"),
        format!("validation:{validation_receipt_identity}"),
        format!(
            "topology-domain:{}",
            query_basis.topology_query_domain_identity()
        ),
        format!(
            "persistent-name-live-view:{}",
            query_basis.persistent_name_live_view_identity()
        ),
    ]);
    let basis_digest = BasisDigest::from_domain_parts(&[
        "worth.spatial.planar_boolean.split_persistent_naming.basis".to_string(),
        format!("source-edge:{source_edge_identity}"),
        format!("validation:{validation_receipt_identity}"),
        format!(
            "naming-attachment-report:{}",
            query_basis.naming_attachment_report_identity()
        ),
    ]);
    let query_context = IdentityEvolutionQueryContext::lineage_traversal(
        query_digest,
        basis_digest,
        LineageTraversalDescriptor::direct_split_successors(source_edge_identity),
    );
    let admitted = admit_identity_evolution_query(query_context).map_err(|_| {
        PlanarBooleanSplitPersistentNamingDenial::new(
            PlanarBooleanSplitPersistentNamingDenialKind::DeniedIdentityEvolution,
            source_edge_identity,
            "split identity evolution query admission failed",
        )
    })?;
    counters.admitted_identity_evolution_query();
    let artifact = execute_admitted_identity_evolution_query(&admitted).map_err(|_| {
        PlanarBooleanSplitPersistentNamingDenial::new(
            PlanarBooleanSplitPersistentNamingDenialKind::DeniedIdentityEvolution,
            source_edge_identity,
            "split identity evolution query execution failed",
        )
    })?;
    counters.executed_identity_evolution_query();
    let row =
        PlanarBooleanSplitIdentityEvolutionRow::from_query_artifact(source_edge_identity, artifact)
            .map_err(|denial| {
                record_identity_evolution_denial(&denial, counters);
                denial
            })?;
    match row.outcome_kind() {
        PlanarBooleanSplitIdentityEvolutionOutcomeKind::PluralSplitSuccessors => {
            counters.emitted_plural_successors(row.successor_identities().len());
        }
        PlanarBooleanSplitIdentityEvolutionOutcomeKind::SingularContinuity => {
            counters.emitted_singular_continuity();
        }
    }
    Ok(row)
}

fn record_identity_evolution_denial(
    denial: &PlanarBooleanSplitPersistentNamingDenial,
    counters: &mut PlanarBooleanSplitPersistentNamingCounters,
) {
    match denial.kind() {
        PlanarBooleanSplitPersistentNamingDenialKind::AmbiguousIdentityEvolution => {
            counters.rejected_ambiguous_identity_evolution();
        }
        PlanarBooleanSplitPersistentNamingDenialKind::IdentityEvolutionBreak => {
            counters.rejected_identity_evolution_break();
        }
        PlanarBooleanSplitPersistentNamingDenialKind::DeniedIdentityEvolution
        | PlanarBooleanSplitPersistentNamingDenialKind::AdvisoryIdentityEvolutionNotAuthoritative =>
            {}
        _ => {}
    }
}
