use crate::model::{AccountId, AccountName, BankPrincipalId, BusinessId, InstitutionId};
use crate::schema::{AccountKind, AccountStatus};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BankAccount {
    id: AccountId,
    institution: InstitutionId,
    kind: AccountKind,
    status: AccountStatus,
    display_name: AccountName,
    personal_owner: Option<BankPrincipalId>,
    business_owner: Option<BusinessId>,
}

pub struct BankAccountProjection {
    pub id: AccountId,
    pub institution: InstitutionId,
    pub kind: AccountKind,
    pub status: AccountStatus,
    pub display_name: AccountName,
    pub personal_owner: Option<BankPrincipalId>,
    pub business_owner: Option<BusinessId>,
}

impl BankAccount {
    pub fn from_projection(projection: BankAccountProjection) -> Option<Self> {
        let BankAccountProjection {
            id,
            institution,
            kind,
            status,
            display_name,
            personal_owner,
            business_owner,
        } = projection;
        let ownership_matches = matches!(
            (kind, personal_owner, business_owner),
            (AccountKind::Personal, Some(_), None)
                | (AccountKind::Business, None, Some(_))
                | (
                    AccountKind::InstitutionCash | AccountKind::InstitutionSettlement,
                    None,
                    None
                )
        );
        ownership_matches.then_some(Self {
            id,
            institution,
            kind,
            status,
            display_name,
            personal_owner,
            business_owner,
        })
    }

    pub(crate) fn personal(
        id: AccountId,
        institution: InstitutionId,
        owner: BankPrincipalId,
        display_name: AccountName,
    ) -> Self {
        Self::personal_with_status(id, institution, owner, display_name, AccountStatus::Open)
    }

    pub(crate) fn personal_with_status(
        id: AccountId,
        institution: InstitutionId,
        owner: BankPrincipalId,
        display_name: AccountName,
        status: AccountStatus,
    ) -> Self {
        Self {
            id,
            institution,
            kind: AccountKind::Personal,
            status,
            display_name,
            personal_owner: Some(owner),
            business_owner: None,
        }
    }

    pub(crate) fn business(
        id: AccountId,
        institution: InstitutionId,
        business: BusinessId,
        display_name: AccountName,
    ) -> Self {
        Self::business_with_status(id, institution, business, display_name, AccountStatus::Open)
    }

    pub(crate) fn business_with_status(
        id: AccountId,
        institution: InstitutionId,
        business: BusinessId,
        display_name: AccountName,
        status: AccountStatus,
    ) -> Self {
        Self {
            id,
            institution,
            kind: AccountKind::Business,
            status,
            display_name,
            personal_owner: None,
            business_owner: Some(business),
        }
    }

    pub(crate) fn institution_cash(id: AccountId, institution: InstitutionId) -> Self {
        Self {
            id,
            institution,
            kind: AccountKind::InstitutionCash,
            status: AccountStatus::Open,
            display_name: AccountName::new("institution cash")
                .expect("the built-in institution cash name is valid"),
            personal_owner: None,
            business_owner: None,
        }
    }

    pub const fn id(&self) -> AccountId {
        self.id
    }

    pub const fn institution(&self) -> InstitutionId {
        self.institution
    }

    pub const fn kind(&self) -> AccountKind {
        self.kind
    }

    pub const fn status(&self) -> AccountStatus {
        self.status
    }

    pub fn display_name(&self) -> &AccountName {
        &self.display_name
    }

    pub const fn personal_owner(&self) -> Option<BankPrincipalId> {
        self.personal_owner
    }

    pub const fn business_owner(&self) -> Option<BusinessId> {
        self.business_owner
    }
}
