use super::counters::BaselineBTreeExactCounterValues;
use super::{
    decode_leaf_record, decode_root_record, verify_selected_leaf_partition,
    BaselineBTreeExactCounterWitness, BaselineBTreeExecutionDenial, BaselineBTreeLookupBranch,
    BaselineBTreeLookupExecution, BaselineBTreeReadShape,
};
use forge_store_contracts::AcceptedHandoffReadiness;
use forge_store_physical_format::{
    PersistedPhysicalLayout, PhysicalGenerationAuthority, PhysicalRecordSlot, PhysicalReference,
    PhysicalReferenceAuthority, PhysicalStoreRuntime, PlatformPhysicalOpenRequest,
    PlatformPhysicalReplayArtifact, SlotGenerationCell,
};
use forge_store_physical_isolation::{
    CurrentGenerationPhysicalReference, GenerationCountedPhysicalReference,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BaselineBTreeExecutionWitness {
    readiness: AcceptedHandoffReadiness,
    root_reference: PhysicalReference,
    replay_artifact: PlatformPhysicalReplayArtifact,
}

impl BaselineBTreeExecutionWitness {
    pub fn admit_published_layout(
        readiness: AcceptedHandoffReadiness,
        root_reference: PhysicalReference,
        replay_artifact: PlatformPhysicalReplayArtifact,
    ) -> Result<Self, BaselineBTreeExecutionDenial> {
        let mut facade = reopen_facade(readiness.clone(), &replay_artifact)?;
        let mut page_access = facade.page_access();
        let root = page_access.read_record(root_reference)?;
        decode_root_record(root.record_view().payload().as_bytes())
            .ok_or(BaselineBTreeExecutionDenial::InvalidRootNode)?;
        Ok(Self {
            readiness,
            root_reference,
            replay_artifact,
        })
    }

    pub fn preflight_stable_read(
        self,
    ) -> Result<super::BaselineBTreeReadPreflight, BaselineBTreeExecutionDenial> {
        super::BaselineBTreeReadPreflight::from_published_layout(self)
    }

    pub(in crate::strategy::btree::execution) fn execute_separator_directed_read(
        &self,
        probe_slot: PhysicalRecordSlot,
        shape: BaselineBTreeReadShape,
    ) -> Result<super::BaselineBTreeLookupObservation, BaselineBTreeExecutionDenial> {
        let mut facade = reopen_facade(self.readiness.clone(), &self.replay_artifact)?;
        let node = {
            let mut root_access = facade.page_access();
            let root = root_access.read_record(self.root_reference)?;
            decode_root_record(root.record_view().payload().as_bytes())
                .ok_or(BaselineBTreeExecutionDenial::InvalidRootNode)?
        };
        let (branch, selected_cell) = if probe_slot.get() < node.separator_slot().get() {
            (BaselineBTreeLookupBranch::Left, node.left_child())
        } else {
            (BaselineBTreeLookupBranch::Right, node.right_child())
        };
        let selected_reference = PhysicalReferenceAuthority::for_canonical_physical_format()
            .admit_page_slot(selected_cell)
            .reference();
        facade.page_access().locate_record(selected_reference)?;
        let mut leaf_access = facade.page_access();
        let selected_leaf = leaf_access.read_record(selected_reference)?;
        let leaf = decode_leaf_record(selected_leaf.record_view().payload().as_bytes())
            .ok_or(BaselineBTreeExecutionDenial::InvalidLeafNode)?;
        verify_selected_leaf_partition(node.separator_slot(), branch, leaf)?;
        let counters = lookup_counters(shape);
        if !leaf.slots().contains(&probe_slot) {
            return Ok(super::BaselineBTreeLookupObservation::Absent(
                super::BaselineBTreeLookupAbsence::issue(probe_slot, selected_reference, counters),
            ));
        }
        Ok(super::BaselineBTreeLookupObservation::Found(
            BaselineBTreeLookupExecution::new(
                shape,
                probe_slot,
                node.separator_slot(),
                branch,
                selected_reference,
                counters,
            ),
        ))
    }

    pub const fn root_reference(&self) -> PhysicalReference {
        self.root_reference
    }
    pub fn store_authority_identity(&self) -> forge_store_authority::StoreCurrentAuthorityIdentity {
        self.replay_artifact.store_identity().authority_identity()
    }
    pub fn published_layout(&self) -> &PersistedPhysicalLayout {
        self.replay_artifact.persisted_layout()
    }

    pub(super) fn stable_read_references(
        &self,
    ) -> Result<[CurrentGenerationPhysicalReference; 3], BaselineBTreeExecutionDenial> {
        let mut facade = reopen_facade(self.readiness.clone(), &self.replay_artifact)?;
        let mut root_access = facade.page_access();
        let root = root_access.read_record(self.root_reference)?;
        let node = decode_root_record(root.record_view().payload().as_bytes())
            .ok_or(BaselineBTreeExecutionDenial::InvalidRootNode)?;
        Ok([
            current_reference(self.root_reference)?,
            current_cell_reference(node.left_child()),
            current_cell_reference(node.right_child()),
        ])
    }
}

fn current_reference(
    reference: PhysicalReference,
) -> Result<CurrentGenerationPhysicalReference, BaselineBTreeExecutionDenial> {
    let segment = reference
        .segment_id()
        .ok_or(BaselineBTreeExecutionDenial::InvalidPhysicalReferenceForBTree)?;
    let page = reference
        .page_id()
        .ok_or(BaselineBTreeExecutionDenial::InvalidPhysicalReferenceForBTree)?;
    let slot = reference
        .slot()
        .ok_or(BaselineBTreeExecutionDenial::InvalidPhysicalReferenceForBTree)?;
    let cell = PhysicalGenerationAuthority::for_canonical_physical_format()
        .slot_cell(segment, page, slot)
        .with_slot_generation(reference.generation());
    Ok(current_cell_reference(cell))
}

fn current_cell_reference(cell: SlotGenerationCell) -> CurrentGenerationPhysicalReference {
    GenerationCountedPhysicalReference::from_admitted_reference(
        PhysicalReferenceAuthority::for_canonical_physical_format().admit_page_slot(cell),
    )
    .require_current_generation(cell.generation())
    .expect("the admitted cell and observed generation are identical")
}

fn lookup_counters(shape: BaselineBTreeReadShape) -> BaselineBTreeExactCounterWitness {
    let (point_lookups, range_lookups, range_steps, prefix_steps) = match shape {
        BaselineBTreeReadShape::PointLookup => (1, 0, 0, 0),
        BaselineBTreeReadShape::RangeLookup => (0, 1, 1, 0),
        BaselineBTreeReadShape::PrefixLookup => (0, 1, 0, 1),
    };
    BaselineBTreeExactCounterWitness::new(BaselineBTreeExactCounterValues {
        point_lookups,
        range_lookups,
        page_touches: 2,
        index_probes: 2,
        key_comparisons: 2,
        range_steps,
        prefix_steps,
        bytes_read: 8_192,
        read_amplification: 2,
        ..BaselineBTreeExactCounterValues::default()
    })
}

fn reopen_facade(
    readiness: AcceptedHandoffReadiness,
    replay_artifact: &PlatformPhysicalReplayArtifact,
) -> Result<PhysicalStoreRuntime, BaselineBTreeExecutionDenial> {
    let request = PlatformPhysicalOpenRequest::physical_format_for_store(
        replay_artifact.store_identity().clone(),
    );
    Ok(replay_artifact.reopen_physical_format(readiness, request)?)
}
