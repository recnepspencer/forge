use forge_relational::facade::history::BranchId;
use forge_relational::facade::runtime::RelationalRuntime;
#[cfg(test)]
use forge_relational::facade::snapshots::SnapshotHandle;
#[cfg(test)]
use schema::facade::platform::authority::MutationOrigin;
use schema::facade::platform::authority::RawTopologyIntent;
use schema::facade::topology_authoring::{
    commit_topology_intent_on_branch as commit_seeded_topology_intent_on_branch,
    TopologyIntentCommitError,
};

use crate::certification::support::commit_certification_input::TopologyCommitCertificationInput;
use crate::certification::DeterministicDigest;

#[derive(Debug, Clone)]
pub(crate) struct SchemaTopologyAuthoringBranchSession {
    branch_id: BranchId,
    main_head_before_mutation: Option<forge_relational::facade::history::CommitId>,
}

impl SchemaTopologyAuthoringBranchSession {
    pub(crate) fn branch_id(&self) -> &BranchId {
        &self.branch_id
    }

    pub(crate) fn branch_label(&self) -> &str {
        &self.branch_id.0
    }

    pub(crate) fn main_head_before_mutation(
        &self,
    ) -> Option<&forge_relational::facade::history::CommitId> {
        self.main_head_before_mutation.as_ref()
    }
}

#[derive(Debug, Clone)]
pub(crate) struct SchemaTopologyAuthoringBranchExecutionLedger {
    session: SchemaTopologyAuthoringBranchSession,
    truth_digest_rows: Vec<String>,
}

impl SchemaTopologyAuthoringBranchExecutionLedger {
    pub(crate) fn branch_id(&self) -> &BranchId {
        self.session.branch_id()
    }

    pub(crate) fn branch_label(&self) -> &str {
        self.session.branch_label()
    }

    pub(crate) fn branch_truth_digest(&self) -> DeterministicDigest {
        deterministic_digest_rows(self.truth_digest_rows.iter().cloned())
    }

    pub(crate) fn branch_head_diverged_from_main(
        &self,
        runtime: &RelationalRuntime,
    ) -> Result<bool, String> {
        let branch_head_after_mutation = runtime
            .history()
            .branch_head(self.branch_id())
            .ok_or_else(|| "accepted branch head missing".to_string())?
            .commit_id;
        let main_head_after_mutation = runtime
            .history()
            .branch_head(&BranchId("main".to_string()))
            .ok_or_else(|| "main branch head missing".to_string())?
            .commit_id;
        Ok(branch_head_after_mutation != main_head_after_mutation
            && self.session.main_head_before_mutation() == Some(&main_head_after_mutation))
    }
}

#[derive(Debug, Clone)]
pub(crate) struct SchemaRejectedBranchLocalParityWitness {
    branch_id: BranchId,
    branch_label: String,
    branch_head_unchanged_after_rejection: bool,
    rejection_detail: String,
}

impl SchemaRejectedBranchLocalParityWitness {
    pub(crate) fn branch_id(&self) -> &BranchId {
        &self.branch_id
    }

    pub(crate) fn branch_label(&self) -> &str {
        &self.branch_label
    }

    pub(crate) fn branch_head_unchanged_after_rejection(&self) -> bool {
        self.branch_head_unchanged_after_rejection
    }

    pub(crate) fn rejection_detail(&self) -> &str {
        &self.rejection_detail
    }
}

fn commit_topology_intent_on_branch_through_schema_authority(
    runtime: &mut RelationalRuntime,
    intent: RawTopologyIntent,
    branch_id: BranchId,
) -> Result<TopologyCommitCertificationInput, TopologyIntentCommitError> {
    let seeded = commit_seeded_topology_intent_on_branch(runtime, intent, branch_id)?;
    Ok(TopologyCommitCertificationInput::from_seeded_commit(seeded))
}

