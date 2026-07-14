use forge_store_lsm_authority::{
    AdmittedLsmReplacementOutput, LsmCompactionMembership, LsmMembershipArtifactDeclaration,
    LsmMembershipDenial, LsmMembershipKey, LsmMembershipRecord, LsmPhysicalCompactionIntent,
};
use forge_store_wal::{AdmittedWalAppendReceipt, BlobWalRecordEnvelope};

/// One owner-admitted pairing of semantic membership, durable output, and the
/// exact physical reader horizon that the output is allowed to replace.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdmittedLsmCompactionDemand {
    value: LsmMembershipRecord,
    generation: LsmMembershipRecord,
    tombstone: LsmMembershipRecord,
    membership: LsmCompactionMembership,
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
        let records = plan.membership.record_set();
        let value = records.value().clone();
        let generation = records.generation().clone();
        let tombstone = records.tombstone().clone();
        Ok(Self {
            value,
            generation,
            tombstone,
            membership: plan.membership,
            output,
            physical_intent,
        })
    }

    pub(crate) const fn key(&self) -> &LsmMembershipKey {
        self.membership.key_ref()
    }

    pub(crate) const fn physical_intent(&self) -> &LsmPhysicalCompactionIntent {
        &self.physical_intent
    }

    pub(crate) const fn value_record(&self) -> &BlobWalRecordEnvelope {
        self.value.envelope()
    }

    pub(crate) const fn generation_record(&self) -> &BlobWalRecordEnvelope {
        self.generation.envelope()
    }

    pub(crate) const fn tombstone_record(&self) -> &BlobWalRecordEnvelope {
        self.tombstone.envelope()
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
        LsmMembershipDenial::ValueRecordRequired => {
            super::BaselineLsmExecutionAdmissionDenial::ValueRecordRequired
        }
        LsmMembershipDenial::GenerationRecordRequired => {
            super::BaselineLsmExecutionAdmissionDenial::GenerationRecordRequired
        }
        LsmMembershipDenial::TombstoneRecordRequired => {
            super::BaselineLsmExecutionAdmissionDenial::TombstoneRecordRequired
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
