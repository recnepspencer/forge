use std::collections::BTreeSet;

use worth_ui::facade::{WorthUiProjectionFamily, WorthUiProjectionRebindStatus};

use super::types::ValidationMixedReloadStormStep;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ValidationMixedReloadStormProjectionRoster {
    rows: Vec<ValidationMixedReloadStormProjectionRow>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ValidationMixedReloadStormProjectionRow {
    surface: ValidationMixedReloadStormProjectionSurface,
    projection_identity: String,
    projection_family: WorthUiProjectionFamily,
    status: WorthUiProjectionRebindStatus,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ValidationMixedReloadStormProjectionSurface {
    Header,
    PageHost,
}

impl ValidationMixedReloadStormProjectionRoster {
    pub(super) fn from_steps(steps: &[ValidationMixedReloadStormStep]) -> Self {
        let mut rows = Vec::new();
        for step in steps {
            if let Some(rebind) = step.header_rebind() {
                for row in rebind.rows() {
                    rows.push(ValidationMixedReloadStormProjectionRow {
                        surface: ValidationMixedReloadStormProjectionSurface::Header,
                        projection_identity: row.projection_identity().to_owned(),
                        projection_family: row.projection_family(),
                        status: row.status(),
                    });
                }
            }
            if let Some(rebind) = step.page_host_rebind() {
                for row in rebind.rows() {
                    rows.push(ValidationMixedReloadStormProjectionRow {
                        surface: ValidationMixedReloadStormProjectionSurface::PageHost,
                        projection_identity: row.projection_identity().to_owned(),
                        projection_family: row.projection_family(),
                        status: row.status(),
                    });
                }
            }
        }
        rows.sort_by_cached_key(|row| {
            format!(
                "{:?}|{}|{:?}|{:?}",
                row.surface, row.projection_identity, row.projection_family, row.status
            )
        });
        rows.dedup_by(|left, right| left == right);
        Self { rows }
    }

    pub fn rows(&self) -> &[ValidationMixedReloadStormProjectionRow] {
        &self.rows
    }

    pub fn rebuilt_projection_ids(&self) -> BTreeSet<String> {
        self.identities_matching(|status| {
            matches!(
                status,
                WorthUiProjectionRebindStatus::EquivalentAfterActivation
                    | WorthUiProjectionRebindStatus::ReboundAfterActivation
            )
        })
    }

    pub fn preserved_projection_ids(&self) -> BTreeSet<String> {
        self.identities_matching(|status| {
            matches!(
                status,
                WorthUiProjectionRebindStatus::PreservedEquivalentReload
            )
        })
    }

    pub fn denied_projection_ids(&self) -> BTreeSet<String> {
        self.identities_matching(|status| {
            matches!(
                status,
                WorthUiProjectionRebindStatus::PreservedDeniedReload
                    | WorthUiProjectionRebindStatus::DeniedReloadNotActivated
            )
        })
    }

    fn identities_matching(
        &self,
        include: impl Fn(WorthUiProjectionRebindStatus) -> bool,
    ) -> BTreeSet<String> {
        self.rows
            .iter()
            .filter(|row| include(row.status))
            .map(|row| row.projection_identity.clone())
            .collect()
    }
}

impl ValidationMixedReloadStormProjectionRow {
    pub fn surface(&self) -> ValidationMixedReloadStormProjectionSurface {
        self.surface
    }

    pub fn projection_identity(&self) -> &str {
        &self.projection_identity
    }

    pub fn projection_family(&self) -> WorthUiProjectionFamily {
        self.projection_family
    }

    pub fn status(&self) -> WorthUiProjectionRebindStatus {
        self.status
    }
}
