//! Installing the Bank's outbound external-effect transport.
//!
//! The Bank owns the rail; Query owns the dispatch decision. This is the one
//! seam where the two meet: the Bank hands Query a port, once, for the life of
//! the runtime, and Query calls it after a declared effect has durably
//! committed. Nothing here decides what the effect means.

use std::sync::Arc;

use worth_query_host::facade::primary_graph::{
    WorthQueryExternalEffectTransport, WorthQueryExternalTransportInstallationDenial,
};

use crate::BankIdentityRuntime;

/// Why the Bank could not install an outbound transport.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BankExternalEffectTransportDenial {
    /// A transport is already installed on this runtime.
    AlreadyInstalled,
}

impl BankIdentityRuntime {
    /// Installs the rail this bank dispatches declared external effects over.
    ///
    /// Operations that declare no external effect never reach the transport,
    /// so installing one costs unrelated mutations nothing.
    pub fn install_external_effect_transport(
        &self,
        transport: Arc<dyn WorthQueryExternalEffectTransport>,
    ) -> Result<(), BankExternalEffectTransportDenial> {
        self.application_runtime()
            .install_external_effect_transport(transport)
            .map_err(|denial| match denial {
                WorthQueryExternalTransportInstallationDenial::AlreadyInstalled => {
                    BankExternalEffectTransportDenial::AlreadyInstalled
                }
            })
    }

    /// True when a rail is installed and declared effects can leave the bank.
    pub fn has_external_effect_transport(&self) -> bool {
        self.application_runtime().has_external_effect_transport()
    }
}

impl std::fmt::Display for BankExternalEffectTransportDenial {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AlreadyInstalled => {
                write!(
                    formatter,
                    "an external-effect transport is already installed"
                )
            }
        }
    }
}

impl std::error::Error for BankExternalEffectTransportDenial {}
