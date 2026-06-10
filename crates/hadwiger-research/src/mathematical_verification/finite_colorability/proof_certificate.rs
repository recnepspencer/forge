use std::collections::BTreeMap;

use crate::aspect_authority::ColorabilityAspectRecord;
use crate::domain_artifacts::{
    ColorabilityEncoding, ColorabilityVerification, ColorabilityVerificationPosture, GraphVersion,
    HadwigerCanonicalArtifact, SolverRun, SolverRunPosture,
};
use crate::domain_declarations::ColorabilityDeclaration;
use crate::query_entry::HadwigerResearchHandle;

use super::cnf_encoding::encode_graph_coloring;
use super::varisat_proof::{generate_varisat_native_proof, replay_varisat_native_proof};
use super::{checker_evidence, HadwigerColorabilityError};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ColoringProofCertificateFormat {
    Lrat,
    VarisatNative,
}

impl ColoringProofCertificateFormat {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Lrat => "lrat",
            Self::VarisatNative => "varisat_native",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ColoringProofCertificate {
    format: ColoringProofCertificateFormat,
    cnf_digest: String,
    added_clauses: Vec<Vec<i32>>,
    proof_bytes: Vec<u8>,
}

impl ColoringProofCertificate {
    pub fn lrat_from_bytes(bytes: &[u8]) -> Result<Self, HadwigerColorabilityError> {
        let text = std::str::from_utf8(bytes)
            .map_err(|_| HadwigerColorabilityError::CorruptRefutationCertificate)?;
        let mut cnf_digest = None;
        let mut added_clauses = Vec::new();
        for line in text.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('c') {
                continue;
            }
            if let Some(rest) = line.strip_prefix("cnf ") {
                cnf_digest = Some(rest.trim().to_string());
                continue;
            }
            added_clauses.push(parse_clause(line)?);
        }
        let cnf_digest =
            cnf_digest.ok_or(HadwigerColorabilityError::CorruptRefutationCertificate)?;
        if added_clauses.is_empty() {
            return Err(HadwigerColorabilityError::CorruptRefutationCertificate);
        }
        Ok(Self {
            format: ColoringProofCertificateFormat::Lrat,
            cnf_digest,
            added_clauses,
            proof_bytes: Vec::new(),
        })
    }

    pub fn from_rup_clauses(
        cnf_digest: impl Into<String>,
        added_clauses: Vec<Vec<i32>>,
    ) -> Result<Self, HadwigerColorabilityError> {
        let cnf_digest = cnf_digest.into();
        if cnf_digest.trim().is_empty() || added_clauses.is_empty() {
            return Err(HadwigerColorabilityError::CorruptRefutationCertificate);
        }
        Ok(Self {
            format: ColoringProofCertificateFormat::Lrat,
            cnf_digest,
            added_clauses,
            proof_bytes: Vec::new(),
        })
    }

    pub fn varisat_native_from_bytes(
        cnf_digest: impl Into<String>,
        proof_bytes: Vec<u8>,
    ) -> Result<Self, HadwigerColorabilityError> {
        let cnf_digest = cnf_digest.into();
        if cnf_digest.trim().is_empty() || proof_bytes.is_empty() {
            return Err(HadwigerColorabilityError::CorruptRefutationCertificate);
        }
        Ok(Self {
            format: ColoringProofCertificateFormat::VarisatNative,
            cnf_digest,
            added_clauses: Vec::new(),
            proof_bytes,
        })
    }

    pub fn format(&self) -> ColoringProofCertificateFormat {
        self.format
    }

    pub fn cnf_digest(&self) -> &str {
        &self.cnf_digest
    }

    pub fn added_clauses(&self) -> &[Vec<i32>] {
        &self.added_clauses
    }

