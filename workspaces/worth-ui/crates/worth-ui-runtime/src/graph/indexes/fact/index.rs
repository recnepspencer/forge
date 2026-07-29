use std::collections::BTreeMap;

use crate::capability::{CapabilitySnapshot, ComponentId};
use crate::declaration::{UiAspectName, UiAspectSemanticSlice};
use crate::fact_contract::{
    UiAuthoredFactSelector, UiConsumedFactContract, UiProducedFact, UiProducedFactFamily,
    UiSubsystemConsumedFactRule,
};
use crate::graph::{UiGraphAspectConsumerKind, UiGraphAspectPublisherKind, UiGraphSnapshot};

use super::{
    UiAuthoredDeclarationLookup, UiGraphFactConsumerIdentity, UiGraphFactConsumerKey,
    UiGraphFactConsumerKind, UiGraphFactIndexBasis, UiGraphFactIndexEntry, UiGraphFactLookupDenial,
    UiGraphFactLookupReceipt,
};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
struct UiGraphSubsystemFactIndex {
    host_viewport: Box<[UiGraphFactIndexEntry]>,
    host_device_scale: Box<[UiGraphFactIndexEntry]>,
    measurement: Box<[UiGraphFactIndexEntry]>,
    query: Box<[UiGraphFactIndexEntry]>,
    committed_scroll_extent: Box<[UiGraphFactIndexEntry]>,
    committed_portal_anchor: Box<[UiGraphFactIndexEntry]>,
}

