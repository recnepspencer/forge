use crate::planar_contracts::structural_identity::PlanarStructuralIdentityBasis;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlanarStructuralIdentityInspectionKind {
    StructuralAuthority,
    CanonicalTransformBasis,
    ContrastOnly,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarStructuralIdentityInspectionRow {
    kind: PlanarStructuralIdentityInspectionKind,
    locus: &'static str,
    value: String,
}

impl PlanarStructuralIdentityInspectionRow {
    pub(crate) fn from_basis(basis: &PlanarStructuralIdentityBasis) -> Vec<Self> {
        let transform = basis.canonical_transform_basis();
        vec![
            row(
                PlanarStructuralIdentityInspectionKind::StructuralAuthority,
                "boolean_readiness.fact",
                basis.boolean_readiness_receipt().fact_digest(),
            ),
            row(
                PlanarStructuralIdentityInspectionKind::CanonicalTransformBasis,
                "transform.local_frame",
                transform.local_frame_identity(),
            ),
            row(
                PlanarStructuralIdentityInspectionKind::CanonicalTransformBasis,
                "transform.movement_rotation",
                transform.movement_rotation_posture_identity(),
            ),
            row(
                PlanarStructuralIdentityInspectionKind::CanonicalTransformBasis,
                "transform.chain",
                transform.transform_chain_digest(),
            ),
            row(
                PlanarStructuralIdentityInspectionKind::CanonicalTransformBasis,
                "transform.orientation",
                transform.orientation_policy().as_str(),
            ),
            row(
                PlanarStructuralIdentityInspectionKind::ContrastOnly,
                "contrast.topology",
                basis.topology_identity(),
            ),
            row(
                PlanarStructuralIdentityInspectionKind::ContrastOnly,
                "contrast.name",
                basis.persistent_name(),
            ),
            row(
                PlanarStructuralIdentityInspectionKind::ContrastOnly,
                "contrast.binding",
                basis.binding_identity(),
            ),
            row(
                PlanarStructuralIdentityInspectionKind::ContrastOnly,
                "contrast.lineage",
                basis.lineage_identity(),
            ),
        ]
    }

    pub fn kind(&self) -> PlanarStructuralIdentityInspectionKind {
        self.kind
    }

    pub fn locus(&self) -> &'static str {
        self.locus
    }

    pub fn value(&self) -> &str {
        &self.value
    }
}

fn row(
    kind: PlanarStructuralIdentityInspectionKind,
    locus: &'static str,
    value: impl Into<String>,
) -> PlanarStructuralIdentityInspectionRow {
    PlanarStructuralIdentityInspectionRow {
        kind,
        locus,
        value: value.into(),
    }
}
