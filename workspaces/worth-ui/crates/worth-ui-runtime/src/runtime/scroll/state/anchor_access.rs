impl super::UiScrollRuntimeState {
    #[cfg(test)]
    pub(crate) fn install_anchor(
        &mut self,
        owner: crate::runtime::scroll::UiScrollOwnerIdentity,
        incarnation: crate::runtime::scroll::UiScrollOwnerIncarnation,
        anchor: crate::runtime::scroll::UiScrollAnchor,
    ) -> Result<(), crate::runtime::scroll::UiScrollRouteDenial> {
        self.exact_owner_mut(owner, incarnation)?.anchor = Some(anchor);
        Ok(())
    }

    pub(in crate::runtime::scroll) fn owner_anchor(
        &self,
        owner: crate::runtime::scroll::UiScrollOwnerIdentity,
        incarnation: crate::runtime::scroll::UiScrollOwnerIncarnation,
    ) -> Result<
        Option<crate::runtime::scroll::UiScrollAnchor>,
        crate::runtime::scroll::UiScrollRouteDenial,
    > {
        Ok(self.exact_owner(owner, incarnation)?.anchor)
    }
}
