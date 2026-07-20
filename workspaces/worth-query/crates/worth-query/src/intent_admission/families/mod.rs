use super::surface_catalog::*;
use super::WorthQueryIntentAdmissionSurfaceDescriptor;

pub(crate) const INTENT_ADMISSION_FAMILIES_MODULE_ROOT: &str = "intent_admission/families/mod.rs";
pub(crate) const INTENT_ADMISSION_FAMILIES_CHILD_MODULES: &[&str] = &[];
pub(crate) const INTENT_ADMISSION_FAMILIES_EXPORTED_SURFACE: &[&str] = &[
    "WorthQueryIntentAdmissionFamily",
    "WorthQueryIntentAdmissionFamilyInventoryRow",
    "WorthQueryIntentAdmissionFamilyInventory",
    "worth_query_intent_admission_family_inventory",
];

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryIntentAdmissionFamily {
    AuthoritativeUserIntent,
    EffectTriggeredWriteIntent,
    AuthoritativeMutationIntent,
    BasisUseIntent,
    ProjectionConsumptionIntent,
    ReadExecutionIntent,
    InspectionMaterializationIntent,
    LowerRuntimeCapabilityRoutingIntent,
}

impl WorthQueryIntentAdmissionFamily {
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
pub struct WorthQueryIntentAdmissionFamilyInventoryRow {
    family: WorthQueryIntentAdmissionFamily,
    raw_authoring_constructor: WorthQueryIntentAdmissionSurfaceDescriptor,
    common_path_front_door: WorthQueryIntentAdmissionSurfaceDescriptor,
    advanced_path_front_door: WorthQueryIntentAdmissionSurfaceDescriptor,
}

impl WorthQueryIntentAdmissionFamilyInventoryRow {
    pub(crate) const fn new(
        family: WorthQueryIntentAdmissionFamily,
        raw_authoring_constructor: WorthQueryIntentAdmissionSurfaceDescriptor,
        common_path_front_door: WorthQueryIntentAdmissionSurfaceDescriptor,
        advanced_path_front_door: WorthQueryIntentAdmissionSurfaceDescriptor,
    ) -> Self {
        Self {
            family,
            raw_authoring_constructor,
            common_path_front_door,
            advanced_path_front_door,
        }
    }

    pub fn family(&self) -> WorthQueryIntentAdmissionFamily {
        self.family
    }

    pub fn raw_authoring_constructor(&self) -> WorthQueryIntentAdmissionSurfaceDescriptor {
        self.raw_authoring_constructor
    }

    pub fn common_path_front_door(&self) -> WorthQueryIntentAdmissionSurfaceDescriptor {
        self.common_path_front_door
    }

