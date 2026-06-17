use crate::workload_platform::planar_boolean_edge_splitting::canonical_parameter::canonical_parameter_bits;
use crate::workload_platform::planar_boolean_edge_splitting::duplicate_split_normalization::PlanarBooleanNormalizedSplitCut;
use crate::workload_platform::planar_boolean_edge_splitting::micro_interval_normalization::PlanarBooleanNormalizedIntervalSubdivisionRow;

use super::denial::{
    PlanarBooleanSplitVertexIdentityDenial, PlanarBooleanSplitVertexIdentityDenialKind,
};

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(super) enum SplitVertexInputKind {
    PointCut,
    IntervalStart,
    IntervalEnd,
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(super) struct SplitVertexCoalescenceKey {
    pub(super) source_edge_identity: String,
    pub(super) carrier_identity: String,
    pub(super) parameter_bits: u64,
    pub(super) local_frame_identity: String,
    pub(super) precision_basis_identity: String,
}

#[derive(Clone, Debug)]
pub(super) struct SplitVertexInputRow {
    pub(super) input_identity: String,
    pub(super) input_kind: SplitVertexInputKind,
    pub(super) source_edge_identity: String,
    pub(super) carrier_identity: String,
    pub(super) parameter_bits: u64,
    pub(super) local_frame_identity: String,
    pub(super) precision_basis_identity: String,
    pub(super) point_cut_identity: Option<String>,
    pub(super) parameter_fact_identities: Vec<String>,
    pub(super) interval_subdivision_identity: Option<String>,
    pub(super) normalized_interval_identity: Option<String>,
    pub(super) coordinate_fact_identity: Option<String>,
    pub(super) provenance_identities: Vec<String>,
    pub(super) event_group_identities: Vec<String>,
}

impl SplitVertexInputRow {
    pub(super) fn from_point_cut(
        cut: &PlanarBooleanNormalizedSplitCut,
    ) -> Result<Self, PlanarBooleanSplitVertexIdentityDenial> {
        reject_non_finite_parameter(cut.cut_identity(), cut.parameter())?;
        let coordinate_fact_identity = cut
            .exact_projected_endpoint_fact_identity()
            .or_else(|| {
                cut.shared_endpoint_projection_fact_digests()
                    .first()
                    .map(String::as_str)
            })
            .map(str::to_string);
        let mut provenance_identities = canonical_strings(
            cut.provenance_entry_identities()
                .iter()
                .chain(cut.event_identities().iter())
                .cloned()
                .collect(),
        );
        if let Some(endpoint_identity) = cut.exact_endpoint_source_identity() {
            provenance_identities.push(endpoint_identity.to_string());
        }
        provenance_identities.extend(cut.shared_endpoint_source_identities().iter().cloned());
        provenance_identities = canonical_strings(provenance_identities);
        reject_missing_point_provenance(
            cut.cut_identity(),
            &provenance_identities,
            coordinate_fact_identity.as_deref(),
        )?;
        Ok(Self {
            input_identity: cut.cut_identity().to_string(),
            input_kind: SplitVertexInputKind::PointCut,
            source_edge_identity: cut.source_edge_identity().to_string(),
            carrier_identity: cut.carrier_identity().to_string(),
            parameter_bits: cut.parameter_bits(),
            local_frame_identity: cut.local_frame_identity().to_string(),
            precision_basis_identity: cut.precision_basis_identity().to_string(),
            point_cut_identity: Some(cut.cut_identity().to_string()),
            parameter_fact_identities: canonical_strings(cut.parameter_fact_identities().to_vec()),
            interval_subdivision_identity: None,
            normalized_interval_identity: None,
            coordinate_fact_identity,
            provenance_identities,
            event_group_identities: canonical_strings(cut.event_group_identities().to_vec()),
        })
    }

    pub(super) fn from_interval_endpoint(
        subdivision: &PlanarBooleanNormalizedIntervalSubdivisionRow,
        input_kind: SplitVertexInputKind,
    ) -> Result<Self, PlanarBooleanSplitVertexIdentityDenial> {
        let range = subdivision.admitted_parameter_range();
        let parameter = match input_kind {
            SplitVertexInputKind::IntervalStart => range[0],
            SplitVertexInputKind::IntervalEnd => range[1],
            SplitVertexInputKind::PointCut => unreachable!("point cuts use from_point_cut"),
        };
        reject_non_finite_parameter(subdivision.subdivision_identity(), parameter)?;
        let endpoint_name = match input_kind {
            SplitVertexInputKind::IntervalStart => "start",
            SplitVertexInputKind::IntervalEnd => "end",
            SplitVertexInputKind::PointCut => unreachable!("point cuts use from_point_cut"),
        };
        let provenance_identities =
            canonical_strings(subdivision.provenance_entry_identities().to_vec());
        reject_missing_provenance(subdivision.subdivision_identity(), &provenance_identities)?;
        Ok(Self {
            input_identity: format!("{}:{endpoint_name}", subdivision.subdivision_identity()),
            input_kind,
            source_edge_identity: subdivision.source_edge_identity().to_string(),
            carrier_identity: subdivision.carrier_identity().to_string(),
            parameter_bits: canonical_parameter_bits(parameter),
            local_frame_identity: subdivision.local_frame_identity().to_string(),
            precision_basis_identity: subdivision.precision_basis_identity().to_string(),
            point_cut_identity: None,
            parameter_fact_identities: Vec::new(),
            interval_subdivision_identity: Some(subdivision.subdivision_identity().to_string()),
            normalized_interval_identity: Some(
                subdivision.normalized_interval_identity().to_string(),
            ),
            coordinate_fact_identity: None,
            provenance_identities,
            event_group_identities: canonical_strings(
                subdivision.event_group_identities().to_vec(),
            ),
        })
    }

    pub(super) fn coalescence_key(&self) -> SplitVertexCoalescenceKey {
        SplitVertexCoalescenceKey {
            source_edge_identity: self.source_edge_identity.clone(),
            carrier_identity: self.carrier_identity.clone(),
            parameter_bits: self.parameter_bits,
            local_frame_identity: self.local_frame_identity.clone(),
            precision_basis_identity: self.precision_basis_identity.clone(),
        }
    }
}

pub(super) fn canonical_strings(mut values: Vec<String>) -> Vec<String> {
    values.sort();
    values.dedup();
    values
}

fn reject_non_finite_parameter(
    evidence_identity: &str,
    parameter: f64,
) -> Result<(), PlanarBooleanSplitVertexIdentityDenial> {
    if parameter.is_finite() {
        return Ok(());
    }
    Err(PlanarBooleanSplitVertexIdentityDenial::new(
        PlanarBooleanSplitVertexIdentityDenialKind::NonFiniteSplitVertexParameter,
        evidence_identity,
        "split vertex identity requires a finite normalized source-edge parameter",
    ))
}

fn reject_missing_provenance(
    evidence_identity: &str,
    provenance_identities: &[String],
) -> Result<(), PlanarBooleanSplitVertexIdentityDenial> {
    if !provenance_identities.is_empty() {
        return Ok(());
    }
    Err(PlanarBooleanSplitVertexIdentityDenial::new(
        PlanarBooleanSplitVertexIdentityDenialKind::MissingCertifiedSplitVertexProvenance,
        evidence_identity,
        "split vertex identity requires certified event or interval provenance",
    ))
}

fn reject_missing_point_provenance(
    evidence_identity: &str,
    provenance_identities: &[String],
    coordinate_fact_identity: Option<&str>,
) -> Result<(), PlanarBooleanSplitVertexIdentityDenial> {
    if !provenance_identities.is_empty() {
        return Ok(());
    }
    if coordinate_fact_identity.is_some() {
        return Err(PlanarBooleanSplitVertexIdentityDenial::new(
            PlanarBooleanSplitVertexIdentityDenialKind::CoordinateOnlySplitVertexIdentity,
            evidence_identity,
            "split vertex identity cannot be minted from coordinate evidence alone",
        ));
    }
    reject_missing_provenance(evidence_identity, provenance_identities)
}
