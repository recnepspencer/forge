use crate::domain_artifacts::core_artifact::{
    impl_hadwiger_artifact, HadwigerArtifactAuthorityOwner, HadwigerArtifactCore,
    HadwigerArtifactKind, HadwigerArtifactSourceReference,
};
use crate::domain_artifacts::digest_basis::{artifact_core, HadwigerArtifactPayloadEntry};
use crate::domain_artifacts::HadwigerCanonicalArtifact;
use crate::query_entry::HadwigerResearchHandle;

use super::g27_finite_fractional_core_audit::audit_g27_w_circles_607_finite_fractional_core_checked;
use super::g27_geometric_fractional::G27GeometricFractionalError;

const SOURCE_URL: &str =
    "https://www.labri.fr/perso/pecher/pmwiki/pmwiki.php/Research/AvoidingDistance1";
const DOWNLOAD_NAME: &str = "avoidingDistance1b.zip";
const ARCHIVE_SHA256: &str =
    "sha256:c9a563a82f9e1a097329f72ab8b4baaa9104f5530990802ab2295f7afce09a09";
const DATA_SHA256: &str = "sha256:be181cad41b7156208a583235ab6937c51eb2292b7bed952bb98f68e0b1b4dad";
const VERTICES_SHA256: &str =
    "sha256:5ccc75a58b5768f49816c4231a228f4e0430118f5fafa03f0f660e23c0469e95";
const TARGET_WEIGHT: u128 = 512_933;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum G27WCirclesPublicArtifactKind {
    Data,
    Model,
    Script,
    GeneratedCode,
}

impl G27WCirclesPublicArtifactKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Data => "data",
            Self::Model => "model",
            Self::Script => "script",
            Self::GeneratedCode => "generated_code",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct G27WCirclesPublicArtifactRow {
    filename: &'static str,
    size_bytes: u64,
    sha256: &'static str,
    kind: G27WCirclesPublicArtifactKind,
    proof_like: bool,
}

impl G27WCirclesPublicArtifactRow {
    pub fn filename(&self) -> &'static str {
        self.filename
    }

    pub fn size_bytes(&self) -> u64 {
        self.size_bytes
    }

    pub fn sha256(&self) -> &'static str {
        self.sha256
    }

    pub fn kind(&self) -> G27WCirclesPublicArtifactKind {
        self.kind
    }

    pub fn proof_like(&self) -> bool {
        self.proof_like
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum G27WCirclesPublicArtifactInventoryStatus {
    RetiredPublicPackageNoReplayableProof,
    FoundCandidateProofArtifact,
}

impl G27WCirclesPublicArtifactInventoryStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::RetiredPublicPackageNoReplayableProof => {
                "retired_public_package_no_replayable_proof"
            }
            Self::FoundCandidateProofArtifact => "found_candidate_proof_artifact",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct G27WCirclesPublicArtifactInventoryReport {
    core: HadwigerArtifactCore,
    source_url: String,
    download_name: String,
    archive_sha256: String,
    retained_data_sha256: String,
    retained_vertices_sha256: String,
    file_count: usize,
    proof_like_file_count: usize,
    data_file_count: usize,
    model_file_count: usize,
    script_file_count: usize,
    generated_code_file_count: usize,
    target_weight: u128,
    status: G27WCirclesPublicArtifactInventoryStatus,
    required_import_schema: String,
    conclusion: String,
}

impl G27WCirclesPublicArtifactInventoryReport {
    pub fn source_summary(&self) -> (&str, &str, &str) {
        (&self.source_url, &self.download_name, &self.archive_sha256)
    }

    pub fn digest_summary(&self) -> (&str, &str) {
        (&self.retained_data_sha256, &self.retained_vertices_sha256)
    }

    pub fn inventory_summary(&self) -> (usize, usize, usize, usize, usize, usize) {
        (
            self.file_count,
            self.proof_like_file_count,
            self.data_file_count,
            self.model_file_count,
            self.script_file_count,
            self.generated_code_file_count,
        )
    }

    pub fn target_weight(&self) -> u128 {
        self.target_weight
    }

    pub fn status(&self) -> G27WCirclesPublicArtifactInventoryStatus {
        self.status
    }

    pub fn required_import_schema(&self) -> &str {
        &self.required_import_schema
    }

    pub fn conclusion(&self) -> &str {
        &self.conclusion
    }

    pub fn rows(&self) -> &'static [G27WCirclesPublicArtifactRow] {
        INVENTORY_ROWS
    }

    pub fn admits_theorem_authority(&self) -> bool {
        false
    }
}

impl_hadwiger_artifact!(G27WCirclesPublicArtifactInventoryReport, core);

