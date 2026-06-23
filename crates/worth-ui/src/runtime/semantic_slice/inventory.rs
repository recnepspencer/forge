use crate::runtime::WorthUiRuntimeFactFamily;

use super::inventory_data::SEMANTIC_SLICE_INVENTORY;
use super::{
    WorthUiSemanticMeaningClass, WorthUiSemanticSliceConsumers, WorthUiSemanticSliceFactMapping,
    WorthUiSemanticSliceId, WorthUiSemanticSliceOwner,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthUiSemanticSliceDescriptor {
    id: WorthUiSemanticSliceId,
    owner: WorthUiSemanticSliceOwner,
    meaning: WorthUiSemanticMeaningClass,
    runtime_fact_mapping: WorthUiSemanticSliceFactMapping,
    consumers: WorthUiSemanticSliceConsumers,
    preserve_upstream_granularity: bool,
}

impl WorthUiSemanticSliceDescriptor {
    pub const fn id(self) -> WorthUiSemanticSliceId {
        self.id
    }

    pub const fn owner(self) -> WorthUiSemanticSliceOwner {
        self.owner
    }

    pub const fn meaning(self) -> WorthUiSemanticMeaningClass {
        self.meaning
    }

    pub const fn runtime_fact_mapping(self) -> WorthUiSemanticSliceFactMapping {
        self.runtime_fact_mapping
    }

    pub const fn consumers(self) -> WorthUiSemanticSliceConsumers {
        self.consumers
    }

    pub const fn must_preserve_upstream_granularity(self) -> bool {
        self.preserve_upstream_granularity
    }

    pub fn runtime_fact_mapping_families(self) -> &'static [WorthUiRuntimeFactFamily] {
        match self.runtime_fact_mapping {
            crate::runtime::WorthUiSemanticSliceFactMapping::Exact(family) => {
                exact_runtime_fact_family_slice(family)
            }
            crate::runtime::WorthUiSemanticSliceFactMapping::Composite(families) => families,
            crate::runtime::WorthUiSemanticSliceFactMapping::Gap => &[],
        }
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WorthUiSemanticSliceInventory {
    _private: (),
}

impl WorthUiSemanticSliceInventory {
    pub fn current() -> Self {
        Self { _private: () }
    }

    pub fn slices(&self) -> &'static [WorthUiSemanticSliceDescriptor] {
        SEMANTIC_SLICE_INVENTORY
    }

    pub fn slice(
        &self,
        id: WorthUiSemanticSliceId,
    ) -> Option<&'static WorthUiSemanticSliceDescriptor> {
        self.slices()
            .iter()
            .find(|descriptor| descriptor.id() == id)
    }
}

pub(super) const fn row(
    id: WorthUiSemanticSliceId,
    owner: WorthUiSemanticSliceOwner,
    meaning: WorthUiSemanticMeaningClass,
    runtime_fact_mapping: WorthUiSemanticSliceFactMapping,
    consumers: WorthUiSemanticSliceConsumers,
) -> WorthUiSemanticSliceDescriptor {
    row_with_preservation(id, owner, meaning, runtime_fact_mapping, consumers, false)
}

pub(super) const fn row_with_preservation(
    id: WorthUiSemanticSliceId,
    owner: WorthUiSemanticSliceOwner,
    meaning: WorthUiSemanticMeaningClass,
    runtime_fact_mapping: WorthUiSemanticSliceFactMapping,
    consumers: WorthUiSemanticSliceConsumers,
    preserve_upstream_granularity: bool,
) -> WorthUiSemanticSliceDescriptor {
    WorthUiSemanticSliceDescriptor {
        id,
        owner,
        meaning,
        runtime_fact_mapping,
        consumers,
        preserve_upstream_granularity,
    }
}

