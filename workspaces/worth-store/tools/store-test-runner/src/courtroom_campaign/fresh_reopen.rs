//! Strict parser for fresh-process Store reopen evidence.

use std::num::NonZeroU32;

use worth_store::physical_runtime::{
    PhysicalWorkFreshReopenEvidence, PhysicalWorkFreshReopenIdentity,
    PhysicalWorkFreshReopenPosture,
};

use super::process_execution::CapturedProcess;

pub(super) fn parse(process: &CapturedProcess) -> Result<PhysicalWorkFreshReopenEvidence, String> {
    let marker = exactly_one(process.stdout(), "C5_1_COURTROOM_REOPEN ")?;
    let fields = marker.split_whitespace().collect::<Vec<_>>();
    if fields.len() != 10 {
        return Err(format!("malformed reopen marker `{marker}`"));
    }
    let reported = NonZeroU32::new(number(fields[1], "reopen process")?)
        .ok_or_else(|| "reopen process cannot be zero".to_owned())?;
    if reported != process.process() {
        return Err("reopener reported a foreign process identity".into());
    }
    let identity = PhysicalWorkFreshReopenIdentity::new(
        reported,
        fixed_hex(fields[2], "reopen Store identity")?,
        number(fields[3], "reopen runtime")?,
        number(fields[4], "reopen generation")?,
        number(fields[5], "reopen records")?,
    )
    .map_err(|denial| format!("reopen identity evidence denied: {denial:?}"))?;
    let posture = PhysicalWorkFreshReopenPosture::new(
        boolean(fields[6], "reopen residue")?,
        boolean(fields[7], "recovery damage")?,
        number(fields[8], "recovery count")?,
        boolean(fields[9], "inspection posture")?,
    );
    PhysicalWorkFreshReopenEvidence::new(identity, posture)
        .map_err(|denial| format!("reopen evidence denied: {denial:?}"))
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
    if encoded.len() != N * 2 || !encoded.is_ascii() {
        return Err(format!(
            "{label} must contain exactly {N} hexadecimal bytes"
        ));
    }
    let mut bytes = [0_u8; N];
    for (index, byte) in bytes.iter_mut().enumerate() {
        let offset = index * 2;
        *byte = u8::from_str_radix(&encoded[offset..offset + 2], 16)
            .map_err(|_| format!("{label} contains non-hexadecimal data"))?;
    }
    Ok(bytes)
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
