#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryInstalledOwnedConditionalInstance {
    runtime_authority: u64,
    signal_graph_instance: u64,
    instance_identity: u64,
}

#[derive(Clone, Debug)]
pub enum WorthQueryOwnedConditionalInstanceDenial {
    MissingOwnedRuntime,
    ForeignRuntime,
    SuccessorRuntime,
    Installation(crate::domain_installation::WorthQueryConditionalNodeInstallationDenial),
    Delivery(crate::domain_installation::WorthQueryConditionalDeliveryDenial),
}

impl WorthQueryInstalledOwnedConditionalInstance {
    pub(crate) const fn new(
        runtime_authority: u64,
        signal_graph_instance: u64,
        instance_identity: u64,
    ) -> Self {
        Self {
            runtime_authority,
            signal_graph_instance,
            instance_identity,
        }
    }

    pub const fn signal_graph_instance(&self) -> u64 {
        self.signal_graph_instance
    }

    pub(crate) const fn runtime_authority(&self) -> u64 {
        self.runtime_authority
    }

    pub(crate) const fn instance_identity(&self) -> u64 {
        self.instance_identity
    }
}