    pub fn proof_bytes(&self) -> &[u8] {
        &self.proof_bytes
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ColoringRefutationReplayReport {
    cnf_digest: String,
    replayed_clause_count: usize,
    ended_in_empty_clause: bool,
}

impl ColoringRefutationReplayReport {
    pub fn cnf_digest(&self) -> &str {
        &self.cnf_digest
    }

    pub fn replayed_clause_count(&self) -> usize {
        self.replayed_clause_count
    }

    pub fn ended_in_empty_clause(&self) -> bool {
        self.ended_in_empty_clause
    }
}

pub fn generate_k_colorability_certificate_with_varisat_checked(
    graph_version: &GraphVersion,
    color_count: u32,
) -> Result<ColoringProofCertificate, HadwigerColorabilityError> {
    if color_count == 0 {
        return Err(HadwigerColorabilityError::ZeroColorCount);
    }
    if graph_version.vertices().is_empty() {
        return Err(HadwigerColorabilityError::EmptyGraph);
    }
    let (variable_map, clauses) = encode_graph_coloring(graph_version, color_count);
    let encoding = ColorabilityEncoding::checked(
        graph_version.reference(),
        color_count,
        variable_map,
        clauses.clone(),
    )?;
    let proof_bytes = generate_varisat_native_proof(&clauses)?;
    replay_varisat_native_proof(&clauses, &proof_bytes)?;
    ColoringProofCertificate::varisat_native_from_bytes(encoding.cnf_digest_token(), proof_bytes)
}

pub fn verify_k_colorability_with_certificate_checked(
    handle: &HadwigerResearchHandle,
    graph_version: &GraphVersion,
    color_count: u32,
    certificate: ColoringProofCertificate,
) -> Result<super::KColorabilityVerificationChecked, HadwigerColorabilityError> {
    if color_count == 0 {
        return Err(HadwigerColorabilityError::ZeroColorCount);
    }
    if graph_version.vertices().is_empty() {
        return Err(HadwigerColorabilityError::EmptyGraph);
    }
    let declared = handle.declare_checked(ColorabilityDeclaration::new(
        graph_version.version_id(),
        color_count,
    ));
    let query_declaration_reference = super::admitted_declaration_reference(declared)
        .ok_or(HadwigerColorabilityError::QueryDeclarationNotAdmitted)?;
    let (variable_map, clauses) = encode_graph_coloring(graph_version, color_count);
    let encoding = ColorabilityEncoding::checked(
        graph_version.reference(),
        color_count,
        variable_map,
        clauses.clone(),
    )?;
    let replay =
        replay_refutation_certificate(&clauses, encoding.cnf_digest_token(), &certificate)?;
    let query_identity = format!(
        "{}:{}:{}",
        handle.handle_identity_digest(),
        query_declaration_reference.declaration_digest(),
        replay.replayed_clause_count()
    );
    let solver_run = SolverRun::checked(
        encoding.reference(),
        query_declaration_reference.clone(),
        "external-certificate",
        certificate.format().as_str(),
        SolverRunPosture::Unsat,
        Vec::new(),
        checker_evidence("sat-proof-certificate", &query_identity)?,
    )?;
    let verification = ColorabilityVerification::checked(
        solver_run.reference(),
        graph_version.reference(),
        query_declaration_reference,
        "hadwiger.rup_lrat_replay",
        "0.1.0",
        ColorabilityVerificationPosture::UnsatVerified,
        color_count,
        checker_evidence("colorability-proof-replay", &query_identity)?,
    )?;
    let aspect = ColorabilityAspectRecord::admitted_checked(
        graph_version.reference(),
        color_count,
        format!(
            "RUP/LRAT-shaped certificate replayed {} clauses",
            replay.replayed_clause_count()
        ),
    )?;
    Ok(super::KColorabilityVerificationChecked::new(
        encoding,
        solver_run,
        verification,
        aspect,
    ))
}

fn replay_refutation_certificate(
    clauses: &[Vec<i32>],
    cnf_digest: &str,
    certificate: &ColoringProofCertificate,
) -> Result<ColoringRefutationReplayReport, HadwigerColorabilityError> {
    match certificate.format() {
        ColoringProofCertificateFormat::Lrat => {
            replay_rup_refutation(clauses, cnf_digest, certificate)
        }
        ColoringProofCertificateFormat::VarisatNative => {
            if certificate.cnf_digest() != cnf_digest {
                return Err(HadwigerColorabilityError::CertificateDigestMismatch);
            }
            replay_varisat_native_proof(clauses, certificate.proof_bytes())?;
            Ok(ColoringRefutationReplayReport {
                cnf_digest: cnf_digest.to_string(),
                replayed_clause_count: clauses.len(),
                ended_in_empty_clause: true,
            })
        }
    }
}

fn replay_rup_refutation(
    clauses: &[Vec<i32>],
    cnf_digest: &str,
    certificate: &ColoringProofCertificate,
) -> Result<ColoringRefutationReplayReport, HadwigerColorabilityError> {
    if certificate.cnf_digest() != cnf_digest {
        return Err(HadwigerColorabilityError::CertificateDigestMismatch);
    }
    let mut proof_clauses = clauses.to_vec();
    for clause in certificate.added_clauses() {
        if !rup_accepts(&proof_clauses, clause)? {
            return Err(HadwigerColorabilityError::CorruptRefutationCertificate);
        }
        proof_clauses.push(clause.clone());
    }
    let ended_in_empty_clause = certificate
        .added_clauses()
        .last()
        .is_some_and(Vec::is_empty);
    if !ended_in_empty_clause {
        return Err(HadwigerColorabilityError::CorruptRefutationCertificate);
    }
    Ok(ColoringRefutationReplayReport {
        cnf_digest: cnf_digest.to_string(),
        replayed_clause_count: certificate.added_clauses().len(),
        ended_in_empty_clause,
    })
}

fn rup_accepts(clauses: &[Vec<i32>], clause: &[i32]) -> Result<bool, HadwigerColorabilityError> {
    let mut assignment = BTreeMap::new();
    for literal in clause {
        let variable = literal.abs();
        let value = *literal < 0;
        if assignment.insert(variable, value).is_some() {
            return Err(HadwigerColorabilityError::CorruptRefutationCertificate);
        }
    }
    Ok(unit_propagates_to_conflict(clauses, &mut assignment))
}

fn unit_propagates_to_conflict(clauses: &[Vec<i32>], assignment: &mut BTreeMap<i32, bool>) -> bool {
    loop {
        let mut changed = false;
        for clause in clauses {
            let mut unassigned = None;
            let mut satisfied = false;
            for literal in clause {
                match assignment.get(&literal.abs()) {
                    Some(value) if (*literal > 0 && *value) || (*literal < 0 && !*value) => {
                        satisfied = true;
                        break;
                    }
                    Some(_) => {}
                    None => unassigned = Some(*literal),
                }
            }
            if satisfied {
                continue;
            }
            match unassigned {
                None => return true,
                Some(literal) => {
                    if assignment.insert(literal.abs(), literal > 0).is_none() {
                        changed = true;
                    }
                }
            }
        }
        if !changed {
            return false;
        }
    }
}

fn parse_clause(line: &str) -> Result<Vec<i32>, HadwigerColorabilityError> {
    let mut clause = Vec::new();
    for token in line.split_whitespace() {
        let literal = token
            .parse::<i32>()
            .map_err(|_| HadwigerColorabilityError::CorruptRefutationCertificate)?;
        if literal == 0 {
            return Ok(clause);
        }
        clause.push(literal);
    }
    Err(HadwigerColorabilityError::CorruptRefutationCertificate)
}