pub fn inventory_g27_w_circles_public_artifacts_checked(
    handle: &HadwigerResearchHandle,
) -> Result<G27WCirclesPublicArtifactInventoryReport, G27GeometricFractionalError> {
    let source = audit_g27_w_circles_607_finite_fractional_core_checked(handle)?;
    let proof_like_file_count = INVENTORY_ROWS.iter().filter(|row| row.proof_like).count();
    let data_file_count = count_kind(G27WCirclesPublicArtifactKind::Data);
    let model_file_count = count_kind(G27WCirclesPublicArtifactKind::Model);
    let script_file_count = count_kind(G27WCirclesPublicArtifactKind::Script);
    let generated_code_file_count = count_kind(G27WCirclesPublicArtifactKind::GeneratedCode);
    let status = if proof_like_file_count == 0 {
        G27WCirclesPublicArtifactInventoryStatus::RetiredPublicPackageNoReplayableProof
    } else {
        G27WCirclesPublicArtifactInventoryStatus::FoundCandidateProofArtifact
    };
    let required_import_schema = "instance_digest + objective_upper_bound + branch_tree_or_rational_dual_or_weighted_cover + exact_replay_checker".to_string();
    let conclusion = conclusion(status, proof_like_file_count);
    let core = artifact_core(
        HadwigerArtifactKind::G27WCirclesPublicArtifactInventoryReport,
        HadwigerArtifactAuthorityOwner::HadwigerArtifactBuilder,
        HadwigerArtifactSourceReference::ArtifactConstruction {
            operation: "g27_w_circles_public_artifact_inventory".to_string(),
        },
        vec![source.reference()],
        payload(
            proof_like_file_count,
            data_file_count,
            model_file_count,
            script_file_count,
            generated_code_file_count,
            status,
            &required_import_schema,
            &conclusion,
        ),
    )?;
    Ok(G27WCirclesPublicArtifactInventoryReport {
        core,
        source_url: SOURCE_URL.to_string(),
        download_name: DOWNLOAD_NAME.to_string(),
        archive_sha256: ARCHIVE_SHA256.to_string(),
        retained_data_sha256: DATA_SHA256.to_string(),
        retained_vertices_sha256: VERTICES_SHA256.to_string(),
        file_count: INVENTORY_ROWS.len(),
        proof_like_file_count,
        data_file_count,
        model_file_count,
        script_file_count,
        generated_code_file_count,
        target_weight: TARGET_WEIGHT,
        status,
        required_import_schema,
        conclusion,
    })
}

fn count_kind(kind: G27WCirclesPublicArtifactKind) -> usize {
    INVENTORY_ROWS.iter().filter(|row| row.kind == kind).count()
}

fn conclusion(
    status: G27WCirclesPublicArtifactInventoryStatus,
    proof_like_file_count: usize,
) -> String {
    match status {
        G27WCirclesPublicArtifactInventoryStatus::RetiredPublicPackageNoReplayableProof => {
            "public W_circles_607 package contains data, model, and reproduction scripts only; no replayable upper-bound proof artifact is present".to_string()
        }
        G27WCirclesPublicArtifactInventoryStatus::FoundCandidateProofArtifact => {
            format!("public W_circles_607 package has {proof_like_file_count} proof-like files requiring exact replay triage")
        }
    }
}

fn payload(
    proof_like_file_count: usize,
    data_file_count: usize,
    model_file_count: usize,
    script_file_count: usize,
    generated_code_file_count: usize,
    status: G27WCirclesPublicArtifactInventoryStatus,
    required_import_schema: &str,
    conclusion: &str,
) -> Vec<HadwigerArtifactPayloadEntry> {
    vec![
        HadwigerArtifactPayloadEntry::text("schema", "forge.hadwiger.w607_public_inventory.v1"),
        HadwigerArtifactPayloadEntry::text("source_url", SOURCE_URL),
        HadwigerArtifactPayloadEntry::text("download_name", DOWNLOAD_NAME),
        HadwigerArtifactPayloadEntry::text("archive_sha256", ARCHIVE_SHA256),
        HadwigerArtifactPayloadEntry::text("retained_data_sha256", DATA_SHA256),
        HadwigerArtifactPayloadEntry::text("retained_vertices_sha256", VERTICES_SHA256),
        HadwigerArtifactPayloadEntry::unsigned("file_count", INVENTORY_ROWS.len() as u128),
        HadwigerArtifactPayloadEntry::unsigned(
            "proof_like_file_count",
            proof_like_file_count as u128,
        ),
        HadwigerArtifactPayloadEntry::unsigned("data_file_count", data_file_count as u128),
        HadwigerArtifactPayloadEntry::unsigned("model_file_count", model_file_count as u128),
        HadwigerArtifactPayloadEntry::unsigned("script_file_count", script_file_count as u128),
        HadwigerArtifactPayloadEntry::unsigned(
            "generated_code_file_count",
            generated_code_file_count as u128,
        ),
        HadwigerArtifactPayloadEntry::unsigned("target_weight", TARGET_WEIGHT),
        HadwigerArtifactPayloadEntry::text("status", status.as_str()),
        HadwigerArtifactPayloadEntry::text("required_import_schema", required_import_schema),
        HadwigerArtifactPayloadEntry::text("conclusion", conclusion),
    ]
}

