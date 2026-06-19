use forge_query::facade::consumer_kit::ForgeQueryGraphObligationSupportPin;
use forge_query::facade::{
    ForgeQueryGraphObligationKind, ForgeQueryGraphObligationSupportLane,
    ForgeQueryGraphObligationSupportMatrix, ForgeQueryGraphObligationSupportMatrixRow,
    ForgeQueryGraphObligationSupportStatus,
};

use super::topology_operator_graph_obligation_catalog;

pub fn topology_operator_graph_obligation_support_matrix() -> ForgeQueryGraphObligationSupportMatrix
{
    let mut rows = ForgeQueryGraphObligationKind::ALL
        .into_iter()
        .flat_map(|kind| {
            [
                ForgeQueryGraphObligationSupportMatrixRow::new(
                    kind,
                    ForgeQueryGraphObligationSupportLane::AssemblyIndexSelection,
                    ForgeQueryGraphObligationSupportStatus::Supported,
                ),
                ForgeQueryGraphObligationSupportMatrixRow::new(
                    kind,
                    ForgeQueryGraphObligationSupportLane::GraphComposition,
                    topology_operator_lane_status(
                        kind,
                        ForgeQueryGraphObligationSupportLane::GraphComposition,
                    ),
                ),
                ForgeQueryGraphObligationSupportMatrixRow::new(
                    kind,
                    ForgeQueryGraphObligationSupportLane::ContributionOrchestration,
                    topology_operator_lane_status(
                        kind,
                        ForgeQueryGraphObligationSupportLane::ContributionOrchestration,
                    ),
                ),
                ForgeQueryGraphObligationSupportMatrixRow::new(
                    kind,
                    ForgeQueryGraphObligationSupportLane::AuthoritativeCommandBatch,
                    topology_operator_lane_status(
                        kind,
                        ForgeQueryGraphObligationSupportLane::AuthoritativeCommandBatch,
                    ),
                ),
                ForgeQueryGraphObligationSupportMatrixRow::new(
                    kind,
                    ForgeQueryGraphObligationSupportLane::ScalarMutation,
                    topology_operator_lane_status(
                        kind,
                        ForgeQueryGraphObligationSupportLane::ScalarMutation,
                    ),
                ),
            ]
        })
        .collect::<Vec<_>>();
    rows.extend(ForgeQueryGraphObligationKind::ALL.into_iter().map(|kind| {
        ForgeQueryGraphObligationSupportMatrixRow::new(
            kind,
            ForgeQueryGraphObligationSupportLane::WorthTopoOperatorCatalog,
            topology_operator_lane_status(
                kind,
                ForgeQueryGraphObligationSupportLane::WorthTopoOperatorCatalog,
            ),
        )
    }));
    ForgeQueryGraphObligationSupportMatrix::new(rows)
}

pub fn topology_operator_graph_obligation_support_pin() -> ForgeQueryGraphObligationSupportPin {
    ForgeQueryGraphObligationSupportPin::new(
        topology_operator_graph_obligation_catalog()
            .covered_rows()
            .filter_map(|row| {
                Some((
                    row.registration_kind()?,
                    row.support_lane()?,
                    row.support_status()?,
                ))
            }),
    )
}

fn topology_operator_lane_status(
    kind: ForgeQueryGraphObligationKind,
    lane: ForgeQueryGraphObligationSupportLane,
) -> ForgeQueryGraphObligationSupportStatus {
    match (kind, lane) {
        (
            ForgeQueryGraphObligationKind::AdvisoryObligation,
            ForgeQueryGraphObligationSupportLane::GraphComposition
            | ForgeQueryGraphObligationSupportLane::ContributionOrchestration
            | ForgeQueryGraphObligationSupportLane::WorthTopoOperatorCatalog,
        ) => ForgeQueryGraphObligationSupportStatus::DiagnosticOnly,
        (
            ForgeQueryGraphObligationKind::AdvisoryObligation,
            ForgeQueryGraphObligationSupportLane::AuthoritativeCommandBatch
            | ForgeQueryGraphObligationSupportLane::ScalarMutation,
        ) => ForgeQueryGraphObligationSupportStatus::DeferredToBackstop,
        _ => ForgeQueryGraphObligationSupportStatus::NotApplicable,
    }
}
