use forge_query::facade::ForgeQueryGroupedDeclarationInput;

use crate::bindings::query_native_rebinding::PrimitiveRebindingQueryDomain;
use crate::bindings::query_native_rebinding_authoring::PrimitiveRebindingDeclarationEntry;

pub fn primitive_rebinding_local_neighborhood(
    seed_member: PrimitiveRebindingDeclarationEntry,
) -> ForgeQueryGroupedDeclarationInput<
    PrimitiveRebindingQueryDomain,
    PrimitiveRebindingDeclarationEntry,
> {
    ForgeQueryGroupedDeclarationInput::local_neighborhood(seed_member)
}
