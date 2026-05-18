use super::ForgeQueryIntentAdmissionSurfaceDescriptor;

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
        ForgeQueryIntentAdmissionSurfaceDescriptor::available(
            "ForgeQueryRawIntentAdmissionRequest::authoritative_runtime_entrypoint(...)",
        ),
        ForgeQueryIntentAdmissionSurfaceDescriptor::available(
            "runtime.intent(declaration).execute()",
        ),
        ForgeQueryIntentAdmissionSurfaceDescriptor::available(
            "runtime.intent(declaration).review()?.admit()?.execute()",
        ),
    ),
    ForgeQueryIntentAdmissionFamilyInventoryRow::new(
        ForgeQueryIntentAdmissionFamily::EffectTriggeredWriteIntent,
        ForgeQueryIntentAdmissionSurfaceDescriptor::available(
            "ForgeQueryRawIntentAdmissionRequest::effect_runtime_entrypoint(...)",
        ),
        ForgeQueryIntentAdmissionSurfaceDescriptor::available(
            "runtime.next_effect_write_intent(&effect, version, contract).execute()",
        ),
        ForgeQueryIntentAdmissionSurfaceDescriptor::available(
            "runtime.next_effect_write_intent(&effect, version, contract).review()?.admit()?.execute()",
        ),
    ),
    ForgeQueryIntentAdmissionFamilyInventoryRow::new(
        ForgeQueryIntentAdmissionFamily::AuthoritativeMutationIntent,
        ForgeQueryIntentAdmissionSurfaceDescriptor::available(
            "ForgeQueryRawIntentAdmissionRequest::authoritative_write_entrypoint(...)",
        ),
        ForgeQueryIntentAdmissionSurfaceDescriptor::available(
            "runtime.write_intent(command).execute(); workspace.write_intent(command).execute()",
        ),
        ForgeQueryIntentAdmissionSurfaceDescriptor::available(
            "runtime.write_intent(command).review()?.admit()?.execute(); workspace.write_intent(command).review()?.admit()?.execute()",
        ),
    ),
    ForgeQueryIntentAdmissionFamilyInventoryRow::new(
        ForgeQueryIntentAdmissionFamily::BasisUseIntent,
        ForgeQueryIntentAdmissionSurfaceDescriptor::available(
            "ForgeQueryRawIntentAdmissionRequest::basis_observation_lane(...)",
        ),
        ForgeQueryIntentAdmissionSurfaceDescriptor::available(
            "forge_query_basis_observation_intent(raw).admit()",
        ),
        ForgeQueryIntentAdmissionSurfaceDescriptor::available(
            "forge_query_basis_observation_intent(raw).review()?.admit()",
        ),
    ),
    ForgeQueryIntentAdmissionFamilyInventoryRow::new(
        ForgeQueryIntentAdmissionFamily::ProjectionConsumptionIntent,
        ForgeQueryIntentAdmissionSurfaceDescriptor::available(
            "ForgeQueryRawIntentAdmissionRequest::projection_consumption(declaration)",
        ),
        ForgeQueryIntentAdmissionSurfaceDescriptor::available(
            "forge_query_projection_consumption_intent(declaration).admit()",
        ),
        ForgeQueryIntentAdmissionSurfaceDescriptor::available(
            "forge_query_projection_consumption_intent(declaration).review()?.admit()",
        ),
    ),
    ForgeQueryIntentAdmissionFamilyInventoryRow::new(
        ForgeQueryIntentAdmissionFamily::ReadExecutionIntent,
        ForgeQueryIntentAdmissionSurfaceDescriptor::available(
            "ForgeQueryRawIntentAdmissionRequest::read_family_entrypoint(...); ForgeQueryRawIntentAdmissionRequest::live_read_entrypoint(...)",
        ),
        ForgeQueryIntentAdmissionSurfaceDescriptor::available(
            "workspace.read_family_intent(&family).execute(); workspace.read_live_intent(&view).execute()",
        ),
        ForgeQueryIntentAdmissionSurfaceDescriptor::available(
            "workspace.read_family_intent(&family).review()?.admit()?.execute(); workspace.read_live_intent(&view).review()?.admit()?.execute()",
        ),
    ),
    ForgeQueryIntentAdmissionFamilyInventoryRow::new(
        ForgeQueryIntentAdmissionFamily::InspectionMaterializationIntent,
        ForgeQueryIntentAdmissionSurfaceDescriptor::available(
            "ForgeQueryRawIntentAdmissionRequest::generic_inspection_entrypoint(...); ForgeQueryRawIntentAdmissionRequest::derived_materialization_entrypoint(...); ForgeQueryRawIntentAdmissionRequest::derived_inspection_entrypoint(...)",
        ),
        ForgeQueryIntentAdmissionSurfaceDescriptor::available(
            "workspace.inspect_intent(target).execute(); workspace.materialize_intent(&view).execute(); workspace.inspect_derived_intent(&view).execute()",
        ),
        ForgeQueryIntentAdmissionSurfaceDescriptor::available(
            "workspace.inspect_intent(target).review()?.admit()?.execute(); workspace.materialize_intent(&view).review()?.admit()?.execute(); workspace.inspect_derived_intent(&view).review()?.admit()?.execute()",
        ),
    ),
    ForgeQueryIntentAdmissionFamilyInventoryRow::new(
        ForgeQueryIntentAdmissionFamily::LowerRuntimeCapabilityRoutingIntent,
        ForgeQueryIntentAdmissionSurfaceDescriptor::available(
            "ForgeQueryRawIntentAdmissionRequest::existing_truth_probe_entrypoint(...)",
        ),
        ForgeQueryIntentAdmissionSurfaceDescriptor::available(
            "runtime.probe_existing_intent(request).execute(); workspace.probe_existing_intent(request).execute()",
        ),
        ForgeQueryIntentAdmissionSurfaceDescriptor::available(
            "runtime.probe_existing_intent(request).review()?.admit()?.execute(); workspace.probe_existing_intent(request).review()?.admit()?.execute()",
        ),
    ),
];

pub fn forge_query_intent_admission_family_inventory() -> ForgeQueryIntentAdmissionFamilyInventory {
    ForgeQueryIntentAdmissionFamilyInventory::new(&FAMILY_ROWS)
}
