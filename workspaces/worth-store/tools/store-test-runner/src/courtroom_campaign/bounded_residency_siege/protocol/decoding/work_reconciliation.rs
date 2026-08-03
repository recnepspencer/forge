use std::collections::HashMap;
use std::num::NonZeroU64;

use super::{boolean, fields, fixed_hex, number};
use crate::courtroom_campaign::bounded_residency_siege::protocol::{
    BoundedResidencyMediaRole, BoundedResidencySchedulerEvidenceClass,
    BoundedResidencySchedulerProfile, BoundedResidencySignalAspectRole,
    BoundedResidencySignalBindingObservation, BoundedResidencySignalFamily,
    BoundedResidencySignalFamilySet, BoundedResidencySignalLineageObservation,
    BoundedResidencySignalSettlement, BoundedResidencyWorkEffectFate, BoundedResidencyWorkFamily,
    BoundedResidencyWorkReconciliationObservation, BoundedResidencyWorkRecordObservation,
    BoundedResidencyWorkRecovery, BoundedResidencyWorkRouteObservation,
    BoundedResidencyWorkTerminalFate,
};

const HEADER: &str = "BOUNDED_RESIDENCY_WORK_RECONCILIATION ";
const RECORD: &str = "BOUNDED_RESIDENCY_WORK_RECORD ";
const ROUTE: &str = "BOUNDED_RESIDENCY_WORK_ROUTE ";
const SIGNAL_BINDING: &str = "BOUNDED_RESIDENCY_SIGNAL_BINDING ";

pub(in crate::courtroom_campaign::bounded_residency_siege::protocol) fn parse(
    lines: &[String],
) -> Result<BoundedResidencyWorkReconciliationObservation, String> {
    let header = fields(lines, HEADER, 14)?;
    let declared_bindings = number::<usize>(header[12], "Signal binding count")?;
    let declared_records = number::<usize>(header[13], "physical work record count")?;
    let signal_bindings = lines
        .iter()
        .filter(|line| line.starts_with(SIGNAL_BINDING))
        .map(|line| parse_signal_binding(line))
        .collect::<Result<Vec<_>, _>>()?;
    if signal_bindings.len() != declared_bindings {
        return Err(format!(
            "physical work reconciliation declared {declared_bindings} Signal bindings but emitted {}",
            signal_bindings.len()
        ));
    }
    let mut routes = lines
        .iter()
        .filter(|line| line.starts_with(ROUTE))
        .map(|line| parse_route(line))
        .collect::<Result<HashMap<_, _>, _>>()?;
    if routes.len() != declared_records {
        return Err("physical work reconciliation route cardinality drifted".to_owned());
    }
    let records = lines
        .iter()
        .filter(|line| line.starts_with(RECORD))
        .map(|line| parse_record(line, &mut routes))
        .collect::<Result<Vec<_>, _>>()?;
    if records.len() != declared_records || !routes.is_empty() {
        return Err(format!(
            "physical work reconciliation declared {declared_records} records but emitted {}",
            records.len()
        ));
    }
    Ok(BoundedResidencyWorkReconciliationObservation {
        causal_overflow: number(header[1], "causal evidence overflow")?,
        terminal_overflow: number(header[2], "terminal evidence overflow")?,
        safe_evidence_elided: number(header[3], "safe terminal evidence elision")?,
        faults: number(header[4], "physical fault count")?,
        source_loads: number(header[5], "physical source-load count")?,
        exact_writebacks: number(header[6], "exact writeback count")?,
        identified_metadata_reads: number(header[7], "identified metadata reads")?,
        identified_positioned_reads: number(header[8], "identified positioned reads")?,
        identified_positioned_writes: number(header[9], "identified positioned writes")?,
        settled_terminal_fates: number(header[10], "settled terminal Store fates")?,
        continued_terminal_fates: number(
            header[11],
            "continued-after-cancellation terminal Store fates",
        )?,
        signal_bindings: signal_bindings.into_boxed_slice(),
        records: records.into_boxed_slice(),
    })
}

