use super::readiness::{
    signal_committed, signal_level_ready, UiNativeReadinessRegistry,
    UiNativeReadinessSignalDisposition,
};
use super::{UiNativeReadyOwner, UiNativeReadyWork};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiNativeReadinessContractOutcome {
    RedrawRequested,
    Coalesced,
    NoWork,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiNativeReadinessContractWork {
    pub generation: u64,
    pub scale_factor_milli: u32,
    pub client_physical_size: [u32; 2],
}

pub struct UiNativeReadinessContract {
    registry: UiNativeReadinessRegistry,
    committed_owner: UiNativeReadyOwner,
    level_owner: UiNativeReadyOwner,
    redraw_requests: usize,
}

impl UiNativeReadinessContract {
    pub fn new() -> Result<Self, ()> {
        let registry = UiNativeReadinessRegistry::new();
        let committed_owner = registry.register()?;
        let level_owner = registry.register_level()?;
        Ok(Self {
            registry,
            committed_owner,
            level_owner,
            redraw_requests: 0,
        })
    }

    pub fn commit_latest(
        &mut self,
        scale_factor_milli: u32,
        client_physical_size: [u32; 2],
    ) -> Result<u64, ()> {
        self.registry.commit_latest(
            self.committed_owner,
            scale_factor_milli,
            client_physical_size,
        )
    }

    pub fn signal_committed(&mut self) -> Result<UiNativeReadinessContractOutcome, ()> {
        let mut redraw = || self.redraw_requests += 1;
        signal_committed(&mut self.registry, self.committed_owner, &mut redraw).map(map_disposition)
    }

    pub fn take_committed(&mut self) -> Result<UiNativeReadinessContractWork, ()> {
        self.registry.take(self.committed_owner).map(map_work)
    }

    pub fn signal_level_ready(
        &mut self,
        has_ready_work: bool,
    ) -> Result<UiNativeReadinessContractOutcome, ()> {
        let mut redraw = || self.redraw_requests += 1;
        signal_level_ready(
            &mut self.registry,
            self.level_owner,
            has_ready_work,
            &mut redraw,
        )
        .map(map_disposition)
    }

    pub fn take_level(&mut self) -> Result<u64, ()> {
        self.registry
            .take_level(self.level_owner)
            .map(|grant| grant.generation())
    }

    pub fn redraw_requests(&self) -> usize {
        self.redraw_requests
    }

    pub fn close(&mut self) -> usize {
        self.registry.close()
    }
}

impl Default for UiNativeReadinessContract {
    fn default() -> Self {
        Self::new().expect("readiness certification registry capacity")
    }
}

fn map_disposition(
    disposition: UiNativeReadinessSignalDisposition,
) -> UiNativeReadinessContractOutcome {
    match disposition {
        UiNativeReadinessSignalDisposition::RedrawRequested => {
            UiNativeReadinessContractOutcome::RedrawRequested
        }
        UiNativeReadinessSignalDisposition::Coalesced => {
            UiNativeReadinessContractOutcome::Coalesced
        }
        UiNativeReadinessSignalDisposition::NoWork => UiNativeReadinessContractOutcome::NoWork,
    }
}

fn map_work(work: UiNativeReadyWork) -> UiNativeReadinessContractWork {
    UiNativeReadinessContractWork {
        generation: work.generation,
        scale_factor_milli: work.scale_factor_milli,
        client_physical_size: work.client_physical_size,
    }
}
