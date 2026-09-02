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
pub(crate) enum RuntimeWorldCloseDenial {
    AlreadyClosing,
    AlreadyClosed,
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
