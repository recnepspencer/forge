use super::surface_catalog::*;
use super::ForgeQueryIntentAdmissionSurfaceDescriptor;

pub(crate) const INTENT_ADMISSION_FAMILIES_MODULE_ROOT: &str = "intent_admission/families/mod.rs";
pub(crate) const INTENT_ADMISSION_FAMILIES_CHILD_MODULES: &[&str] = &[];
pub(crate) const INTENT_ADMISSION_FAMILIES_EXPORTED_SURFACE: &[&str] = &[
    "ForgeQueryIntentAdmissionFamily",
    "ForgeQueryIntentAdmissionFamilyInventoryRow",
    "ForgeQueryIntentAdmissionFamilyInventory",
    "forge_query_intent_admission_family_inventory",
];

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeQueryIntentAdmissionFamily {
    AuthoritativeUserIntent,
    EffectTriggeredWriteIntent,
    AuthoritativeMutationIntent,
    BasisUseIntent,
    ProjectionConsumptionIntent,
    ReadExecutionIntent,
    InspectionMaterializationIntent,
    LowerRuntimeCapabilityRoutingIntent,
}

impl ForgeQueryIntentAdmissionFamily {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::AuthoritativeUserIntent => "authoritative-user-intent",
            Self::EffectTriggeredWriteIntent => "effect-triggered-write-intent",
            Self::AuthoritativeMutationIntent => "authoritative-mutation-intent",
            Self::BasisUseIntent => "basis-use-intent",
            Self::ProjectionConsumptionIntent => "projection-consumption-intent",
            Self::ReadExecutionIntent => "read-execution-intent",
            Self::InspectionMaterializationIntent => "inspection-materialization-intent",
            Self::LowerRuntimeCapabilityRoutingIntent => "lower-runtime-capability-routing-intent",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ForgeQueryIntentAdmissionFamilyInventoryRow {
    family: ForgeQueryIntentAdmissionFamily,
    raw_authoring_constructor: ForgeQueryIntentAdmissionSurfaceDescriptor,
    common_path_front_door: ForgeQueryIntentAdmissionSurfaceDescriptor,
    advanced_path_front_door: ForgeQueryIntentAdmissionSurfaceDescriptor,
}

impl ForgeQueryIntentAdmissionFamilyInventoryRow {
    pub(crate) const fn new(
        family: ForgeQueryIntentAdmissionFamily,
        raw_authoring_constructor: ForgeQueryIntentAdmissionSurfaceDescriptor,
        common_path_front_door: ForgeQueryIntentAdmissionSurfaceDescriptor,
        advanced_path_front_door: ForgeQueryIntentAdmissionSurfaceDescriptor,
    ) -> Self {
        Self {
            family,
            raw_authoring_constructor,
            common_path_front_door,
            advanced_path_front_door,
        }
    }

    pub fn family(&self) -> ForgeQueryIntentAdmissionFamily {
        self.family
    }

    pub fn raw_authoring_constructor(&self) -> ForgeQueryIntentAdmissionSurfaceDescriptor {
        self.raw_authoring_constructor
    }

    pub fn common_path_front_door(&self) -> ForgeQueryIntentAdmissionSurfaceDescriptor {
        self.common_path_front_door
    }

