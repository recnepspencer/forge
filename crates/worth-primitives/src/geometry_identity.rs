use crate::digest_protocol::{truth_digest_parts, TruthDigestScope};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PrimitiveSupportPlaneIdentity {
    coefficient_a: String,
    coefficient_b: String,
    coefficient_c: String,
    coefficient_d: String,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PrimitiveTriaxialEllipsoidIdentity {
    center_bits: [u64; 3],
    axis_u_bits: [u64; 3],
    axis_v_bits: [u64; 3],
    axis_w_bits: [u64; 3],
    radius_a_bits: u64,
    radius_b_bits: u64,
    radius_c_bits: u64,
}

impl PrimitiveTriaxialEllipsoidIdentity {
    pub fn new(
        center: [f64; 3],
        axis_u: [f64; 3],
        axis_v: [f64; 3],
        axis_w: [f64; 3],
        radius_a: f64,
        radius_b: f64,
        radius_c: f64,
    ) -> Self {
        Self {
            center_bits: center.map(f64::to_bits),
            axis_u_bits: axis_u.map(f64::to_bits),
            axis_v_bits: axis_v.map(f64::to_bits),
            axis_w_bits: axis_w.map(f64::to_bits),
            radius_a_bits: radius_a.to_bits(),
            radius_b_bits: radius_b.to_bits(),
            radius_c_bits: radius_c.to_bits(),
        }
    }

    pub fn digest_parts(self) -> [String; 15] {
        [
            self.center_bits[0].to_string(),
            self.center_bits[1].to_string(),
            self.center_bits[2].to_string(),
            self.axis_u_bits[0].to_string(),
            self.axis_u_bits[1].to_string(),
            self.axis_u_bits[2].to_string(),
            self.axis_v_bits[0].to_string(),
            self.axis_v_bits[1].to_string(),
            self.axis_v_bits[2].to_string(),
            self.axis_w_bits[0].to_string(),
            self.axis_w_bits[1].to_string(),
            self.axis_w_bits[2].to_string(),
            self.radius_a_bits.to_string(),
            self.radius_b_bits.to_string(),
            self.radius_c_bits.to_string(),
        ]
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PrimitiveCurvedSupportIdentity {
    TriaxialEllipsoid(PrimitiveTriaxialEllipsoidIdentity),
}

impl PrimitiveCurvedSupportIdentity {
    pub fn digest_parts(&self) -> Vec<String> {
        match self {
            Self::TriaxialEllipsoid(identity) => {
                let mut parts = vec!["triaxial-ellipsoid".to_string()];
                parts.extend(identity.digest_parts());
                parts
            }
        }
    }
}

impl PrimitiveSupportPlaneIdentity {
    pub fn new(
        coefficient_a: String,
        coefficient_b: String,
        coefficient_c: String,
        coefficient_d: String,
    ) -> Self {
        Self {
            coefficient_a,
            coefficient_b,
            coefficient_c,
            coefficient_d,
        }
    }

    pub fn digest_parts(&self) -> [String; 4] {
        [
            self.coefficient_a.clone(),
            self.coefficient_b.clone(),
            self.coefficient_c.clone(),
            self.coefficient_d.clone(),
        ]
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PrimitiveVertexIdentity {
    x_bits: u64,
    y_bits: u64,
    z_bits: u64,
}

impl PrimitiveVertexIdentity {
    pub fn from_position(position: [f64; 3]) -> Self {
        Self {
            x_bits: position[0].to_bits(),
            y_bits: position[1].to_bits(),
            z_bits: position[2].to_bits(),
        }
    }

    pub fn digest_parts(self) -> [String; 3] {
        [
            self.x_bits.to_string(),
            self.y_bits.to_string(),
            self.z_bits.to_string(),
        ]
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PrimitiveRealizedSupportIdentity {
    planes: Vec<PrimitiveSupportPlaneIdentity>,
    curved_support: Vec<PrimitiveCurvedSupportIdentity>,
}

impl PrimitiveRealizedSupportIdentity {
    pub fn new(planes: Vec<PrimitiveSupportPlaneIdentity>) -> Self {
        Self {
            planes,
            curved_support: vec![],
        }
    }

    pub fn with_curved_support(
        planes: Vec<PrimitiveSupportPlaneIdentity>,
        curved_support: Vec<PrimitiveCurvedSupportIdentity>,
    ) -> Self {
        Self {
            planes,
            curved_support,
        }
    }

    pub fn planes(&self) -> &[PrimitiveSupportPlaneIdentity] {
        &self.planes
    }

    pub fn curved_support(&self) -> &[PrimitiveCurvedSupportIdentity] {
        &self.curved_support
    }

    pub fn has_any_support(&self) -> bool {
        !self.planes.is_empty() || !self.curved_support.is_empty()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PrimitiveGeometryIdentityBundle {
    realized_support: PrimitiveRealizedSupportIdentity,
    vertices: Vec<PrimitiveVertexIdentity>,
}

impl PrimitiveGeometryIdentityBundle {
    pub fn new(
        planes: Vec<PrimitiveSupportPlaneIdentity>,
        vertices: Vec<PrimitiveVertexIdentity>,
    ) -> Self {
        Self {
            realized_support: PrimitiveRealizedSupportIdentity::new(planes),
            vertices,
        }
    }

    pub fn with_curved_support(
        planes: Vec<PrimitiveSupportPlaneIdentity>,
        curved_support: Vec<PrimitiveCurvedSupportIdentity>,
        vertices: Vec<PrimitiveVertexIdentity>,
    ) -> Self {
        Self {
            realized_support: PrimitiveRealizedSupportIdentity::with_curved_support(
                planes,
                curved_support,
            ),
            vertices,
        }
    }

    pub fn support_planes(&self) -> &[PrimitiveSupportPlaneIdentity] {
        self.realized_support.planes()
    }

    pub fn curved_support(&self) -> &[PrimitiveCurvedSupportIdentity] {
        self.realized_support.curved_support()
    }

    pub fn has_any_support(&self) -> bool {
        self.realized_support.has_any_support()
    }

    pub fn vertices(&self) -> &[PrimitiveVertexIdentity] {
        &self.vertices
    }

    pub fn scaffold_geometry_digest(&self) -> PrimitiveScaffoldGeometryDigest {
        PrimitiveScaffoldGeometryDigest(truth_digest_parts(
            TruthDigestScope::GeometryIdentity,
            &self.digest_parts(),
        ))
    }

    pub fn realization_geometry_digest(&self) -> PrimitiveRealizationGeometryDigest {
        PrimitiveRealizationGeometryDigest(truth_digest_parts(
            TruthDigestScope::GeometryIdentity,
            &self.digest_parts(),
        ))
    }

    fn digest_parts(&self) -> Vec<String> {
        let mut parts = vec![
            format!("support-plane-count:{}", self.support_planes().len()),
            format!("curved-support-count:{}", self.curved_support().len()),
            format!("vertex-count:{}", self.vertices.len()),
        ];
        parts.extend(
            self.support_planes()
                .iter()
                .enumerate()
                .flat_map(|(index, plane)| {
                    plane
                        .digest_parts()
                        .into_iter()
                        .enumerate()
                        .map(move |(component, value)| {
                            format!("plane:{index}:component:{component}:{value}")
                        })
                }),
        );
        parts.extend(self.curved_support().iter().enumerate().flat_map(
            |(index, curved_support)| {
                curved_support.digest_parts().into_iter().enumerate().map(
                    move |(component, value)| {
                        format!("curved-support:{index}:component:{component}:{value}")
                    },
                )
            },
        ));
        parts.extend(
            self.vertices
                .iter()
                .enumerate()
                .flat_map(|(index, vertex)| {
                    vertex
                        .digest_parts()
                        .into_iter()
                        .enumerate()
                        .map(move |(component, value)| {
                            format!("vertex:{index}:component:{component}:{value}")
                        })
                }),
        );
        parts
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PrimitiveScaffoldGeometryDigest(String);

impl PrimitiveScaffoldGeometryDigest {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PrimitiveRealizationGeometryDigest(String);

impl PrimitiveRealizationGeometryDigest {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}
