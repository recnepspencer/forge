use crate::graph_read_access_declarations::{
    WorthGraphReadAdmissionCapabilityGap, WorthGraphReadDeclarationDeletionLedgerRow,
    WorthGraphReadDeclarationDeletionStatus, WorthGraphReadDeclarationReadFamilyIdentity,
    WorthGraphReadRequirementDerivationCapabilityGap, WorthGraphReadRequirementRowDigestProjection,
};

use super::inventory_disposition::WorthGraphReadAccessPlanAdoptionExecutionFolkloreDisposition;
use super::stable_digest;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthGraphReadAccessPlanAdoptionExecutionFolkloreRow {
    source_path: String,
    owner: String,
    current_caller: String,
    execution_folklore_class: String,
    disposition: WorthGraphReadAccessPlanAdoptionExecutionFolkloreDisposition,
    displacement_target: String,
    migration_target: String,
    deletion_trigger: String,
    blocker: Option<String>,
    row_digest: String,
}

impl WorthGraphReadAccessPlanAdoptionExecutionFolkloreRow {
    pub(crate) fn from_read_family_identity(
        identity: &WorthGraphReadDeclarationReadFamilyIdentity,
    ) -> Self {
        Self::new(RowParts {
            source_path: format!(
                "milestone-seven/read-family/{}/{}",
                identity.read_family_target(),
                identity.identity_digest()
            ),
            owner: "worth_graph_read_access_plan_adoption".to_string(),
            current_caller: identity.touched_authority_input().to_string(),
            execution_folklore_class: format!(
                "covered-read-family-adoption:{}",
                identity.query_family_name()
            ),
            disposition: WorthGraphReadAccessPlanAdoptionExecutionFolkloreDisposition::Migrate,
            displacement_target: "Query graph-read access-plan admission".to_string(),
            migration_target: "graph_read_access_plan_adoption/phase_two_parallel_adoption_lane"
                .to_string(),
            deletion_trigger: format!(
                "Read family {} must enter Query access-plan admission before execution",
                identity.query_family_name()
            ),
            blocker: None,
        })
    }

    pub(crate) fn from_requirement_row(row: &WorthGraphReadRequirementRowDigestProjection) -> Self {
        Self::new(RowParts {
            source_path: format!(
                "milestone-seven/requirement-row/{}",
                row.requirement_row_digest()
            ),
            owner: "worth_graph_read_access_plan_adoption".to_string(),
            current_caller: row.source_requirement_record_digest().to_string(),
            execution_folklore_class: "covered-requirement-row-adoption".to_string(),
            disposition: WorthGraphReadAccessPlanAdoptionExecutionFolkloreDisposition::Migrate,
            displacement_target: "Query graph-read access-plan requirement admission".to_string(),
            migration_target: "graph_read_access_plan_adoption/phase_two_parallel_adoption_lane"
                .to_string(),
            deletion_trigger: format!(
                "Requirement row {} must be admitted or postured by Query before execution",
                row.requirement_row_digest()
            ),
            blocker: None,
        })
    }

    pub(crate) fn from_deletion_row(row: &WorthGraphReadDeclarationDeletionLedgerRow) -> Self {
        let disposition = match row.status() {
            WorthGraphReadDeclarationDeletionStatus::Deleted => {
                WorthGraphReadAccessPlanAdoptionExecutionFolkloreDisposition::Delete
            }
            WorthGraphReadDeclarationDeletionStatus::CappedResidue => {
                WorthGraphReadAccessPlanAdoptionExecutionFolkloreDisposition::Cap
            }
        };
        Self::new(RowParts {
            source_path: row.source_path().to_string(),
            owner: row.owner().to_string(),
            current_caller: row.current_caller().to_string(),
            execution_folklore_class: "declaration-deletion-ledger-execution-folklore".to_string(),
            disposition,
            displacement_target: "Query graph-read access-plan adoption lane".to_string(),
            migration_target: "graph_read_access_plan_adoption".to_string(),
            deletion_trigger: row.deletion_trigger().to_string(),
            blocker: row.blocker().map(str::to_string),
        })
    }

