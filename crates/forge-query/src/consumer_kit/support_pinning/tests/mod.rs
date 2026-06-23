mod declaration_dx;
mod drift_localization;
mod evaluation_success;
mod hostile_terminal_document;
mod rejection;

use crate::consumer_kit::support_snapshot::{project_support_snapshot, ForgeQuerySupportSnapshot};
use crate::runtime::{
    ForgeQueryRuntimeFamilySupport, ForgeQueryRuntimePublicApiContract,
    ForgeQueryRuntimePublicSupportMatrix, ForgeQueryRuntimeSupportProfile,
};

fn scaffold_snapshot() -> ForgeQuerySupportSnapshot {
    snapshot_from_profile(ForgeQueryRuntimeSupportProfile::scaffold_backend_profile())
}

fn snapshot_from_profile(profile: ForgeQueryRuntimeSupportProfile) -> ForgeQuerySupportSnapshot {
    let contract = ForgeQueryRuntimePublicApiContract::from_support_profile(&profile);
    let matrix = ForgeQueryRuntimePublicSupportMatrix::from_public_api_contract(&contract);
    project_support_snapshot(&matrix)
}

fn write_deferred_snapshot() -> ForgeQuerySupportSnapshot {
    snapshot_from_profile(
        ForgeQueryRuntimeSupportProfile::scaffold_backend_profile().with_family_support(
            ForgeQueryRuntimeFamilySupport::deferred(
                crate::runtime::ForgeQueryRuntimeFacadeFamily::Write,
                "phase five hostile drift fixture",
            ),
        ),
    )
}

fn empty_family_snapshot() -> ForgeQuerySupportSnapshot {
    snapshot_from_profile(ForgeQueryRuntimeSupportProfile::new(std::iter::empty::<
        ForgeQueryRuntimeFamilySupport,
    >()))
}
