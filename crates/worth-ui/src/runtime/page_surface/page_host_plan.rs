use std::collections::BTreeMap;

use crate::runtime::{
    WorthUiProjectionDependencyDeclaration, WorthUiProjectionDependencySet,
    WorthUiProjectionEquivalenceBasisKind, WorthUiProjectionFamily, WorthUiProjectionIdentity,
    WorthUiProjectionPlanContract, WorthUiRuntimeAuthoringSnapshot, WorthUiRuntimeFactId,
    WorthUiRuntimeHost,
};
use crate::source::{
    WorthUiLayoutSizingSpec, WorthUiLayoutSizingValue, WorthUiLayoutTopologyChild,
    WorthUiLayoutTopologyNode,
};

use super::{
    WorthUiPageHostFrameReceipt, WorthUiPageHostRequest, WorthUiPageHostSlotMountReceipt,
    WorthUiPageHostSlotReceipt,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiPageHostPlan {
    page_name: String,
    slots: Vec<WorthUiPageHostSlotReceipt>,
    slot_index_by_name: BTreeMap<String, usize>,
    frame_digest: u64,
    dependencies: WorthUiProjectionDependencySet,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthUiPageHostPlanDenial {
    MissingAuthoringSnapshot,
    MissingPage(String),
}

impl WorthUiPageHostPlan {
    pub fn from_runtime(
        runtime: &WorthUiRuntimeHost,
        request: WorthUiPageHostRequest,
    ) -> Result<Self, WorthUiPageHostPlanDenial> {
        let authoring = runtime
            .active_authoring_snapshot()
            .ok_or(WorthUiPageHostPlanDenial::MissingAuthoringSnapshot)?;
        Self::from_active_authoring(authoring, authoring.witness(), request)
    }

    pub fn from_active_authoring(
        authoring: &WorthUiRuntimeAuthoringSnapshot,
        _witness: &crate::runtime::WorthUiActiveAuthoringSnapshotWitness,
        request: WorthUiPageHostRequest,
    ) -> Result<Self, WorthUiPageHostPlanDenial> {
        let content_slots = authoring
            .content_slots()
            .page(request.page_name())
            .ok_or_else(|| {
                WorthUiPageHostPlanDenial::MissingPage(request.page_name().to_owned())
            })?;
        let slots = content_slots
            .assignments()
            .iter()
            .map(|assignment| {
                WorthUiPageHostSlotReceipt::new(assignment.slot_name(), assignment.surface_id())
            })
            .collect::<Vec<_>>();
        let slot_index_by_name = slots
            .iter()
            .enumerate()
            .map(|(index, slot)| (slot.slot_name().to_owned(), index))
            .collect::<BTreeMap<_, _>>();
        let dependencies = page_dependencies(authoring, request.page_name(), &slots);
        let frame_digest = digest_page_host_frame(authoring, request.page_name(), &slots);

        Ok(Self {
            page_name: request.page_name().to_owned(),
            slots,
            slot_index_by_name,
            frame_digest,
            dependencies,
        })
    }

    pub fn execute_frame(&self) -> WorthUiPageHostFrameReceipt {
        WorthUiPageHostFrameReceipt::new(
            self.page_name.clone(),
            self.slots.clone(),
            self.frame_digest,
        )
    }

    pub fn frame_digest(&self) -> u64 {
        self.frame_digest
    }

    pub fn page_name(&self) -> &str {
        &self.page_name
    }

    pub fn dependencies(&self) -> &WorthUiProjectionDependencySet {
        &self.dependencies
    }

    pub fn resolve_slot_mount(&self, slot_name: &str) -> Option<WorthUiPageHostSlotMountReceipt> {
        let slot = self
            .slot_index_by_name
            .get(slot_name)
            .and_then(|index| self.slots.get(*index))?
            .clone();
        Some(WorthUiPageHostSlotMountReceipt::new(
            self.page_name.clone(),
            slot.clone(),
            self.frame_digest,
            page_slot_mount_facts(&self.page_name, &slot),
        ))
    }
}

impl WorthUiProjectionPlanContract for WorthUiPageHostPlan {
    fn projection_identity(&self) -> WorthUiProjectionIdentity {
        WorthUiProjectionIdentity::runtime(format!("worth-ui.page-host.{}", self.page_name))
    }

    fn projection_family(&self) -> WorthUiProjectionFamily {
        WorthUiProjectionFamily::PageHost
    }

    fn projection_dependency_declaration(&self) -> WorthUiProjectionDependencyDeclaration {
        WorthUiProjectionDependencyDeclaration::from_set(self.dependencies.clone())
    }

    fn projection_equivalence_digest(&self) -> u64 {
        self.frame_digest
    }

    fn projection_equivalence_basis_kind(&self) -> WorthUiProjectionEquivalenceBasisKind {
        WorthUiProjectionEquivalenceBasisKind::FrameDigest
    }
}

impl crate::runtime::projection_contract::plan_contract::private::Sealed for WorthUiPageHostPlan {}

fn page_dependencies(
    _authoring: &WorthUiRuntimeAuthoringSnapshot,
    page_name: &str,
    slots: &[WorthUiPageHostSlotReceipt],
) -> WorthUiProjectionDependencySet {
    let page_template = crate::runtime::WorthUiPageTemplateId::new(page_name)
        .expect("page name is a valid template id");
    let page_instance = crate::runtime::WorthUiPageInstanceId::new(page_name)
        .expect("page name is a valid page instance id");
    let mut dependencies = WorthUiProjectionDependencySet::empty()
        .depends_on(WorthUiRuntimeFactId::layout_topology(page_name))
        .depends_on(WorthUiRuntimeFactId::layout_gap(page_name))
        .depends_on(WorthUiRuntimeFactId::layout_padding(page_name))
        .depends_on(WorthUiRuntimeFactId::page_template(&page_template))
        .depends_on(WorthUiRuntimeFactId::page_instance(&page_instance))
        .depends_on(WorthUiRuntimeFactId::page_instance_template_binding(
            &page_instance,
            &page_template,
        ));
    for slot in slots {
        let content_slot = crate::runtime::WorthUiContentSlotId::new(slot.slot_name())
            .expect("slot name is a valid content slot id");
        dependencies = dependencies
            .depends_on(WorthUiRuntimeFactId::page_content_slot(
                &page_template,
                &content_slot,
            ))
            .depends_on(WorthUiRuntimeFactId::surface_mount_raw(slot.surface_id()))
            .depends_on(WorthUiRuntimeFactId::authored_mount_component_selection(
                slot.surface_id(),
            ))
            .depends_on(WorthUiRuntimeFactId::authored_surface_props(
                slot.surface_id(),
            ));
        dependencies = dependencies.depends_on(WorthUiRuntimeFactId::content_mount(format!(
            "{page_name}.{}",
            slot.slot_name()
        )));
    }
    dependencies
}

fn page_slot_mount_facts(
    page_name: &str,
    slot: &WorthUiPageHostSlotReceipt,
) -> Vec<WorthUiRuntimeFactId> {
    let page_template = crate::runtime::WorthUiPageTemplateId::new(page_name)
        .expect("page name is a valid template id");
    let content_slot = crate::runtime::WorthUiContentSlotId::new(slot.slot_name())
        .expect("slot name is a valid content slot id");
    vec![
        WorthUiRuntimeFactId::page_content_slot(&page_template, &content_slot),
        WorthUiRuntimeFactId::surface_mount_raw(slot.surface_id()),
        WorthUiRuntimeFactId::authored_mount_component_selection(slot.surface_id()),
        WorthUiRuntimeFactId::authored_surface_props(slot.surface_id()),
        WorthUiRuntimeFactId::content_mount(format!("{page_name}.{}", slot.slot_name())),
    ]
}

fn digest_page_host_frame(
    authoring: &WorthUiRuntimeAuthoringSnapshot,
    page_name: &str,
    slots: &[WorthUiPageHostSlotReceipt],
) -> u64 {
    let mut digest = fold_bytes(0xcbf2_9ce4_8422_2325, page_name.as_bytes());
    if let Some(layout) = authoring.layout_topology().page(page_name) {
        digest = digest_layout_node(digest, layout.root());
    }
    for slot in slots {
        digest = fold_bytes(digest, slot.slot_name().as_bytes());
        digest = fold_bytes(digest, slot.surface_id().as_bytes());
        if let Some(component_id) = authoring
            .authored_surfaces()
            .component_id_for_surface(slot.surface_id())
        {
            digest = fold_bytes(digest, component_id.as_bytes());
        }
        if let Some(surface_digest) = authoring
            .authored_surface_props()
            .surface_digest(slot.surface_id())
        {
            digest = fold_bytes(digest, surface_digest.to_string().as_bytes());
        }
    }
    digest
}

fn digest_layout_node(mut digest: u64, node: &WorthUiLayoutTopologyNode) -> u64 {
    digest = fold_bytes(
        digest,
        match node.axis() {
            crate::source::WorthUiLayoutAxis::Row => b"axis:row",
            crate::source::WorthUiLayoutAxis::Column => b"axis:column",
        },
    );
    digest = fold_bytes(
        digest,
        match node.dimension() {
            Some(crate::source::WorthUiLayoutDimension::Width) => b"dimension:width",
            Some(crate::source::WorthUiLayoutDimension::Height) => b"dimension:height",
            None => b"dimension:none",
        },
    );
    digest = digest_sizing_opt(digest, node.sizing());
    digest = digest_sizing_value_opt(digest, "gap", node.gap());
    digest = digest_sizing_value_opt(digest, "padding", node.padding());
    digest = fold_bytes(digest, format!("scroll:{}", node.scroll_owner()).as_bytes());
    digest = fold_bytes(digest, format!("resize:{}", node.resizable()).as_bytes());
    digest = fold_bytes(digest, format!("restore:{}", node.restorable()).as_bytes());
    for child in node.children() {
        digest = match child {
            WorthUiLayoutTopologyChild::Region(region) => {
                fold_bytes(digest_layout_node(digest, region), b"child-region")
            }
            WorthUiLayoutTopologyChild::Slot(slot) => {
                let digest = fold_bytes(digest, b"child-slot");
                fold_bytes(digest, slot.slot_name().as_bytes())
            }
        };
    }
    digest
}

fn digest_sizing_opt(mut digest: u64, sizing: Option<&WorthUiLayoutSizingSpec>) -> u64 {
    match sizing {
        Some(sizing) => digest_sizing(digest, sizing),
        None => {
            digest = fold_bytes(digest, b"sizing:none");
            digest
        }
    }
}

fn digest_sizing(mut digest: u64, sizing: &WorthUiLayoutSizingSpec) -> u64 {
    match sizing {
        WorthUiLayoutSizingSpec::Fit => fold_bytes(digest, b"sizing:fit"),
        WorthUiLayoutSizingSpec::Fill => fold_bytes(digest, b"sizing:fill"),
        WorthUiLayoutSizingSpec::Fixed(value) => {
            digest = fold_bytes(digest, b"sizing:fixed");
            digest_sizing_value(digest, value)
        }
        WorthUiLayoutSizingSpec::Share(value) => {
            fold_bytes(digest, format!("sizing:share:{value}").as_bytes())
        }
        WorthUiLayoutSizingSpec::Ratio {
            numerator,
            denominator,
        } => fold_bytes(
            digest,
            format!("sizing:ratio:{numerator}:{denominator}").as_bytes(),
        ),
        WorthUiLayoutSizingSpec::Clamp {
            min,
            preferred,
            max,
        } => {
            digest = fold_bytes(digest, b"sizing:clamp");
            digest = digest_sizing_value(digest, min);
            digest = digest_sizing(digest, preferred);
            digest_sizing_value(digest, max)
        }
    }
}

fn digest_sizing_value_opt(
    mut digest: u64,
    label: &str,
    value: Option<&WorthUiLayoutSizingValue>,
) -> u64 {
    match value {
        Some(value) => {
            digest = fold_bytes(digest, format!("{label}:some").as_bytes());
            digest_sizing_value(digest, value)
        }
        None => fold_bytes(digest, format!("{label}:none").as_bytes()),
    }
}

fn digest_sizing_value(mut digest: u64, value: &WorthUiLayoutSizingValue) -> u64 {
    match value {
        WorthUiLayoutSizingValue::NamedToken(token) => {
            digest = fold_bytes(digest, b"value:named");
            fold_bytes(digest, token.as_bytes())
        }
        WorthUiLayoutSizingValue::Number(value) => {
            fold_bytes(digest, format!("value:number:{value}").as_bytes())
        }
    }
}

fn fold_bytes(mut accumulator: u64, bytes: &[u8]) -> u64 {
    for byte in bytes {
        accumulator ^= u64::from(*byte);
        accumulator = accumulator.wrapping_mul(0x0000_0100_0000_01b3);
    }
    accumulator
}
