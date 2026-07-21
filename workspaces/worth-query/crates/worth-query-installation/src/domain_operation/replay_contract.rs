#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryOperationReplayContract {
    NotSupported,
    ReExecutable,
    CertReplayable {
        comparator: WorthQueryOperationReplayComparatorContract,
    },
    CertReplayableWithNoise {
        comparator: WorthQueryOperationReplayComparatorContract,
        noise: WorthQueryOperationReplayNoiseContract,
    },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthQueryOperationReplayComparatorContract {
    pub family: &'static str,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WorthQueryOperationReplayNoiseContract {
    pub diagnostic_warnings: bool,
}
