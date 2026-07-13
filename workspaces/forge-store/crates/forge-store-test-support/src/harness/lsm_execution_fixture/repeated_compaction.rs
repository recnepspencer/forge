use super::durability::{durable_record, manifest_receipt_for_artifact, wal_receipt, wal_scope};
use super::{
    admit_checkpoint_publication, admit_durable_append, lsm_membership_replacement_crash_fixture,
    lsm_strategy, physical_compaction_fixture, BlobWalRecordIdentity, BlobWalRecordKind,
    LsmMembershipArtifactDeclaration,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RepeatedLsmMembershipFixture {
    first_output: BlobWalRecordIdentity,
    selected_base: BlobWalRecordIdentity,
    second_output: BlobWalRecordIdentity,
    reopened_output: BlobWalRecordIdentity,
}

impl RepeatedLsmMembershipFixture {
    pub const fn first_output(self) -> BlobWalRecordIdentity {
        self.first_output
    }

    pub const fn selected_base(self) -> BlobWalRecordIdentity {
        self.selected_base
    }

    pub const fn second_output(self) -> BlobWalRecordIdentity {
        self.second_output
    }

    pub const fn reopened_output(self) -> BlobWalRecordIdentity {
        self.reopened_output
    }
}

pub fn execute_repeated_lsm_membership_fixture() -> RepeatedLsmMembershipFixture {
    let first = lsm_membership_replacement_crash_fixture();

    let access = lsm_strategy();
    let mut session = super::open_lsm_index(first.anchor()).unwrap();
    let key = first.key();
    durable_record(&access, &mut session, key, 45, BlobWalRecordKind::LsmValue);
    durable_record(
        &access,
        &mut session,
        key,
        46,
        BlobWalRecordKind::GenerationPublication,
    );
    durable_record(
        &access,
        &mut session,
        key,
        47,
        BlobWalRecordKind::LsmTombstone,
    );

    let selected = session.select_compaction(key).unwrap();
    let selected_base = selected
        .base()
        .expect("published base is mandatory")
        .output();
    let (physical_intent, physical_publication) = physical_compaction_fixture();
    let output_scope = wal_scope(
        selected.expected_output_identity().unwrap().sequence(),
        selected.compaction_output_digest(
            physical_intent.root_scope(),
            physical_intent.target_epoch(),
            physical_intent.manifest_epoch(),
        ),
        4096,
    );
    let output_artifact = LsmMembershipArtifactDeclaration::compaction_output(&output_scope);
    let output_durable =
        admit_durable_append(&wal_receipt(output_scope, output_artifact.bytes())).unwrap();
    let output = forge_store_lsm_authority::admit_lsm_replacement_output(
        &selected,
        output_durable,
        physical_intent,
    )
    .unwrap();
    let activation = forge_store_lsm_authority::prepare_lsm_membership_activation(
        &selected,
        output,
        &physical_publication,
    )
    .unwrap();
    let artifact = activation.artifact();
    let checkpoint = admit_checkpoint_publication(&manifest_receipt_for_artifact(
        activation.scope().clone(),
        artifact.bytes(),
    ))
    .unwrap();
    let replacement = forge_store_lsm_authority::admit_lsm_membership_replacement(
        &selected, activation, checkpoint,
    )
    .unwrap();
    let second = session.replace(&selected, &replacement).unwrap();
    drop(session);

    let reopened = super::open_lsm_index(first.anchor()).unwrap();
    let reopened_output = reopened.published_replacement(key).unwrap().output();
    RepeatedLsmMembershipFixture {
        first_output: first.replacement_output(),
        selected_base,
        second_output: second.output(),
        reopened_output,
    }
}

pub fn substituted_lsm_base_is_rejected_before_compaction(
) -> forge_store_lsm_authority::LsmMembershipDenial {
    let first = lsm_membership_replacement_crash_fixture();

    let access = lsm_strategy();
    let mut session = super::open_lsm_index(first.anchor()).unwrap();
    let key = first.key();
    durable_record(&access, &mut session, key, 45, BlobWalRecordKind::LsmValue);
    durable_record(
        &access,
        &mut session,
        key,
        46,
        BlobWalRecordKind::GenerationPublication,
    );
    durable_record(
        &access,
        &mut session,
        key,
        47,
        BlobWalRecordKind::LsmTombstone,
    );
    let selected = session.select_compaction(key).unwrap();
    let mut substituted = std::fs::read(first.replacement_path()).unwrap();
    substituted[0] ^= 0x01;
    std::fs::write(first.replacement_path(), substituted).unwrap();
    selected.revalidate_artifacts().unwrap_err()
}
