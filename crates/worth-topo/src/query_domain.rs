use forge_query::facade::{
    ForgeQueryAdmittedConfiguredDomainHandle, ForgeQueryApplicationFacade,
    ForgeQueryCapabilityFamily, ForgeQueryConfigSectionFamily,
    ForgeQueryConfiguredDomainHandleChecked, ForgeQueryDomainEntryChecked,
    ForgeQueryDomainEntryMarker, ForgeQueryDomainEntryProofRoot, ForgeQueryDomainEntryRoot,
    ForgeQueryDomainOperatingContext,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TopologyQueryDomain {
    _sealed: (),
}

impl TopologyQueryDomain {
    const fn new() -> Self {
        Self { _sealed: () }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TopologyCurrentHeadAuthoritativeContext {
    _sealed: (),
}

impl TopologyCurrentHeadAuthoritativeContext {
    const fn new() -> Self {
        Self { _sealed: () }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TopologySnapshotReadOnlyContext {
    _sealed: (),
}

impl TopologySnapshotReadOnlyContext {
    const fn new() -> Self {
        Self { _sealed: () }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct TopologyEntrySupportContract {
    required_capability_families: &'static [ForgeQueryCapabilityFamily],
    required_config_sections: &'static [ForgeQueryConfigSectionFamily],
}

impl TopologyEntrySupportContract {
    const fn new(
        required_capability_families: &'static [ForgeQueryCapabilityFamily],
        required_config_sections: &'static [ForgeQueryConfigSectionFamily],
    ) -> Self {
        Self {
            required_capability_families,
            required_config_sections,
        }
    }

    const fn required_capability_families(&self) -> &'static [ForgeQueryCapabilityFamily] {
        self.required_capability_families
    }

    const fn required_config_sections(&self) -> &'static [ForgeQueryConfigSectionFamily] {
        self.required_config_sections
    }
}

const TOPOLOGY_DOMAIN_ENTRY_SUPPORT: TopologyEntrySupportContract =
    TopologyEntrySupportContract::new(
        &[
            ForgeQueryCapabilityFamily::QueryComposition,
            ForgeQueryCapabilityFamily::QueryContext,
        ],
        &[ForgeQueryConfigSectionFamily::Query],
    );

const TOPOLOGY_CURRENT_HEAD_SUPPORT: TopologyEntrySupportContract =
    TopologyEntrySupportContract::new(
        &[
            ForgeQueryCapabilityFamily::QueryRead,
            ForgeQueryCapabilityFamily::WorkflowOrchestration,
            ForgeQueryCapabilityFamily::IdentityEvolution,
        ],
        &[
            ForgeQueryConfigSectionFamily::Query,
            ForgeQueryConfigSectionFamily::Relational,
        ],
    );

const TOPOLOGY_SNAPSHOT_SUPPORT: TopologyEntrySupportContract = TopologyEntrySupportContract::new(
    &[
        ForgeQueryCapabilityFamily::QueryRead,
        ForgeQueryCapabilityFamily::HistoricalEvaluation,
    ],
    &[
        ForgeQueryConfigSectionFamily::Query,
        ForgeQueryConfigSectionFamily::Relational,
    ],
);

pub(crate) const TOPOLOGY_CURRENT_HEAD_AUTHORITATIVE_CONTEXT_IDENTITY: &str =
    "topology/current_head_authoritative";
pub(crate) const TOPOLOGY_SNAPSHOT_READ_ONLY_CONTEXT_IDENTITY: &str = "topology/snapshot_read_only";

pub type TopologyCurrentHeadConfiguredDomainHandle = ForgeQueryAdmittedConfiguredDomainHandle<
    TopologyQueryDomain,
    TopologyCurrentHeadAuthoritativeContext,
>;
pub type TopologyCurrentHeadConfiguredDomainHandleChecked = ForgeQueryConfiguredDomainHandleChecked<
    TopologyQueryDomain,
    TopologyCurrentHeadAuthoritativeContext,
>;
pub type TopologySnapshotReadOnlyConfiguredDomainHandle =
    ForgeQueryAdmittedConfiguredDomainHandle<TopologyQueryDomain, TopologySnapshotReadOnlyContext>;
pub type TopologySnapshotReadOnlyConfiguredDomainHandleChecked =
    ForgeQueryConfiguredDomainHandleChecked<TopologyQueryDomain, TopologySnapshotReadOnlyContext>;

pub fn topology_query_domain() -> TopologyQueryDomain {
    TopologyQueryDomain::new()
}

pub fn topology_current_head_authoritative_context() -> TopologyCurrentHeadAuthoritativeContext {
    TopologyCurrentHeadAuthoritativeContext::new()
}

pub fn topology_snapshot_read_only_context() -> TopologySnapshotReadOnlyContext {
    TopologySnapshotReadOnlyContext::new()
}

pub fn topology_query_domain_entry(
    query: &ForgeQueryApplicationFacade,
) -> ForgeQueryDomainEntryRoot<TopologyQueryDomain> {
    query.domain(topology_query_domain())
}

pub fn topology_query_domain_entry_checked(
    query: &ForgeQueryApplicationFacade,
) -> ForgeQueryDomainEntryChecked<TopologyQueryDomain> {
    query.domain_checked(topology_query_domain())
}

pub fn topology_query_domain_proof_root(
    query: &ForgeQueryApplicationFacade,
) -> ForgeQueryDomainEntryProofRoot<TopologyQueryDomain> {
    query.domain_proof_root(topology_query_domain())
}

impl ForgeQueryDomainEntryMarker for TopologyQueryDomain {
    fn domain_key(&self) -> &'static str {
        "worth.topology"
    }

    fn display_name(&self) -> &'static str {
        "TopologyQueryDomain"
    }

    fn required_capability_families(&self) -> &'static [ForgeQueryCapabilityFamily] {
        TOPOLOGY_DOMAIN_ENTRY_SUPPORT.required_capability_families()
    }
}

impl ForgeQueryDomainOperatingContext<TopologyQueryDomain>
    for TopologyCurrentHeadAuthoritativeContext
{
    fn required_capability_families(&self) -> &'static [ForgeQueryCapabilityFamily] {
        TOPOLOGY_CURRENT_HEAD_SUPPORT.required_capability_families()
    }

    fn required_config_sections(&self) -> &'static [ForgeQueryConfigSectionFamily] {
        TOPOLOGY_CURRENT_HEAD_SUPPORT.required_config_sections()
    }

    fn context_identity_digest(&self) -> String {
        TOPOLOGY_CURRENT_HEAD_AUTHORITATIVE_CONTEXT_IDENTITY.to_string()
    }
}

