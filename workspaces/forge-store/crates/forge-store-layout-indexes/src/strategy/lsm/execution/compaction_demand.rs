use forge_store_lsm_authority::{
    AdmittedLsmReplacementOutput, LsmCompactionMembership, LsmMembershipArtifactDeclaration,
    LsmMembershipDenial, LsmMembershipKey, LsmMembershipRecord, LsmPhysicalCompactionIntent,
};
use forge_store_wal::{AdmittedWalAppendReceipt, BlobWalRecordEnvelope, BlobWalRecordIdentity};

/// One owner-admitted pairing of semantic membership, durable output, and the
/// exact physical reader horizon that the output is allowed to replace.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdmittedLsmCompactionDemand {
    value: LsmMembershipRecord,
    generation: LsmMembershipRecord,
    tombstone: LsmMembershipRecord,
    membership: LsmCompactionMembership,
    compaction_admission: super::BaselineLsmCompactionAdmission,
    output: AdmittedLsmReplacementOutput,
    physical_intent: LsmPhysicalCompactionIntent,
}

impl AdmittedLsmCompactionDemand {
    pub(crate) fn admit(
        plan: super::BaselineLsmCompactionPlan,
        output_append: AdmittedWalAppendReceipt,
        physical_intent: LsmPhysicalCompactionIntent,
    ) -> Result<Self, super::BaselineLsmExecutionAdmissionDenial> {
        plan.membership
            .revalidate_artifacts()
            .map_err(map_membership_denial)?;
        let expected_output =
            LsmMembershipArtifactDeclaration::compaction_output(output_append.scope());
        if output_append.persisted_bytes() != expected_output.bytes().len() as u64
            || std::fs::read(output_append.persisted_path())
                .ok()
                .as_deref()
                != Some(expected_output.bytes())
        {
            return Err(super::BaselineLsmExecutionAdmissionDenial::OutputPublicationMismatch);
        }
        let output = forge_store_lsm_authority::admit_lsm_replacement_output(
            &plan.membership,
            output_append,
            physical_intent.clone(),
        )
        .map_err(map_membership_denial)?;
        let [value, generation, tombstone] = plan.membership.records().clone();
        Ok(Self {
            value,
            generation,
            tombstone,
            membership: plan.membership,
            compaction_admission: plan.admission,
            output,
            physical_intent,
        })
    }

    pub(crate) const fn key(&self) -> &LsmMembershipKey {
        self.membership.key_ref()
    }

    pub(crate) const fn compaction_admission(&self) -> &super::BaselineLsmCompactionAdmission {
        &self.compaction_admission
    }

    pub(crate) const fn physical_intent(&self) -> &LsmPhysicalCompactionIntent {
        &self.physical_intent
    }

    pub(crate) fn replay_tail(&self) -> [&BlobWalRecordEnvelope; 3] {
        [
            self.value.envelope(),
            self.generation.envelope(),
            self.tombstone.envelope(),
        ]
    }

    pub(crate) fn identities(&self) -> [BlobWalRecordIdentity; 3] {
        self.replay_tail().map(|record| record.identity())
    }

    pub(crate) const fn records(&self) -> [&LsmMembershipRecord; 3] {
        [&self.value, &self.generation, &self.tombstone]
    }

    pub(crate) const fn membership(&self) -> &LsmCompactionMembership {
        &self.membership
    }

    pub(crate) const fn output(&self) -> &AdmittedLsmReplacementOutput {
        &self.output
    }
}

pub(crate) fn map_membership_denial(
    denial: LsmMembershipDenial,
) -> super::BaselineLsmExecutionAdmissionDenial {
    match denial {
        LsmMembershipDenial::MembershipAmbiguous => {
            super::BaselineLsmExecutionAdmissionDenial::PersistedMembershipAmbiguous
        }
        LsmMembershipDenial::MembershipIncomplete => {
            super::BaselineLsmExecutionAdmissionDenial::PersistedMembershipIncomplete
        }
        LsmMembershipDenial::MembershipStale => {
            super::BaselineLsmExecutionAdmissionDenial::PersistedMembershipStale
        }
        LsmMembershipDenial::ManifestMembershipMismatch => {
            super::BaselineLsmExecutionAdmissionDenial::ManifestMembershipMismatch
        }
        LsmMembershipDenial::ReplacementOutputMismatch => {
            super::BaselineLsmExecutionAdmissionDenial::OutputPublicationMismatch
        }
        LsmMembershipDenial::PhysicalPublicationBindingMismatch => {
            super::BaselineLsmExecutionAdmissionDenial::PhysicalPublicationBindingMismatch
        }
        LsmMembershipDenial::StoreBindingMismatch => {
            super::BaselineLsmExecutionAdmissionDenial::RecordKeyScopeMismatch
        }
        LsmMembershipDenial::DurableRecordBindingMismatch => {
            super::BaselineLsmExecutionAdmissionDenial::DurableRecordBindingMismatch
        }
        _ => super::BaselineLsmExecutionAdmissionDenial::PersistedIndexIo,
    }
}
