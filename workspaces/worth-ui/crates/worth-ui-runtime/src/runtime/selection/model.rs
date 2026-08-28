pub(crate) const UI_SELECTION_CATALOG_LIMIT: usize = 4_096;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum UiSelectionPolicy {
    Single,
    Multiple,
    MultipleWithRange,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum UiSelectionCatalogPosture {
    Complete,
    Partial,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct UiSelectionRegistration {
    owner: super::UiSelectionOwnerIdentity,
    incarnation: super::UiSelectionOwnerIncarnation,
    policy: UiSelectionPolicy,
    catalog: std::sync::Arc<[super::UiSelectionStableKey]>,
    catalog_positions:
        std::sync::Arc<std::collections::BTreeMap<super::UiSelectionStableKey, usize>>,
    catalog_posture: UiSelectionCatalogPosture,
    catalog_revision: u64,
}

/// Application-side stable-item mapping admitted for one declared interaction.
/// Query may have supplied data used by the application, but no Query identity
/// crosses this boundary or becomes Selection authority.
pub(crate) struct UiDeclaredSelectionBinding {
    action: crate::runtime::session::service_proposal::UiDeclaredFocusSelectionAction,
    registration: Option<UiSelectionRegistration>,
}

impl UiSelectionRegistration {
    pub(crate) fn new(
        owner: super::UiSelectionOwnerIdentity,
        incarnation: super::UiSelectionOwnerIncarnation,
        policy: UiSelectionPolicy,
        catalog: Vec<super::UiSelectionStableKey>,
        catalog_posture: UiSelectionCatalogPosture,
    ) -> Result<Self, super::UiSelectionRequestDenial> {
        let catalog_positions = super::state::validate_catalog(owner, &catalog)?;
        Ok(Self {
            owner,
            incarnation,
            policy,
            catalog: catalog.into(),
            catalog_positions: std::sync::Arc::new(catalog_positions),
            catalog_posture,
            catalog_revision: 0,
        })
    }

    pub(crate) fn with_catalog_revision(mut self, catalog_revision: u64) -> Self {
        self.catalog_revision = catalog_revision;
        self
    }

    pub(super) const fn owner(&self) -> super::UiSelectionOwnerIdentity {
        self.owner
    }
    pub(super) const fn incarnation(&self) -> super::UiSelectionOwnerIncarnation {
        self.incarnation
    }
    pub(super) const fn policy(&self) -> UiSelectionPolicy {
        self.policy
    }
    pub(super) fn catalog(&self) -> &[super::UiSelectionStableKey] {
        &self.catalog
    }
    pub(super) fn catalog_positions(
        &self,
    ) -> &std::sync::Arc<std::collections::BTreeMap<super::UiSelectionStableKey, usize>> {
        &self.catalog_positions
    }
    pub(super) const fn catalog_posture(&self) -> UiSelectionCatalogPosture {
        self.catalog_posture
    }
    pub(super) const fn catalog_revision(&self) -> u64 {
        self.catalog_revision
    }
}

impl UiDeclaredSelectionBinding {
    pub(crate) const fn new(
        action: crate::runtime::session::service_proposal::UiDeclaredFocusSelectionAction,
        registration: UiSelectionRegistration,
    ) -> Self {
        Self {
            action,
            registration: Some(registration),
        }
    }

    pub(crate) const fn current(
        action: crate::runtime::session::service_proposal::UiDeclaredFocusSelectionAction,
    ) -> Self {
        Self {
            action,
            registration: None,
        }
    }

    pub(crate) const fn action(
        &self,
    ) -> crate::runtime::session::service_proposal::UiDeclaredFocusSelectionAction {
        self.action
    }

    pub(crate) fn into_parts(
        self,
    ) -> (
        crate::runtime::session::service_proposal::UiDeclaredFocusSelectionAction,
        Option<UiSelectionRegistration>,
    ) {
        (self.action, self.registration)
    }
}
