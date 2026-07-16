use crate::application::WorthQueryInstalledDomainDeclarationContext;

use super::domain::{
    AuthorityRichFamily, BridgeSignalFamily, DeferredSignalFamily, GeometryDomain, GeometryWorld,
    MixedFamily, RelationalFamily, SignallessWorld,
};
use super::r#async::{AsyncCurrentFamily, AsyncPreviewFamily, DeferredAsyncFamily};
use super::temporal::{TemporalCurrentFamily, TemporalHistoricalFamily, TemporalPreviewFamily};

pub fn handle(
    regime: &'static str,
) -> WorthQueryInstalledDomainDeclarationContext<GeometryDomain, GeometryWorld> {
    workspace_and_handle(regime).1
}

pub fn workspace_and_handle(
    regime: &'static str,
) -> (
    crate::runtime::WorthQueryWorkspace,
    WorthQueryInstalledDomainDeclarationContext<GeometryDomain, GeometryWorld>,
) {
    crate::application::domain_test_support::installed_declaration_workspace(
        GeometryDomain,
        GeometryWorld(regime),
        declaration_families(),
    )
}

pub fn signal_disabled_handle(
    regime: &'static str,
) -> WorthQueryInstalledDomainDeclarationContext<GeometryDomain, SignallessWorld> {
    crate::application::domain_test_support::installed_declaration_context(
        GeometryDomain,
        SignallessWorld(regime),
        declaration_families(),
    )
}

fn declaration_families(
) -> [crate::domain_installation::WorthQueryDomainDeclarationFamilyDefinition; 11] {
    [
        crate::application::domain_test_support::family::<GeometryDomain, RelationalFamily>(),
        crate::application::domain_test_support::family::<GeometryDomain, BridgeSignalFamily>(),
        crate::application::domain_test_support::family::<GeometryDomain, DeferredSignalFamily>(),
        crate::application::domain_test_support::family::<GeometryDomain, MixedFamily>(),
        crate::application::domain_test_support::family::<GeometryDomain, AuthorityRichFamily>(),
        crate::application::domain_test_support::family::<GeometryDomain, AsyncCurrentFamily>(),
        crate::application::domain_test_support::family::<GeometryDomain, AsyncPreviewFamily>(),
        crate::application::domain_test_support::family::<GeometryDomain, DeferredAsyncFamily>(),
        crate::application::domain_test_support::family::<GeometryDomain, TemporalCurrentFamily>(),
        crate::application::domain_test_support::family::<GeometryDomain, TemporalPreviewFamily>(),
        crate::application::domain_test_support::family::<GeometryDomain, TemporalHistoricalFamily>(
        ),
    ]
}
