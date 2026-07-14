use crate::application::{
    WorthQueryDeclarationFamilyMarker, WorthQueryDeclarationInput, WorthQueryDomainEntryMarker,
    WorthQueryDomainOperatingContext, WorthQueryInstalledDomainDeclarationContext,
};

use super::row::{crossing_rows_for_family, WorthQueryDeclarationEntryCrossingRow};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryDeclarationEntryCrossingInventory<
    D: WorthQueryDomainEntryMarker,
    I: WorthQueryDeclarationInput<D>,
> {
    declaration_family_key: &'static str,
    rows: Vec<WorthQueryDeclarationEntryCrossingRow>,
    inventory_digest: String,
    _marker: std::marker::PhantomData<(D, I)>,
}

impl<D: WorthQueryDomainEntryMarker, I: WorthQueryDeclarationInput<D>>
    WorthQueryDeclarationEntryCrossingInventory<D, I>
{
    pub(crate) fn new(
        declaration_family_key: &'static str,
        rows: Vec<WorthQueryDeclarationEntryCrossingRow>,
        inventory_digest: String,
    ) -> Self {
        Self {
            declaration_family_key,
            rows,
            inventory_digest,
            _marker: std::marker::PhantomData,
        }
    }

    pub fn declaration_family_key(&self) -> &'static str {
        self.declaration_family_key
    }
    pub fn rows(&self) -> &[WorthQueryDeclarationEntryCrossingRow] {
        &self.rows
    }
    pub fn inventory_digest(&self) -> &str {
        &self.inventory_digest
    }
}

pub(crate) fn worth_query_declaration_entry_crossing_inventory<
    D: WorthQueryDomainEntryMarker,
    C: WorthQueryDomainOperatingContext<D>,
    I: WorthQueryDeclarationInput<D>,
>(
    handle: &WorthQueryInstalledDomainDeclarationContext<D, C>,
) -> WorthQueryDeclarationEntryCrossingInventory<D, I> {
    let rows = crossing_rows_for_family::<D, C, I>(handle);
    let inventory_digest = crate::identity::hash_parts(
        &rows
            .iter()
            .map(|row| row.row_digest().to_string())
            .collect::<Vec<_>>(),
    );
    WorthQueryDeclarationEntryCrossingInventory::new(
        I::Family::semantic_family_key(),
        rows,
        inventory_digest,
    )
}
