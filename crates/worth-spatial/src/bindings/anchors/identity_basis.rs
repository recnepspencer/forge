use worth_geom::facade::ParameterSpacePoint;

pub(crate) enum SpatialAnchorIdentityBasis {
    PointAnchor {
        binding_identity: String,
        carrier_kind: String,
        carrier_identity: String,
        parameter_semantics_signature: String,
        parameter_u_bits: String,
        parameter_v_bits: String,
    },
    DirectionAnchor {
        binding_identity: String,
        carrier_kind: String,
        carrier_identity: String,
        parameter_semantics_signature: String,
        parameter_u_bits: String,
        parameter_v_bits: String,
        direction_role: String,
    },
}

impl SpatialAnchorIdentityBasis {
    pub(crate) fn digest_parts(&self) -> Vec<String> {
        match self {
            Self::PointAnchor {
                binding_identity,
                carrier_kind,
                carrier_identity,
                parameter_semantics_signature,
                parameter_u_bits,
                parameter_v_bits,
            } => vec![
                "point-anchor".to_string(),
                binding_identity.clone(),
                carrier_kind.clone(),
                carrier_identity.clone(),
                format!("parameter-semantics:{parameter_semantics_signature}"),
                format!("parameter-u-bits:{parameter_u_bits}"),
                format!("parameter-v-bits:{parameter_v_bits}"),
            ],
            Self::DirectionAnchor {
                binding_identity,
                carrier_kind,
                carrier_identity,
                parameter_semantics_signature,
                parameter_u_bits,
                parameter_v_bits,
                direction_role,
            } => vec![
                "direction-anchor".to_string(),
                binding_identity.clone(),
                carrier_kind.clone(),
                carrier_identity.clone(),
                format!("parameter-semantics:{parameter_semantics_signature}"),
                format!("parameter-u-bits:{parameter_u_bits}"),
                format!("parameter-v-bits:{parameter_v_bits}"),
                format!("direction-role:{direction_role}"),
            ],
        }
    }
}

pub(crate) fn point_anchor_identity_basis(
    binding_identity: &str,
    carrier_kind: &str,
    carrier_identity: &str,
    parameter_semantics_signature: &str,
    point: ParameterSpacePoint,
) -> SpatialAnchorIdentityBasis {
    SpatialAnchorIdentityBasis::PointAnchor {
        binding_identity: binding_identity.to_string(),
        carrier_kind: carrier_kind.to_string(),
        carrier_identity: carrier_identity.to_string(),
        parameter_semantics_signature: parameter_semantics_signature.to_string(),
        parameter_u_bits: format!("{:016x}", point.u().to_bits()),
        parameter_v_bits: format!("{:016x}", point.v().to_bits()),
    }
}

pub(crate) fn direction_anchor_identity_basis(
    binding_identity: &str,
    carrier_kind: &str,
    carrier_identity: &str,
    parameter_semantics_signature: &str,
    point: ParameterSpacePoint,
    direction_role: &str,
) -> SpatialAnchorIdentityBasis {
    SpatialAnchorIdentityBasis::DirectionAnchor {
        binding_identity: binding_identity.to_string(),
        carrier_kind: carrier_kind.to_string(),
        carrier_identity: carrier_identity.to_string(),
        parameter_semantics_signature: parameter_semantics_signature.to_string(),
        parameter_u_bits: format!("{:016x}", point.u().to_bits()),
        parameter_v_bits: format!("{:016x}", point.v().to_bits()),
        direction_role: direction_role.to_string(),
    }
}
