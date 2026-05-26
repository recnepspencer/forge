use crate::application::{
    ForgeQueryAdmittedConfiguredDomainHandle, ForgeQueryDeclarationFamilyMarker,
    ForgeQueryDeclarationInput, ForgeQueryDomainEntryMarker, ForgeQueryDomainOperatingContext,
};

use super::row::{crossing_rows_for_family, ForgeQueryDeclarationEntryCrossingRow};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryDeclarationEntryCrossingInventory<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
> {
    declaration_family_key: &'static str,
    rows: Vec<ForgeQueryDeclarationEntryCrossingRow>,
    inventory_digest: String,
    _marker: std::marker::PhantomData<(D, I)>,
}

impl<D: ForgeQueryDomainEntryMarker, I: ForgeQueryDeclarationInput<D>>
    ForgeQueryDeclarationEntryCrossingInventory<D, I>
{
    pub(crate) fn new(
        declaration_family_key: &'static str,
        rows: Vec<ForgeQueryDeclarationEntryCrossingRow>,
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
    pub fn rows(&self) -> &[ForgeQueryDeclarationEntryCrossingRow] {
        &self.rows
    }
    pub fn inventory_digest(&self) -> &str {
        &self.inventory_digest
    }
}

pub(crate) fn forge_query_declaration_entry_crossing_inventory<
    D: ForgeQueryDomainEntryMarker,
    C: ForgeQueryDomainOperatingContext<D>,
    I: ForgeQueryDeclarationInput<D>,
>(
    handle: &ForgeQueryAdmittedConfiguredDomainHandle<D, C>,
) -> ForgeQueryDeclarationEntryCrossingInventory<D, I> {
    let rows = crossing_rows_for_family::<D, C, I>(handle);
    let inventory_digest = crate::identity::hash_parts(
        &rows
            .iter()
            .map(|row| row.row_digest().to_string())
            .collect::<Vec<_>>(),
    );
    ForgeQueryDeclarationEntryCrossingInventory::new(
        I::Family::semantic_family_key(),
        rows,
        inventory_digest,
    )
}