fn exact_runtime_fact_family_slice(
    family: WorthUiRuntimeFactFamily,
) -> &'static [WorthUiRuntimeFactFamily] {
    match family {
        WorthUiRuntimeFactFamily::ActiveArtifact => &[WorthUiRuntimeFactFamily::ActiveArtifact],
        WorthUiRuntimeFactFamily::ExecutionPlan => &[WorthUiRuntimeFactFamily::ExecutionPlan],
        WorthUiRuntimeFactFamily::ThemeToken => &[WorthUiRuntimeFactFamily::ThemeToken],
        WorthUiRuntimeFactFamily::Command => &[WorthUiRuntimeFactFamily::Command],
        WorthUiRuntimeFactFamily::CommandProjection => {
            &[WorthUiRuntimeFactFamily::CommandProjection]
        }
        WorthUiRuntimeFactFamily::DropdownSelectionState => {
            &[WorthUiRuntimeFactFamily::DropdownSelectionState]
        }
        WorthUiRuntimeFactFamily::ComponentInteractionState => {
            &[WorthUiRuntimeFactFamily::ComponentInteractionState]
        }
        WorthUiRuntimeFactFamily::QueryBinding => &[WorthUiRuntimeFactFamily::QueryBinding],
        WorthUiRuntimeFactFamily::QueryResultPosture => {
            &[WorthUiRuntimeFactFamily::QueryResultPosture]
        }
        WorthUiRuntimeFactFamily::QueryProjectionFact => {
            &[WorthUiRuntimeFactFamily::QueryProjectionFact]
        }
        WorthUiRuntimeFactFamily::QueryComputedView => {
            &[WorthUiRuntimeFactFamily::QueryComputedView]
        }
        WorthUiRuntimeFactFamily::QueryStateSnapshot => {
            &[WorthUiRuntimeFactFamily::QueryStateSnapshot]
        }
        WorthUiRuntimeFactFamily::QueryEffectPosture => {
            &[WorthUiRuntimeFactFamily::QueryEffectPosture]
        }
        WorthUiRuntimeFactFamily::QueryRecoveryPosture => {
            &[WorthUiRuntimeFactFamily::QueryRecoveryPosture]
        }
        WorthUiRuntimeFactFamily::QueryInspectionTarget => {
            &[WorthUiRuntimeFactFamily::QueryInspectionTarget]
        }
        WorthUiRuntimeFactFamily::LayoutTopology => &[WorthUiRuntimeFactFamily::LayoutTopology],
        WorthUiRuntimeFactFamily::LayoutGap => &[WorthUiRuntimeFactFamily::LayoutGap],
        WorthUiRuntimeFactFamily::LayoutPadding => &[WorthUiRuntimeFactFamily::LayoutPadding],
        WorthUiRuntimeFactFamily::ContentMount => &[WorthUiRuntimeFactFamily::ContentMount],
        WorthUiRuntimeFactFamily::ShellSurface => &[WorthUiRuntimeFactFamily::ShellSurface],
        WorthUiRuntimeFactFamily::ShellSlotAssignment => {
            &[WorthUiRuntimeFactFamily::ShellSlotAssignment]
        }
        WorthUiRuntimeFactFamily::PageTemplate => &[WorthUiRuntimeFactFamily::PageTemplate],
        WorthUiRuntimeFactFamily::PageInstance => &[WorthUiRuntimeFactFamily::PageInstance],
        WorthUiRuntimeFactFamily::PageInstanceTemplateBinding => {
            &[WorthUiRuntimeFactFamily::PageInstanceTemplateBinding]
        }
        WorthUiRuntimeFactFamily::PageContentSlot => &[WorthUiRuntimeFactFamily::PageContentSlot],
        WorthUiRuntimeFactFamily::SurfaceMount => &[WorthUiRuntimeFactFamily::SurfaceMount],
        WorthUiRuntimeFactFamily::AuthoredMountComponentSelection => {
            &[WorthUiRuntimeFactFamily::AuthoredMountComponentSelection]
        }
        WorthUiRuntimeFactFamily::AuthoredSurfaceProps => {
            &[WorthUiRuntimeFactFamily::AuthoredSurfaceProps]
        }
        WorthUiRuntimeFactFamily::PrimitiveContent => &[WorthUiRuntimeFactFamily::PrimitiveContent],
        WorthUiRuntimeFactFamily::PrimitiveContainer => {
            &[WorthUiRuntimeFactFamily::PrimitiveContainer]
        }
        WorthUiRuntimeFactFamily::PrimitiveMeasurement => {
            &[WorthUiRuntimeFactFamily::PrimitiveMeasurement]
        }
        WorthUiRuntimeFactFamily::PrimitiveAppearance => {
            &[WorthUiRuntimeFactFamily::PrimitiveAppearance]
        }
        WorthUiRuntimeFactFamily::PrimitiveAppearanceState => {
            &[WorthUiRuntimeFactFamily::PrimitiveAppearanceState]
        }
        WorthUiRuntimeFactFamily::PrimitiveInteraction => {
            &[WorthUiRuntimeFactFamily::PrimitiveInteraction]
        }
        WorthUiRuntimeFactFamily::PrimitiveMotion => &[WorthUiRuntimeFactFamily::PrimitiveMotion],
        WorthUiRuntimeFactFamily::PrimitiveFlowLayout => {
            &[WorthUiRuntimeFactFamily::PrimitiveFlowLayout]
        }
        WorthUiRuntimeFactFamily::PrimitiveEventGeometry => {
            &[WorthUiRuntimeFactFamily::PrimitiveEventGeometry]
        }
        WorthUiRuntimeFactFamily::AuthoredQueryBindingShape => {
            &[WorthUiRuntimeFactFamily::AuthoredQueryBindingShape]
        }
        WorthUiRuntimeFactFamily::Component => &[WorthUiRuntimeFactFamily::Component],
        WorthUiRuntimeFactFamily::Appearance => &[WorthUiRuntimeFactFamily::Appearance],
        WorthUiRuntimeFactFamily::AppearanceRecipe => &[WorthUiRuntimeFactFamily::AppearanceRecipe],
        WorthUiRuntimeFactFamily::DensityToken => &[WorthUiRuntimeFactFamily::DensityToken],
        WorthUiRuntimeFactFamily::ActionPosture => &[WorthUiRuntimeFactFamily::ActionPosture],
        WorthUiRuntimeFactFamily::LiveViewBinding => &[WorthUiRuntimeFactFamily::LiveViewBinding],
        WorthUiRuntimeFactFamily::VirtualizedDataFrame => {
            &[WorthUiRuntimeFactFamily::VirtualizedDataFrame]
        }
        WorthUiRuntimeFactFamily::DurableStateFamily => {
            &[WorthUiRuntimeFactFamily::DurableStateFamily]
        }
        WorthUiRuntimeFactFamily::OverlaySurface => &[WorthUiRuntimeFactFamily::OverlaySurface],
        WorthUiRuntimeFactFamily::ToastSurface => &[WorthUiRuntimeFactFamily::ToastSurface],
        WorthUiRuntimeFactFamily::InspectorSurface => &[WorthUiRuntimeFactFamily::InspectorSurface],
        WorthUiRuntimeFactFamily::InteractionPolicy => {
            &[WorthUiRuntimeFactFamily::InteractionPolicy]
        }
    }
}