pub(super) fn open_schema_topology_authoring_branch(
    runtime: &mut RelationalRuntime,
    branch_label: impl Into<String>,
) -> Result<SchemaTopologyAuthoringBranchSession, String> {
    let branch_id = BranchId(branch_label.into());
    runtime
        .history_authority()
        .create_branch(branch_id.clone(), &BranchId("main".to_string()))
        .map_err(|error| format!("{error:?}"))?;
    let main_head_before_mutation = runtime
        .history()
        .branch_head(&BranchId("main".to_string()))
        .map(|head| head.commit_id);
    Ok(SchemaTopologyAuthoringBranchSession {
        branch_id,
        main_head_before_mutation,
    })
}

pub(crate) fn witness_rejected_branch_local_intent_through_schema_execution(
    runtime: &mut RelationalRuntime,
    branch_label: impl Into<String>,
    intent: RawTopologyIntent,
) -> Result<SchemaRejectedBranchLocalParityWitness, String> {
    let branch = open_schema_topology_authoring_branch(runtime, branch_label)?;
    let branch_head_before_rejection = runtime
        .history()
        .branch_head(branch.branch_id())
        .ok_or_else(|| "rejected branch head missing".to_string())?
        .commit_id;
    let rejection_detail = match commit_topology_intent_on_branch_through_schema_authority(
        runtime,
        intent,
        branch.branch_id().clone(),
    ) {
        Ok(_) => return Err(
            "rejected branch-local parity witness unexpectedly admitted the branch-local intent"
                .to_string(),
        ),
        Err(error) => error.to_string(),
    };
    let branch_head_after_rejection = runtime
        .history()
        .branch_head(branch.branch_id())
        .ok_or_else(|| "rejected branch head missing".to_string())?
        .commit_id;
    Ok(SchemaRejectedBranchLocalParityWitness {
        branch_id: branch.branch_id().clone(),
        branch_label: branch.branch_label().to_string(),
        branch_head_unchanged_after_rejection: branch_head_before_rejection
            == branch_head_after_rejection,
        rejection_detail,
    })
}

pub(crate) fn open_schema_topology_authoring_branch_execution(
    runtime: &mut RelationalRuntime,
    branch_label: impl Into<String>,
) -> Result<SchemaTopologyAuthoringBranchExecutionLedger, String> {
    Ok(SchemaTopologyAuthoringBranchExecutionLedger {
        session: open_schema_topology_authoring_branch(runtime, branch_label)?,
        truth_digest_rows: Vec::new(),
    })
}

pub(crate) fn commit_topology_intent_on_branch_through_schema_execution(
    runtime: &mut RelationalRuntime,
    branch_execution: &mut SchemaTopologyAuthoringBranchExecutionLedger,
    intent: RawTopologyIntent,
) -> Result<TopologyCommitCertificationInput, TopologyIntentCommitError> {
    let commit_input = commit_topology_intent_on_branch_through_schema_authority(
        runtime,
        intent,
        branch_execution.branch_id().clone(),
    )?;
    branch_execution.truth_digest_rows.extend(
        commit_input
            .authority_mutations()
            .iter()
            .map(|mutation| serde_json::to_string(mutation).expect("mutation serializes")),
    );
    Ok(commit_input)
}

#[cfg(test)]
pub(crate) fn empty_branch_local_commit_input_through_schema_execution(
    runtime: &mut RelationalRuntime,
    snapshot: SnapshotHandle,
    branch_label: impl Into<String>,
    mutation_origin: MutationOrigin,
) -> Result<TopologyCommitCertificationInput, String> {
    let branch = open_schema_topology_authoring_branch(runtime, branch_label)?;
    Ok(TopologyCommitCertificationInput::empty_from_intent(
        snapshot,
        branch.branch_id().clone(),
        RawTopologyIntent::new(Vec::new(), mutation_origin),
    ))
}

fn deterministic_digest_rows(rows: impl Iterator<Item = String>) -> DeterministicDigest {
    let mut count = 0usize;
    let mut hash = 0xcbf29ce484222325u64;
    for row in rows {
        count += 1;
        for byte in row.as_bytes() {
            hash ^= u64::from(*byte);
            hash = hash.wrapping_mul(0x100000001b3);
        }
        hash ^= u64::from(b'\n');
        hash = hash.wrapping_mul(0x100000001b3);
    }

    DeterministicDigest {
        algorithm: "fnv1a64".to_string(),
        digest_hex: format!("{hash:016x}"),
        row_count: count,
    }
}
