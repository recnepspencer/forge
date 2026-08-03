use std::{collections::BTreeSet, num::NonZeroU32};

use super::{require_text, PhysicalWorkRunProvenanceDenial};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PhysicalWorkProcessFateEvidence {
    ExitedSuccess,
    KilledAtYieldpoint(Box<str>),
    ActiveEvidenceProducer,
}

impl PhysicalWorkProcessFateEvidence {
    pub const fn label(&self) -> &'static str {
        match self {
            Self::ExitedSuccess => "exited-success",
            Self::KilledAtYieldpoint(_) => "killed-at-yieldpoint",
            Self::ActiveEvidenceProducer => "active-evidence-producer",
        }
    }

    pub fn yieldpoint(&self) -> Option<&str> {
        match self {
            Self::KilledAtYieldpoint(yieldpoint) => Some(yieldpoint),
            Self::ExitedSuccess | Self::ActiveEvidenceProducer => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PhysicalWorkProcessEvidence {
    role: Box<str>,
    process: NonZeroU32,
    fate: PhysicalWorkProcessFateEvidence,
}

impl PhysicalWorkProcessEvidence {
    pub fn exited_success(
        role: impl Into<Box<str>>,
        process: NonZeroU32,
    ) -> Result<Self, PhysicalWorkRunProvenanceDenial> {
        Self::new(
            role,
            process,
            PhysicalWorkProcessFateEvidence::ExitedSuccess,
        )
    }

    pub fn killed_at_yieldpoint(
        role: impl Into<Box<str>>,
        process: NonZeroU32,
        yieldpoint: impl Into<Box<str>>,
    ) -> Result<Self, PhysicalWorkRunProvenanceDenial> {
        let yieldpoint = yieldpoint.into();
        require_text(
            &yieldpoint,
            PhysicalWorkRunProvenanceDenial::EmptyYieldpoint,
        )?;
        Self::new(
            role,
            process,
            PhysicalWorkProcessFateEvidence::KilledAtYieldpoint(yieldpoint),
        )
    }

    pub fn active_evidence_producer(
        role: impl Into<Box<str>>,
        process: NonZeroU32,
    ) -> Result<Self, PhysicalWorkRunProvenanceDenial> {
        Self::new(
            role,
            process,
            PhysicalWorkProcessFateEvidence::ActiveEvidenceProducer,
        )
    }

    fn new(
        role: impl Into<Box<str>>,
        process: NonZeroU32,
        fate: PhysicalWorkProcessFateEvidence,
    ) -> Result<Self, PhysicalWorkRunProvenanceDenial> {
        let role = role.into();
        require_text(&role, PhysicalWorkRunProvenanceDenial::EmptyProcessRole)?;
        Ok(Self {
            role,
            process,
            fate,
        })
    }

    pub fn role(&self) -> &str {
        &self.role
    }

    pub const fn process(&self) -> NonZeroU32 {
        self.process
    }

    pub const fn fate(&self) -> &PhysicalWorkProcessFateEvidence {
        &self.fate
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PhysicalWorkExecutionContext {
    workload_seed: PhysicalWorkWorkloadSeed,
    schedule_seed: PhysicalWorkScheduleSeed,
    schedule: Box<str>,
    processes: Box<[PhysicalWorkProcessEvidence]>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PhysicalWorkWorkloadSeed(u64);

impl PhysicalWorkWorkloadSeed {
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    pub const fn value(self) -> u64 {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PhysicalWorkScheduleSeed(u64);

impl PhysicalWorkScheduleSeed {
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    pub const fn value(self) -> u64 {
        self.0
    }
}

impl PhysicalWorkExecutionContext {
    pub fn new(
        workload_seed: PhysicalWorkWorkloadSeed,
        schedule_seed: PhysicalWorkScheduleSeed,
        schedule: impl Into<Box<str>>,
        processes: impl IntoIterator<Item = PhysicalWorkProcessEvidence>,
    ) -> Result<Self, PhysicalWorkRunProvenanceDenial> {
        let schedule = schedule.into();
        require_text(
            &schedule,
            PhysicalWorkRunProvenanceDenial::EmptyScheduleBinding,
        )?;
        let processes = processes.into_iter().collect::<Vec<_>>();
        if processes.is_empty() {
            return Err(PhysicalWorkRunProvenanceDenial::EmptyProcessSet);
        }
        let identities = processes
            .iter()
            .map(PhysicalWorkProcessEvidence::process)
            .collect::<BTreeSet<_>>();
        if identities.len() != processes.len() {
            return Err(PhysicalWorkRunProvenanceDenial::DuplicateProcessIdentity);
        }
        let roles = processes
            .iter()
            .map(PhysicalWorkProcessEvidence::role)
            .collect::<BTreeSet<_>>();
        if roles.len() != processes.len() {
            return Err(PhysicalWorkRunProvenanceDenial::DuplicateProcessRole);
        }
        Ok(Self {
            workload_seed,
            schedule_seed,
            schedule,
            processes: processes.into_boxed_slice(),
        })
    }

    pub const fn workload_seed(&self) -> PhysicalWorkWorkloadSeed {
        self.workload_seed
    }

    pub const fn schedule_seed(&self) -> PhysicalWorkScheduleSeed {
        self.schedule_seed
    }

    pub fn schedule(&self) -> &str {
        &self.schedule
    }

    pub const fn processes(&self) -> &[PhysicalWorkProcessEvidence] {
        &self.processes
    }
}

#[cfg(test)]
mod tests {
    use std::num::NonZeroU32;

    use super::PhysicalWorkRunProvenanceDenial;
    use super::{
        PhysicalWorkExecutionContext, PhysicalWorkProcessEvidence, PhysicalWorkScheduleSeed,
        PhysicalWorkWorkloadSeed,
    };

    #[test]
    fn execution_requires_distinct_roles_and_processes() {
        let process = NonZeroU32::new(41).unwrap();
        let first = PhysicalWorkProcessEvidence::exited_success("writer", process).unwrap();
        let duplicate = PhysicalWorkProcessEvidence::exited_success("observer", process).unwrap();
        assert_eq!(
            PhysicalWorkExecutionContext::new(
                PhysicalWorkWorkloadSeed::new(7),
                PhysicalWorkScheduleSeed::new(7),
                "schedule",
                [first, duplicate],
            ),
            Err(PhysicalWorkRunProvenanceDenial::DuplicateProcessIdentity)
        );
    }

    #[test]
    fn killed_process_requires_a_named_yieldpoint() {
        let process = NonZeroU32::new(41).unwrap();
        assert_eq!(
            PhysicalWorkProcessEvidence::killed_at_yieldpoint("writer", process, " "),
            Err(PhysicalWorkRunProvenanceDenial::EmptyYieldpoint)
        );
    }
}