    pub fn advanced_path_front_door(&self) -> ForgeQueryIntentAdmissionSurfaceDescriptor {
        self.advanced_path_front_door
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ForgeQueryIntentAdmissionFamilyInventory {
    rows: &'static [ForgeQueryIntentAdmissionFamilyInventoryRow],
}

impl ForgeQueryIntentAdmissionFamilyInventory {
    pub(crate) const fn new(rows: &'static [ForgeQueryIntentAdmissionFamilyInventoryRow]) -> Self {
        Self { rows }
    }

    pub fn rows(&self) -> &'static [ForgeQueryIntentAdmissionFamilyInventoryRow] {
        self.rows
    }
}

const FAMILY_ROWS: [ForgeQueryIntentAdmissionFamilyInventoryRow; 8] = [
    ForgeQueryIntentAdmissionFamilyInventoryRow::new(
        ForgeQueryIntentAdmissionFamily::AuthoritativeUserIntent,
        ForgeQueryIntentAdmissionSurfaceDescriptor::available(AUTHORITATIVE_RUNTIME_RAW_ENTRYPOINT),
        ForgeQueryIntentAdmissionSurfaceDescriptor::available(AUTHORITATIVE_RUNTIME_COMMON_PATH),
        ForgeQueryIntentAdmissionSurfaceDescriptor::available(AUTHORITATIVE_RUNTIME_ADVANCED_PATH),
    ),
    ForgeQueryIntentAdmissionFamilyInventoryRow::new(
        ForgeQueryIntentAdmissionFamily::EffectTriggeredWriteIntent,
        ForgeQueryIntentAdmissionSurfaceDescriptor::available(EFFECT_RUNTIME_RAW_ENTRYPOINT),
        ForgeQueryIntentAdmissionSurfaceDescriptor::available(EFFECT_RUNTIME_COMMON_PATH),
        ForgeQueryIntentAdmissionSurfaceDescriptor::available(EFFECT_RUNTIME_ADVANCED_PATH),
    ),
    ForgeQueryIntentAdmissionFamilyInventoryRow::new(
        ForgeQueryIntentAdmissionFamily::AuthoritativeMutationIntent,
        ForgeQueryIntentAdmissionSurfaceDescriptor::available(
            AUTHORITATIVE_MUTATION_FAMILY_RAW_ENTRYPOINTS,
        ),
        ForgeQueryIntentAdmissionSurfaceDescriptor::available(
            AUTHORITATIVE_MUTATION_FAMILY_COMMON_PATHS,
        ),
        ForgeQueryIntentAdmissionSurfaceDescriptor::available(
            AUTHORITATIVE_MUTATION_FAMILY_ADVANCED_PATHS,
        ),
    ),
    ForgeQueryIntentAdmissionFamilyInventoryRow::new(
        ForgeQueryIntentAdmissionFamily::BasisUseIntent,
        ForgeQueryIntentAdmissionSurfaceDescriptor::available(BASIS_OBSERVATION_RAW_ENTRYPOINT),
        ForgeQueryIntentAdmissionSurfaceDescriptor::available(BASIS_OBSERVATION_COMMON_PATH),
        ForgeQueryIntentAdmissionSurfaceDescriptor::available(BASIS_OBSERVATION_ADVANCED_PATH),
    ),
    ForgeQueryIntentAdmissionFamilyInventoryRow::new(
        ForgeQueryIntentAdmissionFamily::ProjectionConsumptionIntent,
        ForgeQueryIntentAdmissionSurfaceDescriptor::available(
            PROJECTION_CONSUMPTION_RAW_ENTRYPOINT,
        ),
        ForgeQueryIntentAdmissionSurfaceDescriptor::available(PROJECTION_CONSUMPTION_COMMON_PATH),
        ForgeQueryIntentAdmissionSurfaceDescriptor::available(PROJECTION_CONSUMPTION_ADVANCED_PATH),
    ),
    ForgeQueryIntentAdmissionFamilyInventoryRow::new(
        ForgeQueryIntentAdmissionFamily::ReadExecutionIntent,
        ForgeQueryIntentAdmissionSurfaceDescriptor::available(
            READ_EXECUTION_FAMILY_RAW_ENTRYPOINTS,
        ),
        ForgeQueryIntentAdmissionSurfaceDescriptor::available(READ_EXECUTION_FAMILY_COMMON_PATHS),
        ForgeQueryIntentAdmissionSurfaceDescriptor::available(READ_EXECUTION_FAMILY_ADVANCED_PATHS),
    ),
    ForgeQueryIntentAdmissionFamilyInventoryRow::new(
        ForgeQueryIntentAdmissionFamily::InspectionMaterializationIntent,
        ForgeQueryIntentAdmissionSurfaceDescriptor::available(INSPECTION_FAMILY_RAW_ENTRYPOINTS),
        ForgeQueryIntentAdmissionSurfaceDescriptor::available(INSPECTION_FAMILY_COMMON_PATHS),
        ForgeQueryIntentAdmissionSurfaceDescriptor::available(INSPECTION_FAMILY_ADVANCED_PATHS),
    ),
    ForgeQueryIntentAdmissionFamilyInventoryRow::new(
        ForgeQueryIntentAdmissionFamily::LowerRuntimeCapabilityRoutingIntent,
        ForgeQueryIntentAdmissionSurfaceDescriptor::available(EXISTING_TRUTH_PROBE_RAW_ENTRYPOINT),
        ForgeQueryIntentAdmissionSurfaceDescriptor::available(
            EXISTING_TRUTH_PROBE_FAMILY_COMMON_PATHS,
        ),
        ForgeQueryIntentAdmissionSurfaceDescriptor::available(EXISTING_TRUTH_PROBE_ADVANCED_PATHS),
    ),
];

pub fn forge_query_intent_admission_family_inventory() -> ForgeQueryIntentAdmissionFamilyInventory {
    ForgeQueryIntentAdmissionFamilyInventory::new(&FAMILY_ROWS)
}