const INVENTORY_ROWS: &[G27WCirclesPublicArtifactRow] = &[
    row(
        "computeWeightedIndependentNumber.sh",
        138,
        "sha256:275a4ce73d9db092629dd4236f0ca018afacfef46ce07cea28d6960cd5338b0e",
        G27WCirclesPublicArtifactKind::Script,
    ),
    row(
        "graphw_example.dat",
        3905,
        "sha256:3ce235603f1cda6982dd024ffb1fa301bb95aa0e858ee48378a1ccf8cad2c234",
        G27WCirclesPublicArtifactKind::Data,
    ),
    row(
        "makeDatFile.sage",
        874,
        "sha256:2b9e1da86d318b987c0e2137dfb560858821c714e31da704dec32b0e0b86c1a3",
        G27WCirclesPublicArtifactKind::Script,
    ),
    row(
        "makeDatFile.sage.py",
        1264,
        "sha256:722e7e49d20588897cf93211c7359751e6ebb4a31e3bad11e2611a620bfc9659",
        G27WCirclesPublicArtifactKind::GeneratedCode,
    ),
    row(
        "makeEdges.sage",
        2002,
        "sha256:748719753f7b7ac896bfa0cc810e296dae4cbfc52c2236ed8d6a8cefe4d446c2",
        G27WCirclesPublicArtifactKind::Script,
    ),
    row(
        "makeEdges.sage.py",
        4854,
        "sha256:b8b34def01c3f066ca2b3df7825b4e08fa3afea09037382dc1b0d129062f0d73",
        G27WCirclesPublicArtifactKind::GeneratedCode,
    ),
    row(
        "run_W_circles_607.sh",
        420,
        "sha256:f12646adb1d136b2241e70af974611f6d0b258eac00500103d55692c5bc8fc4b",
        G27WCirclesPublicArtifactKind::Script,
    ),
    row(
        "sumOfWeights.sage.py",
        348,
        "sha256:b4ad76f28437a9967fcc16fabc785411a4cc639f107f23a3908702eae0b2e66d",
        G27WCirclesPublicArtifactKind::GeneratedCode,
    ),
    row(
        "W_circles_607.dat",
        47229,
        "sha256:be181cad41b7156208a583235ab6937c51eb2292b7bed952bb98f68e0b1b4dad",
        G27WCirclesPublicArtifactKind::Data,
    ),
    row(
        "W_circles_607_edges.sage",
        36276,
        "sha256:555dfe2183b10b8c85f2ec5e34e3d0cf89ddda28b4f7505985dfd663faf5760b",
        G27WCirclesPublicArtifactKind::Data,
    ),
    row(
        "W_circles_607_integer_weights.sage",
        4095,
        "sha256:9e1ad3fac5140b859eadb9d485027fe449964bed7ff441c3cfaa1dacf497b0d8",
        G27WCirclesPublicArtifactKind::Data,
    ),
    row(
        "W_circles_607_integers.dat",
        47229,
        "sha256:be181cad41b7156208a583235ab6937c51eb2292b7bed952bb98f68e0b1b4dad",
        G27WCirclesPublicArtifactKind::Data,
    ),
    row(
        "W_circles_607_vertices.sage",
        23881,
        "sha256:5ccc75a58b5768f49816c4231a228f4e0430118f5fafa03f0f660e23c0469e95",
        G27WCirclesPublicArtifactKind::Data,
    ),
    row(
        "W_circles_607_weights.sage",
        13321,
        "sha256:dc64e5505250908b6ee8e875bc30464664e30aeadcd57d654ef349b9c3c51830",
        G27WCirclesPublicArtifactKind::Data,
    ),
    row(
        "weightedIndependentNumber.mod",
        349,
        "sha256:c85cb1841c22e80b0610ec2183e7799f1c9f8b703084f5f331f183b4fce51bc4",
        G27WCirclesPublicArtifactKind::Model,
    ),
];

const fn row(
    filename: &'static str,
    size_bytes: u64,
    sha256: &'static str,
    kind: G27WCirclesPublicArtifactKind,
) -> G27WCirclesPublicArtifactRow {
    G27WCirclesPublicArtifactRow {
        filename,
        size_bytes,
        sha256,
        kind,
        proof_like: false,
    }
}
