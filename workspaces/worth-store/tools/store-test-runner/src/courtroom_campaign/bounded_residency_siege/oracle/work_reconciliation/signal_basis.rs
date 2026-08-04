use std::collections::{HashMap, HashSet};

use super::super::super::protocol::{
    BoundedResidencySignalAspectRole, BoundedResidencySignalBindingObservation,
    BoundedResidencySignalFamilySet, BoundedResidencyWorkFamily,
    BoundedResidencyWorkRecordObservation,
};

const ROOT_READ: &str = "store.physical.record.root-read-basis";
const ARTIFACT_READ: &str = "store.physical.record.artifact-read-basis";
const FRAME_READ: &str = "store.physical.record.frame-read-basis";
const SCAN_READ: &str = "store.physical.record.scan-read-basis";
const FRAME_WRITEBACK: &str = "store.physical.record.frame-writeback-basis";
const PUBLICATION: &str = "store.physical.record.publication-basis";
const DURABILITY_POLICY: &str = "store.physical.durability.policy-binding-basis";
const WAL_APPEND: &str = "store.physical.durability.wal-append-basis";
const WAL_BARRIER: &str = "store.physical.durability.wal-barrier-basis";
const CHECKPOINT_CAPTURE: &str = "store.physical.durability.checkpoint-capture-basis";
const ROOT_PUBLICATION: &str = "store.physical.durability.root-publication-basis";
const WAL_RECLAMATION: &str = "store.physical.durability.wal-reclamation-basis";

const REQUIRED_READ_BASES: usize = 4;

const READ_ONLY: BoundedResidencySignalFamilySet = BoundedResidencySignalFamilySet {
    read_fault: true,
    exact_writeback: false,
    publication: false,
    lifecycle: false,
    wal_append: false,
    durability_barrier: false,
    checkpoint_capture: false,
    root_publication: false,
    wal_reclamation: false,
};
const WRITEBACK_ONLY: BoundedResidencySignalFamilySet = BoundedResidencySignalFamilySet {
    read_fault: false,
    exact_writeback: true,
    publication: false,
    lifecycle: false,
    wal_append: false,
    durability_barrier: false,
    checkpoint_capture: false,
    root_publication: false,
    wal_reclamation: false,
};
const PUBLICATION_ONLY: BoundedResidencySignalFamilySet = BoundedResidencySignalFamilySet {
    read_fault: false,
    exact_writeback: false,
    publication: true,
    lifecycle: false,
    wal_append: false,
    durability_barrier: false,
    checkpoint_capture: false,
    root_publication: false,
    wal_reclamation: false,
};
const DURABILITY_POLICY_FAMILIES: BoundedResidencySignalFamilySet =
    BoundedResidencySignalFamilySet {
        read_fault: false,
        exact_writeback: false,
        publication: false,
        lifecycle: false,
        wal_append: true,
        durability_barrier: true,
        checkpoint_capture: true,
        root_publication: true,
        wal_reclamation: true,
    };

const fn durability_only(index: u8) -> BoundedResidencySignalFamilySet {
    BoundedResidencySignalFamilySet {
        read_fault: false,
        exact_writeback: false,
        publication: false,
        lifecycle: false,
        wal_append: index == 0,
        durability_barrier: index == 1,
        checkpoint_capture: index == 2,
        root_publication: index == 3,
        wal_reclamation: index == 4,
    }
}

const REQUIRED_NATIVE_BINDINGS: [NativeSignalBasis; 12] = [
    NativeSignalBasis::read(ROOT_READ, "store.physical.record.root"),
    NativeSignalBasis::read(ARTIFACT_READ, "store.physical.record.artifact"),
    NativeSignalBasis::read(FRAME_READ, "store.physical.record.frame"),
    NativeSignalBasis::read(SCAN_READ, "store.physical.record.scan"),
    NativeSignalBasis::mutation(FRAME_WRITEBACK, WRITEBACK_ONLY),
    NativeSignalBasis::mutation(PUBLICATION, PUBLICATION_ONLY),
    NativeSignalBasis::policy(DURABILITY_POLICY, DURABILITY_POLICY_FAMILIES),
    NativeSignalBasis::durability(WAL_APPEND, durability_only(0)),
    NativeSignalBasis::durability(WAL_BARRIER, durability_only(1)),
    NativeSignalBasis::durability(CHECKPOINT_CAPTURE, durability_only(2)),
    NativeSignalBasis::durability(ROOT_PUBLICATION, durability_only(3)),
    NativeSignalBasis::durability(WAL_RECLAMATION, durability_only(4)),
];

