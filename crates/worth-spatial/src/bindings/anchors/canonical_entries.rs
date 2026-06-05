use crate::bindings::anchors::{
    AnchorDirectionRole, CarrierOwnedParameterDirectionAnchorSpec,
    CarrierOwnedParameterPointAnchorSpec,
};
use crate::bindings::canonical_projection::SpatialCanonicalDeclarationField;

impl CarrierOwnedParameterPointAnchorSpec {
    pub fn canonical_declaration_fields(&self) -> Vec<SpatialCanonicalDeclarationField> {
        vec![
            SpatialCanonicalDeclarationField::new("anchor_kind", "parameter_space_point"),
            SpatialCanonicalDeclarationField::new(
                "anchor_carrier_kind",
                self.ownership().carrier_kind().as_str(),
            ),
            SpatialCanonicalDeclarationField::new(
                "anchor_carrier_identity",
                self.ownership().carrier_identity(),
            ),
            SpatialCanonicalDeclarationField::new(
                "anchor_parameter_u_bits",
                format!("{:016x}", self.parameter().u().to_bits()),
            ),
            SpatialCanonicalDeclarationField::new(
                "anchor_parameter_v_bits",
                format!("{:016x}", self.parameter().v().to_bits()),
            ),
        ]
    }
}

impl CarrierOwnedParameterDirectionAnchorSpec {
    pub fn canonical_declaration_fields(&self) -> Vec<SpatialCanonicalDeclarationField> {
        vec![
            SpatialCanonicalDeclarationField::new("anchor_kind", "parameter_space_direction"),
            SpatialCanonicalDeclarationField::new(
                "anchor_carrier_kind",
                self.ownership().carrier_kind().as_str(),
            ),
            SpatialCanonicalDeclarationField::new(
                "anchor_carrier_identity",
                self.ownership().carrier_identity(),
            ),
            SpatialCanonicalDeclarationField::new(
                "anchor_parameter_u_bits",
                format!("{:016x}", self.parameter().u().to_bits()),
            ),
            SpatialCanonicalDeclarationField::new(
                "anchor_parameter_v_bits",
                format!("{:016x}", self.parameter().v().to_bits()),
            ),
            SpatialCanonicalDeclarationField::new(
                "anchor_direction_role",
                direction_role_as_str(self.role()),
            ),
        ]
    }
}

fn direction_role_as_str(role: AnchorDirectionRole) -> &'static str {
    match role {
        AnchorDirectionRole::Tangent => "tangent",
        AnchorDirectionRole::Normal => "normal",
        AnchorDirectionRole::TangentU => "tangent_u",
        AnchorDirectionRole::TangentV => "tangent_v",
    }
}