impl UiGraphSubsystemFactIndex {
    fn entries_for(&self, family: UiProducedFactFamily) -> &[UiGraphFactIndexEntry] {
        match family {
            UiProducedFactFamily::AuthoredSource => &[],
            UiProducedFactFamily::HostViewport => &self.host_viewport,
            UiProducedFactFamily::HostDeviceScale => &self.host_device_scale,
            UiProducedFactFamily::Measurement => &self.measurement,
            UiProducedFactFamily::Query => &self.query,
            UiProducedFactFamily::CommittedScrollExtent => &self.committed_scroll_extent,
            UiProducedFactFamily::CommittedPortalAnchor => &self.committed_portal_anchor,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiGraphConsumedFactIndex {
    basis: UiGraphFactIndexBasis,
    authored_by_declaration: BTreeMap<Box<str>, Box<[UiGraphFactIndexEntry]>>,
    subsystem: UiGraphSubsystemFactIndex,
}

impl UiGraphConsumedFactIndex {
    pub(crate) fn rebuild(
        snapshot: &UiGraphSnapshot,
        capabilities: &CapabilitySnapshot,
        authored_declarations: &UiAuthoredDeclarationLookup,
    ) -> Self {
        let mut authored_by_declaration =
            direct_authored_consumers(snapshot, authored_declarations);
        add_authored_aspect_consumers(
            snapshot,
            authored_declarations,
            &mut authored_by_declaration,
        );
        add_static_paint_token_consumers(
            snapshot,
            capabilities,
            authored_declarations,
            &mut authored_by_declaration,
        );
        let authored_by_declaration = authored_by_declaration
            .into_iter()
            .map(|(identity, entries)| (identity, canonical_entries(entries)))
            .collect();

        Self {
            basis: UiGraphFactIndexBasis::from_generation(snapshot, capabilities),
            authored_by_declaration,
            subsystem: build_subsystem_index(snapshot),
        }
    }

    #[cfg(any(test, feature = "certification-support"))]
    pub const fn basis(&self) -> UiGraphFactIndexBasis {
        self.basis
    }

    pub(crate) fn lookup(
        &self,
        requested_basis: UiGraphFactIndexBasis,
        fact: &UiProducedFact,
    ) -> Result<UiGraphFactLookupReceipt, UiGraphFactLookupDenial> {
        if requested_basis != self.basis {
            return Err(UiGraphFactLookupDenial::BasisMismatch {
                index_basis: self.basis,
                requested_basis,
            });
        }

        let entries = match fact {
            UiProducedFact::AuthoredSource(authored) => match authored.selector() {
                UiAuthoredFactSelector::Module(_) => &[][..],
                UiAuthoredFactSelector::Node(identity) => self
                    .authored_by_declaration
                    .get(identity)
                    .map(Box::as_ref)
                    .ok_or_else(|| UiGraphFactLookupDenial::UnknownAuthoredDeclaration {
                        authored_identity: identity.clone(),
                    })?,
            },
            _ => self.subsystem.entries_for(fact.family()),
        };
        debug_assert!(entries
            .iter()
            .all(|entry| entry.consumed_fact_contract().matches(fact)));
        Ok(UiGraphFactLookupReceipt::new(
            self.basis,
            entries.to_vec().into_boxed_slice(),
        ))
    }
}

fn add_static_paint_token_consumers(
    snapshot: &UiGraphSnapshot,
    capabilities: &CapabilitySnapshot,
    authored_declarations: &UiAuthoredDeclarationLookup,
    by_declaration: &mut BTreeMap<Box<str>, Vec<UiGraphFactIndexEntry>>,
) {
    for node in snapshot.nodes() {
        let Some(component) =
            component_capability_for_node(node, capabilities, authored_declarations)
        else {
            continue;
        };
        let Some(static_paint) = component.static_paint_contract() else {
            continue;
        };
        let token_capability_identity = static_paint.theme_token().as_str();
        let token_identity: Box<str> = authored_declarations
            .theme_token_declaration_identity(token_capability_identity)
            .unwrap_or(token_capability_identity)
            .into();
        let contract = UiConsumedFactContract::authored(token_identity.clone());
        let affected_aspect =
            UiAspectName::from_semantic_slice(UiAspectSemanticSlice::AppearanceBackground);
        let entries = by_declaration.entry(token_identity).or_default();
        push_component_consumer(entries, snapshot, node, contract, affected_aspect);
    }
}

fn component_capability_for_node<'capability>(
    node: &crate::graph::UiGraphNode,
    capabilities: &'capability CapabilitySnapshot,
    authored_declarations: &UiAuthoredDeclarationLookup,
) -> Option<&'capability crate::capability::ComponentDescriptor> {
    let source_backed = authored_declarations
        .unique_component_capability_identity(node.authored_provenance_digest())
        .and_then(|identity| ComponentId::new(identity).ok())
        .and_then(|identity| capabilities.components().get(&identity));
    source_backed.or_else(|| {
        ComponentId::new(node.declaration_identity().authored_semantic_name())
            .ok()
            .and_then(|identity| capabilities.components().get(&identity))
    })
}

fn fact_selector_identity<'identity>(
    provenance_digest: u64,
    fallback_identity: &'identity str,
    authored_declarations: &'identity UiAuthoredDeclarationLookup,
) -> &'identity str {
    authored_declarations
        .unique_identity(provenance_digest)
        .unwrap_or(fallback_identity)
}

fn push_component_consumer(
    entries: &mut Vec<UiGraphFactIndexEntry>,
    snapshot: &UiGraphSnapshot,
    node: &crate::graph::UiGraphNode,
    contract: UiConsumedFactContract,
    affected_aspect: UiAspectName,
) {
    let authored_identity: Box<str> = node.declaration_identity().authored_semantic_name().into();
    let repeated = node.repeated_instance_basis().identity_digest();
    entries.push(UiGraphFactIndexEntry::new(
        UiGraphFactConsumerKey::new(
            UiGraphFactConsumerKind::GraphNode,
            authored_identity.clone(),
            repeated,
        ),
        UiGraphFactConsumerIdentity::GraphNode(node.graph_node_identity()),
        Some(affected_aspect.clone()),
        contract.clone(),
    ));
    if let Some(slot) = snapshot.mount_eligibility_slot_for_node(node.graph_node_identity()) {
        entries.push(UiGraphFactIndexEntry::new(
            UiGraphFactConsumerKey::new(
                UiGraphFactConsumerKind::MountEligibilitySlot,
                authored_identity,
                repeated,
            ),
            UiGraphFactConsumerIdentity::MountEligibilitySlot(slot.mount_eligibility_identity()),
            Some(affected_aspect),
            contract,
        ));
    }
}

