//! Fresh Query/native world execution for one matrix row.

use worth_ui_native_platform::{
    UiNativePlatformCloseReceipt, UiNativePlatformOutcome, UiNativePlatformProfile,
    UiNativeWindowSpec, WorthUiNativePlatform,
};

use super::application::Phase5LocalityMatrixApplication;
use super::case::Phase5LocalityCase;
use super::timings::Phase5LocalityApplicationTimingSnapshot;

pub(super) struct Phase5LocalityEvidence {
    case: Phase5LocalityCase,
    receipt: UiNativePlatformCloseReceipt,
    world_elapsed_millis: u64,
    profile_micros: u64,
    platform_prepare_micros: u64,
    query_install_micros: u64,
    native_run_micros: u64,
    application: Phase5LocalityApplicationTimingSnapshot,
}

impl Phase5LocalityEvidence {
    pub(super) const fn case(&self) -> Phase5LocalityCase {
        self.case
    }

    pub(super) const fn receipt(&self) -> &UiNativePlatformCloseReceipt {
        &self.receipt
    }

    pub(super) const fn world_elapsed_millis(&self) -> u64 {
        self.world_elapsed_millis
    }

    pub(super) const fn timing(&self) -> Phase5LocalityTimingView {
        Phase5LocalityTimingView {
            profile_micros: self.profile_micros,
            platform_prepare_micros: self.platform_prepare_micros,
            query_install_micros: self.query_install_micros,
            native_run_micros: self.native_run_micros,
            application: self.application,
        }
    }
}

#[derive(Clone, Copy)]
pub(super) struct Phase5LocalityTimingView {
    pub(super) profile_micros: u64,
    pub(super) platform_prepare_micros: u64,
    pub(super) query_install_micros: u64,
    pub(super) native_run_micros: u64,
    pub(super) application: Phase5LocalityApplicationTimingSnapshot,
}

pub(super) fn execute(case: Phase5LocalityCase) -> Result<Phase5LocalityEvidence, String> {
    let started = std::time::Instant::now();
    eprintln!(
        "phase5-locality prepare axis={} retained={} paragraphs={}",
        case.axis().label(),
        case.retained_size(),
        case.retained_paragraphs()
    );
    let profile_started = std::time::Instant::now();
    let profile = UiNativePlatformProfile::single_window(UiNativeWindowSpec::new(
        format!(
            "WORTH UI Phase 5 Locality {} {}",
            case.axis().label(),
            case.retained_size()
        ),
        [160, 96],
    ))
    .with_native_qualification_plan(case.qualification());
    let profile_micros = elapsed_micros(profile_started);
    eprintln!("phase5-locality timing phase=profile elapsed_us={profile_micros}");
    let platform_started = std::time::Instant::now();
    let platform = WorthUiNativePlatform::prepare(profile)
        .map_err(|denial| format!("native platform preparation: {denial:?}"))?;
    let platform_prepare_micros = elapsed_micros(platform_started);
    eprintln!("phase5-locality timing phase=platform-prepare elapsed_us={platform_prepare_micros}");
    eprintln!("phase5-locality install-query");
    let query_started = std::time::Instant::now();
    let presentation_async = install_presentation_async()?;
    let query_install_micros = elapsed_micros(query_started);
    eprintln!("phase5-locality timing phase=query-install elapsed_us={query_install_micros}");
    let (application, application_timings) =
        Phase5LocalityMatrixApplication::new(case, presentation_async);
    eprintln!("phase5-locality run-native");
    let native_started = std::time::Instant::now();
    let outcome = platform.run(application);
    let native_run_micros = elapsed_micros(native_started);
    match outcome {
        UiNativePlatformOutcome::Closed(receipt) => Ok(Phase5LocalityEvidence {
            case,
            receipt,
            world_elapsed_millis: u64::try_from(started.elapsed().as_millis()).unwrap_or(u64::MAX),
            profile_micros,
            platform_prepare_micros,
            query_install_micros,
            native_run_micros,
            application: application_timings.snapshot(),
        }),
        outcome => Err(format!("native matrix world stopped: {outcome:?}")),
    }
}

fn elapsed_micros(started: std::time::Instant) -> u64 {
    u64::try_from(started.elapsed().as_micros()).unwrap_or(u64::MAX)
}

fn install_presentation_async(
) -> Result<worth_ui_query_binding::WorthUiPresentationAsyncInstallation, String> {
    let plan = worth_ui_query_binding::WorthUiPresentationAsyncHostPlan::prepare()
        .map_err(|denial| format!("presentation async plan: {denial:?}"))?;
    let (request, completion) = plan.into_parts();
    let installation =
        worth_query_host::facade::runtime::WorthQueryExecutionRuntimeInstaller::new()
            .install(request.generation(), request.into_packages())
            .map_err(|denial| format!("Query runtime installation: {denial:?}"))?;
    completion
        .complete(installation)
        .map_err(|denial| format!("presentation async completion: {denial:?}"))
}
