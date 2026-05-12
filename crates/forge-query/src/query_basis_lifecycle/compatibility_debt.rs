#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BasisCompatibilityDebtPosture {
    ScopedMigrationPending,
    CompatibilityAdapterPending,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BasisCompatibilityDebtRow {
    surface_label: &'static str,
    current_input_shape: &'static str,
    target_scoped_shape: &'static str,
    posture: BasisCompatibilityDebtPosture,
    note: &'static str,
}

impl BasisCompatibilityDebtRow {
    pub fn surface_label(&self) -> &'static str {
        self.surface_label
    }

    pub fn current_input_shape(&self) -> &'static str {
        self.current_input_shape
    }

    pub fn target_scoped_shape(&self) -> &'static str {
        self.target_scoped_shape
    }

    pub fn posture(&self) -> BasisCompatibilityDebtPosture {
        self.posture
    }

    pub fn note(&self) -> &'static str {
        self.note
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BasisCompatibilityDebtRegistry {
    rows: &'static [BasisCompatibilityDebtRow],
}

impl BasisCompatibilityDebtRegistry {
    pub fn rows(&self) -> &'static [BasisCompatibilityDebtRow] {
        self.rows
    }
}

const COMPATIBILITY_DEBT_ROWS: &[BasisCompatibilityDebtRow] = &[
    BasisCompatibilityDebtRow {
        surface_label: "query_context::{bind_query_basis_context,admit_query_basis_context,execute_query_basis_context}",
        current_input_shape: "QueryBasisContextRequest + QueryContextBindingSource | AdmittedQueryBasisContext",
        target_scoped_shape: "ScopedQueryBasisContext via admit_scoped_query_basis_context",
        posture: BasisCompatibilityDebtPosture::ScopedMigrationPending,
        note: "a scoped query-context path now exists, but the legacy raw/admitted query-context entrypoints are still exported and ordinary consumers must migrate onto the scoped wrapper path",
    },
    BasisCompatibilityDebtRow {
        surface_label: "preview::{assess_preview_live_drift,PreviewLiveExecutionEnvelope::preview_live}",
        current_input_shape: "PreviewLiveSessionPlanBinding | PreviewLiveExecutionEnvelope",
        target_scoped_shape: "ScopedPreviewLiveSessionPlanBinding | scoped preview-live envelope",
        posture: BasisCompatibilityDebtPosture::CompatibilityAdapterPending,
        note: "preview certification now goes through the scoped preview-live path, but drift and execution-envelope follow-on surfaces still expose the legacy preview-owned live binding shape",
    },
    BasisCompatibilityDebtRow {
        surface_label: "runtime::inspection::causal::*",
        current_input_shape: "QueryObservationReceipt",
        target_scoped_shape: "ScopedInspectionBasis",
        posture: BasisCompatibilityDebtPosture::CompatibilityAdapterPending,
        note: "causal inspection remains downstream of observation receipts today and needs an adapter or migration onto scoped inspection proof",
    },
    BasisCompatibilityDebtRow {
        surface_label: "subscription::{declaration,activation,support,diagnostic}::*",
        current_input_shape: "subscription declaration and activation artifacts",
        target_scoped_shape: "ScopedSubscriptionDeclarationBasis | ScopedSubscriptionActivationBasis",
        posture: BasisCompatibilityDebtPosture::ScopedMigrationPending,
        note: "subscription runtime surfaces still compose around pre-lifecycle admission artifacts instead of phase-3 scoped subscription proof",
    },
];

pub fn basis_compatibility_debt_registry() -> BasisCompatibilityDebtRegistry {
    BasisCompatibilityDebtRegistry {
        rows: COMPATIBILITY_DEBT_ROWS,
    }
}