impl ForgeQueryDomainOperatingContext<TopologyQueryDomain> for TopologySnapshotReadOnlyContext {
    fn required_capability_families(&self) -> &'static [ForgeQueryCapabilityFamily] {
        TOPOLOGY_SNAPSHOT_SUPPORT.required_capability_families()
    }

    fn required_config_sections(&self) -> &'static [ForgeQueryConfigSectionFamily] {
        TOPOLOGY_SNAPSHOT_SUPPORT.required_config_sections()
    }

    fn context_identity_digest(&self) -> String {
        TOPOLOGY_SNAPSHOT_READ_ONLY_CONTEXT_IDENTITY.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::{
        topology_current_head_authoritative_context, topology_query_domain,
        topology_query_domain_entry, topology_query_domain_entry_checked,
        topology_query_domain_proof_root, topology_snapshot_read_only_context,
        TopologyCurrentHeadConfiguredDomainHandleChecked,
        TopologySnapshotReadOnlyConfiguredDomainHandleChecked,
    };
    use forge_query::facade::{
        ForgeQueryApplicationFacade, ForgeQueryConfiguredDomainHandleChecked,
    };

    #[test]
    fn ordinary_checked_and_proof_entry_roots_share_the_same_domain_identity() {
        let query = ForgeQueryApplicationFacade::runtime_backed_default();
        let ordinary = topology_query_domain_entry(&query);
        let checked = topology_query_domain_entry_checked(&query);
        let proof = topology_query_domain_proof_root(&query);

        assert_eq!(ordinary.marker(), topology_query_domain());
        assert_eq!(ordinary.domain_key(), "worth.topology");
        assert_eq!(ordinary.display_name(), "TopologyQueryDomain");
        assert_eq!(ordinary.support_snapshot(), proof.support_snapshot());
        match checked {
            forge_query::facade::ForgeQueryDomainEntryChecked::Admitted(value) => {
                assert_eq!(ordinary.support_snapshot(), value.support_snapshot());
            }
            other => panic!("expected admitted topology domain entry, got {other:?}"),
        }
    }

    #[test]
    fn current_head_context_admits_through_ordinary_and_checked_paths() {
        let query = ForgeQueryApplicationFacade::runtime_backed_default();
        let ordinary = topology_query_domain_entry(&query)
            .with_operating_context(topology_current_head_authoritative_context())
            .validate()
            .expect("current-head context should validate")
            .admit()
            .expect("current-head context should admit");
        let checked: TopologyCurrentHeadConfiguredDomainHandleChecked =
            topology_query_domain_entry_checked(&query)
                .with_operating_context(topology_current_head_authoritative_context());

        match checked {
            ForgeQueryConfiguredDomainHandleChecked::Admitted(handle) => {
                assert_eq!(
                    ordinary.handle_identity_digest(),
                    handle.handle_identity_digest()
                );
                assert_eq!(ordinary.support_snapshot(), handle.support_snapshot());
            }
            other => panic!("expected admitted current-head handle, got {other:?}"),
        }
    }

    #[test]
    fn snapshot_context_admits_through_ordinary_and_checked_paths() {
        let query = ForgeQueryApplicationFacade::runtime_backed_default();
        let ordinary = topology_query_domain_entry(&query)
            .with_operating_context(topology_snapshot_read_only_context())
            .validate()
            .expect("snapshot context should validate")
            .admit()
            .expect("snapshot context should admit");
        let checked: TopologySnapshotReadOnlyConfiguredDomainHandleChecked =
            topology_query_domain_entry_checked(&query)
                .with_operating_context(topology_snapshot_read_only_context());

        match checked {
            ForgeQueryConfiguredDomainHandleChecked::Admitted(handle) => {
                assert_eq!(
                    ordinary.handle_identity_digest(),
                    handle.handle_identity_digest()
                );
                assert_eq!(ordinary.support_snapshot(), handle.support_snapshot());
            }
            other => panic!("expected admitted snapshot handle, got {other:?}"),
        }
    }

    #[test]
    fn current_head_and_snapshot_contexts_have_distinct_handle_identity() {
        let query = ForgeQueryApplicationFacade::runtime_backed_default();
        let current_head = topology_query_domain_entry(&query)
            .with_operating_context(topology_current_head_authoritative_context())
            .validate()
            .expect("current-head context should validate")
            .admit()
            .expect("current-head context should admit");
        let snapshot = topology_query_domain_entry(&query)
            .with_operating_context(topology_snapshot_read_only_context())
            .validate()
            .expect("snapshot context should validate")
            .admit()
            .expect("snapshot context should admit");

        assert_ne!(
            current_head.handle_identity_digest(),
            snapshot.handle_identity_digest()
        );
        assert_ne!(
            current_head.operating_context_identity_digest(),
            snapshot.operating_context_identity_digest()
        );
    }
}