    pub(crate) fn from_admission_gap(gap: &WorthGraphReadAdmissionCapabilityGap) -> Self {
        Self::new(RowParts {
            source_path: format!("milestone-seven/admission-gap/{}", gap.gap_digest()),
            owner: gap.owner().to_string(),
            current_caller: gap.source_requirement_record_digest().to_string(),
            execution_folklore_class: format!("admission-gap:{:?}", gap.kind()),
            disposition: WorthGraphReadAccessPlanAdoptionExecutionFolkloreDisposition::QueryGap,
            displacement_target: "Query access posture matrix".to_string(),
            migration_target: "graph_read_access_plan_adoption/access_posture".to_string(),
            deletion_trigger: gap.removal_trigger().to_string(),
            blocker: Some(gap.blocker().to_string()),
        })
    }

    pub(crate) fn from_requirement_gap(
        gap: &WorthGraphReadRequirementDerivationCapabilityGap,
    ) -> Self {
        Self::new(RowParts {
            source_path: format!("milestone-seven/requirement-gap/{}", gap.gap_digest()),
            owner: "worth_graph_read_declarations".to_string(),
            current_caller: gap.source_catalog_record_digest().to_string(),
            execution_folklore_class: format!("requirement-gap:{}", gap.kind().as_str()),
            disposition: WorthGraphReadAccessPlanAdoptionExecutionFolkloreDisposition::QueryGap,
            displacement_target: "Query requirement derivation and access admission".to_string(),
            migration_target: "graph_read_access_plan_adoption/access_posture".to_string(),
            deletion_trigger: gap.removal_trigger().to_string(),
            blocker: Some(gap.blocker().to_string()),
        })
    }

    fn new(parts: RowParts) -> Self {
        let row_digest = stable_digest(&[
            "worth_graph_read_access_plan_adoption_execution_folklore_row_v1".to_string(),
            format!("source_path:{}", parts.source_path),
            format!("owner:{}", parts.owner),
            format!("current_caller:{}", parts.current_caller),
            format!("class:{}", parts.execution_folklore_class),
            format!("disposition:{}", parts.disposition.as_str()),
            format!("displacement_target:{}", parts.displacement_target),
            format!("migration_target:{}", parts.migration_target),
            format!("deletion_trigger:{}", parts.deletion_trigger),
            format!("blocker:{}", parts.blocker.as_deref().unwrap_or("none")),
        ]);
        Self {
            source_path: parts.source_path,
            owner: parts.owner,
            current_caller: parts.current_caller,
            execution_folklore_class: parts.execution_folklore_class,
            disposition: parts.disposition,
            displacement_target: parts.displacement_target,
            migration_target: parts.migration_target,
            deletion_trigger: parts.deletion_trigger,
            blocker: parts.blocker,
            row_digest,
        }
    }

    pub fn source_path(&self) -> &str {
        &self.source_path
    }

    pub fn owner(&self) -> &str {
        &self.owner
    }

    pub fn current_caller(&self) -> &str {
        &self.current_caller
    }

    pub fn execution_folklore_class(&self) -> &str {
        &self.execution_folklore_class
    }

    pub const fn disposition(
        &self,
    ) -> WorthGraphReadAccessPlanAdoptionExecutionFolkloreDisposition {
        self.disposition
    }

    pub fn displacement_target(&self) -> &str {
        &self.displacement_target
    }

    pub fn migration_target(&self) -> &str {
        &self.migration_target
    }

    pub fn deletion_trigger(&self) -> &str {
        &self.deletion_trigger
    }

    pub fn blocker(&self) -> Option<&str> {
        self.blocker.as_deref()
    }

    pub fn row_digest(&self) -> &str {
        &self.row_digest
    }
}

struct RowParts {
    source_path: String,
    owner: String,
    current_caller: String,
    execution_folklore_class: String,
    disposition: WorthGraphReadAccessPlanAdoptionExecutionFolkloreDisposition,
    displacement_target: String,
    migration_target: String,
    deletion_trigger: String,
    blocker: Option<String>,
}