fn parse_signal_binding(line: &str) -> Result<BoundedResidencySignalBindingObservation, String> {
    let value = line.split_whitespace().collect::<Vec<_>>();
    if value.len() != 14 || value[0] != SIGNAL_BINDING.trim_end() {
        return Err(format!(
            "malformed Courtroom C Signal binding observation `{line}`"
        ));
    }
    Ok(BoundedResidencySignalBindingObservation {
        digest: fixed_hex(value[1], "Signal binding digest")?,
        aspect_key: value[2].to_owned(),
        role: signal_role(value[3])?,
        families: BoundedResidencySignalFamilySet {
            read_fault: boolean(value[4], "read-fault family membership")?,
            exact_writeback: boolean(value[5], "exact-writeback family membership")?,
            publication: boolean(value[6], "publication family membership")?,
            lifecycle: boolean(value[7], "lifecycle family membership")?,
            wal_append: boolean(value[8], "WAL-append family membership")?,
            durability_barrier: boolean(value[9], "durability-barrier family membership")?,
            checkpoint_capture: boolean(value[10], "checkpoint-capture family membership")?,
            root_publication: boolean(value[11], "root-publication family membership")?,
            wal_reclamation: boolean(value[12], "WAL-reclamation family membership")?,
        },
        partition: (value[13] != "none").then(|| value[13].to_owned()),
    })
}

fn signal_role(encoded: &str) -> Result<BoundedResidencySignalAspectRole, String> {
    match encoded {
        "dependency" => Ok(BoundedResidencySignalAspectRole::Dependency),
        "output" => Ok(BoundedResidencySignalAspectRole::Output),
        "dependency-and-output" => Ok(BoundedResidencySignalAspectRole::DependencyAndOutput),
        _ => Err("Signal binding named an unknown aspect role".to_owned()),
    }
}

fn parse_record(
    line: &str,
    routes: &mut HashMap<u64, BoundedResidencyWorkRouteObservation>,
) -> Result<BoundedResidencyWorkRecordObservation, String> {
    let value = line.split_whitespace().collect::<Vec<_>>();
    if value.len() != 11 || value[0] != RECORD.trim_end() {
        return Err(format!(
            "malformed Courtroom C physical work record `{line}`"
        ));
    }
    let runtime = nonzero(value[2], "physical work runtime")?;
    let generation = nonzero(value[3], "physical work generation")?;
    let operation = nonzero(value[4], "physical work operation")?;
    Ok(BoundedResidencyWorkRecordObservation {
        store: fixed_hex(value[1], "physical work Store identity")?,
        runtime,
        generation,
        operation,
        family: family(value[5])?,
        backend_operation: nonzero(value[6], "backend operation")?,
        backend_role: media_role(value[7])?,
        effect_fate: effect_fate(value[8])?,
        recovery: recovery(value[9])?,
        route: routes.remove(&operation).ok_or_else(|| {
            "physical work reconciliation record omitted its causal route".to_owned()
        })?,
        terminal: terminal(value[10])?,
    })
}

fn media_role(encoded: &str) -> Result<BoundedResidencyMediaRole, String> {
    match encoded {
        "create-new" => Ok(BoundedResidencyMediaRole::CreateNew),
        "positioned-read" => Ok(BoundedResidencyMediaRole::PositionedRead),
        "positioned-write" => Ok(BoundedResidencyMediaRole::PositionedWrite),
        "read-metadata" => Ok(BoundedResidencyMediaRole::ReadMetadata),
        "synchronize-file-state" => Ok(BoundedResidencyMediaRole::SynchronizeFileState),
        "synchronize-directory-publication" => {
            Ok(BoundedResidencyMediaRole::SynchronizeDirectoryPublication)
        }
        "atomic-replace" => Ok(BoundedResidencyMediaRole::AtomicReplace),
        "delete" => Ok(BoundedResidencyMediaRole::Delete),
        _ => Err("physical work record named an unknown backend media role".to_owned()),
    }
}

