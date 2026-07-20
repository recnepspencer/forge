use std::path::Path;

use sha2::{Digest, Sha256};
use worth_store_offline_verifier::{
    compose_operational_truth, OfflineInspectionBudget, OfflineStoreInspection,
    OfflineTruthEvidenceSet, OperationalTruthCompositionBudget, OperationalTruthReport,
    UntrustedOfflineMediaSet,
};
use worth_store_physical_backend::{OfflineMediaClosureEntry, OfflineMediaConsistencyBasis};

use crate::{OperationalCounterReceipt, OperationalOperationId};

pub struct InspectedScenarioTruth {
    operation: OperationalOperationId,
    report: OperationalTruthReport,
    counters: OperationalCounterReceipt,
}

pub fn inspect_scenario_truth(operation_name: &str, root: &Path) -> InspectedScenarioTruth {
    let operation =
        OperationalOperationId::new(operation_name).expect("offline verification operation");
    let walked = scenario_inspection(operation_name, root)
        .start()
        .expect("offline scenario inspection start")
        .finish()
        .expect("offline scenario inspection completion");
    let counters = OperationalCounterReceipt::from_offline_verification(&operation, &walked);
    let report = compose_operational_truth(
        walked,
        &OfflineTruthEvidenceSet::default(),
        OperationalTruthCompositionBudget::bounded(16 * 1024 * 1024)
            .expect("offline scenario truth budget"),
    )
    .expect("offline scenario truth composition");
    InspectedScenarioTruth {
        operation,
        report,
        counters,
    }
}

pub fn certify_scenario_truth_restarts(
    operation_name: &str,
    root: &Path,
) -> worth_store_offline_verifier::RestartingOfflineScanReceipt {
    scenario_inspection(operation_name, root)
        .certify_bounded_restart_matrix(16)
        .expect("every offline scenario chunk boundary restarts")
}

fn scenario_inspection(operation_name: &str, root: &Path) -> OfflineStoreInspection {
    let entries = std::fs::read_dir(root)
        .expect("offline scenario media root")
        .map(|entry| {
            let path = entry.expect("offline scenario media entry").path();
            let bytes = std::fs::read(&path).expect("offline scenario media bytes");
            OfflineMediaClosureEntry::new(path, bytes.len() as u64, Sha256::digest(bytes).into())
                .expect("offline scenario closure entry")
        })
        .collect::<Vec<_>>();
    let basis = OfflineMediaConsistencyBasis::content_addressed_closure(
        format!("{operation_name}/media-closure"),
        entries,
    )
    .expect("offline scenario consistency basis");
    OfflineStoreInspection::open(UntrustedOfflineMediaSet::from_root(root, basis)).budget(
        OfflineInspectionBudget::bounded(64 * 1024, u64::MAX)
            .expect("offline scenario inspection budget"),
    )
}

impl InspectedScenarioTruth {
    pub const fn operation(&self) -> &OperationalOperationId {
        &self.operation
    }

    pub const fn report(&self) -> &OperationalTruthReport {
        &self.report
    }

    pub const fn counters(&self) -> OperationalCounterReceipt {
        self.counters
    }

    pub fn into_report(self) -> OperationalTruthReport {
        self.report
    }
}
