use crate::identity::hash_parts;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BasisLifecycleMigrationSurface {
    BranchPreviewAdmission,
    ReadCompositionBasisContext,
    SubscriptionBasisPosture,
    CausalInspectionBasisEvidence,
    HistoricalMaterializationBasis,
    LowerRuntimeReadmissionEvidence,
    FutureNeighborStoreDurableBasis,
}

impl BasisLifecycleMigrationSurface {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::BranchPreviewAdmission => "branch_preview_admission",
            Self::ReadCompositionBasisContext => "read_composition_basis_context",
            Self::SubscriptionBasisPosture => "subscription_basis_posture",
            Self::CausalInspectionBasisEvidence => "causal_inspection_basis_evidence",
            Self::HistoricalMaterializationBasis => "historical_materialization_basis",
            Self::LowerRuntimeReadmissionEvidence => "lower_runtime_readmission_evidence",
            Self::FutureNeighborStoreDurableBasis => "future_neighbor_store_durable_basis",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BasisLifecycleMigrationPosture {
    LifecycleNative,
    LifecycleAdapterCovered,
    CompatibilityDebt,
    DeferredFutureNeighbor,
}

impl BasisLifecycleMigrationPosture {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::LifecycleNative => "lifecycle_native",
            Self::LifecycleAdapterCovered => "lifecycle_adapter_covered",
            Self::CompatibilityDebt => "compatibility_debt",
            Self::DeferredFutureNeighbor => "deferred_future_neighbor",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BasisLifecycleMigrationAuditRow {
    surface: BasisLifecycleMigrationSurface,
    posture: BasisLifecycleMigrationPosture,
    existing_consumer: &'static str,
    lifecycle_artifact: &'static str,
    compatibility_debt: Option<&'static str>,
    row_digest: String,
}

impl BasisLifecycleMigrationAuditRow {
    fn new(
        surface: BasisLifecycleMigrationSurface,
        posture: BasisLifecycleMigrationPosture,
        existing_consumer: &'static str,
        lifecycle_artifact: &'static str,
        compatibility_debt: Option<&'static str>,
    ) -> Self {
        let row_digest = hash_parts(&[
            "basis_lifecycle_migration_audit_row_v1".to_string(),
            format!("surface:{}", surface.as_str()),
            format!("posture:{}", posture.as_str()),
            format!("existing:{existing_consumer}"),
            format!("lifecycle:{lifecycle_artifact}"),
            format!("debt:{}", compatibility_debt.unwrap_or("none")),
        ]);
        Self {
            surface,
            posture,
            existing_consumer,
            lifecycle_artifact,
            compatibility_debt,
            row_digest,
        }
    }

    pub fn surface(&self) -> BasisLifecycleMigrationSurface {
        self.surface
    }

    pub fn posture(&self) -> BasisLifecycleMigrationPosture {
        self.posture
    }

    pub fn existing_consumer(&self) -> &'static str {
        self.existing_consumer
    }

    pub fn lifecycle_artifact(&self) -> &'static str {
        self.lifecycle_artifact
    }

