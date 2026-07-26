use super::yield_fixture::YieldExecution;
use super::*;
use crate::facade::provider_session::bounded_step::WorthQueryProviderCheckpointExport;

pub(super) struct YieldCheckpoint {
    pub(super) retained_bytes: u64,
    pub(super) retained: WorthQueryGraphProviderRetainedMemory,
}

pub(super) struct PanicProbeCheckpoint {
    pub(super) retained: WorthQueryGraphProviderRetainedMemory,
}

pub(super) struct PanicDropCheckpoint {
    pub(super) retained_bytes: u64,
    pub(super) retained: WorthQueryGraphProviderRetainedMemory,
}

pub(super) struct PanicProbeAndDropCheckpoint {
    pub(super) retained: WorthQueryGraphProviderRetainedMemory,
}

pub(super) struct RestoreFailureCheckpoint {
    pub(super) retained_bytes: u64,
    pub(super) retained: WorthQueryGraphProviderRetainedMemory,
}

pub(super) struct RestorePanicCheckpoint {
    pub(super) retained_bytes: u64,
    pub(super) retained: WorthQueryGraphProviderRetainedMemory,
}

pub(super) struct RestoreExecutionDropPanicCheckpoint {
    pub(super) retained_bytes: u64,
    pub(super) checkpoint_drop_panics: bool,
    pub(super) retained: WorthQueryGraphProviderRetainedMemory,
}

impl crate::domain_computation::WorthQueryGraphProviderCheckpoint for PanicProbeCheckpoint {
    fn retained_bytes(&self) -> u64 {
        panic!("yield fixture checkpoint probe panicked")
    }

    fn restore(
        &self,
        _call: &WorthQueryGraphProviderCall,
        _memory: &mut WorthQueryGraphProviderRestoreMemory,
    ) -> Result<Box<dyn WorthQueryGraphProviderExecution>, WorthQueryGraphProviderFailure> {
        Err(WorthQueryGraphProviderFailure::new(
            "panicking checkpoint probe must never restore",
        ))
    }
}

impl crate::domain_computation::WorthQueryGraphProviderCheckpoint for PanicDropCheckpoint {
    fn retained_bytes(&self) -> u64 {
        self.retained_bytes
    }

    fn restore(
        &self,
        _call: &WorthQueryGraphProviderCall,
        memory: &mut WorthQueryGraphProviderRestoreMemory,
    ) -> Result<Box<dyn WorthQueryGraphProviderExecution>, WorthQueryGraphProviderFailure> {
        Ok(Box::new(YieldExecution::restored(
            memory.rebind(&self.retained).map_err(step_failure)?,
        )))
    }
}

impl Drop for PanicDropCheckpoint {
    fn drop(&mut self) {
        panic!("yield fixture checkpoint drop panicked")
    }
}

impl crate::domain_computation::WorthQueryGraphProviderCheckpoint for PanicProbeAndDropCheckpoint {
    fn retained_bytes(&self) -> u64 {
        panic!("yield fixture checkpoint probe panicked before panicking drop")
    }

    fn restore(
        &self,
        _call: &WorthQueryGraphProviderCall,
        _memory: &mut WorthQueryGraphProviderRestoreMemory,
    ) -> Result<Box<dyn WorthQueryGraphProviderExecution>, WorthQueryGraphProviderFailure> {
        Err(WorthQueryGraphProviderFailure::new(
            "double-panicking checkpoint must never restore",
        ))
    }
}

impl Drop for PanicProbeAndDropCheckpoint {
    fn drop(&mut self) {
        panic!("yield fixture checkpoint drop panicked after panicking probe")
    }
}

impl crate::domain_computation::WorthQueryGraphProviderCheckpoint for RestoreFailureCheckpoint {
    fn retained_bytes(&self) -> u64 {
        self.retained_bytes
    }

    fn restore(
        &self,
        _call: &WorthQueryGraphProviderCall,
        _memory: &mut WorthQueryGraphProviderRestoreMemory,
    ) -> Result<Box<dyn WorthQueryGraphProviderExecution>, WorthQueryGraphProviderFailure> {
        Err(WorthQueryGraphProviderFailure::new(
            "yield fixture restore denied",
        ))
    }
}

impl crate::domain_computation::WorthQueryGraphProviderCheckpoint for RestorePanicCheckpoint {
    fn retained_bytes(&self) -> u64 {
        self.retained_bytes
    }

    fn restore(
        &self,
        _call: &WorthQueryGraphProviderCall,
        _memory: &mut WorthQueryGraphProviderRestoreMemory,
    ) -> Result<Box<dyn WorthQueryGraphProviderExecution>, WorthQueryGraphProviderFailure> {
        panic!("yield fixture restore panicked")
    }

    fn export(&self) -> Result<WorthQueryProviderCheckpointExport, WorthQueryGraphProviderFailure> {
        panic!("yield fixture checkpoint export panicked")
    }
}

impl crate::domain_computation::WorthQueryGraphProviderCheckpoint
    for RestoreExecutionDropPanicCheckpoint
{
    fn retained_bytes(&self) -> u64 {
        self.retained_bytes
    }

    fn restore(
        &self,
        _call: &WorthQueryGraphProviderCall,
        memory: &mut WorthQueryGraphProviderRestoreMemory,
    ) -> Result<Box<dyn WorthQueryGraphProviderExecution>, WorthQueryGraphProviderFailure> {
        Ok(Box::new(YieldExecution::restored_with_drop_panic(
            memory.rebind(&self.retained).map_err(step_failure)?,
        )))
    }
}

impl Drop for RestoreExecutionDropPanicCheckpoint {
    fn drop(&mut self) {
        if self.checkpoint_drop_panics {
            panic!("yield fixture checkpoint and restored execution drop panicked")
        }
    }
}

impl crate::domain_computation::WorthQueryGraphProviderCheckpoint for YieldCheckpoint {
    fn retained_bytes(&self) -> u64 {
        self.retained_bytes
    }

    fn restore(
        &self,
        _call: &WorthQueryGraphProviderCall,
        memory: &mut WorthQueryGraphProviderRestoreMemory,
    ) -> Result<Box<dyn WorthQueryGraphProviderExecution>, WorthQueryGraphProviderFailure> {
        Ok(Box::new(YieldExecution::restored(
            memory.rebind(&self.retained).map_err(step_failure)?,
        )))
    }

    fn export(&self) -> Result<WorthQueryProviderCheckpointExport, WorthQueryGraphProviderFailure> {
        WorthQueryProviderCheckpointExport::new(
            "worth-query-tests-yield",
            1,
            "worth-query-tests-yield-v1",
            format!("retained-bytes:{}", self.retained_bytes).into_bytes(),
        )
    }
}
