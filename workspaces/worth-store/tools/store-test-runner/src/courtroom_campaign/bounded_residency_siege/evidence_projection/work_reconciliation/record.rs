use serde::ser::{Serialize, SerializeMap, SerializeSeq, Serializer};

use super::super::super::protocol::{
    BoundedResidencyMediaRole, BoundedResidencySchedulerEvidenceClass,
    BoundedResidencySchedulerProfile, BoundedResidencySignalFamily,
    BoundedResidencySignalLineageObservation, BoundedResidencySignalSettlement,
    BoundedResidencyWorkEffectFate, BoundedResidencyWorkFamily,
    BoundedResidencyWorkRecordObservation, BoundedResidencyWorkRecovery,
    BoundedResidencyWorkTerminalFate,
};
use crate::physical_work_evidence::hex;

pub(super) struct Records<'evidence>(pub(super) &'evidence [BoundedResidencyWorkRecordObservation]);

impl Serialize for Records<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut sequence = serializer.serialize_seq(Some(self.0.len()))?;
        for record in self.0 {
            sequence.serialize_element(&Record(record))?;
        }
        sequence.end()
    }
}

struct Record<'evidence>(&'evidence BoundedResidencyWorkRecordObservation);

impl Serialize for Record<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let record = self.0;
        let mut map = serializer.serialize_map(Some(11))?;
        map.serialize_entry("store", &hex(&record.store))?;
        map.serialize_entry("runtime", &record.runtime)?;
        map.serialize_entry("generation", &record.generation)?;
        map.serialize_entry("operation", &record.operation)?;
        map.serialize_entry("family", family(record.family))?;
        map.serialize_entry("backend_operation", &record.backend_operation)?;
        map.serialize_entry("backend_role", media_role(record.backend_role))?;
        map.serialize_entry("effect_fate", effect_fate(record.effect_fate))?;
        map.serialize_entry("recovery", recovery(record.recovery))?;
        map.serialize_entry("route", &Route(record))?;
        map.serialize_entry("terminal", terminal(record.terminal))?;
        map.end()
    }
}

struct Route<'evidence>(&'evidence BoundedResidencyWorkRecordObservation);

impl Serialize for Route<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let route = self.0.route;
        let predecessor = route.predecessor.map(Lineage);
        let mut map = serializer.serialize_map(Some(4))?;
        map.serialize_entry("signal", &Signal(self.0))?;
        map.serialize_entry("predecessor", &predecessor)?;
        map.serialize_entry("scheduler", &Scheduler(self.0))?;
        map.serialize_entry(
            "signal_settlement",
            signal_settlement(route.signal_settlement),
        )?;
        map.end()
    }
}

struct Signal<'evidence>(&'evidence BoundedResidencyWorkRecordObservation);

impl Serialize for Signal<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let route = self.0.route;
        let signal = route.signal;
        let mut map = serializer.serialize_map(Some(7))?;
        map.serialize_entry("request", &signal.request)?;
        map.serialize_entry("generation", &signal.generation)?;
        map.serialize_entry("branch", &signal.branch)?;
        map.serialize_entry("restore_epoch", &signal.restore_epoch)?;
        map.serialize_entry("attempt", &route.signal_attempt)?;
        map.serialize_entry("family", signal_family(route.signal_family))?;
        map.serialize_entry("binding", &hex(&route.signal_binding))?;
        map.end()
    }
}

struct Lineage(BoundedResidencySignalLineageObservation);

impl Serialize for Lineage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let lineage = self.0;
        let mut map = serializer.serialize_map(Some(4))?;
        map.serialize_entry("request", &lineage.request)?;
        map.serialize_entry("generation", &lineage.generation)?;
        map.serialize_entry("branch", &lineage.branch)?;
        map.serialize_entry("restore_epoch", &lineage.restore_epoch)?;
        map.end()
    }
}

struct Scheduler<'evidence>(&'evidence BoundedResidencyWorkRecordObservation);

impl Serialize for Scheduler<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let route = self.0.route;
        let mut map = serializer.serialize_map(Some(5))?;
        map.serialize_entry("profile", scheduler_profile(route.scheduler_profile))?;
        map.serialize_entry(
            "evidence_class",
            scheduler_evidence_class(route.scheduler_evidence_class),
        )?;
        map.serialize_entry("grouped_writes", &route.scheduler_grouped_writes)?;
        map.serialize_entry("primary_requirement", &route.scheduler_primary_requirement)?;
        map.serialize_entry("secondary_present", &route.scheduler_secondary_present)?;
        map.end()
    }
}

