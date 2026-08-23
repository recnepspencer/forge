use std::marker::PhantomData;

use super::BuildProfile;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct WriterProcessRole {
    _sealed: (),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct ObserverProcessRole {
    _sealed: (),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct RecoveryProcessRole {
    _sealed: (),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum AuthorityLane {
    Ordinary,
    Recovery,
    CourtroomWriter,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct TargetSpec<R> {
    pub(crate) package: &'static str,
    pub(crate) binary: &'static str,
    pub(crate) features: &'static [&'static str],
    pub(crate) lane: AuthorityLane,
    pub(crate) _role: PhantomData<fn() -> R>,
}

#[derive(Clone, Copy)]
pub(crate) struct TargetSet {
    pub(crate) writer: TargetSpec<WriterProcessRole>,
    pub(crate) observer: TargetSpec<ObserverProcessRole>,
    pub(crate) recovery: TargetSpec<RecoveryProcessRole>,
}

#[derive(Clone, Copy)]
pub(crate) struct Recipe {
    pub(crate) targets: TargetSet,
    pub(crate) profile: BuildProfile,
    pub(crate) metadata_features: &'static [&'static str],
    pub(crate) source_packages: &'static [&'static str],
}

impl Recipe {
    pub(crate) fn metadata_features(self) -> &'static [&'static str] {
        self.metadata_features
    }
}

pub(crate) mod recipe {
    use super::{AuthorityLane, Recipe, TargetSet, TargetSpec};
    use crate::fresh_recovery_processes::BuildProfile;

    pub(crate) const fn production() -> Recipe {
        Recipe {
            targets: TargetSet {
                writer: TargetSpec {
                    package: "worth-store",
                    binary: "physical_store_c8_writer",
                    features: &[],
                    lane: AuthorityLane::Ordinary,
                    _role: std::marker::PhantomData,
                },
                observer: TargetSpec {
                    package: "worth-store-offline-verifier",
                    binary: "physical_store_offline_observer",
                    features: &[],
                    lane: AuthorityLane::Ordinary,
                    _role: std::marker::PhantomData,
                },
                recovery: TargetSpec {
                    package: "worth-store-recovery-runtime",
                    binary: "physical_store_recover",
                    features: &["worth-store-recovery-runtime/certification-test-authority"],
                    lane: AuthorityLane::Recovery,
                    _role: std::marker::PhantomData,
                },
            },
            profile: BuildProfile::Debug,
            metadata_features: &[],
            source_packages: &[
                "worth-store",
                "worth-store-offline-verifier",
                "worth-store-recovery-runtime",
            ],
        }
    }

    pub(crate) const fn bounded_residency() -> Recipe {
        Recipe {
            targets: TargetSet {
                writer: TargetSpec {
                    package: "worth-store",
                    binary: "physical_store_work_courtroom",
                    features: &["worth-store/certification-test-authority"],
                    lane: AuthorityLane::CourtroomWriter,
                    _role: std::marker::PhantomData,
                },
                observer: TargetSpec {
                    package: "worth-store-offline-verifier",
                    binary: "physical_store_offline_observer",
                    features: &[],
                    lane: AuthorityLane::Ordinary,
                    _role: std::marker::PhantomData,
                },
                recovery: TargetSpec {
                    package: "worth-store-recovery-runtime",
                    binary: "physical_store_recover",
                    features: &["worth-store-recovery-runtime/certification-test-authority"],
                    lane: AuthorityLane::Recovery,
                    _role: std::marker::PhantomData,
                },
            },
            profile: BuildProfile::Release,
            metadata_features: &[
                "worth-store/certification-test-authority",
                "store-test-runner/physical-work-evidence",
            ],
            source_packages: &[
                "worth-store",
                "worth-store-offline-verifier",
                "worth-store-recovery-runtime",
                "store-test-runner",
            ],
        }
    }
}