fn parse_route(line: &str) -> Result<(u64, BoundedResidencyWorkRouteObservation), String> {
    let value = line.split_whitespace().collect::<Vec<_>>();
    if value.len() != 13 || value[0] != ROUTE.trim_end() {
        return Err(format!(
            "malformed Courtroom C physical work route `{line}`"
        ));
    }
    let operation = nonzero(value[1], "routed physical work operation")?;
    Ok((
        operation,
        BoundedResidencyWorkRouteObservation {
            signal: lineage(value[2], "Signal request lineage")?,
            predecessor: if value[3] == "none" {
                None
            } else {
                Some(lineage(value[3], "Signal predecessor lineage")?)
            },
            signal_attempt: number(value[4], "Signal attempt")?,
            signal_family: signal_family(value[5])?,
            signal_binding: fixed_hex(value[6], "Signal binding")?,
            scheduler_profile: scheduler_profile(value[7])?,
            scheduler_evidence_class: scheduler_evidence_class(value[8])?,
            scheduler_grouped_writes: number(value[9], "scheduler grouped writes")?,
            scheduler_primary_requirement: number(
                value[10],
                "scheduler primary backend requirement",
            )?,
            scheduler_secondary_present: boolean(value[11], "scheduler secondary presence")?,
            signal_settlement: signal_settlement(value[12])?,
        },
    ))
}

fn lineage(encoded: &str, label: &str) -> Result<BoundedResidencySignalLineageObservation, String> {
    let fields = encoded.split(':').collect::<Vec<_>>();
    if fields.len() != 4 {
        return Err(format!("{label} must contain four exact coordinates"));
    }
    Ok(BoundedResidencySignalLineageObservation {
        request: number(fields[0], label)?,
        generation: number(fields[1], label)?,
        branch: number(fields[2], label)?,
        restore_epoch: number(fields[3], label)?,
    })
}

fn signal_family(encoded: &str) -> Result<BoundedResidencySignalFamily, String> {
    match encoded {
        "read-fault" => Ok(BoundedResidencySignalFamily::ReadFault),
        "exact-writeback" => Ok(BoundedResidencySignalFamily::ExactWriteback),
        "publication" => Ok(BoundedResidencySignalFamily::Publication),
        "lifecycle" => Ok(BoundedResidencySignalFamily::Lifecycle),
        "wal-append" => Ok(BoundedResidencySignalFamily::WalAppend),
        "durability-barrier" => Ok(BoundedResidencySignalFamily::DurabilityBarrier),
        "checkpoint-capture" => Ok(BoundedResidencySignalFamily::CheckpointCapture),
        "root-publication" => Ok(BoundedResidencySignalFamily::RootPublication),
        "wal-reclamation" => Ok(BoundedResidencySignalFamily::WalReclamation),
        _ => Err("physical work route named an unknown Signal family".to_owned()),
    }
}

fn scheduler_profile(encoded: &str) -> Result<BoundedResidencySchedulerProfile, String> {
    match encoded {
        "simulated-strict-durable" => Ok(BoundedResidencySchedulerProfile::SimulatedStrictDurable),
        "posix-file-fsync-dir-sync" => Ok(BoundedResidencySchedulerProfile::PosixFileFsyncDirSync),
        "windows-flush-file-buffers" => {
            Ok(BoundedResidencySchedulerProfile::WindowsFlushFileBuffers)
        }
        "mmap-flush-not-durability-certified" => {
            Ok(BoundedResidencySchedulerProfile::MmapFlushNotDurabilityCertified)
        }
        "adversarial-lost-flush" => Ok(BoundedResidencySchedulerProfile::AdversarialLostFlush),
        "adversarial-reordered-flush" => {
            Ok(BoundedResidencySchedulerProfile::AdversarialReorderedFlush)
        }
        _ => Err("physical work route named an unknown scheduler profile".to_owned()),
    }
}

