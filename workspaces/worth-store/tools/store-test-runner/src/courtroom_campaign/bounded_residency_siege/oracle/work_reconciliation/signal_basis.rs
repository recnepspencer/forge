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

const REQUIRED_READ_BASES: usize = 4;

const READ_ONLY: BoundedResidencySignalFamilySet = BoundedResidencySignalFamilySet {
    read_fault: true,
    exact_writeback: false,
    publication: false,
    lifecycle: false,
};
const WRITEBACK_ONLY: BoundedResidencySignalFamilySet = BoundedResidencySignalFamilySet {
    read_fault: false,
    exact_writeback: true,
    publication: false,
    lifecycle: false,
};
const PUBLICATION_ONLY: BoundedResidencySignalFamilySet = BoundedResidencySignalFamilySet {
    read_fault: false,
    exact_writeback: false,
    publication: true,
    lifecycle: false,
};

const REQUIRED_NATIVE_BINDINGS: [NativeSignalBasis; 6] = [
    NativeSignalBasis::read(ROOT_READ, "store.physical.record.root"),
    NativeSignalBasis::read(ARTIFACT_READ, "store.physical.record.artifact"),
    NativeSignalBasis::read(FRAME_READ, "store.physical.record.frame"),
    NativeSignalBasis::read(SCAN_READ, "store.physical.record.scan"),
    NativeSignalBasis::mutation(FRAME_WRITEBACK, WRITEBACK_ONLY),
    NativeSignalBasis::mutation(PUBLICATION, PUBLICATION_ONLY),
];

pub(super) struct InstalledSignalBindings<'a> {
    by_digest: HashMap<[u8; 32], &'a BoundedResidencySignalBindingObservation>,
    required_native: HashSet<[u8; 32]>,
    used_native: HashSet<[u8; 32]>,
}

impl<'a> InstalledSignalBindings<'a> {
    pub(super) fn require(
        bindings: &'a [BoundedResidencySignalBindingObservation],
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
        for expected in REQUIRED_NATIVE_BINDINGS {
            let binding = by_aspect.get(expected.aspect_key).ok_or_else(|| {
                format!(
                    "physical work reconciliation omitted native Signal basis `{}`",
                    expected.aspect_key
                )
            })?;
            expected.require(binding)?;
            required_native.insert(binding.digest);
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
        self.used_native.insert(binding.digest);
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
    }
}

#[derive(Clone, Copy)]
struct NativeSignalBasis {
    aspect_key: &'static str,
    role: BoundedResidencySignalAspectRole,
    families: BoundedResidencySignalFamilySet,
    partition: Option<&'static str>,
}

impl NativeSignalBasis {
    const fn read(aspect_key: &'static str, partition: &'static str) -> Self {
        Self {
            aspect_key,
            role: BoundedResidencySignalAspectRole::Dependency,
            families: READ_ONLY,
            partition: Some(partition),
        }
    }

    const fn mutation(aspect_key: &'static str, families: BoundedResidencySignalFamilySet) -> Self {
        Self {
            aspect_key,
            role: BoundedResidencySignalAspectRole::DependencyAndOutput,
            families,
            partition: None,
        }
    }

    fn require(self, binding: &BoundedResidencySignalBindingObservation) -> Result<(), String> {
        if binding.role == self.role
            && binding.families == self.families
            && binding.partition.as_deref() == self.partition
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
