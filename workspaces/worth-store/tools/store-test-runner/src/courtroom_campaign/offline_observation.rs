//! Strict parser for the separately linked offline observer process.

use std::num::NonZeroU32;

use worth_store::physical_runtime::{
    PhysicalWorkArtifactBinding, PhysicalWorkEvidenceDigest, PhysicalWorkHostileArtifactEvidence,
    PhysicalWorkHostileCurrentTruth,
};

use super::process_execution::CapturedProcess;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct OfflineArtifactObservation {
    path: Box<str>,
    byte_length: u64,
    digest: [u8; 32],
    prefix: Box<[u8]>,
    recovery_obligation: bool,
}

impl OfflineArtifactObservation {
    #[cfg(test)]
    pub(super) fn for_test(
        path: &str,
        byte_length: u64,
        digest: [u8; 32],
        prefix: &[u8],
        recovery_obligation: bool,
    ) -> Self {
        Self {
            path: path.into(),
            byte_length,
            digest,
            prefix: prefix.into(),
            recovery_obligation,
        }
    }

    pub(super) fn path(&self) -> &str {
        &self.path
    }

    pub(super) const fn byte_length(&self) -> u64 {
        self.byte_length
    }

    pub(super) const fn digest(&self) -> [u8; 32] {
        self.digest
    }

    pub(super) fn prefix(&self) -> &[u8] {
        &self.prefix
    }

    pub(super) const fn is_recovery_obligation(&self) -> bool {
        self.recovery_obligation
    }

    pub(super) fn lower(&self) -> Result<PhysicalWorkHostileArtifactEvidence, String> {
        let digest = evidence_digest(self.digest, &self.path)?;
        let binding = PhysicalWorkArtifactBinding::new(self.path.clone(), self.byte_length, digest)
            .map_err(|denial| format!("offline artifact binding denied: {denial:?}"))?;
        PhysicalWorkHostileArtifactEvidence::new(
            binding,
            self.prefix.clone(),
            self.recovery_obligation,
        )
        .map_err(|denial| format!("offline artifact evidence denied: {denial:?}"))
    }
}

pub(super) struct OfflineObservation {
    process: NonZeroU32,
    current: PhysicalWorkHostileCurrentTruth,
    artifacts: Box<[OfflineArtifactObservation]>,
    recovery_obligations: u64,
}

impl OfflineObservation {
    pub(super) const fn process(&self) -> NonZeroU32 {
        self.process
    }

    pub(super) const fn current(&self) -> PhysicalWorkHostileCurrentTruth {
        self.current
    }

    pub(super) fn artifacts(&self) -> &[OfflineArtifactObservation] {
        &self.artifacts
    }

    pub(super) const fn recovery_obligations(&self) -> u64 {
        self.recovery_obligations
    }

    pub(super) fn lower_artifacts(
        &self,
    ) -> Result<Vec<PhysicalWorkHostileArtifactEvidence>, String> {
        self.artifacts
            .iter()
            .map(OfflineArtifactObservation::lower)
            .collect()
    }
}

pub(super) fn parse(process: &CapturedProcess) -> Result<OfflineObservation, String> {
    if let Some(denial) = process
        .stdout()
        .iter()
        .find(|line| line.starts_with("C5_1_OFFLINE_DENIED"))
    {
        return Err(format!("offline observer denied current truth: {denial}"));
    }
    let process_marker = exactly_one(process.stdout(), "C5_1_OFFLINE_PROCESS ")?;
    let reported = parse_process(process_marker)?;
    if reported != process.process() {
        return Err("offline observer reported a foreign process identity".into());
    }
    let current = parse_current(exactly_one(process.stdout(), "C5_1_OFFLINE_CURRENT ")?)?;
    let artifacts = process
        .stdout()
        .iter()
        .filter(|line| line.starts_with("C5_1_OFFLINE_ARTIFACT "))
        .map(|line| parse_artifact(line))
        .collect::<Result<Vec<_>, _>>()?;
    let (count, total_bytes, recovery_obligations) =
        parse_summary(exactly_one(process.stdout(), "C5_1_OFFLINE_SUMMARY ")?)?;
    if count != artifacts.len() as u64
        || total_bytes
            != artifacts
                .iter()
                .map(OfflineArtifactObservation::byte_length)
                .sum::<u64>()
        || recovery_obligations
            != artifacts
                .iter()
                .filter(|artifact| artifact.is_recovery_obligation())
                .count() as u64
    {
        return Err("offline observer summary does not match its artifact observations".into());
    }
    Ok(OfflineObservation {
        process: reported,
        current,
        artifacts: artifacts.into_boxed_slice(),
        recovery_obligations,
    })
}