    pub fn compatibility_debt(&self) -> Option<&'static str> {
        self.compatibility_debt
    }

    pub fn row_digest(&self) -> &str {
        &self.row_digest
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BasisLifecycleMigrationCounters {
    audited_surface_count: usize,
    lifecycle_covered_count: usize,
    compatibility_debt_count: usize,
    deferred_future_neighbor_count: usize,
}

impl BasisLifecycleMigrationCounters {
    fn from_rows(rows: &[BasisLifecycleMigrationAuditRow]) -> Self {
        Self {
            audited_surface_count: rows.len(),
            lifecycle_covered_count: rows
                .iter()
                .filter(|row| {
                    matches!(
                        row.posture(),
                        BasisLifecycleMigrationPosture::LifecycleNative
                            | BasisLifecycleMigrationPosture::LifecycleAdapterCovered
                    )
                })
                .count(),
            compatibility_debt_count: rows
                .iter()
                .filter(|row| row.posture() == BasisLifecycleMigrationPosture::CompatibilityDebt)
                .count(),
            deferred_future_neighbor_count: rows
                .iter()
                .filter(|row| {
                    row.posture() == BasisLifecycleMigrationPosture::DeferredFutureNeighbor
                })
                .count(),
        }
    }

    pub fn audited_surface_count(&self) -> usize {
        self.audited_surface_count
    }

    pub fn lifecycle_covered_count(&self) -> usize {
        self.lifecycle_covered_count
    }

    pub fn compatibility_debt_count(&self) -> usize {
        self.compatibility_debt_count
    }

    pub fn deferred_future_neighbor_count(&self) -> usize {
        self.deferred_future_neighbor_count
    }

    pub fn digest(&self) -> String {
        hash_parts(&[
            format!("audited:{}", self.audited_surface_count),
            format!("covered:{}", self.lifecycle_covered_count),
            format!("debt:{}", self.compatibility_debt_count),
            format!("deferred:{}", self.deferred_future_neighbor_count),
        ])
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BasisLifecycleMigrationAudit {
    rows: Vec<BasisLifecycleMigrationAuditRow>,
    counters: BasisLifecycleMigrationCounters,
    audit_digest: String,
}

impl BasisLifecycleMigrationAudit {
    fn new(rows: Vec<BasisLifecycleMigrationAuditRow>) -> Self {
        let counters = BasisLifecycleMigrationCounters::from_rows(&rows);
        let audit_digest = hash_parts(&[
            "basis_lifecycle_migration_audit_v1".to_string(),
            format!("rows:{}", rows_digest(&rows)),
            format!("counters:{}", counters.digest()),
        ]);
        Self {
            rows,
            counters,
            audit_digest,
        }
    }

    pub fn rows(&self) -> &[BasisLifecycleMigrationAuditRow] {
        &self.rows
    }

    pub fn counters(&self) -> &BasisLifecycleMigrationCounters {
        &self.counters
    }

    pub fn audit_digest(&self) -> &str {
        &self.audit_digest
    }

    pub fn row_for(
        &self,
        surface: BasisLifecycleMigrationSurface,
    ) -> Option<&BasisLifecycleMigrationAuditRow> {
        self.rows.iter().find(|row| row.surface() == surface)
    }
}

pub fn basis_lifecycle_migration_audit() -> BasisLifecycleMigrationAudit {
    BasisLifecycleMigrationAudit::new(vec![
        BasisLifecycleMigrationAuditRow::new(
            BasisLifecycleMigrationSurface::BranchPreviewAdmission,
            BasisLifecycleMigrationPosture::LifecycleNative,
            "WorthQueryBranchBasisAdmission / WorthQueryPreviewBasisAdmission",
            "basis_lifecycle branch_head / preview declarative scoped paths",
            None,
        ),
        BasisLifecycleMigrationAuditRow::new(
            BasisLifecycleMigrationSurface::ReadCompositionBasisContext,
            BasisLifecycleMigrationPosture::LifecycleNative,
            "QueryBasisContextRequest / ExecutionBasisIntent / ResolvedSnapshotBasis",
            "ScopedObservationQueryBasisContext / ScopedMaterializationQueryBasisContext",
            None,
        ),
        BasisLifecycleMigrationAuditRow::new(
            BasisLifecycleMigrationSurface::SubscriptionBasisPosture,
            BasisLifecycleMigrationPosture::LifecycleNative,
            "QuerySubscriptionBasisPosture and bridge basis request digests",
            "ScopedSubscriptionDeclarationBasis / ScopedSubscriptionActivationBasis",
            None,
        ),
        BasisLifecycleMigrationAuditRow::new(
            BasisLifecycleMigrationSurface::CausalInspectionBasisEvidence,
            BasisLifecycleMigrationPosture::LifecycleNative,
            "causal observation anchors and bridge causal envelopes",
            "QueryObservationReceipt plus ScopedInspectionBasis",
            None,
        ),
        BasisLifecycleMigrationAuditRow::new(
            BasisLifecycleMigrationSurface::HistoricalMaterializationBasis,
            BasisLifecycleMigrationPosture::LifecycleNative,
            "HistoricalMaterializationDescriptor basis digest strings",
            "ScopedMaterializationBasis",
            None,
        ),
        BasisLifecycleMigrationAuditRow::new(
            BasisLifecycleMigrationSurface::LowerRuntimeReadmissionEvidence,
            BasisLifecycleMigrationPosture::LifecycleNative,
            "bridge/relational/signal facade evidence digests",
            "LowerRuntimeBasisEvidence and LowerRuntimeBoundBasis",
            None,
        ),
        BasisLifecycleMigrationAuditRow::new(
            BasisLifecycleMigrationSurface::FutureNeighborStoreDurableBasis,
            BasisLifecycleMigrationPosture::DeferredFutureNeighbor,
            "store-backed parity and durable reload basis claims",
            "typed deferred RawBasisIntent neighbors",
            None,
        ),
    ])
}

pub fn basis_lifecycle_migration_audit_digest() -> String {
    basis_lifecycle_migration_audit().audit_digest().to_string()
}

fn rows_digest(rows: &[BasisLifecycleMigrationAuditRow]) -> String {
    hash_parts(
        &rows
            .iter()
            .map(|row| row.row_digest().to_string())
            .collect::<Vec<_>>(),
    )
}

#[cfg(test)]
mod tests;