fn direct_authored_consumers(
    snapshot: &UiGraphSnapshot,
    authored_declarations: &UiAuthoredDeclarationLookup,
) -> BTreeMap<Box<str>, Vec<UiGraphFactIndexEntry>> {
    let mut by_declaration = BTreeMap::<Box<str>, Vec<UiGraphFactIndexEntry>>::new();
    for node in snapshot.nodes() {
        let consumer_identity: Box<str> =
            node.declaration_identity().authored_semantic_name().into();
        let selector_identity: Box<str> = fact_selector_identity(
            node.authored_provenance_digest(),
            node.declaration_identity().authored_semantic_name(),
            authored_declarations,
        )
        .into();
        let contract = UiConsumedFactContract::authored(selector_identity.clone());
        let entries = by_declaration.entry(selector_identity).or_default();
        entries.push(UiGraphFactIndexEntry::new(
            UiGraphFactConsumerKey::new(
                UiGraphFactConsumerKind::GraphNode,
                consumer_identity.clone(),
                node.repeated_instance_basis().identity_digest(),
            ),
            UiGraphFactConsumerIdentity::GraphNode(node.graph_node_identity()),
            None,
            contract.clone(),
        ));
        if let Some(slot) = snapshot.mount_eligibility_slot_for_node(node.graph_node_identity()) {
            entries.push(UiGraphFactIndexEntry::new(
                UiGraphFactConsumerKey::new(
                    UiGraphFactConsumerKind::MountEligibilitySlot,
                    consumer_identity,
                    node.repeated_instance_basis().identity_digest(),
                ),
                UiGraphFactConsumerIdentity::MountEligibilitySlot(
                    slot.mount_eligibility_identity(),
                ),
                None,
                contract,
            ));
        }
    }
    by_declaration
}

fn add_authored_aspect_consumers(
    snapshot: &UiGraphSnapshot,
    authored_declarations: &UiAuthoredDeclarationLookup,
    by_declaration: &mut BTreeMap<Box<str>, Vec<UiGraphFactIndexEntry>>,
) {
    let indexes = snapshot.core_indexes();
    for (aspect, publishers) in indexes.published_aspects().iter() {
        for publisher in publishers {
            let UiGraphAspectPublisherKind::GraphNode(publisher_node) = publisher.kind() else {
                continue;
            };
            let publisher_lookup = snapshot
                .lookup()
                .graph_node(publisher_node)
                .expect("every published graph node remains indexed");
            let publisher = publisher_lookup.value();
            let selector_identity: Box<str> = fact_selector_identity(
                publisher.authored_provenance_digest(),
                publisher.declaration_identity().authored_semantic_name(),
                authored_declarations,
            )
            .into();
            let contract = UiConsumedFactContract::authored(selector_identity.clone());
            let entries = by_declaration.entry(selector_identity).or_default();
            for consumer in indexes.consumed_aspects().consumers_for(aspect) {
                entries.push(UiGraphFactIndexEntry::new(
                    consumer_key(snapshot, consumer.kind()),
                    consumer_identity(consumer.kind()),
                    Some(aspect.clone()),
                    contract.clone(),
                ));
            }
        }
    }
}

fn build_subsystem_index(snapshot: &UiGraphSnapshot) -> UiGraphSubsystemFactIndex {
    let mut by_family = BTreeMap::<UiProducedFactFamily, Vec<UiGraphFactIndexEntry>>::new();
    for (aspect, consumers) in snapshot.core_indexes().consumed_aspects().iter() {
        add_subsystem_aspect_entries(snapshot, aspect, consumers, &mut by_family);
    }
    UiGraphSubsystemFactIndex {
        host_viewport: take_family(&mut by_family, UiProducedFactFamily::HostViewport),
        host_device_scale: take_family(&mut by_family, UiProducedFactFamily::HostDeviceScale),
        measurement: take_family(&mut by_family, UiProducedFactFamily::Measurement),
        query: take_family(&mut by_family, UiProducedFactFamily::Query),
        committed_scroll_extent: take_family(
            &mut by_family,
            UiProducedFactFamily::CommittedScrollExtent,
        ),
        committed_portal_anchor: take_family(
            &mut by_family,
            UiProducedFactFamily::CommittedPortalAnchor,
        ),
    }
}

