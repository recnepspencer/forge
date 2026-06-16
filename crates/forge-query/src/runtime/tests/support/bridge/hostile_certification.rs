use crate::evidence_identity::{ForgeQueryEvidenceIdentity, ForgeQueryEvidenceScope};
use crate::runtime::tests::support::*;
use crate::{
    authoring::{
        AspectFieldSelector, AuthoredResultShapeField, GuidedAuthoringPath, RawAuthoredQuery,
        RawAuthoredResultShape, RootEntityKey,
    },
    authorized_projection::{
        derive_authorized_projection, AuthorizedProjectionArtifact, PolicyAspectMask,
        PolicyInfluenceSet,
    },
    canonicalization::CanonicalResultShapeArtifact,
    projection_consumption::ProjectMaterializedFacts,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub(in crate::runtime::tests) struct RuntimeHostileCertificationCounters {
    committed_read_hot_path_lock_count: usize,
    reader_derived_evaluation_count: usize,
    orphaned_snapshot_generation_count: usize,
    unretired_read_pin_count: usize,
    journal_gap_count: usize,
    delivery_residue_count: usize,
    digest: ForgeQueryEvidenceIdentity,
}

impl RuntimeHostileCertificationCounters {
    pub(in crate::runtime::tests) fn for_runtime(
        runtime: &ForgeQueryRuntime,
        journal_gap_count: usize,
        reader_derived_evaluation_count: usize,
        delivery_residue_count: usize,
    ) -> Self {
        let committed_read_hot_path_lock_count =
            shared_read_structural_lock_acquisition_count(runtime);
        let orphaned_snapshot_generation_count =
            shared_read_structural_orphaned_generation_count(runtime);
        let unretired_read_pin_count = shared_read_structural_unretired_pin_count(runtime);
        let digest = ForgeQueryEvidenceIdentity::compose(
            ForgeQueryEvidenceScope::RuntimeHostileCertificationArtifact,
        )
        .field_usize(
            crate::ForgeQueryEvidenceTag::new("committed_read_hot_path_lock_count"),
            committed_read_hot_path_lock_count,
        )
        .field_usize(
            crate::ForgeQueryEvidenceTag::new("reader_derived_evaluation_count"),
            reader_derived_evaluation_count,
        )
        .field_usize(
            crate::ForgeQueryEvidenceTag::new("orphaned_snapshot_generation_count"),
            orphaned_snapshot_generation_count,
        )
        .field_usize(
            crate::ForgeQueryEvidenceTag::new("unretired_read_pin_count"),
            unretired_read_pin_count,
        )
        .field_usize(
            crate::ForgeQueryEvidenceTag::new("journal_gap_count"),
            journal_gap_count,
        )
        .field_usize(
            crate::ForgeQueryEvidenceTag::new("delivery_residue_count"),
            delivery_residue_count,
        )
        .seal();
        Self {
            committed_read_hot_path_lock_count,
            reader_derived_evaluation_count,
            orphaned_snapshot_generation_count,
            unretired_read_pin_count,
            journal_gap_count,
            delivery_residue_count,
            digest,
        }
    }

    pub(in crate::runtime::tests) fn committed_read_hot_path_lock_count(&self) -> usize {
        self.committed_read_hot_path_lock_count
    }

    pub(in crate::runtime::tests) fn reader_derived_evaluation_count(&self) -> usize {
        self.reader_derived_evaluation_count
    }

    pub(in crate::runtime::tests) fn orphaned_snapshot_generation_count(&self) -> usize {
        self.orphaned_snapshot_generation_count
    }

    pub(in crate::runtime::tests) fn unretired_read_pin_count(&self) -> usize {
        self.unretired_read_pin_count
    }

    pub(in crate::runtime::tests) fn journal_gap_count(&self) -> usize {
        self.journal_gap_count
    }

    pub(in crate::runtime::tests) fn delivery_residue_count(&self) -> usize {
        self.delivery_residue_count
    }

    pub(in crate::runtime::tests) fn digest(&self) -> &ForgeQueryEvidenceIdentity {
        &self.digest
    }
}

fn shared_read_structural_lock_acquisition_count(_runtime: &ForgeQueryRuntime) -> usize {
    _runtime
        .shared_read_counters()
        .committed_read_hot_path_lock_count()
}

fn shared_read_structural_orphaned_generation_count(_runtime: &ForgeQueryRuntime) -> usize {
    _runtime.shared_read_counters().orphaned_generation_count()
}

fn shared_read_structural_unretired_pin_count(_runtime: &ForgeQueryRuntime) -> usize {
    _runtime.shared_read_counters().unretired_pin_count()
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(in crate::runtime::tests) struct RuntimeHostileCertificationArtifact {
    receipt_digests: Vec<String>,
    reader_result_digests: Vec<String>,
    published_artifact_digests: Vec<String>,
    preview_closeout_digests: Vec<String>,
    branch_basis_digests: Vec<String>,
    counters: RuntimeHostileCertificationCounters,
    digest: ForgeQueryEvidenceIdentity,
}

impl RuntimeHostileCertificationArtifact {
    pub(in crate::runtime::tests) fn new(
        receipt_digests: Vec<String>,
        reader_result_digests: Vec<String>,
        published_artifact_digests: Vec<String>,
        preview_closeout_digests: Vec<String>,
        branch_basis_digests: Vec<String>,
        counters: RuntimeHostileCertificationCounters,
    ) -> Self {
        let digest = ForgeQueryEvidenceIdentity::compose(
            ForgeQueryEvidenceScope::RuntimeHostileCertificationArtifact,
        )
        .field_value_sequence(
            crate::ForgeQueryEvidenceTag::new("receipt_digest"),
            receipt_digests.iter().map(String::as_str),
        )
        .field_value_sequence(
            crate::ForgeQueryEvidenceTag::new("reader_result_digest"),
            reader_result_digests.iter().map(String::as_str),
        )
        .field_value_sequence(
            crate::ForgeQueryEvidenceTag::new("published_artifact_digest"),
            published_artifact_digests.iter().map(String::as_str),
        )
        .field_value_sequence(
            crate::ForgeQueryEvidenceTag::new("preview_closeout_digest"),
            preview_closeout_digests.iter().map(String::as_str),
        )
        .field_value_sequence(
            crate::ForgeQueryEvidenceTag::new("branch_basis_digest"),
            branch_basis_digests.iter().map(String::as_str),
        )
        .field_value(
            crate::ForgeQueryEvidenceTag::new("counter_digest"),
            counters.digest().terminal_projection_for_reporting(),
        )
        .seal();
        Self {
            receipt_digests,
            reader_result_digests,
            published_artifact_digests,
            preview_closeout_digests,
            branch_basis_digests,
            counters,
            digest,
        }
    }

    pub(in crate::runtime::tests) fn counters(&self) -> &RuntimeHostileCertificationCounters {
        &self.counters
    }

    pub(in crate::runtime::tests) fn digest(&self) -> &ForgeQueryEvidenceIdentity {
        &self.digest
    }
}

pub(in crate::runtime::tests) fn hostile_write_receipt_digest(
    receipt: &ForgeQueryWriteReceipt,
) -> String {
    ForgeQueryEvidenceIdentity::compose(
        ForgeQueryEvidenceScope::RuntimeHostileCertificationArtifact,
    )
    .field_shape(
        crate::ForgeQueryEvidenceTag::new("mutation_family"),
        receipt.mutation_family().as_str(),
    )
    .field_shape(
        crate::ForgeQueryEvidenceTag::new("authority_lane"),
        receipt.authority_lane().as_str(),
    )
    .field_evidence_identity(
        crate::ForgeQueryEvidenceTag::new("commit_identity"),
        receipt.commit_evidence_identity(),
    )
    .field_evidence_identity(
        crate::ForgeQueryEvidenceTag::new("snapshot_identity"),
        receipt.snapshot_evidence_identity(),
    )
    .field_shape(
        crate::ForgeQueryEvidenceTag::new("declared_aspect_value_digest"),
        receipt.declared_aspect_value_digest().unwrap_or("none"),
    )
    .seal()
    .terminal_projection_for_reporting()
    .to_string()
}

pub(in crate::runtime::tests) fn hostile_published_artifact_digest(
    artifact: &ForgeQueryPublishedDerivedArtifactHandle,
    consumed_title: Option<&str>,
) -> String {
    let inspection = artifact.inspect_projection_consumption();
    ForgeQueryEvidenceIdentity::compose(
        ForgeQueryEvidenceScope::RuntimeHostileCertificationArtifact,
    )
    .field_evidence_identity(
        crate::ForgeQueryEvidenceTag::new("snapshot_identity"),
        &artifact.snapshot_identity().evidence_identity(),
    )
    .field_shape(
        crate::ForgeQueryEvidenceTag::new("view_name"),
        artifact.view_name(),
    )
    .field_shape(
        crate::ForgeQueryEvidenceTag::new("published"),
        if inspection.published() {
            "true"
        } else {
            "false"
        },
    )
    .field_value(
        crate::ForgeQueryEvidenceTag::new("binding_digest"),
        inspection
            .artifact_binding_for_reporting()
            .unwrap_or("none"),
    )
    .field_value(
        crate::ForgeQueryEvidenceTag::new("async_result_state_digest"),
        inspection
            .async_result_state()
            .map(|state| state.result_state_for_reporting())
            .unwrap_or("none"),
    )
    .field_shape(
        crate::ForgeQueryEvidenceTag::new("consumed_title"),
        consumed_title.unwrap_or("none"),
    )
    .seal()
    .terminal_projection_for_reporting()
    .to_string()
}

pub(in crate::runtime::tests) fn hostile_preview_closeout_digest(
    outcome: &ForgeQueryPreviewOutcome,
) -> String {
    outcome.closeout_evidence().closeout_digest().to_string()
}

pub(in crate::runtime::tests) fn hostile_preview_delivery_residue_count(
    outcome: &ForgeQueryPreviewOutcome,
) -> usize {
    outcome.effect_delivery_residue_count()
}

pub(in crate::runtime::tests) fn hostile_branch_basis_digest(
    session: &ForgeQueryBranchSession<'_>,
) -> String {
    session
        .basis_admission()
        .admission_digest()
        .terminal_projection_for_reporting()
        .to_string()
}

pub(in crate::runtime::tests) fn hostile_journal_gap_count(
    receipts: &[ForgeQueryWriteReceipt],
) -> usize {
    let ordinals = receipts
        .iter()
        .filter_map(|receipt| {
            receipt
                .commit_identity()
                .bridge_identity()
                .and_then(|identity| identity.relational_commit_id())
        })
        .collect::<Vec<_>>();
    ordinals
        .windows(2)
        .filter(|window| matches!(window, [left, right] if *right != *left + 1))
        .count()
}

pub(in crate::runtime::tests) fn hostile_insert_task_command(
    id: &str,
    title: &str,
) -> ForgeQueryWriteCommand {
    ForgeQueryAspectMutationBuilder::new()
        .aspect("identity.id", id)
        .aspect("title.value", title)
        .build_insert("Task")
        .expect("insert command should build")
}

pub(in crate::runtime::tests) fn hostile_consume_title_attempt(
    artifact: &ForgeQueryPublishedDerivedArtifactHandle,
) -> ForgeQueryPublishedProjectionConsumption {
    let (result_shape, authorized_projection) = hostile_projection_artifacts();
    artifact
        .consume_projection_facts(
            &result_shape,
            &authorized_projection,
            ProjectMaterializedFacts::declare().display_field("title.value"),
        )
        .expect("projection consumption should remain on the typed artifact lane")
}

fn hostile_projection_artifacts() -> (CanonicalResultShapeArtifact, AuthorizedProjectionArtifact) {
    let query = RawAuthoredQuery::detail_builder(RootEntityKey::new("task").unwrap())
        .project(AspectFieldSelector::new("title", "value").unwrap())
        .build()
        .unwrap();
    let result_shape = RawAuthoredResultShape::detail_builder()
        .field(AuthoredResultShapeField::new("title", "value", "title.value").unwrap())
        .build()
        .unwrap();
    let canonical = GuidedAuthoringPath::canonicalize_detail(query, result_shape).unwrap();
    let authorized_projection = derive_authorized_projection(
        canonical.query(),
        canonical.result_shape(),
        "policy:test",
        "schema:test",
        &PolicyAspectMask::allow_all(),
        &PolicyInfluenceSet::none(),
        8,
        8,
    )
    .unwrap();
    (canonical.result_shape().clone(), authorized_projection)
}
