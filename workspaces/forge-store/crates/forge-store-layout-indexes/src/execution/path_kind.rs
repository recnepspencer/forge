use crate::access_shape::S8AccessShapeDetail;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum S8AccessPathKind {
    ExactForegroundRead(S8AccessShapeDetail),
    ExactDegradedScan(S8AccessShapeDetail),
    BaselineBTreeRead(S8AccessShapeDetail),
    BaselineBTreeRootPublication,
    BaselineBTreeReplayRecovery,
    BaselineLsmRead(S8AccessShapeDetail),
    BaselineLsmManifestPublication,
    BaselineLsmWalReplay,
}

impl S8AccessPathKind {
    pub(crate) const fn is_degraded_exact_scan(self) -> bool {
        matches!(self, Self::ExactDegradedScan(_))
    }
}
