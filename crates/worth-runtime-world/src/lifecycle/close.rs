/// Internal close progression for a managed Runtime World owner. The state
/// is explicit so close cannot be represented by a boolean with no transition
/// semantics.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum RuntimeWorldCloseState {
    Open,
    Closing,
    Closed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RuntimeWorldCloseDenial {
    AlreadyClosing,
    AlreadyClosed,
}

use super::owner::RuntimeWorldOwnerRoot;

impl<D, I, E, Ctx, T> RuntimeWorldOwnerRoot<D, I, E, Ctx, T>
where
    D: Copy + Ord + std::fmt::Debug + Send + Sync + 'static,
    I: Copy + Ord + Send + Sync + 'static,
    T: Copy + Ord + Send + Sync + 'static,
{
    pub fn close(&self) -> Result<(), RuntimeWorldCloseDenial> {
        let bootstrap = self
            .state
            .bootstrap
            .lock()
            .unwrap_or_else(|error| error.into_inner());
        if *bootstrap == super::owner::RuntimeWorldBootstrapState::InProgress {
            return Err(RuntimeWorldCloseDenial::AlreadyClosing);
        }
        let operation = self
            .state
            .operation
            .lock()
            .unwrap_or_else(|error| error.into_inner());
        if *operation != super::owner::RuntimeWorldOperationState::Idle {
            return Err(RuntimeWorldCloseDenial::AlreadyClosing);
        }
        let mut close = self
            .state
            .close
            .lock()
            .unwrap_or_else(|error| error.into_inner());
        close.begin()?;
        close.finish()
    }

    pub fn lifecycle_observation(&self) -> super::RuntimeWorldOwnerLifecycleObservation {
        match self
            .state
            .close
            .lock()
            .unwrap_or_else(|error| error.into_inner())
            .state()
        {
            RuntimeWorldCloseState::Open => super::RuntimeWorldOwnerLifecycleObservation::Open,
            RuntimeWorldCloseState::Closing => {
                super::RuntimeWorldOwnerLifecycleObservation::Closing
            }
            RuntimeWorldCloseState::Closed => super::RuntimeWorldOwnerLifecycleObservation::Closed,
        }
    }
}

impl<D, I, E, Ctx, T> super::ports::RuntimeWorldLifecycleService
    for RuntimeWorldOwnerRoot<D, I, E, Ctx, T>
where
    D: Copy + Ord + std::fmt::Debug + Send + Sync + 'static,
    I: Copy + Ord + Send + Sync + 'static,
    T: Copy + Ord + Send + Sync + 'static,
{
    fn bootstrap_root(
        &self,
        intent: crate::branch::RuntimeWorldBootstrapIntent,
    ) -> crate::branch::RuntimeWorldBootstrapOutcome {
        RuntimeWorldOwnerRoot::bootstrap_root(self, intent)
    }

    fn close(&self) -> Result<(), RuntimeWorldCloseDenial> {
        RuntimeWorldOwnerRoot::close(self)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct RuntimeWorldCloseContract {
    state: RuntimeWorldCloseState,
}

impl RuntimeWorldCloseContract {
    pub(crate) const fn open() -> Self {
        Self {
            state: RuntimeWorldCloseState::Open,
        }
    }

    pub(crate) const fn state(self) -> RuntimeWorldCloseState {
        self.state
    }

    pub(crate) fn begin(&mut self) -> Result<(), RuntimeWorldCloseDenial> {
        match self.state {
            RuntimeWorldCloseState::Open => {
                self.state = RuntimeWorldCloseState::Closing;
                Ok(())
            }
            RuntimeWorldCloseState::Closing => Err(RuntimeWorldCloseDenial::AlreadyClosing),
            RuntimeWorldCloseState::Closed => Err(RuntimeWorldCloseDenial::AlreadyClosed),
        }
    }

    pub(crate) fn finish(&mut self) -> Result<(), RuntimeWorldCloseDenial> {
        match self.state {
            RuntimeWorldCloseState::Closing => {
                self.state = RuntimeWorldCloseState::Closed;
                Ok(())
            }
            RuntimeWorldCloseState::Open => Err(RuntimeWorldCloseDenial::AlreadyClosing),
            RuntimeWorldCloseState::Closed => Err(RuntimeWorldCloseDenial::AlreadyClosed),
        }
    }
}
