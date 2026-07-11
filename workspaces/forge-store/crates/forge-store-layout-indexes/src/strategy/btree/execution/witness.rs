use super::counters::BaselineBTreeExactCounterValues;
use super::{
    decode_leaf_record, decode_root_record, encode_root_record, BaselineBTreeCorruptionMarker,
    BaselineBTreeExactCounterWitness, BaselineBTreeExecutionDenial, BaselineBTreeLookupBranch,
    BaselineBTreeLookupExecution, BaselineBTreeReadShape, BaselineBTreeReplayRecoveryExecution,
    BaselineBTreeRootPublicationExecution,
};
use forge_store_budgets::S8PreExecutionPlanBinding;
use forge_store_contracts::AcceptedHandoffReadiness;
use forge_store_physical_format::{
    PersistedPhysicalLayout, PhysicalRecordSlot, PhysicalReference, PhysicalReferenceAuthority,
    PlatformPhysicalAppendRequest, PlatformPhysicalFacade, PlatformPhysicalOpenRequest,
    PlatformPhysicalReplayArtifact, SlotGenerationCell,
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

    pub fn execute_separator_directed_lookup(
        &self,
        plan_binding: S8PreExecutionPlanBinding,
        probe_slot: PhysicalRecordSlot,
    ) -> Result<BaselineBTreeLookupExecution, BaselineBTreeExecutionDenial> {
        self.execute_separator_directed_read(
            plan_binding,
            probe_slot,
            BaselineBTreeReadShape::PointLookup,
        )
    }

    pub fn execute_separator_directed_range_lookup(
        &self,
        plan_binding: S8PreExecutionPlanBinding,
        probe_slot: PhysicalRecordSlot,
    ) -> Result<BaselineBTreeLookupExecution, BaselineBTreeExecutionDenial> {
        self.execute_separator_directed_read(
            plan_binding,
            probe_slot,
            BaselineBTreeReadShape::RangeLookup,
        )
    }

    pub fn execute_separator_directed_prefix_lookup(
        &self,
        plan_binding: S8PreExecutionPlanBinding,
        probe_slot: PhysicalRecordSlot,
    ) -> Result<BaselineBTreeLookupExecution, BaselineBTreeExecutionDenial> {
        self.execute_separator_directed_read(
            plan_binding,
            probe_slot,
            BaselineBTreeReadShape::PrefixLookup,
        )
    }

    fn execute_separator_directed_read(
        &self,
        plan_binding: S8PreExecutionPlanBinding,
        probe_slot: PhysicalRecordSlot,
        shape: BaselineBTreeReadShape,
    ) -> Result<BaselineBTreeLookupExecution, BaselineBTreeExecutionDenial> {
        let mut facade = reopen_facade(self.readiness.clone(), &self.replay_artifact)?;
        let mut root_access = facade.page_access();
        let root = root_access.read_record(self.root_reference)?;
        let node = decode_root_record(root.record_view().payload().as_bytes())
            .ok_or(BaselineBTreeExecutionDenial::InvalidRootNode)?;
        drop(root_access);
        let (branch, selected_cell) = if probe_slot.get() < node.separator_slot().get() {
            (BaselineBTreeLookupBranch::Left, node.left_child())
        } else {
            (BaselineBTreeLookupBranch::Right, node.right_child())
        };
        let selected_reference = PhysicalReferenceAuthority::s1()
            .admit_page_slot(selected_cell)
            .reference();
        facade.page_access().locate_record(selected_reference)?;
        let mut leaf_access = facade.page_access();
        let selected_leaf = leaf_access.read_record(selected_reference)?;
        let leaf = decode_leaf_record(selected_leaf.record_view().payload().as_bytes())
            .ok_or(BaselineBTreeExecutionDenial::InvalidLeafNode)?;
        if !leaf.slots().contains(&probe_slot) {
            return Err(BaselineBTreeExecutionDenial::ProbeMissingFromSelectedLeaf);
        }
        let counters = lookup_counters(shape);
        Ok(BaselineBTreeLookupExecution::new(
            plan_binding,
            shape,
            probe_slot,
            node.separator_slot(),
            branch,
            selected_reference,
            counters,
        ))
    }

    pub fn execute_root_publication(
        plan_binding: S8PreExecutionPlanBinding,
        facade: &mut PlatformPhysicalFacade,
        root_cell: SlotGenerationCell,
        separator_slot: PhysicalRecordSlot,
        left_child: SlotGenerationCell,
        right_child: SlotGenerationCell,
    ) -> Result<BaselineBTreeRootPublicationExecution, BaselineBTreeExecutionDenial> {
        let root_payload = encode_root_record(
            BaselineBTreeCorruptionMarker::Header,
            separator_slot,
            left_child,
            right_child,
        );
        let root = facade.append_physical_record(PlatformPhysicalAppendRequest::page_slot(
            root_cell,
            &root_payload,
        ))?;
        let published = facade.publish_physical_root()?;
        let exact_counters =
            BaselineBTreeExactCounterWitness::new(BaselineBTreeExactCounterValues {
                publications: 1,
                page_touches: 1,
                manifest_reads: 1,
                bytes_read: 4_096,
                bytes_written: 4_096,
                write_fanout: 1,
                read_amplification: 1,
                write_amplification: 1,
                ..BaselineBTreeExactCounterValues::default()
            });
        Ok(BaselineBTreeRootPublicationExecution::new(
            plan_binding,
            published,
            root.reference(),
            root_payload,
            left_child,
            right_child,
            exact_counters,
        ))
    }

    pub fn execute_replay_recovery(
        &self,
        plan_binding: S8PreExecutionPlanBinding,
    ) -> Result<BaselineBTreeReplayRecoveryExecution, BaselineBTreeExecutionDenial> {
        let mut facade = reopen_facade(self.readiness.clone(), &self.replay_artifact)?;
        let mut root_access = facade.page_access();
        let root = root_access.read_record(self.root_reference)?;
        let node = decode_root_record(root.record_view().payload().as_bytes())
            .ok_or(BaselineBTreeExecutionDenial::InvalidRootNode)?;
        drop(root_access);
        let left = read_leaf(&mut facade, node.left_child())?;
        let right = read_leaf(&mut facade, node.right_child())?;
        let authority_records = left.slots().len().saturating_add(right.slots().len()) as u16;
        let exact_counters =
            BaselineBTreeExactCounterWitness::new(BaselineBTreeExactCounterValues {
                maintenance_reads: 1,
                page_touches: 1,
                manifest_reads: 1,
                bytes_read: 4_096,
                read_amplification: 1,
                ..BaselineBTreeExactCounterValues::default()
            });
        Ok(BaselineBTreeReplayRecoveryExecution::new(
            plan_binding,
            self.replay_artifact.persisted_layout().clone(),
            self.root_reference,
            self.replay_artifact
                .persisted_layout()
                .root_manifest_candidates()[0]
                .clone(),
            authority_records,
            authority_records,
            self.replay_artifact
                .persisted_layout()
                .root_manifest_candidates()
                .len()
                == 1,
            exact_counters,
        ))
    }

    pub const fn root_reference(&self) -> PhysicalReference {
        self.root_reference
    }
    pub fn published_layout(&self) -> &PersistedPhysicalLayout {
        self.replay_artifact.persisted_layout()
    }
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

fn read_leaf(
    facade: &mut PlatformPhysicalFacade,
    cell: SlotGenerationCell,
) -> Result<super::BaselineBTreeLeafRecord, BaselineBTreeExecutionDenial> {
    let reference = PhysicalReferenceAuthority::s1()
        .admit_page_slot(cell)
        .reference();
    let mut page_access = facade.page_access();
    let leaf = page_access.read_record(reference)?;
    decode_leaf_record(leaf.record_view().payload().as_bytes())
        .ok_or(BaselineBTreeExecutionDenial::InvalidLeafNode)
}

fn reopen_facade(
    readiness: AcceptedHandoffReadiness,
    replay_artifact: &PlatformPhysicalReplayArtifact,
) -> Result<PlatformPhysicalFacade, BaselineBTreeExecutionDenial> {
    let request = PlatformPhysicalOpenRequest::s1_canonical();
    Ok(replay_artifact.reopen_s1(readiness, request)?)
}
