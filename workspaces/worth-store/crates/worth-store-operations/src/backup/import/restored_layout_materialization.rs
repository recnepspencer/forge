use worth_store_layout_indexes::access_planning;
use worth_store_layout_indexes::bootstrap::BootstrapCatalogReadAdmission;
use worth_store_layout_indexes::declarations::{
    AdmittedPhysicalArtifactFamily, PhysicalArtifactFamily,
};
use worth_store_layout_indexes::integrity::{
    layout_corruption, offline_readmission, OfflineReadmissionView,
};
use worth_store_layout_indexes::materialization::{
    AdmittedLayoutMaterialization, MaterializationDenial,
};
use worth_store_recovery_physics::ReopenedRecoveryArtifactAdmission;

use crate::BackupExportCustodyAdmission;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RestoredLayoutMaterializationCaseId(&'static str);

impl RestoredLayoutMaterializationCaseId {
    pub const fn as_str(self) -> &'static str {
        self.0
    }
}

const MATERIALIZED: RestoredLayoutMaterializationCaseId =
    RestoredLayoutMaterializationCaseId("restore.layout.materialization.materialized");
const CUSTODY_READMISSION_REQUIRED: RestoredLayoutMaterializationCaseId =
    RestoredLayoutMaterializationCaseId(
        "restore.layout.materialization.denied.custody_readmission_required",
    );
const MATERIALIZATION_DENIED: RestoredLayoutMaterializationCaseId =
    RestoredLayoutMaterializationCaseId("restore.layout.materialization.denied.layout");

pub fn restored_layout_materialization_cases(
) -> impl Iterator<Item = RestoredLayoutMaterializationCaseId> {
    [
        MATERIALIZED,
        CUSTODY_READMISSION_REQUIRED,
        MATERIALIZATION_DENIED,
    ]
    .into_iter()
}

#[derive(Debug, PartialEq, Eq)]
enum RestoredLayoutMaterializationCase {
    Materialized(AdmittedLayoutMaterialization),
    CustodyReadmissionRequired,
    MaterializationDenied(MaterializationDenial),
}

#[derive(Debug, PartialEq, Eq)]
pub struct RestoredLayoutMaterializationOutcome {
    case: RestoredLayoutMaterializationCase,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RestoredLayoutMaterializationObservation {
    case_id: RestoredLayoutMaterializationCaseId,
}

impl RestoredLayoutMaterializationObservation {
    pub const fn case_id(self) -> RestoredLayoutMaterializationCaseId {
        self.case_id
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RestoredLayoutMaterializationView<'a> {
    Materialized(&'a AdmittedLayoutMaterialization),
    CustodyReadmissionRequired,
    MaterializationDenied(&'a MaterializationDenial),
}

impl RestoredLayoutMaterializationOutcome {
    fn materialized(materialization: AdmittedLayoutMaterialization) -> Self {
        Self {
            case: RestoredLayoutMaterializationCase::Materialized(materialization),
        }
    }

    fn custody_readmission_required() -> Self {
        Self {
            case: RestoredLayoutMaterializationCase::CustodyReadmissionRequired,
        }
    }

    fn materialization_denied(denial: MaterializationDenial) -> Self {
        Self {
            case: RestoredLayoutMaterializationCase::MaterializationDenied(denial),
        }
    }

    pub fn view(&self) -> RestoredLayoutMaterializationView<'_> {
        match &self.case {
            RestoredLayoutMaterializationCase::Materialized(value) => {
                RestoredLayoutMaterializationView::Materialized(value)
            }
            RestoredLayoutMaterializationCase::CustodyReadmissionRequired => {
                RestoredLayoutMaterializationView::CustodyReadmissionRequired
            }
            RestoredLayoutMaterializationCase::MaterializationDenied(denial) => {
                RestoredLayoutMaterializationView::MaterializationDenied(denial)
            }
        }
    }

    pub fn case_id(&self) -> RestoredLayoutMaterializationCaseId {
        match self.case {
            RestoredLayoutMaterializationCase::Materialized(_) => MATERIALIZED,
            RestoredLayoutMaterializationCase::CustodyReadmissionRequired => {
                CUSTODY_READMISSION_REQUIRED
            }
            RestoredLayoutMaterializationCase::MaterializationDenied(_) => MATERIALIZATION_DENIED,
        }
    }

    pub fn owner_case_observation(&self) -> RestoredLayoutMaterializationObservation {
        RestoredLayoutMaterializationObservation {
            case_id: self.case_id(),
        }
    }

    pub fn into_materialized(self) -> Result<AdmittedLayoutMaterialization, Self> {
        match self.case {
            RestoredLayoutMaterializationCase::Materialized(value) => Ok(value),
            case => Err(Self { case }),
        }
    }
}

pub fn admit_restored_layout_materialization(
    family: PhysicalArtifactFamily,
    admitted_family: AdmittedPhysicalArtifactFamily,
    catalog: &BootstrapCatalogReadAdmission,
    reopened: &ReopenedRecoveryArtifactAdmission,
    custody: &BackupExportCustodyAdmission,
) -> RestoredLayoutMaterializationOutcome {
    let requirement = layout_corruption()
        .require_offline_readmission(admitted_family, reopened)
        .into_offline_readmission_requirement()
        .expect("offline recovery evidence always classifies for offline readmission");
    let recovery_witness = worth_store_recovery_physics::layout_readmission()
        .admit_offline(family.id(), reopened)
        .expect("reopened recovery admission issues offline readmission");
    let readmission = offline_readmission().admit(requirement, recovery_witness);
    let readmission = match readmission.view() {
        OfflineReadmissionView::Readmitted(witness) => *witness,
        OfflineReadmissionView::Denied(_) => {
            unreachable!("owner-derived recovery witness must satisfy its own requirement")
        }
    };
    let Some(custody) = custody.readmitted_security_scope() else {
        return RestoredLayoutMaterializationOutcome::custody_readmission_required();
    };

    match access_planning()
        .admit_restored_artifact_materialization(admitted_family, catalog, readmission, custody)
        .into_result()
    {
        Ok(materialization) => RestoredLayoutMaterializationOutcome::materialized(materialization),
        Err(denial) => RestoredLayoutMaterializationOutcome::materialization_denied(denial),
    }
}
