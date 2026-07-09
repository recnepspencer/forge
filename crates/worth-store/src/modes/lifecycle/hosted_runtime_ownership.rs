use worth_relational::facade::runtime::RelationalRuntime;

#[derive(Debug)]
pub(crate) struct HostedRuntimeOwnershipProof {
    runtime: RelationalRuntime,
}

impl HostedRuntimeOwnershipProof {
    pub(crate) fn verify(runtime: RelationalRuntime) -> Self {
        Self { runtime }
    }

    pub(crate) fn runtime(&self) -> &RelationalRuntime {
        &self.runtime
    }

    pub(crate) fn runtime_mut(&mut self) -> &mut RelationalRuntime {
        &mut self.runtime
    }

    pub(crate) fn into_runtime(self) -> RelationalRuntime {
        self.runtime
    }
}
