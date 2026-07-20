use worth_proof::TransitionOutcome;
#[cfg(feature = "certification-test-authority")]
use worth_store_physical_backend::MediaFaultSchedule;
use worth_store_physical_backend::{
    FilesystemAccessPosture, FilesystemQualificationMode, FilesystemQualificationRequest,
    MediaQualificationDenial, RootProfileQualificationReport,
};

use super::{
    admission_outcome::{
        MediaAdmissionDeferred, MediaAdmissionDenial, MediaAdmissionInspectionRequired,
        MediaAdmissionRebindRequired, MediaAdmissionStale,
    },
    MediaAdmissionOutcome, MediaOwnedPhysicalRuntime,
};
use crate::physical_runtime::{runtime::AdmittedPhysicalRuntime, RuntimeIdentity};

#[derive(Debug, Clone)]
pub struct FilesystemMediaAdmission {
    mode: FilesystemQualificationMode,
    access: FilesystemAccessPosture,
    #[cfg(feature = "certification-test-authority")]
    fault_schedule: MediaFaultSchedule,
    expected_profile: Option<RootProfileQualificationReport>,
}

impl FilesystemMediaAdmission {
    pub fn production(access: FilesystemAccessPosture) -> Self {
        Self {
            mode: FilesystemQualificationMode::Production,
            access,
            #[cfg(feature = "certification-test-authority")]
            fault_schedule: MediaFaultSchedule::default(),
            expected_profile: None,
        }
    }

    #[cfg(feature = "certification-test-authority")]
    pub fn certification(access: FilesystemAccessPosture) -> Self {
        Self {
            mode: FilesystemQualificationMode::Certification,
            access,
            fault_schedule: MediaFaultSchedule::default(),
            expected_profile: None,
        }
    }

    pub fn require_current_profile(mut self, report: RootProfileQualificationReport) -> Self {
        self.expected_profile = Some(report);
        self
    }

    #[cfg(feature = "certification-test-authority")]
    pub fn with_fault_schedule(mut self, schedule: MediaFaultSchedule) -> Self {
        self.fault_schedule = schedule;
        self
    }

    #[cfg(feature = "certification-test-authority")]
    pub fn fault_schedule_authority(
        &self,
    ) -> crate::physical_runtime::certification::CertificationMediaFaultAuthority {
        worth_store_physical_backend::certification_media_fault_authority()
    }
}

pub(in crate::physical_runtime) fn try_admit(
    runtime: AdmittedPhysicalRuntime,
    admission: FilesystemMediaAdmission,
) -> MediaAdmissionOutcome {
    let core = runtime.into_core();
    let runtime_identity = core.runtime_identity();
    let root = core.declared_store_root().as_path().to_owned();
    let request = match admission.mode {
        FilesystemQualificationMode::Production => {
            FilesystemQualificationRequest::production(root, admission.access)
        }
        #[cfg(feature = "certification-test-authority")]
        FilesystemQualificationMode::Certification => {
            FilesystemQualificationRequest::certification(root, admission.access)
        }
    };
    let request = request.for_runtime_incarnation(runtime_identity.get());
    #[cfg(feature = "certification-test-authority")]
    let request = request.with_fault_schedule(admission.fault_schedule);
    let request = match admission.expected_profile {
        Some(report) => request.require_current_profile(report),
        None => request,
    };
    match worth_store_physical_backend::qualify_filesystem_media(request).into_raw() {
        TransitionOutcome::Success(media) => {
            core.progress_to_media_owned();
            TransitionOutcome::success(MediaOwnedPhysicalRuntime::new(core, media)).into()
        }
        TransitionOutcome::Denied(
            denial @ MediaQualificationDenial::UnmanagedWriterPosture { .. },
        ) => TransitionOutcome::denied(MediaAdmissionDenial::new(
            AdmittedPhysicalRuntime::from_core(core),
            denial,
        ))
        .into(),
        TransitionOutcome::Denied(denial @ MediaQualificationDenial::OwnerPreEffect { .. }) => {
            TransitionOutcome::denied(MediaAdmissionDenial::new(
                AdmittedPhysicalRuntime::from_core(core),
                denial,
            ))
            .into()
        }
        TransitionOutcome::Denied(denial) => inspection_required(core, runtime_identity, denial),
        TransitionOutcome::Deferred(deferred) => TransitionOutcome::deferred(
            MediaAdmissionDeferred::new(AdmittedPhysicalRuntime::from_core(core), deferred),
        )
        .into(),
        TransitionOutcome::Stale(stale) => TransitionOutcome::stale(MediaAdmissionStale::new(
            AdmittedPhysicalRuntime::from_core(core),
            stale,
        ))
        .into(),
        TransitionOutcome::RebindRequired(rebind) => TransitionOutcome::rebind_required(
            MediaAdmissionRebindRequired::new(AdmittedPhysicalRuntime::from_core(core), rebind),
        )
        .into(),
        TransitionOutcome::Failed(failure) => {
            let terminal = core.abort();
            TransitionOutcome::failed(MediaAdmissionInspectionRequired::backend_failure(
                runtime_identity,
                terminal,
                failure,
            ))
            .into()
        }
    }
}

fn inspection_required(
    core: crate::physical_runtime::runtime::PhysicalRuntimeCore,
    runtime_identity: RuntimeIdentity,
    denial: MediaQualificationDenial,
) -> MediaAdmissionOutcome {
    let terminal = core.abort();
    TransitionOutcome::failed(MediaAdmissionInspectionRequired::post_effect_denial(
        runtime_identity,
        terminal,
        denial,
    ))
    .into()
}
