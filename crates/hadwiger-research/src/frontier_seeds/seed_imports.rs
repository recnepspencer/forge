use crate::domain_artifacts::HadwigerArtifactShapeError;

const HEULE_PARTS_517_EDGE_LIST: &str = include_str!("heule_parts_517.edge");
const HEULE_PARTS_517_DIGEST: &str =
    "sha256:dc5085db9682aa246c3fc56efed9767e2a294a43e621a3e67a690d0489bdadc9";
const HEULE_PARTS_517_URL: &str =
    "https://github.com/simon-tiger/Hadwiger-Nelson-Project-Data/blob/master/517.edge";
const HEULE_510_EDGE_LIST: &str = include_str!("heule_510.edge");
const HEULE_510_ALGEBRAIC_CERTIFICATE: &str = include_str!("heule_510.alg");
const HEULE_510_EDGE_DIGEST: &str =
    "sha256:a6ac512bb50d6c9e380b6b0eedb7236b916a6010b54b4adc0ca171a1bf7f3272";
const HEULE_510_COORDINATE_DIGEST: &str =
    "sha256:f6aba3ec7158445875229ff1bf52b68a9a709cd871bdfe468ca65ad865b8622a";
const HEULE_510_URL: &str =
    "https://github.com/vasnesterov/HadwigerNelson/blob/master/vtx/510_heule.vtx";
const G27_GEOMETRIC_FRACTIONAL_URL: &str = "https://static.renyi.hu/ai-shared/daniel/fcn-4/";
const G27_GEOMETRIC_FRACTIONAL_DIGEST: &str = concat!(
    "g27_coeffs=sha256:9b368fcc538940cda9678c87fda77aee532c5f153cb7bc3e6903fd05862e6168;",
    "adjacency=sha256:7a47c3d25bb459ef4cdd73ffa1372734c967bf93cb319265738a01c4d6882fa0;",
    "atoms=sha256:b425ec7045d74028aec379440823903801e15703834d31652c0a3cff354bab74;",
    "isometries=sha256:ec556af4c50e1e170b585d123513258732c9e4b9859938e22ace574e41b700ad;",
    "witness=sha256:1581c9d2139b12d56a3f9b0e2b88f0d985f2244aa0239b8fe004d496bd151adb"
);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FrontierSeedFormat {
    DimacsEdgeList,
}

impl FrontierSeedFormat {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::DimacsEdgeList => "dimacs_edge_list",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FrontierGraphSeedImport {
    seed_id: String,
    version_id: String,
    source_url: String,
    source_digest: String,
    source_family: String,
    format: FrontierSeedFormat,
    edge_list: String,
    algebraic_embedding_certificate: Option<String>,
}

impl FrontierGraphSeedImport {
    pub fn heule_parts_517() -> Self {
        Self {
            seed_id: "heule-parts-517".to_string(),
            version_id: "517.edge".to_string(),
            source_url: HEULE_PARTS_517_URL.to_string(),
            source_digest: HEULE_PARTS_517_DIGEST.to_string(),
            source_family: "heule_parts_517".to_string(),
            format: FrontierSeedFormat::DimacsEdgeList,
            edge_list: HEULE_PARTS_517_EDGE_LIST.to_string(),
            algebraic_embedding_certificate: None,
        }
    }

    pub fn heule_510_exact() -> Self {
        Self {
            seed_id: "heule-510-exact".to_string(),
            version_id: "510.heule.exact".to_string(),
            source_url: HEULE_510_URL.to_string(),
            source_digest: format!("{HEULE_510_EDGE_DIGEST};{HEULE_510_COORDINATE_DIGEST}"),
            source_family: "heule_510_exact_algebraic".to_string(),
            format: FrontierSeedFormat::DimacsEdgeList,
            edge_list: HEULE_510_EDGE_LIST.to_string(),
            algebraic_embedding_certificate: Some(HEULE_510_ALGEBRAIC_CERTIFICATE.to_string()),
        }
    }

    pub fn g27_geometric_fractional() -> Self {
        Self {
            seed_id: "matolcsi-ruzsa-varga-zsamboki-g27".to_string(),
            version_id: "g27.geometric_fractional.v1".to_string(),
            source_url: G27_GEOMETRIC_FRACTIONAL_URL.to_string(),
            source_digest: G27_GEOMETRIC_FRACTIONAL_DIGEST.to_string(),
            source_family: "geometric_fractional_chromatic_g27".to_string(),
            format: FrontierSeedFormat::DimacsEdgeList,
            edge_list: super::g27_geometric_fractional::g27_dimacs_edge_list(),
            algebraic_embedding_certificate: None,
        }
    }

    pub fn dimacs_edge_list(
        seed_id: impl Into<String>,
        version_id: impl Into<String>,
        edge_list: impl Into<String>,
    ) -> Result<Self, HadwigerArtifactShapeError> {
        Ok(Self {
            seed_id: require_text(seed_id, "seed_id")?,
            version_id: require_text(version_id, "version_id")?,
            source_url: "local:frontier-seed".to_string(),
            source_digest: "sha256:caller-supplied".to_string(),
            source_family: "custom_frontier_seed".to_string(),
            format: FrontierSeedFormat::DimacsEdgeList,
            edge_list: require_text(edge_list, "edge_list")?,
            algebraic_embedding_certificate: None,
        })
    }

    pub fn with_source_url(mut self, source_url: impl Into<String>) -> Self {
        self.source_url = require_text(source_url, "source_url").expect("source_url is non-empty");
        self
    }

    pub fn with_source_digest(mut self, source_digest: impl Into<String>) -> Self {
        self.source_digest =
            require_text(source_digest, "source_digest").expect("source_digest is non-empty");
        self
    }

    pub fn with_source_family(mut self, source_family: impl Into<String>) -> Self {
        self.source_family =
            require_text(source_family, "source_family").expect("source_family is non-empty");
        self
    }

    pub fn with_algebraic_embedding_certificate(mut self, certificate: impl Into<String>) -> Self {
        self.algebraic_embedding_certificate = Some(
            require_text(certificate, "algebraic_embedding_certificate")
                .expect("algebraic_embedding_certificate must be non-empty"),
        );
        self
    }

    pub(crate) fn seed_id(&self) -> &str {
        &self.seed_id
    }

    pub(crate) fn version_id(&self) -> &str {
        &self.version_id
    }

    pub(crate) fn source_url(&self) -> &str {
        &self.source_url
    }

    pub(crate) fn source_digest(&self) -> &str {
        &self.source_digest
    }

    pub(crate) fn source_family(&self) -> &str {
        &self.source_family
    }

    pub(crate) fn format(&self) -> FrontierSeedFormat {
        self.format
    }

    pub(crate) fn edge_list(&self) -> &str {
        &self.edge_list
    }

    pub(crate) fn algebraic_embedding_certificate(&self) -> Option<&str> {
        self.algebraic_embedding_certificate.as_deref()
    }
}

fn require_text(
    value: impl Into<String>,
    field: &'static str,
) -> Result<String, HadwigerArtifactShapeError> {
    let value = value.into();
    if value.trim().is_empty() {
        Err(HadwigerArtifactShapeError::EmptyField { field })
    } else {
        Ok(value)
    }
}
