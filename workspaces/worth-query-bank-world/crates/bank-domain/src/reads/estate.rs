use crate::estate::{
    BranchId, DeathNoticeId, DeathNoticeStatus, EstateCaseId, EstateCaseStatus,
    EstateWorkflowStage, LegalAuthorityId, LegalAuthorityKind, MandatoryReviewId,
    MandatoryReviewKind, MandatoryReviewStatus,
};
use crate::model::{AccountId, AccountName, BankPrincipalId, EmployeeAssignmentId, EmployeeRole};
use crate::schema::AccountStatus;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EstateAccountView {
    id: AccountId,
    display_name: AccountName,
    status: AccountStatus,
}

impl EstateAccountView {
    pub const fn from_projection(
        id: AccountId,
        display_name: AccountName,
        status: AccountStatus,
    ) -> Self {
        Self {
            id,
            display_name,
            status,
        }
    }

    pub const fn id(&self) -> AccountId {
        self.id
    }

    pub const fn display_name(&self) -> &AccountName {
        &self.display_name
    }

    pub const fn status(&self) -> AccountStatus {
        self.status
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EstateDeathNoticeView {
    id: DeathNoticeId,
    status: DeathNoticeStatus,
}

impl EstateDeathNoticeView {
    pub const fn from_projection(id: DeathNoticeId, status: DeathNoticeStatus) -> Self {
        Self { id, status }
    }

    pub const fn id(self) -> DeathNoticeId {
        self.id
    }

    pub const fn status(self) -> DeathNoticeStatus {
        self.status
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EstateLegalAuthorityView {
    id: LegalAuthorityId,
    holder: BankPrincipalId,
    kind: LegalAuthorityKind,
    recognized: bool,
}

impl EstateLegalAuthorityView {
    pub const fn from_projection(
        id: LegalAuthorityId,
        holder: BankPrincipalId,
        kind: LegalAuthorityKind,
        recognized: bool,
    ) -> Self {
        Self {
            id,
            holder,
            kind,
            recognized,
        }
    }

    pub const fn holder(self) -> BankPrincipalId {
        self.holder
    }

    pub const fn id(self) -> LegalAuthorityId {
        self.id
    }

    pub const fn kind(self) -> LegalAuthorityKind {
        self.kind
    }

    pub const fn recognized(self) -> bool {
        self.recognized
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EstateAssignmentView {
    assignment: EmployeeAssignmentId,
    principal: BankPrincipalId,
    role: EmployeeRole,
}

impl EstateAssignmentView {
    pub const fn from_projection(
        assignment: EmployeeAssignmentId,
        principal: BankPrincipalId,
        role: EmployeeRole,
    ) -> Self {
        Self {
            assignment,
            principal,
            role,
        }
    }

    pub const fn principal(self) -> BankPrincipalId {
        self.principal
    }

    pub const fn assignment(self) -> EmployeeAssignmentId {
        self.assignment
    }

    pub const fn role(self) -> EmployeeRole {
        self.role
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EstateReviewView {
    id: MandatoryReviewId,
    kind: MandatoryReviewKind,
    status: MandatoryReviewStatus,
    reviewer: Option<BankPrincipalId>,
}

impl EstateReviewView {
    pub const fn from_projection(
        id: MandatoryReviewId,
        kind: MandatoryReviewKind,
        status: MandatoryReviewStatus,
        reviewer: Option<BankPrincipalId>,
    ) -> Self {
        Self {
            id,
            kind,
            status,
            reviewer,
        }
    }

    pub const fn status(self) -> MandatoryReviewStatus {
        self.status
    }

    pub const fn id(self) -> MandatoryReviewId {
        self.id
    }

    pub const fn kind(self) -> MandatoryReviewKind {
        self.kind
    }

    pub const fn reviewer(self) -> Option<BankPrincipalId> {
        self.reviewer
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EstateCaseOverview {
    id: EstateCaseId,
    stage: EstateWorkflowStage,
    status: EstateCaseStatus,
    branch: BranchId,
    account: EstateAccountView,
    death_notice: EstateDeathNoticeView,
    deceased: BankPrincipalId,
    executors: Vec<BankPrincipalId>,
    beneficiaries: Vec<BankPrincipalId>,
    assignments: Vec<EstateAssignmentView>,
    legal_authorities: Vec<EstateLegalAuthorityView>,
    reviews: Vec<EstateReviewView>,
}

pub(crate) struct EstateCaseOverviewProjection {
    pub(crate) id: EstateCaseId,
    pub(crate) stage: EstateWorkflowStage,
    pub(crate) status: EstateCaseStatus,
    pub(crate) branch: BranchId,
    pub(crate) account: EstateAccountView,
    pub(crate) death_notice: EstateDeathNoticeView,
    pub(crate) deceased: BankPrincipalId,
    pub(crate) executors: Vec<BankPrincipalId>,
    pub(crate) beneficiaries: Vec<BankPrincipalId>,
    pub(crate) assignments: Vec<EstateAssignmentView>,
    pub(crate) legal_authorities: Vec<EstateLegalAuthorityView>,
    pub(crate) reviews: Vec<EstateReviewView>,
}

impl EstateCaseOverview {
    pub(crate) fn from_projection(projection: EstateCaseOverviewProjection) -> Self {
        Self {
            id: projection.id,
            stage: projection.stage,
            status: projection.status,
            branch: projection.branch,
            account: projection.account,
            death_notice: projection.death_notice,
            deceased: projection.deceased,
            executors: projection.executors,
            beneficiaries: projection.beneficiaries,
            assignments: projection.assignments,
            legal_authorities: projection.legal_authorities,
            reviews: projection.reviews,
        }
    }

    pub const fn id(&self) -> EstateCaseId {
        self.id
    }

    pub const fn stage(&self) -> EstateWorkflowStage {
        self.stage
    }

    pub const fn status(&self) -> EstateCaseStatus {
        self.status
    }

    pub const fn branch(&self) -> BranchId {
        self.branch
    }

    pub const fn account(&self) -> &EstateAccountView {
        &self.account
    }

    pub const fn death_notice(&self) -> EstateDeathNoticeView {
        self.death_notice
    }

    pub const fn deceased(&self) -> BankPrincipalId {
        self.deceased
    }

    pub fn executors(&self) -> &[BankPrincipalId] {
        &self.executors
    }

    pub fn beneficiaries(&self) -> &[BankPrincipalId] {
        &self.beneficiaries
    }

    pub fn assignments(&self) -> &[EstateAssignmentView] {
        &self.assignments
    }

    pub fn legal_authorities(&self) -> &[EstateLegalAuthorityView] {
        &self.legal_authorities
    }

    pub fn reviews(&self) -> &[EstateReviewView] {
        &self.reviews
    }
}
