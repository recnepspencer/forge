use crate::ProtectedPhysicalByteView;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IntegrityEntryRequest<'lease> {
    protected_view: ProtectedPhysicalByteView<'lease>,
}

impl<'lease> IntegrityEntryRequest<'lease> {
    pub const fn new(protected_view: ProtectedPhysicalByteView<'lease>) -> Self {
        Self { protected_view }
    }

    pub const fn protected_view(self) -> ProtectedPhysicalByteView<'lease> {
        self.protected_view
    }
}