pub(super) struct InstalledSignalBindings<'a> {
    by_digest: HashMap<[u8; 32], &'a BoundedResidencySignalBindingObservation>,
    required_native: HashSet<[u8; 32]>,
    used_native: HashSet<[u8; 32]>,
}

impl<'a> InstalledSignalBindings<'a> {
    pub(super) fn require(
        bindings: &'a [BoundedResidencySignalBindingObservation],
        store: [u8; 16],
    ) -> Result<Self, String> {
        let mut by_digest = HashMap::with_capacity(bindings.len());
        let mut by_aspect = HashMap::with_capacity(bindings.len());
        for binding in bindings {
            require_well_formed(binding)?;
            if by_digest.insert(binding.digest, binding).is_some() {
                return Err(
                    "physical work reconciliation duplicated an installed Signal binding digest"
                        .to_owned(),
                );
            }
            if by_aspect
                .insert(binding.aspect_key.as_str(), binding)
                .is_some()
            {
                return Err(
                    "physical work reconciliation duplicated an installed Signal aspect identity"
                        .to_owned(),
                );
            }
        }

        let mut required_native = HashSet::with_capacity(REQUIRED_NATIVE_BINDINGS.len());
        let mut read_digests = HashSet::with_capacity(REQUIRED_READ_BASES);
        let store_partition = format!("physical-durability-store/{}", hex(&store));
        for expected in REQUIRED_NATIVE_BINDINGS {
            let binding = by_aspect.get(expected.aspect_key).ok_or_else(|| {
                format!(
                    "physical work reconciliation omitted native Signal basis `{}`",
                    expected.aspect_key
                )
            })?;
            expected.require(binding, &store_partition)?;
            if expected.require_use {
                required_native.insert(binding.digest);
            }
            if expected.families == READ_ONLY {
                read_digests.insert(binding.digest);
            }
        }
        if read_digests.len() != REQUIRED_READ_BASES {
            return Err(
                "physical work reconciliation collapsed the four native read Signal bases"
                    .to_owned(),
            );
        }

        Ok(Self {
            by_digest,
            required_native,
            used_native: HashSet::with_capacity(REQUIRED_NATIVE_BINDINGS.len()),
        })
    }

    pub(super) fn require_record(
        &mut self,
        record: &BoundedResidencyWorkRecordObservation,
    ) -> Result<(), String> {
        let binding = self
            .by_digest
            .get(&record.route.signal_binding)
            .ok_or_else(|| {
                "physical work reconciliation selected an uninstalled Signal binding".to_owned()
            })?;
        if !binding.families.serves(record.route.signal_family) {
            return Err(
                "physical work reconciliation selected a binding outside its Signal family"
                    .to_owned(),
            );
        }
        if !operation_allows(record.family, &binding.aspect_key) {
            return Err(
                "physical work reconciliation selected the wrong native Signal basis".to_owned(),
            );
        }
        if self.required_native.contains(&binding.digest) {
            self.used_native.insert(binding.digest);
        }
        Ok(())
    }

    pub(super) fn require_complete_native_use(self) -> Result<(), String> {
        if self.used_native == self.required_native {
            Ok(())
        } else {
            Err(
                "physical work reconciliation did not exercise every native Signal basis"
                    .to_owned(),
            )
        }
    }
}

fn require_well_formed(binding: &BoundedResidencySignalBindingObservation) -> Result<(), String> {
    if binding.digest.iter().all(|byte| *byte == 0)
        || binding.aspect_key.is_empty()
        || binding.families.is_empty()
        || binding.partition.as_ref().is_some_and(String::is_empty)
    {
        return Err(
            "physical work reconciliation emitted an inexact installed Signal binding".to_owned(),
        );
    }
    Ok(())
}

