#[derive(Debug)]
pub(crate) struct UiGraphConstraintMintAuthority(());

impl UiGraphConstraintMintAuthority {
    pub(super) const fn mint() -> Self {
        Self(())
    }
}
