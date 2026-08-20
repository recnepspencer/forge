use std::any::TypeId;
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct ConditionalOperationKey {
    domain: TypeId,
    operation: TypeId,
    family: TypeId,
}

pub(crate) struct WorthQueryInstalledConditionalNode {
    pub(crate) lowering: Arc<worth_runtime_bridge::facade::BridgeInstalledConditionalLowering>,
    pub(crate) location: worth_query_installation::facade::WorthQueryConditionalNodeLocation,
    pub(crate) declaration:
        worth_query_installation::facade::WorthQueryPortableConditionalNodeDeclaration,
    pub(crate) graph_authority:
        Arc<worth_query_installation::facade::WorthQueryInstalledGraphParticipationAuthority>,
    pub(crate) operation_identity: String,
    pub(crate) runtime_authority: u64,
    pub(crate) installation_runtime_authority: u64,
    pub(crate) installation_generation: u64,
    pub(crate) resource_support: crate::domain_installation::WorthQueryExecutionResourceSupport,
}

#[derive(Clone)]
pub(crate) struct WorthQueryInstalledConditionalInstanceFamily {
    pub(crate) location: worth_query_installation::facade::WorthQueryConditionalNodeLocation,
    pub(crate) operation_identity: String,
    pub(crate) runtime_authority: u64,
    pub(crate) installation_generation: u64,
    pub(crate) resource_support: crate::domain_installation::WorthQueryExecutionResourceSupport,
}

struct AuthoritativeConditionalInstallation {
    key: ConditionalOperationKey,
    node: Arc<WorthQueryInstalledConditionalNode>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WorthQueryConditionalExecutionIndexRebuildReport {
    authoritative_installations: usize,
    rebuilt_lookup_entries: usize,
    exact_index_parity: bool,
}

impl WorthQueryConditionalExecutionIndexRebuildReport {
    pub const fn authoritative_installations(self) -> usize {
        self.authoritative_installations
    }

    pub const fn rebuilt_lookup_entries(self) -> usize {
        self.rebuilt_lookup_entries
    }

    pub const fn exact_index_parity(self) -> bool {
        self.exact_index_parity
    }
}

#[derive(Default)]
pub(crate) struct WorthQueryConditionalExecutionRegistry {
    authoritative: Vec<AuthoritativeConditionalInstallation>,
    by_operation: HashMap<ConditionalOperationKey, Vec<Arc<WorthQueryInstalledConditionalNode>>>,
    owned_instances: HashMap<u64, AuthoritativeConditionalInstallation>,
    owned_instance_families:
        HashMap<ConditionalOperationKey, Vec<WorthQueryInstalledConditionalInstanceFamily>>,
    next_owned_instance: u64,
}

impl WorthQueryConditionalExecutionRegistry {
    pub(crate) fn install<D: 'static, O: 'static, F: 'static>(
        &mut self,
        node: WorthQueryInstalledConditionalNode,
    ) -> Result<(), ()> {
        let key = operation_key::<D, O, F>();
        if self
            .authoritative
            .iter()
            .any(|installed| installed.key == key && installed.node.location == node.location)
        {
            return Err(());
        }
        let node = Arc::new(node);
        self.authoritative
            .push(AuthoritativeConditionalInstallation {
                key,
                node: Arc::clone(&node),
            });
        let nodes = self.by_operation.entry(key).or_default();
        nodes.push(node);
        nodes.sort_by(|left, right| left.location.cmp(&right.location));
        Ok(())
    }

