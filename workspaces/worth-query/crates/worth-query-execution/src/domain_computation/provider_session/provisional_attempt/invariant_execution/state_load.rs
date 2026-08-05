use std::sync::Arc;

use super::{WorthQueryInvariantExecutionDenialKind, WorthQueryInvariantExecutionFailure};

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct WorthQueryInvariantStateLocator {
    family: Arc<str>,
    identity: Arc<str>,
}

impl WorthQueryInvariantStateLocator {
    pub fn new(
        family: impl Into<Arc<str>>,
        identity: impl Into<Arc<str>>,
    ) -> Result<Self, WorthQueryInvariantExecutionFailure> {
        let family = canonical(family)?;
        let identity = canonical(identity)?;
        Ok(Self { family, identity })
    }

    pub fn family(&self) -> &str {
        &self.family
    }

    pub fn identity(&self) -> &str {
        &self.identity
    }
}

pub struct WorthQueryAdmittedInvariantStateLoadPlan {
    identity: Arc<str>,
    locators: Arc<[WorthQueryInvariantStateLocator]>,
}

impl WorthQueryAdmittedInvariantStateLoadPlan {
    pub(crate) fn admit(
        identity: impl Into<Arc<str>>,
        locators: impl IntoIterator<Item = WorthQueryInvariantStateLocator>,
        allowed_families: &[String],
    ) -> Result<Self, WorthQueryInvariantExecutionFailure> {
        let mut locators = locators.into_iter().collect::<Vec<_>>();
        locators.sort();
        locators.dedup();
        if locators.is_empty() {
            return Err(failure(
                WorthQueryInvariantExecutionDenialKind::EmptyStateLoadPlan,
            ));
        }
        if locators.iter().any(|locator| {
            !allowed_families
                .iter()
                .any(|family| family == locator.family())
        }) {
            return Err(failure(
                WorthQueryInvariantExecutionDenialKind::UndeclaredStateLoadFamily,
            ));
        }
        Ok(Self {
            identity: identity.into(),
            locators: locators.into(),
        })
    }

    pub fn identity(&self) -> &str {
        &self.identity
    }

    pub fn locators(&self) -> &[WorthQueryInvariantStateLocator] {
        &self.locators
    }
}

fn canonical(value: impl Into<Arc<str>>) -> Result<Arc<str>, WorthQueryInvariantExecutionFailure> {
    let value = value.into();
    if value.trim().is_empty() || value.trim() != value.as_ref() {
        Err(failure(
            WorthQueryInvariantExecutionDenialKind::ProviderRejected,
        ))
    } else {
        Ok(value)
    }
}

fn failure(kind: WorthQueryInvariantExecutionDenialKind) -> WorthQueryInvariantExecutionFailure {
    WorthQueryInvariantExecutionFailure::new(kind, "invariant state-load admission denied")
}
