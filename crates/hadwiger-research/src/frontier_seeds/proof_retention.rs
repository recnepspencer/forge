use std::collections::BTreeMap;
use std::fs::File;
use std::io::{Error as IoError, Read};
use std::path::{Path, PathBuf};

use sha2::{Digest, Sha256};

use crate::mathematical_verification::{
    ColoringProofCertificate, ColoringProofCertificateFormat, HadwigerColorabilityError,
};

const HEULE_510_PROOF_MANIFEST: &str = include_str!("heule_510.proof.manifest");
const HEULE_510_PROOF_PATH_ENV: &str = "HADWIGER_HEULE_510_VARISAT_PROOF";

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RetainedFrontierProofError {
    MalformedManifest { field: &'static str },
    UnsupportedFormat { format: String },
    ProofUnavailable { path: PathBuf },
    ProofIo { path: PathBuf, error: String },
    ProofLengthMismatch { expected: u64, actual: u64 },
    ProofDigestMismatch { expected: String, actual: String },
    Colorability(HadwigerColorabilityError),
}

impl From<HadwigerColorabilityError> for RetainedFrontierProofError {
    fn from(value: HadwigerColorabilityError) -> Self {
        Self::Colorability(value)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RetainedFrontierColoringProof {
    proof_id: String,
    seed_id: String,
    format: ColoringProofCertificateFormat,
    color_count: u32,
    cnf_digest: String,
    proof_sha256: String,
    proof_byte_length: u64,
    local_path: PathBuf,
}

impl RetainedFrontierColoringProof {
    pub fn heule_510_not_four_colorable() -> Result<Self, RetainedFrontierProofError> {
        let manifest = Manifest::parse(HEULE_510_PROOF_MANIFEST)?;
        let default_path = manifest.required("default_local_path")?;
        let local_path = std::env::var(HEULE_510_PROOF_PATH_ENV)
            .map(PathBuf::from)
            .unwrap_or_else(|_| resolve_default_path(default_path));
        Ok(Self {
            proof_id: manifest.required("proof_id")?.to_string(),
            seed_id: manifest.required("seed_id")?.to_string(),
            format: parse_format(manifest.required("proof_format")?)?,
            color_count: parse_u32(manifest.required("color_count")?, "color_count")?,
            cnf_digest: manifest.required("cnf_digest")?.to_string(),
            proof_sha256: manifest.required("proof_sha256")?.to_string(),
            proof_byte_length: parse_u64(
                manifest.required("proof_byte_length")?,
                "proof_byte_length",
            )?,
            local_path,
        })
    }

    pub fn proof_id(&self) -> &str {
        &self.proof_id
    }

    pub fn seed_id(&self) -> &str {
        &self.seed_id
    }

    pub fn format(&self) -> ColoringProofCertificateFormat {
        self.format
    }

    pub fn color_count(&self) -> u32 {
        self.color_count
    }

    pub fn cnf_digest(&self) -> &str {
        &self.cnf_digest
    }

    pub fn proof_sha256(&self) -> &str {
        &self.proof_sha256
    }

    pub fn proof_byte_length(&self) -> u64 {
        self.proof_byte_length
    }

    pub fn local_path(&self) -> &Path {
        &self.local_path
    }

    pub fn load_certificate(&self) -> Result<ColoringProofCertificate, RetainedFrontierProofError> {
        let (bytes, digest) = read_and_hash(&self.local_path)?;
        if bytes.len() as u64 != self.proof_byte_length {
            return Err(RetainedFrontierProofError::ProofLengthMismatch {
                expected: self.proof_byte_length,
                actual: bytes.len() as u64,
            });
        }
        if digest != self.proof_sha256 {
            return Err(RetainedFrontierProofError::ProofDigestMismatch {
                expected: self.proof_sha256.clone(),
                actual: digest,
            });
        }
        match self.format {
            ColoringProofCertificateFormat::VarisatNative => {
                ColoringProofCertificate::varisat_native_from_bytes(self.cnf_digest.clone(), bytes)
                    .map_err(Into::into)
            }
            ColoringProofCertificateFormat::Lrat => {
                ColoringProofCertificate::lrat_from_bytes(&bytes).map_err(Into::into)
            }
        }
    }

    pub fn proof_file_available(&self) -> bool {
        self.local_path.is_file()
    }
}

pub fn load_heule_510_not_four_colorability_certificate_checked(
) -> Result<ColoringProofCertificate, RetainedFrontierProofError> {
    RetainedFrontierColoringProof::heule_510_not_four_colorable()?.load_certificate()
}

struct Manifest {
    fields: BTreeMap<String, String>,
}

impl Manifest {
    fn parse(text: &str) -> Result<Self, RetainedFrontierProofError> {
        let mut fields = BTreeMap::new();
        for line in text.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            let (key, value) =
                line.split_once(' ')
                    .ok_or(RetainedFrontierProofError::MalformedManifest {
                        field: "manifest_line",
                    })?;
            fields.insert(key.to_string(), value.trim().to_string());
        }
        Ok(Self { fields })
    }

    fn required(&self, field: &'static str) -> Result<&str, RetainedFrontierProofError> {
        self.fields
            .get(field)
            .map(String::as_str)
            .filter(|value| !value.trim().is_empty())
            .ok_or(RetainedFrontierProofError::MalformedManifest { field })
    }
}

fn parse_format(value: &str) -> Result<ColoringProofCertificateFormat, RetainedFrontierProofError> {
    match value {
        "varisat_native" => Ok(ColoringProofCertificateFormat::VarisatNative),
        "lrat" => Ok(ColoringProofCertificateFormat::Lrat),
        _ => Err(RetainedFrontierProofError::UnsupportedFormat {
            format: value.to_string(),
        }),
    }
}

fn parse_u32(value: &str, field: &'static str) -> Result<u32, RetainedFrontierProofError> {
    value
        .parse::<u32>()
        .map_err(|_| RetainedFrontierProofError::MalformedManifest { field })
}

fn parse_u64(value: &str, field: &'static str) -> Result<u64, RetainedFrontierProofError> {
    value
        .parse::<u64>()
        .map_err(|_| RetainedFrontierProofError::MalformedManifest { field })
}

fn read_and_hash(path: &Path) -> Result<(Vec<u8>, String), RetainedFrontierProofError> {
    let mut file = File::open(path).map_err(|error| proof_io_error(path, error))?;
    let mut hasher = Sha256::new();
    let mut bytes = Vec::new();
    let mut buffer = [0_u8; 1024 * 1024];
    loop {
        let read = file
            .read(&mut buffer)
            .map_err(|error| proof_io_error(path, error))?;
        if read == 0 {
            break;
        }
        hasher.update(&buffer[..read]);
        bytes.extend_from_slice(&buffer[..read]);
    }
    Ok((bytes, format!("{:x}", hasher.finalize())))
}

fn proof_io_error(path: &Path, error: IoError) -> RetainedFrontierProofError {
    if error.kind() == std::io::ErrorKind::NotFound {
        RetainedFrontierProofError::ProofUnavailable {
            path: path.to_path_buf(),
        }
    } else {
        RetainedFrontierProofError::ProofIo {
            path: path.to_path_buf(),
            error: error.to_string(),
        }
    }
}

fn resolve_default_path(manifest_path: &str) -> PathBuf {
    let workspace_relative = PathBuf::from(manifest_path);
    if workspace_relative.is_file() {
        return workspace_relative;
    }
    let crate_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let crate_relative = crate_root.join("src/frontier_seeds/heule_510.varisat");
    if crate_relative.is_file() {
        return crate_relative;
    }
    workspace_relative
}
