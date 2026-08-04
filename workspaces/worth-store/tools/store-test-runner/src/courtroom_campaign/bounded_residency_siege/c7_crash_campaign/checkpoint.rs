use super::{process_execution, C7DurabilityCrashSeam};

pub(super) fn verify(
    process: &process_execution::CapturedProcess,
    marker: &str,
    seam: C7DurabilityCrashSeam,
) -> Result<(), String> {
    verify_signature(marker, process.process().get(), seam)
}

pub(super) fn verify_signature(
    marker: &str,
    process: u32,
    seam: C7DurabilityCrashSeam,
) -> Result<(), String> {
    let fields = marker.split_whitespace().collect::<Vec<_>>();
    if fields.len() != 5
        || fields[0] != "C7_COURTROOM_CRASH_CHECKPOINT"
        || fields[1] != seam.label()
        || fields[3] != process.to_string()
    {
        return Err(format!(
            "malformed or foreign C7 crash checkpoint `{marker}`"
        ));
    }
    match expected_signature(seam) {
        ExpectedCheckpointSignature::Media(expected) => {
            verify_media_signature(fields[2], fields[4], expected)
        }
        ExpectedCheckpointSignature::Mutation(checkpoint) => {
            if fields[2] != checkpoint || fields[4] != "-" {
                return Err(format!(
                    "C7 checkpoint `{marker}` does not identify `{checkpoint}` exactly"
                ));
            }
            Ok(())
        }
    }
}

#[derive(Clone, Copy)]
enum ExpectedCheckpointSignature {
    Media(ExpectedMediaSignature),
    Mutation(&'static str),
}

#[derive(Clone, Copy)]
struct ExpectedMediaSignature {
    role: &'static str,
    selected_match: u64,
    requested_bytes_are_positive: bool,
}

fn verify_media_signature(
    checkpoint: &str,
    detail: &str,
    expected: ExpectedMediaSignature,
) -> Result<(), String> {
    let detail = detail.split(':').collect::<Vec<_>>();
    let parsed = detail
        .get(1..)
        .filter(|_| detail.len() == 5)
        .and_then(|numbers| {
            numbers
                .iter()
                .map(|number| number.parse::<u64>())
                .collect::<Result<Vec<_>, _>>()
                .ok()
        });
    let Some(numbers) = parsed else {
        return Err("C7 media checkpoint detail is malformed".to_owned());
    };
    let [role_ordinal, identified_ordinal, requested_bytes, selected_match] = numbers.as_slice()
    else {
        return Err("C7 media checkpoint detail has the wrong arity".to_owned());
    };
    if checkpoint != "MediaEffect"
        || detail[0] != expected.role
        || *role_ordinal == 0
        || *identified_ordinal == 0
        || *selected_match != expected.selected_match
        || (*requested_bytes > 0) != expected.requested_bytes_are_positive
    {
        return Err(format!(
            "C7 media checkpoint does not match role `{}` and selection {}",
            expected.role, expected.selected_match,
        ));
    }
    Ok(())
}

const fn expected_signature(seam: C7DurabilityCrashSeam) -> ExpectedCheckpointSignature {
    match seam {
        C7DurabilityCrashSeam::BeforeWalAppend
        | C7DurabilityCrashSeam::DuringWalAppendPrefix
        | C7DurabilityCrashSeam::AfterWalWriteBeforeBarrier => {
            ExpectedCheckpointSignature::Media(ExpectedMediaSignature {
                role: "positioned_write",
                selected_match: 1,
                requested_bytes_are_positive: true,
            })
        }
        C7DurabilityCrashSeam::AfterWalBarrierBeforeDataDispatch => {
            ExpectedCheckpointSignature::Media(ExpectedMediaSignature {
                role: "synchronize_file_state",
                selected_match: 1,
                requested_bytes_are_positive: false,
            })
        }
        C7DurabilityCrashSeam::DuringDataWritePrefix => {
            ExpectedCheckpointSignature::Media(ExpectedMediaSignature {
                role: "positioned_write",
                selected_match: 2,
                requested_bytes_are_positive: true,
            })
        }
        C7DurabilityCrashSeam::AfterDataSettlementBeforeRootPublication => {
            ExpectedCheckpointSignature::Mutation("AfterDataSettlement")
        }
        C7DurabilityCrashSeam::AfterRootReplacementBeforeNamespaceDurability => {
            ExpectedCheckpointSignature::Media(ExpectedMediaSignature {
                role: "atomic_replace",
                selected_match: 1,
                requested_bytes_are_positive: false,
            })
        }
        C7DurabilityCrashSeam::AfterPhysicalDurabilityBeforeAcknowledgment => {
            ExpectedCheckpointSignature::Mutation("BeforeTerminalFinalization")
        }
    }
}
