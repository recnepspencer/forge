use worth_geom::facade::ParameterSpacePoint;

use crate::bindings::admitted_binding::SpatialAdmittedPrimitiveBinding;
use crate::bindings::authority::SpatialBindingCompleteness;

use super::{neighborhood::NeighborhoodBindingFamily, SpatialRebindingAuthorityError};

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

impl BindingSnapshot {
    pub(crate) fn from_binding(
        binding: &SpatialAdmittedPrimitiveBinding,
    ) -> Result<Self, SpatialRebindingAuthorityError> {
        let family = NeighborhoodBindingFamily::from_binding(binding)?;
        Ok(match binding {
            SpatialAdmittedPrimitiveBinding::FaceSurface(binding) => Self {
                family,
                birth_class: binding.birth_contract().topology_birth_class().to_string(),
                geometry_digest: binding
                    .geometry_identity()
                    .scaffold_geometry_digest()
                    .as_str()
                    .to_string(),
                completeness: *binding.completeness(),
                anchor: None,
            },
            SpatialAdmittedPrimitiveBinding::EdgeCurve(binding) => Self {
                family,
                birth_class: binding.birth_contract().topology_birth_class().to_string(),
                geometry_digest: binding
                    .geometry_identity()
                    .scaffold_geometry_digest()
                    .as_str()
                    .to_string(),
                completeness: *binding.completeness(),
                anchor: None,
            },
            SpatialAdmittedPrimitiveBinding::CoedgePCurve(binding) => Self {
                family,
                birth_class: binding.birth_contract().topology_birth_class().to_string(),
                geometry_digest: binding
                    .geometry_identity()
                    .scaffold_geometry_digest()
                    .as_str()
                    .to_string(),
                completeness: *binding.completeness(),
                anchor: None,
            },
            SpatialAdmittedPrimitiveBinding::VertexGeometry(binding) => Self {
                family,
                birth_class: binding.birth_contract().topology_birth_class().to_string(),
                geometry_digest: binding
                    .geometry_identity()
                    .scaffold_geometry_digest()
                    .as_str()
                    .to_string(),
                completeness: *binding.completeness(),
                anchor: None,
            },
            SpatialAdmittedPrimitiveBinding::FaceSurfacePointAnchor(binding) => Self {
                family,
                birth_class: binding
                    .binding()
                    .birth_contract()
                    .topology_birth_class()
                    .to_string(),
                geometry_digest: binding
                    .binding()
                    .geometry_identity()
                    .scaffold_geometry_digest()
                    .as_str()
                    .to_string(),
                completeness: *binding.completeness(),
                anchor: Some(AnchorSnapshot::point(
                    binding.anchor().ownership().carrier_kind().as_str(),
                    binding.anchor().ownership().carrier_identity(),
                    &binding.anchor().ownership().parameter_semantics_signature(),
                    binding.anchor().canonical_parameter().point(),
                )),
            },
            SpatialAdmittedPrimitiveBinding::EdgeCurvePointAnchor(binding) => Self {
                family,
                birth_class: binding
                    .binding()
                    .birth_contract()
                    .topology_birth_class()
                    .to_string(),
                geometry_digest: binding
                    .binding()
                    .geometry_identity()
                    .scaffold_geometry_digest()
                    .as_str()
                    .to_string(),
                completeness: *binding.completeness(),
                anchor: Some(AnchorSnapshot::point(
                    binding.anchor().ownership().carrier_kind().as_str(),
                    binding.anchor().ownership().carrier_identity(),
                    &binding.anchor().ownership().parameter_semantics_signature(),
                    binding.anchor().canonical_parameter().point(),
                )),
            },
            SpatialAdmittedPrimitiveBinding::CoedgePCurvePointAnchor(binding) => Self {
                family,
                birth_class: binding
                    .binding()
                    .birth_contract()
                    .topology_birth_class()
                    .to_string(),
                geometry_digest: binding
                    .binding()
                    .geometry_identity()
                    .scaffold_geometry_digest()
                    .as_str()
                    .to_string(),
                completeness: *binding.completeness(),
                anchor: Some(AnchorSnapshot::point(
                    binding.anchor().ownership().carrier_kind().as_str(),
                    binding.anchor().ownership().carrier_identity(),
                    &binding.anchor().ownership().parameter_semantics_signature(),
                    binding.anchor().canonical_parameter().point(),
                )),
            },
            SpatialAdmittedPrimitiveBinding::FaceSurfaceDirectionAnchor(binding) => Self {
                family,
                birth_class: binding
                    .binding()
                    .birth_contract()
                    .topology_birth_class()
                    .to_string(),
                geometry_digest: binding
                    .binding()
                    .geometry_identity()
                    .scaffold_geometry_digest()
                    .as_str()
                    .to_string(),
                completeness: *binding.completeness(),
                anchor: Some(AnchorSnapshot::direction(
                    binding.anchor().ownership().carrier_kind().as_str(),
                    binding.anchor().ownership().carrier_identity(),
                    &binding.anchor().ownership().parameter_semantics_signature(),
                    binding.anchor().canonical_parameter().point(),
                    binding.anchor().role_as_str(),
                )),
            },
            SpatialAdmittedPrimitiveBinding::EdgeCurveDirectionAnchor(binding) => Self {
                family,
                birth_class: binding
                    .binding()
                    .birth_contract()
                    .topology_birth_class()
                    .to_string(),
                geometry_digest: binding
                    .binding()
                    .geometry_identity()
                    .scaffold_geometry_digest()
                    .as_str()
                    .to_string(),
                completeness: *binding.completeness(),
                anchor: Some(AnchorSnapshot::direction(
                    binding.anchor().ownership().carrier_kind().as_str(),
                    binding.anchor().ownership().carrier_identity(),
                    &binding.anchor().ownership().parameter_semantics_signature(),
                    binding.anchor().canonical_parameter().point(),
                    binding.anchor().role_as_str(),
                )),
            },
            SpatialAdmittedPrimitiveBinding::CoedgePCurveDirectionAnchor(binding) => Self {
                family,
                birth_class: binding
                    .binding()
                    .birth_contract()
                    .topology_birth_class()
                    .to_string(),
                geometry_digest: binding
                    .binding()
                    .geometry_identity()
                    .scaffold_geometry_digest()
                    .as_str()
                    .to_string(),
                completeness: *binding.completeness(),
                anchor: Some(AnchorSnapshot::direction(
                    binding.anchor().ownership().carrier_kind().as_str(),
                    binding.anchor().ownership().carrier_identity(),
                    &binding.anchor().ownership().parameter_semantics_signature(),
                    binding.anchor().canonical_parameter().point(),
                    binding.anchor().role_as_str(),
                )),
            },
        })
    }
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