const fn media_role(role: BoundedResidencyMediaRole) -> &'static str {
    match role {
        BoundedResidencyMediaRole::CreateNew => "create-new",
        BoundedResidencyMediaRole::PositionedRead => "positioned-read",
        BoundedResidencyMediaRole::PositionedWrite => "positioned-write",
        BoundedResidencyMediaRole::ReadMetadata => "read-metadata",
        BoundedResidencyMediaRole::SynchronizeFileState => "synchronize-file-state",
        BoundedResidencyMediaRole::SynchronizeDirectoryPublication => {
            "synchronize-directory-publication"
        }
        BoundedResidencyMediaRole::AtomicReplace => "atomic-replace",
    }
}

const fn signal_family(family: BoundedResidencySignalFamily) -> &'static str {
    match family {
        BoundedResidencySignalFamily::ReadFault => "read-fault",
        BoundedResidencySignalFamily::ExactWriteback => "exact-writeback",
        BoundedResidencySignalFamily::Publication => "publication",
        BoundedResidencySignalFamily::Lifecycle => "lifecycle",
    }
}

const fn scheduler_profile(profile: BoundedResidencySchedulerProfile) -> &'static str {
    match profile {
        BoundedResidencySchedulerProfile::SimulatedStrictDurable => "simulated-strict-durable",
        BoundedResidencySchedulerProfile::PosixFileFsyncDirSync => "posix-file-fsync-dir-sync",
        BoundedResidencySchedulerProfile::WindowsFlushFileBuffers => "windows-flush-file-buffers",
        BoundedResidencySchedulerProfile::MmapFlushNotDurabilityCertified => {
            "mmap-flush-not-durability-certified"
        }
        BoundedResidencySchedulerProfile::AdversarialLostFlush => "adversarial-lost-flush",
        BoundedResidencySchedulerProfile::AdversarialReorderedFlush => {
            "adversarial-reordered-flush"
        }
    }
}

const fn scheduler_evidence_class(class: BoundedResidencySchedulerEvidenceClass) -> &'static str {
    match class {
        BoundedResidencySchedulerEvidenceClass::DeclaredByConfig => "declared-by-config",
        BoundedResidencySchedulerEvidenceClass::ObservedByProbe => "observed-by-probe",
        BoundedResidencySchedulerEvidenceClass::EstablishedByFilesystemAdmission => {
            "established-by-filesystem-admission"
        }
        BoundedResidencySchedulerEvidenceClass::ExternallyGuaranteed => "externally-guaranteed",
        BoundedResidencySchedulerEvidenceClass::UnverifiableAssumption => "unverifiable-assumption",
        BoundedResidencySchedulerEvidenceClass::CertifiedBackendProfile => {
            "certified-backend-profile"
        }
    }
}

const fn signal_settlement(settlement: BoundedResidencySignalSettlement) -> &'static str {
    match settlement {
        BoundedResidencySignalSettlement::Committed => "committed",
        BoundedResidencySignalSettlement::ReconciledFromPhysicalTruth => {
            "reconciled-from-physical-truth"
        }
        BoundedResidencySignalSettlement::DerivedStateUnavailable => "derived-state-unavailable",
    }
}

const fn family(family: BoundedResidencyWorkFamily) -> &'static str {
    match family {
        BoundedResidencyWorkFamily::ArtifactMetadataRead => "artifact-metadata-read",
        BoundedResidencyWorkFamily::ArtifactRangeRead => "artifact-range-read",
        BoundedResidencyWorkFamily::ArtifactRangeWrite => "artifact-range-write",
        BoundedResidencyWorkFamily::ArtifactPublication => "artifact-publication",
    }
}

const fn effect_fate(fate: BoundedResidencyWorkEffectFate) -> &'static str {
    match fate {
        BoundedResidencyWorkEffectFate::ReadCompleted => "read-completed",
        BoundedResidencyWorkEffectFate::WriteCompleted => "write-completed",
        BoundedResidencyWorkEffectFate::PublicationCompleted => "publication-completed",
    }
}

const fn recovery(recovery: BoundedResidencyWorkRecovery) -> &'static str {
    match recovery {
        BoundedResidencyWorkRecovery::NoEffect => "no-effect",
        BoundedResidencyWorkRecovery::ContinueSettlement => "continue-settlement",
    }
}

const fn terminal(terminal: BoundedResidencyWorkTerminalFate) -> &'static str {
    match terminal {
        BoundedResidencyWorkTerminalFate::Settled => "settled",
        BoundedResidencyWorkTerminalFate::ContinuedAfterConsumerCancellation => {
            "continued-after-consumer-cancellation"
        }
    }
}