fn add_subsystem_aspect_entries(
    snapshot: &UiGraphSnapshot,
    aspect: &UiAspectName,
    consumers: &[crate::graph::UiGraphAspectConsumer],
    by_family: &mut BTreeMap<UiProducedFactFamily, Vec<UiGraphFactIndexEntry>>,
) {
    for rule in UiSubsystemConsumedFactRule::all() {
        if rule.affected_aspect_family() != aspect.family() {
            continue;
        }
        let contract = UiConsumedFactContract::declared_aspect(rule.fact_family(), aspect.clone())
            .expect("a matching subsystem rule constructs one consumed-fact contract");
        for consumer in consumers {
            by_family
                .entry(rule.fact_family())
                .or_default()
                .push(UiGraphFactIndexEntry::new(
                    consumer_key(snapshot, consumer.kind()),
                    consumer_identity(consumer.kind()),
                    Some(aspect.clone()),
                    contract.clone(),
                ));
        }
    }
}

fn take_family(
    by_family: &mut BTreeMap<UiProducedFactFamily, Vec<UiGraphFactIndexEntry>>,
    family: UiProducedFactFamily,
) -> Box<[UiGraphFactIndexEntry]> {
    canonical_entries(by_family.remove(&family).unwrap_or_default())
}

fn canonical_entries(mut entries: Vec<UiGraphFactIndexEntry>) -> Box<[UiGraphFactIndexEntry]> {
    entries.sort_by(|left, right| {
        left.consumer_key()
            .cmp(right.consumer_key())
            .then_with(|| left.consumer().cmp(&right.consumer()))
            .then_with(|| left.affected_aspect().cmp(&right.affected_aspect()))
    });
    entries.dedup();
    entries.into_boxed_slice()
}

fn consumer_key(
    snapshot: &UiGraphSnapshot,
    kind: UiGraphAspectConsumerKind,
) -> UiGraphFactConsumerKey {
    let (consumer_kind, node_identity) = match kind {
        UiGraphAspectConsumerKind::GraphNode(identity) => {
            (UiGraphFactConsumerKind::GraphNode, identity)
        }
        UiGraphAspectConsumerKind::MountEligibilitySlot(identity) => {
            let node_identity = snapshot
                .mount_eligibilities()
                .slot(identity)
                .expect("every indexed mount-eligibility consumer has a graph-owned slot")
                .graph_node_identity();
            (UiGraphFactConsumerKind::MountEligibilitySlot, node_identity)
        }
    };
    let node = snapshot
        .nodes()
        .iter()
        .find(|node| node.graph_node_identity() == node_identity)
        .expect("every indexed fact consumer has one graph node");
    let declaration = snapshot
        .core_indexes()
        .declaration_correspondence()
        .declaration_identity_for(node_identity)
        .expect("every indexed fact consumer has declaration correspondence");
    UiGraphFactConsumerKey::new(
        consumer_kind,
        declaration.authored_semantic_name(),
        node.repeated_instance_basis().identity_digest(),
    )
}

fn consumer_identity(kind: UiGraphAspectConsumerKind) -> UiGraphFactConsumerIdentity {
    match kind {
        UiGraphAspectConsumerKind::GraphNode(identity) => {
            UiGraphFactConsumerIdentity::GraphNode(identity)
        }
        UiGraphAspectConsumerKind::MountEligibilitySlot(identity) => {
            UiGraphFactConsumerIdentity::MountEligibilitySlot(identity)
        }
    }
}
