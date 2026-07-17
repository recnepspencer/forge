use worth_query::facade::domain::{
    WorthQueryDomainRebindDenial, WorthQueryDomainRebindDenialKind,
    WorthQueryDomainRebindNextAction,
};
use worth_query::facade::runtime::WorthQueryWorkspace;

use super::WorthUiInstalledQueryDomain;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiQueryDomainRebindDenialKind {
    DomainNotInstalled,
    PackageMeaningChanged,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiQueryDomainRebindDenial {
    kind: WorthUiQueryDomainRebindDenialKind,
    next_action: &'static str,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiQueryDomainRebindReceipt {
    prior: WorthUiInstalledQueryDomain,
    current: WorthUiInstalledQueryDomain,
}

impl WorthUiInstalledQueryDomain {
    pub fn rebind_to(
        &self,
        workspace: &WorthQueryWorkspace,
    ) -> Result<WorthUiQueryDomainRebindReceipt, WorthUiQueryDomainRebindDenial> {
        let rebound = workspace
            .rebind_domain(self.handle().rebind_request())
            .map_err(WorthUiQueryDomainRebindDenial::from_query)?;
        Ok(WorthUiQueryDomainRebindReceipt {
            prior: self.clone(),
            current: WorthUiInstalledQueryDomain::from_handle(rebound.into_handle()),
        })
    }
}

impl WorthUiQueryDomainRebindDenial {
    fn from_query(denial: WorthQueryDomainRebindDenial) -> Self {
        let kind = match denial.kind() {
            WorthQueryDomainRebindDenialKind::DomainNotInstalled => {
                WorthUiQueryDomainRebindDenialKind::DomainNotInstalled
            }
            WorthQueryDomainRebindDenialKind::PackageMeaningChanged => {
                WorthUiQueryDomainRebindDenialKind::PackageMeaningChanged
            }
        };
        let next_action = match denial.next_action() {
            WorthQueryDomainRebindNextAction::InstallDomainPackage => {
                "install the Worth UI domain package in the target Query workspace"
            }
            WorthQueryDomainRebindNextAction::ReconcilePackageMeaning => {
                "reconcile Worth UI domain package meaning before rebind"
            }
        };
        Self { kind, next_action }
    }

    pub fn kind(&self) -> WorthUiQueryDomainRebindDenialKind {
        self.kind
    }

    pub fn next_action(&self) -> &'static str {
        self.next_action
    }
}

impl WorthUiQueryDomainRebindReceipt {
    pub fn prior(&self) -> &WorthUiInstalledQueryDomain {
        &self.prior
    }

    pub fn current(&self) -> &WorthUiInstalledQueryDomain {
        &self.current
    }

    pub fn into_current(self) -> WorthUiInstalledQueryDomain {
        self.current
    }
}