fn scheduler_evidence_class(
    encoded: &str,
) -> Result<BoundedResidencySchedulerEvidenceClass, String> {
    match encoded {
        "declared-by-config" => Ok(BoundedResidencySchedulerEvidenceClass::DeclaredByConfig),
        "observed-by-probe" => Ok(BoundedResidencySchedulerEvidenceClass::ObservedByProbe),
        "established-by-filesystem-admission" => {
            Ok(BoundedResidencySchedulerEvidenceClass::EstablishedByFilesystemAdmission)
        }
        "externally-guaranteed" => Ok(BoundedResidencySchedulerEvidenceClass::ExternallyGuaranteed),
        "unverifiable-assumption" => {
            Ok(BoundedResidencySchedulerEvidenceClass::UnverifiableAssumption)
        }
        "certified-backend-profile" => {
            Ok(BoundedResidencySchedulerEvidenceClass::CertifiedBackendProfile)
        }
        _ => Err("physical work route named an unknown scheduler evidence class".to_owned()),
    }
}

fn signal_settlement(encoded: &str) -> Result<BoundedResidencySignalSettlement, String> {
    match encoded {
        "committed" => Ok(BoundedResidencySignalSettlement::Committed),
        "reconciled-from-physical-truth" => {
            Ok(BoundedResidencySignalSettlement::ReconciledFromPhysicalTruth)
        }
        "derived-state-unavailable" => {
            Ok(BoundedResidencySignalSettlement::DerivedStateUnavailable)
        }
        _ => Err("physical work route named an unknown Signal settlement".to_owned()),
    }
}

fn nonzero(encoded: &str, label: &str) -> Result<u64, String> {
    number::<NonZeroU64>(encoded, label).map(NonZeroU64::get)
}

fn family(encoded: &str) -> Result<BoundedResidencyWorkFamily, String> {
    match encoded {
        "artifact-metadata-read" => Ok(BoundedResidencyWorkFamily::ArtifactMetadataRead),
        "artifact-range-read" => Ok(BoundedResidencyWorkFamily::ArtifactRangeRead),
        "artifact-range-write" => Ok(BoundedResidencyWorkFamily::ArtifactRangeWrite),
        "artifact-publication" => Ok(BoundedResidencyWorkFamily::ArtifactPublication),
        "wal-append" => Ok(BoundedResidencyWorkFamily::WalAppend),
        "durability-barrier" => Ok(BoundedResidencyWorkFamily::DurabilityBarrier),
        "checkpoint-capture" => Ok(BoundedResidencyWorkFamily::CheckpointCapture),
        "root-publication" => Ok(BoundedResidencyWorkFamily::RootPublication),
        "wal-reclamation" => Ok(BoundedResidencyWorkFamily::WalReclamation),
        _ => Err("physical work record named an unknown operation family".to_owned()),
    }
}

fn effect_fate(encoded: &str) -> Result<BoundedResidencyWorkEffectFate, String> {
    match encoded {
        "read-completed" => Ok(BoundedResidencyWorkEffectFate::ReadCompleted),
        "write-completed" => Ok(BoundedResidencyWorkEffectFate::WriteCompleted),
        "publication-completed" => Ok(BoundedResidencyWorkEffectFate::PublicationCompleted),
        "checkpoint-completed" => Ok(BoundedResidencyWorkEffectFate::CheckpointCompleted),
        "wal-reclamation-completed" => Ok(BoundedResidencyWorkEffectFate::WalReclamationCompleted),
        _ => Err("physical work record named a non-successful effect fate".to_owned()),
    }
}

fn recovery(encoded: &str) -> Result<BoundedResidencyWorkRecovery, String> {
    match encoded {
        "no-effect" => Ok(BoundedResidencyWorkRecovery::NoEffect),
        "continue-settlement" => Ok(BoundedResidencyWorkRecovery::ContinueSettlement),
        _ => Err("physical work record named an inexact recovery posture".to_owned()),
    }
}

fn terminal(encoded: &str) -> Result<BoundedResidencyWorkTerminalFate, String> {
    match encoded {
        "settled" => Ok(BoundedResidencyWorkTerminalFate::Settled),
        "continued-after-consumer-cancellation" => {
            Ok(BoundedResidencyWorkTerminalFate::ContinuedAfterConsumerCancellation)
        }
        _ => Err("physical work record named an unknown terminal Store fate".to_owned()),
    }
}
