use crate::bindings::authority::SpatialBindingCompleteness;

use super::neighborhood::NeighborhoodBindingFamily;
use worth_geom::facade::ParameterSpacePoint;

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct BindingSnapshot {
    pub(crate) family: NeighborhoodBindingFamily,
    pub(crate) birth_class: String,
    pub(crate) geometry_digest: String,
    pub(crate) completeness: SpatialBindingCompleteness,
    pub(crate) anchor: Option<AnchorSnapshot>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct AnchorSnapshot {
    carrier_kind: String,
    carrier_identity: String,
    parameter_semantics_signature: String,
    parameter_bits: [u64; 2],
    direction_role: Option<String>,
}

impl AnchorSnapshot {
    pub(crate) fn point(
        carrier_kind: &str,
        carrier_identity: &str,
        parameter_semantics_signature: &str,
        parameter: ParameterSpacePoint,
    ) -> Self {
        Self {
            carrier_kind: carrier_kind.to_string(),
            carrier_identity: carrier_identity.to_string(),
            parameter_semantics_signature: parameter_semantics_signature.to_string(),
            parameter_bits: [parameter.u().to_bits(), parameter.v().to_bits()],
            direction_role: None,
        }
    }

    pub(crate) fn direction(
        carrier_kind: &str,
        carrier_identity: &str,
        parameter_semantics_signature: &str,
        parameter: ParameterSpacePoint,
        role: &'static str,
    ) -> Self {
        Self {
            carrier_kind: carrier_kind.to_string(),
            carrier_identity: carrier_identity.to_string(),
            parameter_semantics_signature: parameter_semantics_signature.to_string(),
            parameter_bits: [parameter.u().to_bits(), parameter.v().to_bits()],
            direction_role: Some(role.to_string()),
        }
    }

    pub(crate) fn same_semantics(&self, other: &Self) -> bool {
        self.carrier_kind == other.carrier_kind
            && self.carrier_identity == other.carrier_identity
            && self.parameter_semantics_signature == other.parameter_semantics_signature
            && self.parameter_bits == other.parameter_bits
            && self.direction_role == other.direction_role
    }
}