    pub(crate) fn operation_nodes<D: 'static, O: 'static, F: 'static>(
        &self,
    ) -> Vec<Arc<WorthQueryInstalledConditionalNode>> {
        self.by_operation
            .get(&operation_key::<D, O, F>())
            .cloned()
            .unwrap_or_default()
    }

    pub(crate) fn install_owned_instance_family<D: 'static, O: 'static, F: 'static>(
        &mut self,
        family: WorthQueryInstalledConditionalInstanceFamily,
    ) -> Result<(), ()> {
        let families = self
            .owned_instance_families
            .entry(operation_key::<D, O, F>())
            .or_default();
        if families
            .iter()
            .any(|installed| installed.location == family.location)
        {
            return Err(());
        }
        families.push(family);
        families.sort_by(|left, right| left.location.cmp(&right.location));
        Ok(())
    }

    pub(crate) fn owned_instance_families<D: 'static, O: 'static, F: 'static>(
        &self,
    ) -> Vec<WorthQueryInstalledConditionalInstanceFamily> {
        self.owned_instance_families
            .get(&operation_key::<D, O, F>())
            .cloned()
            .unwrap_or_default()
    }

    pub(crate) fn install_owned_instance<D: 'static, O: 'static, F: 'static>(
        &mut self,
        node: WorthQueryInstalledConditionalNode,
    ) -> Result<(u64, Arc<WorthQueryInstalledConditionalNode>), ()> {
        let identity = self.next_owned_instance.checked_add(1).ok_or(())?;
        self.next_owned_instance = identity;
        let key = operation_key::<D, O, F>();
        let node = Arc::new(node);
        self.owned_instances.insert(
            identity,
            AuthoritativeConditionalInstallation {
                key,
                node: Arc::clone(&node),
            },
        );
        Ok((identity, node))
    }

    pub(crate) fn owned_instance<D: 'static, O: 'static, F: 'static>(
        &self,
        identity: u64,
    ) -> Option<Arc<WorthQueryInstalledConditionalNode>> {
        let installed = self.owned_instances.get(&identity)?;
        (installed.key == operation_key::<D, O, F>()).then(|| Arc::clone(&installed.node))
    }

    pub(crate) fn remove_owned_instance<D: 'static, O: 'static, F: 'static>(
        &mut self,
        identity: u64,
    ) -> Option<Arc<WorthQueryInstalledConditionalNode>> {
        let installed = self.owned_instances.get(&identity)?;
        if installed.key != operation_key::<D, O, F>() {
            return None;
        }
        self.owned_instances
            .remove(&identity)
            .map(|entry| entry.node)
    }

    pub(crate) fn len(&self) -> usize {
        self.authoritative.len() + self.owned_instances.len()
    }

    pub(crate) fn registration_len(&self) -> usize {
        self.len()
            + self
                .owned_instance_families
                .values()
                .map(Vec::len)
                .sum::<usize>()
    }

    pub(crate) fn replace_lowerings_for_test<D: 'static, O: 'static, F: 'static>(
        &mut self,
        donor: &[Arc<WorthQueryInstalledConditionalNode>],
        runtime_authority: u64,
        installation_generation: u64,
    ) -> Result<(), &'static str> {
        let key = operation_key::<D, O, F>();
        let current = self
            .by_operation
            .get(&key)
            .ok_or("recipient conditional operation is not installed")?;
        if current.len() != donor.len()
            || current
                .iter()
                .zip(donor)
                .any(|(current, donor)| current.location != donor.location)
        {
            return Err("donor conditional lowering inventory does not match recipient");
        }
        let replacements = current
            .iter()
            .zip(donor)
            .map(|(current, donor)| {
                Arc::new(WorthQueryInstalledConditionalNode {
                    lowering: Arc::clone(&donor.lowering),
                    location: current.location.clone(),
                    declaration: donor.declaration.clone(),
                    graph_authority: Arc::clone(&donor.graph_authority),
                    operation_identity: current.operation_identity.clone(),
                    runtime_authority,
                    installation_runtime_authority: current.installation_runtime_authority,
                    installation_generation,
                    resource_support: donor.resource_support.clone(),
                })
            })
            .collect::<Vec<_>>();
        self.authoritative.retain(|installed| installed.key != key);
        self.authoritative.extend(replacements.iter().map(|node| {
            AuthoritativeConditionalInstallation {
                key,
                node: Arc::clone(node),
            }
        }));
        self.by_operation.insert(key, replacements);
        Ok(())
    }

    pub(crate) fn destroy_and_rebuild_index(
        &mut self,
    ) -> WorthQueryConditionalExecutionIndexRebuildReport {
        let rebuilt = self.rebuilt_index();
        let exact_index_parity = indexes_retain_exact_nodes(&self.by_operation, &rebuilt);
        self.by_operation = HashMap::new();
        self.by_operation = rebuilt;
        WorthQueryConditionalExecutionIndexRebuildReport {
            authoritative_installations: self.authoritative.len(),
            rebuilt_lookup_entries: self.by_operation.values().map(Vec::len).sum(),
            exact_index_parity,
        }
    }

    fn rebuilt_index(
        &self,
    ) -> HashMap<ConditionalOperationKey, Vec<Arc<WorthQueryInstalledConditionalNode>>> {
        let mut rebuilt: HashMap<_, Vec<_>> = HashMap::new();
        for installed in &self.authoritative {
            rebuilt
                .entry(installed.key)
                .or_default()
                .push(Arc::clone(&installed.node));
        }
        for nodes in rebuilt.values_mut() {
            nodes.sort_by(|left, right| left.location.cmp(&right.location));
        }
        rebuilt
    }
}

fn operation_key<D: 'static, O: 'static, F: 'static>() -> ConditionalOperationKey {
    ConditionalOperationKey {
        domain: TypeId::of::<D>(),
        operation: TypeId::of::<O>(),
        family: TypeId::of::<F>(),
    }
}

fn indexes_retain_exact_nodes(
    current: &HashMap<ConditionalOperationKey, Vec<Arc<WorthQueryInstalledConditionalNode>>>,
    rebuilt: &HashMap<ConditionalOperationKey, Vec<Arc<WorthQueryInstalledConditionalNode>>>,
) -> bool {
    current.len() == rebuilt.len()
        && current.iter().all(|(key, current_nodes)| {
            rebuilt.get(key).is_some_and(|rebuilt_nodes| {
                current_nodes.len() == rebuilt_nodes.len()
                    && current_nodes
                        .iter()
                        .zip(rebuilt_nodes)
                        .all(|(current, rebuilt)| Arc::ptr_eq(current, rebuilt))
            })
        })
}