    pub fn advanced_path_front_door(&self) -> WorthQueryIntentAdmissionSurfaceDescriptor {
        self.advanced_path_front_door
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthQueryIntentAdmissionFamilyInventory {
    rows: &'static [WorthQueryIntentAdmissionFamilyInventoryRow],
}

impl WorthQueryIntentAdmissionFamilyInventory {
    pub(crate) const fn new(rows: &'static [WorthQueryIntentAdmissionFamilyInventoryRow]) -> Self {
        Self { rows }
    }

    pub fn rows(&self) -> &'static [WorthQueryIntentAdmissionFamilyInventoryRow] {
        self.rows
    }
}

const FAMILY_ROWS: [WorthQueryIntentAdmissionFamilyInventoryRow; 8] = [
    WorthQueryIntentAdmissionFamilyInventoryRow::new(
        WorthQueryIntentAdmissionFamily::AuthoritativeUserIntent,
        WorthQueryIntentAdmissionSurfaceDescriptor::available(AUTHORITATIVE_RUNTIME_RAW_ENTRYPOINT),
        WorthQueryIntentAdmissionSurfaceDescriptor::available(AUTHORITATIVE_RUNTIME_COMMON_PATH),
        WorthQueryIntentAdmissionSurfaceDescriptor::available(AUTHORITATIVE_RUNTIME_ADVANCED_PATH),
    ),
    WorthQueryIntentAdmissionFamilyInventoryRow::new(
        WorthQueryIntentAdmissionFamily::EffectTriggeredWriteIntent,
        WorthQueryIntentAdmissionSurfaceDescriptor::available(EFFECT_RUNTIME_RAW_ENTRYPOINT),
        WorthQueryIntentAdmissionSurfaceDescriptor::available(EFFECT_RUNTIME_COMMON_PATH),
        WorthQueryIntentAdmissionSurfaceDescriptor::available(EFFECT_RUNTIME_ADVANCED_PATH),
    ),
    WorthQueryIntentAdmissionFamilyInventoryRow::new(
        WorthQueryIntentAdmissionFamily::AuthoritativeMutationIntent,
        WorthQueryIntentAdmissionSurfaceDescriptor::available(
            AUTHORITATIVE_MUTATION_FAMILY_RAW_ENTRYPOINTS,
        ),
        WorthQueryIntentAdmissionSurfaceDescriptor::available(
            AUTHORITATIVE_MUTATION_FAMILY_COMMON_PATHS,
        ),
        WorthQueryIntentAdmissionSurfaceDescriptor::available(
            AUTHORITATIVE_MUTATION_FAMILY_ADVANCED_PATHS,
        ),
    ),
    WorthQueryIntentAdmissionFamilyInventoryRow::new(
        WorthQueryIntentAdmissionFamily::BasisUseIntent,
        WorthQueryIntentAdmissionSurfaceDescriptor::available(BASIS_OBSERVATION_RAW_ENTRYPOINT),
        WorthQueryIntentAdmissionSurfaceDescriptor::available(BASIS_OBSERVATION_COMMON_PATH),
        WorthQueryIntentAdmissionSurfaceDescriptor::available(BASIS_OBSERVATION_ADVANCED_PATH),
    ),
    WorthQueryIntentAdmissionFamilyInventoryRow::new(
        WorthQueryIntentAdmissionFamily::ProjectionConsumptionIntent,
        WorthQueryIntentAdmissionSurfaceDescriptor::available(
            PROJECTION_CONSUMPTION_RAW_ENTRYPOINT,
        ),
        WorthQueryIntentAdmissionSurfaceDescriptor::available(PROJECTION_CONSUMPTION_COMMON_PATH),
        WorthQueryIntentAdmissionSurfaceDescriptor::available(PROJECTION_CONSUMPTION_ADVANCED_PATH),
    ),
    WorthQueryIntentAdmissionFamilyInventoryRow::new(
        WorthQueryIntentAdmissionFamily::ReadExecutionIntent,
        WorthQueryIntentAdmissionSurfaceDescriptor::available(
            READ_EXECUTION_FAMILY_RAW_ENTRYPOINTS,
        ),
        WorthQueryIntentAdmissionSurfaceDescriptor::available(READ_EXECUTION_FAMILY_COMMON_PATHS),
        WorthQueryIntentAdmissionSurfaceDescriptor::available(READ_EXECUTION_FAMILY_ADVANCED_PATHS),
    ),
    WorthQueryIntentAdmissionFamilyInventoryRow::new(
        WorthQueryIntentAdmissionFamily::InspectionMaterializationIntent,
        WorthQueryIntentAdmissionSurfaceDescriptor::available(INSPECTION_FAMILY_RAW_ENTRYPOINTS),
        WorthQueryIntentAdmissionSurfaceDescriptor::available(INSPECTION_FAMILY_COMMON_PATHS),
        WorthQueryIntentAdmissionSurfaceDescriptor::available(INSPECTION_FAMILY_ADVANCED_PATHS),
    ),
    WorthQueryIntentAdmissionFamilyInventoryRow::new(
        WorthQueryIntentAdmissionFamily::LowerRuntimeCapabilityRoutingIntent,
        WorthQueryIntentAdmissionSurfaceDescriptor::available(EXISTING_TRUTH_PROBE_RAW_ENTRYPOINT),
        WorthQueryIntentAdmissionSurfaceDescriptor::available(
            EXISTING_TRUTH_PROBE_FAMILY_COMMON_PATHS,
        ),
        WorthQueryIntentAdmissionSurfaceDescriptor::available(EXISTING_TRUTH_PROBE_ADVANCED_PATHS),
    ),
];

pub fn worth_query_intent_admission_family_inventory() -> WorthQueryIntentAdmissionFamilyInventory {
    WorthQueryIntentAdmissionFamilyInventory::new(&FAMILY_ROWS)
}
