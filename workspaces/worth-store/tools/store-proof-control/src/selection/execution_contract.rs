use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

use serde::{Deserialize, Serialize};

use super::{ProofProcessModel, StoreProofMode};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "kebab-case")]
pub enum ProofExecutionIsolation {
    FreshCargoProcess,
    FreshCargoProcessWithChildren,
    AllocatorGlobalProcess,
    ExternalToolProcess,
}

impl ProofExecutionIsolation {
    pub(crate) const fn for_process_model(model: ProofProcessModel) -> Self {
        match model {
            ProofProcessModel::AllocatorGlobalProcess => Self::AllocatorGlobalProcess,
            ProofProcessModel::ExternalToolProcess => Self::ExternalToolProcess,
            ProofProcessModel::LibtestWithDeclaredSubprocesses
            | ProofProcessModel::LibtestWithFreshChildProcess
            | ProofProcessModel::LibtestWithNestedCargoProcess
            | ProofProcessModel::NestedCargoProcess
            | ProofProcessModel::StandardizedUiHarness => Self::FreshCargoProcessWithChildren,
            _ => Self::FreshCargoProcess,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(tag = "kind", rename_all = "kebab-case")]
pub enum ProofExecutionCommand {
    Cargo,
    ExternalTool {
        program: String,
        arguments: Vec<String>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct ProofExecutionResources {
    pub target_root: String,
    pub process_global_state: BTreeSet<String>,
    pub environment: BTreeMap<String, String>,
    pub store_roots: BTreeSet<String>,
    pub ports: BTreeSet<u16>,
    pub exclusive_external_tools: BTreeSet<String>,
}

impl ProofExecutionResources {
    pub(crate) fn unbound(model: ProofProcessModel) -> Self {
        let mut process_global_state = BTreeSet::new();
        if model == ProofProcessModel::AllocatorGlobalProcess {
            process_global_state.insert("process-allocator-control".to_owned());
        }
        Self {
            target_root: "unbound-target-root".to_owned(),
            process_global_state,
            environment: BTreeMap::new(),
            store_roots: BTreeSet::new(),
            ports: BTreeSet::new(),
            exclusive_external_tools: BTreeSet::new(),
        }
    }

    pub(crate) fn bind_target_root(&mut self, workspace_root: &Path) {
        self.bind_explicit_target_root(&workspace_root.join("target"));
    }

    pub(crate) fn bind_explicit_target_root(&mut self, target_root: &Path) {
        self.target_root = target_root.to_string_lossy().replace('\\', "/");
    }

    pub fn conflicts_with(&self, other: &Self) -> bool {
        self.target_root == other.target_root
            || overlaps(&self.process_global_state, &other.process_global_state)
            || overlaps(&self.store_roots, &other.store_roots)
            || overlaps(&self.ports, &other.ports)
            || overlaps(
                &self.exclusive_external_tools,
                &other.exclusive_external_tools,
            )
            || environment_conflicts(&self.environment, &other.environment)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct ProofRetryPolicy {
    pub maximum_retries: u8,
    pub admitted_exit_codes: BTreeSet<i32>,
}

impl ProofRetryPolicy {
    pub(crate) fn never() -> Self {
        Self {
            maximum_retries: 0,
            admitted_exit_codes: BTreeSet::new(),
        }
    }

    pub fn admits(&self, exit_code: Option<i32>, completed_attempts: usize) -> bool {
        completed_attempts <= usize::from(self.maximum_retries)
            && exit_code.is_some_and(|code| self.admitted_exit_codes.contains(&code))
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum ProofFailurePolicy {
    StopAllAfterFailure,
    ContinueIndependent,
}

impl ProofFailurePolicy {
    pub(crate) const fn for_mode(mode: StoreProofMode) -> Self {
        match mode {
            StoreProofMode::Owner | StoreProofMode::Smoke => Self::StopAllAfterFailure,
            StoreProofMode::Ui
            | StoreProofMode::Ci
            | StoreProofMode::Soak
            | StoreProofMode::Release
            | StoreProofMode::Hardware => Self::ContinueIndependent,
        }
    }
}

pub(crate) const fn timeout_millis(mode: StoreProofMode) -> u64 {
    match mode {
        StoreProofMode::Owner => 120_000,
        StoreProofMode::Smoke => 600_000,
        StoreProofMode::Ui => 1_800_000,
        StoreProofMode::Ci => 3_600_000,
        StoreProofMode::Soak | StoreProofMode::Release | StoreProofMode::Hardware => 7_200_000,
    }
}

fn overlaps<T: Ord>(left: &BTreeSet<T>, right: &BTreeSet<T>) -> bool {
    left.iter().any(|item| right.contains(item))
}

fn environment_conflicts(
    left: &BTreeMap<String, String>,
    right: &BTreeMap<String, String>,
) -> bool {
    left.iter()
        .any(|(key, value)| right.get(key).is_some_and(|other| other != value))
}
