use crate::digest_protocol::{truth_digest_parts, TruthDigestScope};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PrimitiveSupportPlaneIdentity {
    coefficient_a: String,
    coefficient_b: String,
    coefficient_c: String,
    coefficient_d: String,
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
}

impl PrimitiveRealizedSupportIdentity {
    pub fn new(planes: Vec<PrimitiveSupportPlaneIdentity>) -> Self {
        Self { planes }
    }

    pub fn planes(&self) -> &[PrimitiveSupportPlaneIdentity] {
        &self.planes
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

    pub fn support_planes(&self) -> &[PrimitiveSupportPlaneIdentity] {
        self.realized_support.planes()
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
        parts.extend(self.vertices.iter().enumerate().flat_map(|(index, vertex)| {
            vertex
                .digest_parts()
                .into_iter()
                .enumerate()
                .map(move |(component, value)| format!("vertex:{index}:component:{component}:{value}"))
        }));
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
