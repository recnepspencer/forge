use worth_signal::facade::{AspectMask, ChangedRegion, PartitionSubscription};

use super::declarations::{
    UiNativePhysicalSignalAspect, UiNativePhysicalSignalOperation, PHYSICAL_SIGNAL_ASPECT_COUNT,
};
use super::routing::UiNativePhysicalSignalWork;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct UiNativePhysicalSignalLocality {
    operation: UiNativePhysicalSignalOperation,
    work: UiNativePhysicalSignalWork,
}

impl UiNativePhysicalSignalLocality {
    pub(super) const fn new(
        operation: UiNativePhysicalSignalOperation,
        work: UiNativePhysicalSignalWork,
    ) -> Self {
        Self { operation, work }
    }

    pub(super) const fn operation(self) -> UiNativePhysicalSignalOperation {
        self.operation
    }

    pub(super) const fn work(self) -> UiNativePhysicalSignalWork {
        self.work
    }

    pub(super) fn subscription(
        self,
        aspect: UiNativePhysicalSignalAspect,
    ) -> PartitionSubscription {
        PartitionSubscription::partition_and_detail(aspect.partition(), self.detail(aspect))
    }

    pub(super) fn changed_region(self, aspect: UiNativePhysicalSignalAspect) -> ChangedRegion {
        ChangedRegion::new(aspect.partition()).with_detail(self.detail(aspect))
    }

    pub(super) fn scopes_for(
        self,
        reads: AspectMask,
    ) -> [Option<PartitionSubscription>; PHYSICAL_SIGNAL_ASPECT_COUNT] {
        UiNativePhysicalSignalAspect::typed().map(|aspect| {
            reads
                .contains(AspectMask::from([aspect.signal_aspect()]))
                .then(|| self.subscription(aspect))
        })
    }

    fn detail(self, aspect: UiNativePhysicalSignalAspect) -> String {
        let request = self.work.request_identity();
        match aspect {
            UiNativePhysicalSignalAspect::HostLineage => {
                format!(
                    "host-session-{}",
                    request.presentation_basis().host_session_identity()
                )
            }
            UiNativePhysicalSignalAspect::WorkIdentity => format!(
                "{}-attempt-{}-request-{}",
                self.operation.partition(),
                request.presentation_basis().attempt().diagnostic_value(),
                request.sequence()
            ),
            UiNativePhysicalSignalAspect::Demand => {
                let digest = match self.work {
                    UiNativePhysicalSignalWork::AtlasPlanning(identity) => identity.basis_digest(),
                    UiNativePhysicalSignalWork::AtlasUpload(identity) => {
                        identity.request().basis_digest()
                    }
                    UiNativePhysicalSignalWork::Presentation(_) => [0; 32],
                };
                format!(
                    "demand-set-{}-request-{}",
                    full_digest_token(digest),
                    request.sequence()
                )
            }
            UiNativePhysicalSignalAspect::Target => {
                let basis = request.presentation_basis();
                format!(
                    "surface-{}-binding-{}-request-{}",
                    basis.surface().diagnostic_value(),
                    basis.binding().diagnostic_value(),
                    request.sequence()
                )
            }
            UiNativePhysicalSignalAspect::Submission => match self.work {
                UiNativePhysicalSignalWork::AtlasUpload(identity) => format!(
                    "atlas-generation-{}-transaction-{}-request-{}",
                    identity.pending().generation(),
                    identity.pending().transaction(),
                    request.sequence()
                ),
                UiNativePhysicalSignalWork::AtlasPlanning(_)
                | UiNativePhysicalSignalWork::Presentation(_) => {
                    format!("unsubmitted-request-{}", request.sequence())
                }
            },
            UiNativePhysicalSignalAspect::Recovery => {
                format!("recovery-request-{}", request.sequence())
            }
        }
    }
}

fn full_digest_token(digest: [u8; 32]) -> String {
    let mut token = String::with_capacity(64);
    for byte in digest {
        use std::fmt::Write as _;
        write!(&mut token, "{byte:02x}").expect("writing to a String cannot fail");
    }
    token
}
