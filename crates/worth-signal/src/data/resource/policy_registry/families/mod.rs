mod cancellation;
mod diagnostics;
mod observation;
mod output_continuity;
mod replay;
mod retention;
mod retry;
mod revalidation;
mod stale_after;
mod supersession;
mod timeout;

use super::registration::ResourcePolicyRegistration;

pub(crate) fn built_in_policy_registrations() -> Vec<ResourcePolicyRegistration> {
    let mut registrations = Vec::with_capacity(63);
    registrations.extend(retry::built_in_registrations());
    registrations.extend(timeout::built_in_registrations());
    registrations.extend(cancellation::built_in_registrations());
    registrations.extend(stale_after::built_in_registrations());
    registrations.extend(supersession::built_in_registrations());
    registrations.extend(revalidation::built_in_registrations());
    registrations.extend(observation::built_in_registrations());
    registrations.extend(output_continuity::built_in_registrations());
    registrations.extend(retention::built_in_registrations());
    registrations.extend(diagnostics::built_in_registrations());
    registrations.extend(replay::built_in_registrations());
    registrations
}
