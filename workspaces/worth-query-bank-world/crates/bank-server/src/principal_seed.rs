use bank_domain::model::BankPrincipalId;
use bank_domain::schema::{BankSchema, Principal};
use worth_query_host::facade::declaration::authentication::{
    WorthQueryExternalPrincipalIdentity, WorthQueryPrincipalMappingStatus,
};
use worth_query_host::facade::primary_graph::{
    WorthQueryApplicationPrincipalKey, WorthQueryApplicationPrincipalKeyDenial,
};

pub struct BankPrincipalSeed {
    principal_id: BankPrincipalId,
    external_identity: WorthQueryExternalPrincipalIdentity,
    status: WorthQueryPrincipalMappingStatus,
}

impl BankPrincipalSeed {
    pub const fn new(
        principal_id: BankPrincipalId,
        external_identity: WorthQueryExternalPrincipalIdentity,
        status: WorthQueryPrincipalMappingStatus,
    ) -> Self {
        Self {
            principal_id,
            external_identity,
            status,
        }
    }

    pub fn enabled(
        principal_id: BankPrincipalId,
        external_identity: WorthQueryExternalPrincipalIdentity,
    ) -> Self {
        Self::new(
            principal_id,
            external_identity,
            WorthQueryPrincipalMappingStatus::Enabled,
        )
    }

    pub(crate) fn prepare(
        self,
    ) -> Result<PreparedBankPrincipalSeed, WorthQueryApplicationPrincipalKeyDenial> {
        let key = WorthQueryApplicationPrincipalKey::<BankSchema, Principal>::new(format!(
            "bank-principal:{}",
            self.principal_id.get()
        ))?;
        Ok(PreparedBankPrincipalSeed {
            principal_id: self.principal_id,
            key,
            external_identity: self.external_identity,
            status: self.status,
        })
    }
}

pub(crate) struct PreparedBankPrincipalSeed {
    pub(crate) principal_id: BankPrincipalId,
    pub(crate) key: WorthQueryApplicationPrincipalKey<BankSchema, Principal>,
    pub(crate) external_identity: WorthQueryExternalPrincipalIdentity,
    pub(crate) status: WorthQueryPrincipalMappingStatus,
}