fn operation_allows(family: BoundedResidencyWorkFamily, aspect_key: &str) -> bool {
    match family {
        BoundedResidencyWorkFamily::ArtifactMetadataRead => {
            matches!(aspect_key, ROOT_READ | ARTIFACT_READ | SCAN_READ)
        }
        BoundedResidencyWorkFamily::ArtifactRangeRead => {
            matches!(
                aspect_key,
                ROOT_READ | ARTIFACT_READ | FRAME_READ | SCAN_READ
            )
        }
        BoundedResidencyWorkFamily::ArtifactRangeWrite => aspect_key == FRAME_WRITEBACK,
        BoundedResidencyWorkFamily::ArtifactPublication => aspect_key == PUBLICATION,
        BoundedResidencyWorkFamily::WalAppend => aspect_key == WAL_APPEND,
        BoundedResidencyWorkFamily::DurabilityBarrier => aspect_key == WAL_BARRIER,
        BoundedResidencyWorkFamily::CheckpointCapture => aspect_key == CHECKPOINT_CAPTURE,
        BoundedResidencyWorkFamily::RootPublication => aspect_key == ROOT_PUBLICATION,
        BoundedResidencyWorkFamily::WalReclamation => aspect_key == WAL_RECLAMATION,
    }
}

#[derive(Clone, Copy)]
struct NativeSignalBasis {
    aspect_key: &'static str,
    role: BoundedResidencySignalAspectRole,
    families: BoundedResidencySignalFamilySet,
    partition: PartitionRule,
    require_use: bool,
}

#[derive(Clone, Copy)]
enum PartitionRule {
    None,
    Exact(&'static str),
    Store,
    Policy,
}

impl NativeSignalBasis {
    const fn read(aspect_key: &'static str, partition: &'static str) -> Self {
        Self {
            aspect_key,
            role: BoundedResidencySignalAspectRole::Dependency,
            families: READ_ONLY,
            partition: PartitionRule::Exact(partition),
            require_use: true,
        }
    }

    const fn mutation(aspect_key: &'static str, families: BoundedResidencySignalFamilySet) -> Self {
        Self {
            aspect_key,
            role: BoundedResidencySignalAspectRole::DependencyAndOutput,
            families,
            partition: PartitionRule::None,
            require_use: true,
        }
    }

    const fn durability(
        aspect_key: &'static str,
        families: BoundedResidencySignalFamilySet,
    ) -> Self {
        Self {
            aspect_key,
            role: BoundedResidencySignalAspectRole::DependencyAndOutput,
            families,
            partition: PartitionRule::Store,
            require_use: false,
        }
    }

    const fn policy(aspect_key: &'static str, families: BoundedResidencySignalFamilySet) -> Self {
        Self {
            aspect_key,
            role: BoundedResidencySignalAspectRole::Dependency,
            families,
            partition: PartitionRule::Policy,
            require_use: false,
        }
    }

    fn require(
        self,
        binding: &BoundedResidencySignalBindingObservation,
        store_partition: &str,
    ) -> Result<(), String> {
        if binding.role == self.role
            && binding.families == self.families
            && self
                .partition
                .accepts(binding.partition.as_deref(), store_partition)
        {
            Ok(())
        } else {
            Err(format!(
                "physical work reconciliation changed native Signal basis `{}`",
                self.aspect_key
            ))
        }
    }
}

impl PartitionRule {
    fn accepts(self, actual: Option<&str>, store_partition: &str) -> bool {
        match self {
            Self::None => actual.is_none(),
            Self::Exact(expected) => actual == Some(expected),
            Self::Store => actual == Some(store_partition),
            Self::Policy => actual.is_some_and(valid_policy_partition),
        }
    }
}

fn valid_policy_partition(partition: &str) -> bool {
    let Some(identity) = partition.strip_prefix("physical-durability-policy/") else {
        return false;
    };
    identity.len() == 64 && identity.bytes().all(|byte| byte.is_ascii_hexdigit())
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}
