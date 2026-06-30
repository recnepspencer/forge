use crate::workload_platform::evidence_lookup_execution::EvidenceLookupExecutionReceipt;
use crate::workload_platform::evidence_lookup_plan_selection::EvidenceLookupSelectedPlan;

use super::row::{EvidenceLookupDiagnosticRow, EvidenceLookupDiagnosticWitness};

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct EvidenceLookupDiagnosticCounters {
    row_count: usize,
    success_row_count: usize,
    advisory_row_count: usize,
    denial_row_count: usize,
    hidden_lookup_scan_count: usize,
    hidden_broad_receipt_scan_count: usize,
}

impl EvidenceLookupDiagnosticCounters {
    pub(crate) fn from_projection_proof(
        rows: &[EvidenceLookupDiagnosticRow],
        selected_plan: &EvidenceLookupSelectedPlan,
        execution_receipt: &EvidenceLookupExecutionReceipt,
    ) -> Self {
        let mut counters = Self {
            row_count: rows.len(),
            hidden_lookup_scan_count: selected_plan.counters().raw_evidence_row_scan_count()
                + execution_receipt.counters().caller_owned_scan_count(),
            hidden_broad_receipt_scan_count: selected_plan.counters().broad_receipt_scan_count(),
            ..Self::default()
        };
        for row in rows {
            match row.witness() {
                EvidenceLookupDiagnosticWitness::Success => counters.success_row_count += 1,
                EvidenceLookupDiagnosticWitness::Advisory(_) => counters.advisory_row_count += 1,
                EvidenceLookupDiagnosticWitness::Denied(_) => counters.denial_row_count += 1,
            }
        }
        counters
    }

    pub const fn row_count(&self) -> usize {
        self.row_count
    }

    pub const fn success_row_count(&self) -> usize {
        self.success_row_count
    }

    pub const fn advisory_row_count(&self) -> usize {
        self.advisory_row_count
    }

    pub const fn denial_row_count(&self) -> usize {
        self.denial_row_count
    }

    pub const fn hidden_lookup_scan_count(&self) -> usize {
        self.hidden_lookup_scan_count
    }

    pub const fn hidden_broad_receipt_scan_count(&self) -> usize {
        self.hidden_broad_receipt_scan_count
    }
}
