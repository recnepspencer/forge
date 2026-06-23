use crate::runtime::{WorthUiHeaderFrameRebindDenial, WorthUiPageHostRebindDenial};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthUiRebindPhaseExecutionDenial {
    HeaderFrame(WorthUiHeaderFrameRebindDenial),
    PageHost(WorthUiPageHostRebindDenial),
}