fn parse_process(marker: &str) -> Result<NonZeroU32, String> {
    let fields = marker.split_whitespace().collect::<Vec<_>>();
    if fields.len() != 2 {
        return Err(format!("malformed offline process marker `{marker}`"));
    }
    NonZeroU32::new(number(fields[1], "offline process")?)
        .ok_or_else(|| "offline process cannot be zero".to_owned())
}

fn parse_current(marker: &str) -> Result<PhysicalWorkHostileCurrentTruth, String> {
    let fields = marker.split_whitespace().collect::<Vec<_>>();
    if fields.len() != 7 || fields[1] != "accepted" {
        return Err(format!(
            "offline current truth was not accepted: `{marker}`"
        ));
    }
    PhysicalWorkHostileCurrentTruth::new(
        fixed_hex(fields[2], "Store identity")?,
        number(fields[3], "root generation")?,
        number(fields[4], "record count")?,
        number(fields[5], "payload byte count")?,
        evidence_digest(fixed_hex(fields[6], "payload digest")?, "payload digest")?,
    )
    .map_err(|denial| format!("offline current evidence denied: {denial:?}"))
}

fn parse_artifact(marker: &str) -> Result<OfflineArtifactObservation, String> {
    let fields = marker.split_whitespace().collect::<Vec<_>>();
    if fields.len() != 6 {
        return Err(format!("malformed offline artifact marker `{marker}`"));
    }
    let path = String::from_utf8(hex(fields[1], "artifact path")?)
        .map_err(|_| "offline artifact path is not UTF-8".to_owned())?;
    Ok(OfflineArtifactObservation {
        path: path.into_boxed_str(),
        byte_length: number(fields[2], "artifact byte length")?,
        digest: fixed_hex(fields[3], "artifact digest")?,
        prefix: hex(fields[4], "artifact prefix")?.into_boxed_slice(),
        recovery_obligation: boolean(fields[5], "recovery obligation")?,
    })
}

fn parse_summary(marker: &str) -> Result<(u64, u64, u64), String> {
    let fields = marker.split_whitespace().collect::<Vec<_>>();
    if fields.len() != 4 {
        return Err(format!("malformed offline summary `{marker}`"));
    }
    Ok((
        number(fields[1], "artifact count")?,
        number(fields[2], "total bytes")?,
        number(fields[3], "recovery count")?,
    ))
}

fn exactly_one<'lines>(lines: &'lines [String], prefix: &str) -> Result<&'lines str, String> {
    let matching = lines
        .iter()
        .filter(|line| line.starts_with(prefix))
        .collect::<Vec<_>>();
    match matching.as_slice() {
        [line] => Ok(line),
        _ => Err(format!(
            "expected one `{prefix}` marker, found {}",
            matching.len()
        )),
    }
}

fn fixed_hex<const N: usize>(encoded: &str, label: &str) -> Result<[u8; N], String> {
    hex(encoded, label)?
        .try_into()
        .map_err(|_| format!("{label} must contain exactly {} bytes", N))
}

fn hex(encoded: &str, label: &str) -> Result<Vec<u8>, String> {
    if !encoded.len().is_multiple_of(2) || !encoded.is_ascii() {
        return Err(format!("{label} is not even-length ASCII hexadecimal"));
    }
    (0..encoded.len())
        .step_by(2)
        .map(|offset| {
            u8::from_str_radix(&encoded[offset..offset + 2], 16)
                .map_err(|_| format!("{label} contains non-hexadecimal data"))
        })
        .collect()
}

fn evidence_digest(bytes: [u8; 32], label: &str) -> Result<PhysicalWorkEvidenceDigest, String> {
    PhysicalWorkEvidenceDigest::new(bytes)
        .ok_or_else(|| format!("{label} cannot be an all-zero digest"))
}

fn boolean(encoded: &str, label: &str) -> Result<bool, String> {
    match encoded {
        "true" => Ok(true),
        "false" => Ok(false),
        _ => Err(format!("{label} must be `true` or `false`")),
    }
}

fn number<T>(encoded: &str, label: &str) -> Result<T, String>
where
    T: std::str::FromStr,
{
    encoded
        .parse()
        .map_err(|_| format!("{label} is not a valid number"))
}
