use std::collections::BTreeSet;

use super::catalog;
use super::classification::{PlannerOwnedRoutingDisposition, PlannerOwnedRoutingLifecycleRole};
use super::error::PlannerOwnedRoutingInventoryError;
use super::report::PlannerOwnedRoutingInventoryReport;
use super::row::PlannerOwnedRoutingInventoryRow;
use super::source_scan;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlannerOwnedRoutingInventoryCloseout {
    report: PlannerOwnedRoutingInventoryReport,
}

impl PlannerOwnedRoutingInventoryCloseout {
    pub fn report(&self) -> &PlannerOwnedRoutingInventoryReport {
        &self.report
    }
}

pub fn current_planner_owned_routing_inventory(
) -> Result<PlannerOwnedRoutingInventoryCloseout, PlannerOwnedRoutingInventoryError> {
    let rows = catalog::current_rows();
    validate_rows(&rows)?;
    Ok(PlannerOwnedRoutingInventoryCloseout {
        report: PlannerOwnedRoutingInventoryReport::new(rows),
    })
}

fn validate_rows(
    rows: &[PlannerOwnedRoutingInventoryRow],
) -> Result<(), PlannerOwnedRoutingInventoryError> {
    let mut seen = BTreeSet::new();
    for row in rows {
        if !seen.insert(row.surface_identity()) {
            return Err(PlannerOwnedRoutingInventoryError::DuplicateSurface(
                row.surface_identity(),
            ));
        }
        validate_row(row)?;
    }

    for required_surface in catalog::required_surfaces() {
        if !seen.contains(required_surface) {
            return Err(PlannerOwnedRoutingInventoryError::MissingRequiredSurface(
                *required_surface,
            ));
        }
    }

    for role in PlannerOwnedRoutingLifecycleRole::ALL {
        if !rows.iter().any(|row| row.lifecycle_role() == role) {
            return Err(PlannerOwnedRoutingInventoryError::MissingLifecycleRole(
                role.as_str(),
            ));
        }
    }

    source_scan::ensure_inventory_matches_live_sources(rows)
}

fn validate_row(
    row: &PlannerOwnedRoutingInventoryRow,
) -> Result<(), PlannerOwnedRoutingInventoryError> {
    if row.blocker().is_empty() || row.removal_trigger().is_empty() {
        return Err(PlannerOwnedRoutingInventoryError::EmptyExitCondition(
            row.surface_name(),
        ));
    }

    if row.current_authority_sources().is_empty() {
        return Err(
            PlannerOwnedRoutingInventoryError::MissingCurrentAuthoritySource(row.surface_name()),
        );
    }
    for token in row.current_authority_sources() {
        if token.is_empty()
            || token.contains(' ')
            || !token
                .chars()
                .all(|ch| ch.is_ascii_alphanumeric() || matches!(ch, '_' | ':' | '(' | ')' | '.'))
        {
            return Err(
                PlannerOwnedRoutingInventoryError::InvalidCurrentAuthoritySource {
                    surface: row.surface_name(),
                    token,
                },
            );
        }
    }

    if row.displaced_lane().path().is_empty() {
        return Err(PlannerOwnedRoutingInventoryError::MissingDisplacedLanePath(
            row.surface_name(),
        ));
    }

    if row.replacement_lane().path().is_empty() {
        return Err(
            PlannerOwnedRoutingInventoryError::MissingReplacementLanePath(row.surface_name()),
        );
    }

    match row.disposition() {
        PlannerOwnedRoutingDisposition::QueryGap => {
            if row.query_gap().is_none() {
                return Err(PlannerOwnedRoutingInventoryError::MissingQueryGapKind(
                    row.surface_name(),
                ));
            }
        }
        PlannerOwnedRoutingDisposition::Cap => {}
        _ => {
            if row.query_gap().is_some() {
                return Err(PlannerOwnedRoutingInventoryError::UnexpectedQueryGapKind(
                    row.surface_name(),
                ));
            }
        }
    }

    if row.ordinary_path() && matches!(row.disposition(), PlannerOwnedRoutingDisposition::Cap) {
        return Err(
            PlannerOwnedRoutingInventoryError::InvalidOrdinaryDisposition {
                surface: row.surface_name(),
                disposition: row.disposition(),
            },
        );
    }

    if matches!(row.disposition(), PlannerOwnedRoutingDisposition::Cap)
        && row.displaced_lane().path() == row.replacement_lane().path()
    {
        return Err(PlannerOwnedRoutingInventoryError::SelfReplacingCapPath {
            surface: row.surface_name(),
        });
    }

    Ok(())
}
