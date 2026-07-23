use super::authority::{UiAllocationInvalidationAuthority, UiInvalidationAuthorityLookupDenial};

pub(crate) struct UiScrollInvalidationAuthorityLookup<'a> {
    bindings: Option<&'a super::scroll_binding_key_index::BindingKeyIndex>,
    probes: u16,
}

impl UiAllocationInvalidationAuthority {
    pub(crate) fn scroll_target(
        &self,
        witness: crate::evidence::UiHostMeasurementAuthorityWitness,
    ) -> Result<UiScrollInvalidationAuthorityLookup<'_>, UiInvalidationAuthorityLookupDenial> {
        let host = self.host_target(witness)?;
        Ok(UiScrollInvalidationAuthorityLookup {
            bindings: self.scroll_bindings.host_extent(witness),
            probes: host.probes,
        })
    }

    pub(crate) fn scroll_settled_query_target(
        &self,
        source_key: &crate::evidence::measurement::basis::UiQueryAllocationSourceKey,
    ) -> UiScrollInvalidationAuthorityLookup<'_> {
        let (bindings, probes) = self.scroll_bindings.settled_query_extent(source_key);
        UiScrollInvalidationAuthorityLookup { bindings, probes }
    }
}

impl UiScrollInvalidationAuthorityLookup<'_> {
    pub(crate) fn is_empty(&self) -> bool {
        self.bindings
            .is_none_or(|rows| rows.values().next().is_none())
    }
    pub(crate) fn probes(&self) -> u16 {
        self.probes
    }
    pub(crate) fn materialize_bindings(self) -> Box<[super::UiAdmittedScrollInvalidationBinding]> {
        self.bindings
            .into_iter()
            .flat_map(|rows| rows.values())
            .cloned()
            .collect::<Vec<_>>()
            .into_boxed_slice()
    }
}
