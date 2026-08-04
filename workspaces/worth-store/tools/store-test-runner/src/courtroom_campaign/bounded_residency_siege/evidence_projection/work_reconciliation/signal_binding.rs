use serde::ser::{Serialize, SerializeMap, SerializeSeq, Serializer};

use super::super::super::protocol::{
    BoundedResidencySignalAspectRole, BoundedResidencySignalBindingObservation,
};
use crate::physical_work_evidence::hex;

pub(super) struct SignalBindings<'evidence>(
    pub(super) &'evidence [BoundedResidencySignalBindingObservation],
);

impl Serialize for SignalBindings<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut sequence = serializer.serialize_seq(Some(self.0.len()))?;
        for binding in self.0 {
            sequence.serialize_element(&SignalBinding(binding))?;
        }
        sequence.end()
    }
}

struct SignalBinding<'evidence>(&'evidence BoundedResidencySignalBindingObservation);

impl Serialize for SignalBinding<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let binding = self.0;
        let mut map = serializer.serialize_map(Some(5))?;
        map.serialize_entry("digest", &hex(&binding.digest))?;
        map.serialize_entry("aspect_key", &binding.aspect_key)?;
        map.serialize_entry("role", signal_role(binding.role))?;
        map.serialize_entry("families", &SignalFamilies(binding))?;
        map.serialize_entry("partition", &binding.partition)?;
        map.end()
    }
}

struct SignalFamilies<'evidence>(&'evidence BoundedResidencySignalBindingObservation);

impl Serialize for SignalFamilies<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let families = self.0.families;
        let mut map = serializer.serialize_map(Some(9))?;
        map.serialize_entry("read_fault", &families.read_fault)?;
        map.serialize_entry("exact_writeback", &families.exact_writeback)?;
        map.serialize_entry("publication", &families.publication)?;
        map.serialize_entry("lifecycle", &families.lifecycle)?;
        map.serialize_entry("wal_append", &families.wal_append)?;
        map.serialize_entry("durability_barrier", &families.durability_barrier)?;
        map.serialize_entry("checkpoint_capture", &families.checkpoint_capture)?;
        map.serialize_entry("root_publication", &families.root_publication)?;
        map.serialize_entry("wal_reclamation", &families.wal_reclamation)?;
        map.end()
    }
}

const fn signal_role(role: BoundedResidencySignalAspectRole) -> &'static str {
    match role {
        BoundedResidencySignalAspectRole::Dependency => "dependency",
        BoundedResidencySignalAspectRole::Output => "output",
        BoundedResidencySignalAspectRole::DependencyAndOutput => "dependency-and-output",
    }
}
