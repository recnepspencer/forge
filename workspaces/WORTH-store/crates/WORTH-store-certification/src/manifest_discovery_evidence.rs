use crate::PhysicalSubstrateLane;
use worth_store_physical_format::{
    ManifestDiscoveryCounterSnapshot, ManifestDiscoveryDenial, ManifestDiscoveryDenialKind,
    ManifestDiscoveryReport, PhysicalReferenceDenialKind,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalManifestDiscoveryEvidenceRow {
    RootManifestDiscovery,
    RootPublicationGenerationChanged,
    BackendResidueRejected,
    FreeSpaceReuseGenerationChanged,
}

impl PhysicalManifestDiscoveryEvidenceRow {
    pub const fn s1_required() -> [Self; 4] {
        [
            Self::RootManifestDiscovery,
            Self::RootPublicationGenerationChanged,
            Self::BackendResidueRejected,
            Self::FreeSpaceReuseGenerationChanged,
        ]
    }

    pub const fn physical_substrate_lane(self) -> PhysicalSubstrateLane {
        match self {
            Self::RootManifestDiscovery => PhysicalSubstrateLane::HappyAuthority,
            Self::RootPublicationGenerationChanged
            | Self::BackendResidueRejected
            | Self::FreeSpaceReuseGenerationChanged => PhysicalSubstrateLane::HostileFormat,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PhysicalManifestDiscoveryEvidenceReport {
    row: PhysicalManifestDiscoveryEvidenceRow,
    lane: PhysicalSubstrateLane,
    counters: ManifestDiscoveryCounterSnapshot,
}

impl PhysicalManifestDiscoveryEvidenceReport {
    pub fn from_manifest_report(
        row: PhysicalManifestDiscoveryEvidenceRow,
        report: ManifestDiscoveryReport<'_>,
    ) -> Result<Self, PhysicalManifestDiscoveryEvidenceDenial> {
        if row != PhysicalManifestDiscoveryEvidenceRow::RootManifestDiscovery {
            return Err(PhysicalManifestDiscoveryEvidenceDenial::UnexpectedReportRow(row));
        }
        Ok(Self::new(row, report.counters()))
    }

    pub fn from_manifest_denial(
        row: PhysicalManifestDiscoveryEvidenceRow,
        denial: ManifestDiscoveryDenial,
    ) -> Result<Self, PhysicalManifestDiscoveryEvidenceDenial> {
        require_denial_kind(row, denial)?;
        Ok(Self::new(row, denial.counters()))
    }

    pub const fn row(self) -> PhysicalManifestDiscoveryEvidenceRow {
        self.row
    }

    pub const fn lane(self) -> PhysicalSubstrateLane {
        self.lane
    }

    pub const fn counters(self) -> ManifestDiscoveryCounterSnapshot {
        self.counters
    }

    const fn new(
        row: PhysicalManifestDiscoveryEvidenceRow,
        counters: ManifestDiscoveryCounterSnapshot,
    ) -> Self {
        Self {
            row,
            lane: row.physical_substrate_lane(),
            counters,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalManifestDiscoveryEvidenceDenial {
    UnexpectedReportRow(PhysicalManifestDiscoveryEvidenceRow),
    UnexpectedDenialRow(PhysicalManifestDiscoveryEvidenceRow),
    UnexpectedManifestDenial {
        expected: ManifestDiscoveryDenialKind,
        actual: ManifestDiscoveryDenialKind,
    },
    MissingStaleFreeSpaceReuseGeneration,
    MissingStaleRootPublicationGeneration,
}

fn require_denial_kind(
    row: PhysicalManifestDiscoveryEvidenceRow,
    denial: ManifestDiscoveryDenial,
) -> Result<(), PhysicalManifestDiscoveryEvidenceDenial> {
    match row {
        PhysicalManifestDiscoveryEvidenceRow::BackendResidueRejected => require_manifest_denial(
            ManifestDiscoveryDenialKind::BackendResidueDiscoverySource,
            denial,
        ),
        PhysicalManifestDiscoveryEvidenceRow::FreeSpaceReuseGenerationChanged => {
            require_free_space_stale_denial(denial)
        }
        PhysicalManifestDiscoveryEvidenceRow::RootPublicationGenerationChanged => {
            require_root_publication_stale_denial(denial)
        }
        PhysicalManifestDiscoveryEvidenceRow::RootManifestDiscovery => {
            Err(PhysicalManifestDiscoveryEvidenceDenial::UnexpectedDenialRow(row))
        }
    }
}

fn require_manifest_denial(
    expected: ManifestDiscoveryDenialKind,
    denial: ManifestDiscoveryDenial,
) -> Result<(), PhysicalManifestDiscoveryEvidenceDenial> {
    if denial.kind() != expected {
        return Err(
            PhysicalManifestDiscoveryEvidenceDenial::UnexpectedManifestDenial {
                expected,
                actual: denial.kind(),
            },
        );
    }
    Ok(())
}

fn require_free_space_stale_denial(
    denial: ManifestDiscoveryDenial,
) -> Result<(), PhysicalManifestDiscoveryEvidenceDenial> {
    require_manifest_denial(
        ManifestDiscoveryDenialKind::ReferenceValidationDenied,
        denial,
    )?;
    let Some(reference_denial) = denial.reference_denial() else {
        return Err(PhysicalManifestDiscoveryEvidenceDenial::MissingStaleFreeSpaceReuseGeneration);
    };
    if reference_denial.kind() != PhysicalReferenceDenialKind::StaleFreeSpaceReuseGeneration {
        return Err(PhysicalManifestDiscoveryEvidenceDenial::MissingStaleFreeSpaceReuseGeneration);
    }
    Ok(())
}

fn require_root_publication_stale_denial(
    denial: ManifestDiscoveryDenial,
) -> Result<(), PhysicalManifestDiscoveryEvidenceDenial> {
    require_manifest_denial(
        ManifestDiscoveryDenialKind::ReferenceValidationDenied,
        denial,
    )?;
    let Some(reference_denial) = denial.reference_denial() else {
        return Err(PhysicalManifestDiscoveryEvidenceDenial::MissingStaleRootPublicationGeneration);
    };
    if reference_denial.kind() != PhysicalReferenceDenialKind::StaleRootPublicationGeneration {
        return Err(PhysicalManifestDiscoveryEvidenceDenial::MissingStaleRootPublicationGeneration);
    }
    Ok(())
}
