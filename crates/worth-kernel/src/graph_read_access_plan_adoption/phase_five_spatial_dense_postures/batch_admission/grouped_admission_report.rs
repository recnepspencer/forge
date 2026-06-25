use std::collections::BTreeMap;

use super::super::query_posture_projection::WorthGraphReadAccessSpatialDensePostureProjection;
use super::super::stable_digest;
use super::grouped_admission_row::WorthGraphReadAccessGroupedAdmissionRow;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthGraphReadAccessGroupedAdmissionReport {
    rows: Vec<WorthGraphReadAccessGroupedAdmissionRow>,
    grouped_family_count: usize,
    grouped_row_count: usize,
    scalarized_caller_loop_count: usize,
    report_digest: String,
}

pub(crate) fn build_grouped_admission_report(
    projections: &[WorthGraphReadAccessSpatialDensePostureProjection],
) -> WorthGraphReadAccessGroupedAdmissionReport {
    let mut groups: BTreeMap<String, Vec<&WorthGraphReadAccessSpatialDensePostureProjection>> =
        BTreeMap::new();
    for projection in projections {
        groups
            .entry(projection.query_family_digest_seed().to_string())
            .or_default()
            .push(projection);
    }
    let rows = groups
        .into_iter()
        .map(|(query_family, projections)| {
            WorthGraphReadAccessGroupedAdmissionRow::from_projection_group(
                query_family,
                &projections,
            )
        })
        .collect::<Vec<_>>();
    WorthGraphReadAccessGroupedAdmissionReport::from_rows(rows)
}

impl WorthGraphReadAccessGroupedAdmissionReport {
    fn from_rows(rows: Vec<WorthGraphReadAccessGroupedAdmissionRow>) -> Self {
        let grouped_family_count = rows
            .iter()
            .filter(|row| row.grouped_admission_preserved())
            .count();
        let grouped_row_count = rows
            .iter()
            .filter(|row| row.grouped_admission_preserved())
            .map(WorthGraphReadAccessGroupedAdmissionRow::row_count)
            .sum();
        let scalarized_caller_loop_count = rows
            .iter()
            .map(WorthGraphReadAccessGroupedAdmissionRow::scalarized_caller_loop_count)
            .sum();
        let report_digest = stable_digest(
            &std::iter::once("worth_graph_read_access_grouped_admission_report_v1".to_string())
                .chain(rows.iter().map(|row| format!("row:{}", row.row_digest())))
                .chain([
                    format!("grouped_family_count:{grouped_family_count}"),
                    format!("grouped_row_count:{grouped_row_count}"),
                    format!("scalarized:{scalarized_caller_loop_count}"),
                ])
                .collect::<Vec<_>>(),
        );
        Self {
            rows,
            grouped_family_count,
            grouped_row_count,
            scalarized_caller_loop_count,
            report_digest,
        }
    }

    pub fn rows(&self) -> &[WorthGraphReadAccessGroupedAdmissionRow] {
        &self.rows
    }

    pub const fn grouped_family_count(&self) -> usize {
        self.grouped_family_count
    }

    pub const fn grouped_row_count(&self) -> usize {
        self.grouped_row_count
    }

    pub const fn scalarized_caller_loop_count(&self) -> usize {
        self.scalarized_caller_loop_count
    }

    pub fn report_digest(&self) -> &str {
        &self.report_digest
    }
}
