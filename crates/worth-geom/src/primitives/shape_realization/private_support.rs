use sha2::{Digest, Sha256};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum PrimitiveDigestScope {
    ArtifactIdentity,
    WitnessIdentity,
}

impl PrimitiveDigestScope {
    fn as_str(self) -> &'static str {
        match self {
            Self::ArtifactIdentity => "artifact_identity",
            Self::WitnessIdentity => "witness_identity",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct PrimitiveIdentityDigest(String);

impl PrimitiveIdentityDigest {
    pub(super) fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct PrimitiveVertexIdentity {
    x_bits: u64,
    y_bits: u64,
    z_bits: u64,
}

impl PrimitiveVertexIdentity {
    pub(super) fn from_position(position: [f64; 3]) -> Self {
        Self {
            x_bits: position[0].to_bits(),
            y_bits: position[1].to_bits(),
            z_bits: position[2].to_bits(),
        }
    }

    fn digest_fragment(&self) -> String {
        format!("vertex:{}:{}:{}", self.x_bits, self.y_bits, self.z_bits)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct PrimitiveSupportPlaneIdentity {
    a: String,
    b: String,
    c: String,
    d: String,
}

impl PrimitiveSupportPlaneIdentity {
    pub(super) fn new(a: String, b: String, c: String, d: String) -> Self {
        Self { a, b, c, d }
    }

    fn digest_fragment(&self) -> String {
        format!("plane:{}:{}:{}:{}", self.a, self.b, self.c, self.d)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct PrimitiveGeometryIdentityBundle {
    planes: Vec<PrimitiveSupportPlaneIdentity>,
    vertices: Vec<PrimitiveVertexIdentity>,
    realization_geometry_digest: PrimitiveIdentityDigest,
}

impl PrimitiveGeometryIdentityBundle {
    pub(super) fn new(
        planes: Vec<PrimitiveSupportPlaneIdentity>,
        vertices: Vec<PrimitiveVertexIdentity>,
    ) -> Self {
        let mut parts = Vec::with_capacity(planes.len() + vertices.len());
        parts.extend(planes.iter().map(PrimitiveSupportPlaneIdentity::digest_fragment));
        parts.extend(vertices.iter().map(PrimitiveVertexIdentity::digest_fragment));
        let realization_geometry_digest = PrimitiveIdentityDigest(truth_digest_parts(
            PrimitiveDigestScope::ArtifactIdentity,
            &parts,
        ));
        Self {
            planes,
            vertices,
            realization_geometry_digest,
        }
    }

    pub(super) fn realization_geometry_digest(&self) -> &PrimitiveIdentityDigest {
        &self.realization_geometry_digest
    }
}

#[derive(Clone, Debug, PartialEq)]
pub(super) struct CanonicalSimplexVertices {
    local_vertices: [[f64; 3]; 4],
}

impl CanonicalSimplexVertices {
    pub(super) fn local_vertices(&self) -> &[[f64; 3]; 4] {
        &self.local_vertices
    }
}

pub(super) fn canonical_simplex_vertices(
    scale: f64,
    auxiliary_altitude_component: f64,
) -> CanonicalSimplexVertices {
    let root_three = 3.0_f64.sqrt();
    let base_radius = scale;
    let base_z = -scale / 3.0;
    let apex_z = scale + auxiliary_altitude_component;
    CanonicalSimplexVertices {
        local_vertices: [
            [base_radius, 0.0, base_z],
            [-0.5 * base_radius, 0.5 * root_three * base_radius, base_z],
            [-0.5 * base_radius, -0.5 * root_three * base_radius, base_z],
            [0.0, 0.0, apex_z],
        ],
    }
}

pub(super) fn truth_digest_parts(scope: PrimitiveDigestScope, parts: &[String]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(scope.as_str().as_bytes());
    for part in parts {
        hasher.update([0x1f]);
        hasher.update(part.as_bytes());
    }
    format!("worth-geom:{}:{:x}", scope.as_str(), hasher.finalize())
}
