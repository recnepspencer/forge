use forge_store_layout_indexes::access_planning;
use forge_store_layout_indexes::bootstrap::BootstrapCatalogReadAdmission;
use forge_store_layout_indexes::declarations::{
    AdmittedPhysicalArtifactFamily, PhysicalArtifactFamily,
};
use forge_store_layout_indexes::integrity::{
    layout_corruption, LayoutCorruptionInput, OfflineReadmissionView,
};
use forge_store_layout_indexes::materialization::{
    AdmittedLayoutMaterialization, MaterializationDenial,
};
use forge_store_recovery_physics::ReopenedRecoveryArtifactAdmission;

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
    custody: BackupExportCustodyAdmission,
) -> RestoredLayoutMaterializationOutcome {
    let requirement = layout_corruption()
        .classify(LayoutCorruptionInput::OfflineEvidence {
            family,
            admission: reopened.clone(),
        })
        .into_offline_readmission_requirement()
        .expect("offline recovery evidence always classifies for offline readmission");
    let recovery_witness =
        forge_store_recovery_physics::admit_offline_layout_readmission(family.id(), reopened);
    let readmission = layout_corruption().readmit_offline(requirement, recovery_witness);
    let readmission = match readmission.view() {
        OfflineReadmissionView::Readmitted(witness) => *witness,
        OfflineReadmissionView::Denied(_) => {
            unreachable!("owner-derived recovery witness must satisfy its own requirement")
        }
    };
    let Some(custody) = custody.into_readmitted_security_scope() else {
        return RestoredLayoutMaterializationOutcome::custody_readmission_required();
    };

    match access_planning().admit_restored_artifact_materialization(
        admitted_family,
        catalog,
        readmission,
        custody,
    ) {
        Ok(materialization) => RestoredLayoutMaterializationOutcome::materialized(materialization),
        Err(denial) => RestoredLayoutMaterializationOutcome::materialization_denied(denial),
    }
}
