use super::{
    PhysicalWorkExecutionContext, PhysicalWorkFeatureGraphEvidence,
    PhysicalWorkFilesystemProfileEvidence, PhysicalWorkPlatformEvidence, PhysicalWorkRerunEvidence,
};
use crate::physical_runtime::PhysicalWorkSourceBinding;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PhysicalWorkRunEnvironmentEvidence {
    feature_graph: PhysicalWorkFeatureGraphEvidence,
    platform: PhysicalWorkPlatformEvidence,
    filesystem: PhysicalWorkFilesystemProfileEvidence,
    rerun: PhysicalWorkRerunEvidence,
}

impl PhysicalWorkRunEnvironmentEvidence {
    pub const fn new(
        feature_graph: PhysicalWorkFeatureGraphEvidence,
        platform: PhysicalWorkPlatformEvidence,
        filesystem: PhysicalWorkFilesystemProfileEvidence,
        rerun: PhysicalWorkRerunEvidence,
    ) -> Self {
        Self {
            feature_graph,
            platform,
            filesystem,
            rerun,
        }
    }

    pub const fn feature_graph(&self) -> &PhysicalWorkFeatureGraphEvidence {
        &self.feature_graph
    }

    pub const fn platform(&self) -> &PhysicalWorkPlatformEvidence {
        &self.platform
    }

    pub const fn filesystem(&self) -> &PhysicalWorkFilesystemProfileEvidence {
        &self.filesystem
    }

    pub const fn rerun(&self) -> &PhysicalWorkRerunEvidence {
        &self.rerun
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PhysicalWorkCourtroomRunBinding {
    source: PhysicalWorkSourceBinding,
    binary: PhysicalWorkSourceBinding,
    execution: PhysicalWorkExecutionContext,
    environment: PhysicalWorkRunEnvironmentEvidence,
}

impl PhysicalWorkCourtroomRunBinding {
    pub const fn new(
        source: PhysicalWorkSourceBinding,
        binary: PhysicalWorkSourceBinding,
        execution: PhysicalWorkExecutionContext,
        environment: PhysicalWorkRunEnvironmentEvidence,
    ) -> Self {
        Self {
            source,
            binary,
            execution,
            environment,
        }
    }

    pub const fn source(&self) -> &PhysicalWorkSourceBinding {
        &self.source
    }

    pub const fn binary(&self) -> &PhysicalWorkSourceBinding {
        &self.binary
    }

    pub const fn execution(&self) -> &PhysicalWorkExecutionContext {
        &self.execution
    }

    pub const fn environment(&self) -> &PhysicalWorkRunEnvironmentEvidence {
        &self.environment
    }
}
