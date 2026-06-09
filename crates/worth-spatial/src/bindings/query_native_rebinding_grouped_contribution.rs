use forge_query::facade::{
    ForgeQueryAdmittedConfiguredDomainHandle, ForgeQueryDomainOperatingContext,
    ForgeQueryGroupedContributionInput, ForgeQueryGroupedDeclarationInput,
};

use crate::bindings::query_native_rebinding::PrimitiveRebindingQueryDomain;
use crate::bindings::query_native_rebinding_authoring::PrimitiveRebindingDeclarationEntry;
use crate::bindings::query_native_rebinding_contribution::primitive_rebinding_semantic_contributions;

pub fn primitive_rebinding_local_neighborhood_contributions<C>(
    declaration: ForgeQueryGroupedDeclarationInput<
        PrimitiveRebindingQueryDomain,
        PrimitiveRebindingDeclarationEntry,
    >,
    handle: &ForgeQueryAdmittedConfiguredDomainHandle<PrimitiveRebindingQueryDomain, C>,
) -> ForgeQueryGroupedContributionInput<
    PrimitiveRebindingQueryDomain,
    PrimitiveRebindingDeclarationEntry,
>
where
    C: ForgeQueryDomainOperatingContext<PrimitiveRebindingQueryDomain>,
{
    let member_contributions = declaration
        .member_inputs()
        .iter()
        .enumerate()
        .flat_map(|(index, member)| {
            primitive_rebinding_semantic_contributions(member, handle)
                .into_iter()
                .map(move |contribution| (index, contribution))
        })
        .collect::<Vec<_>>();
    member_contributions.into_iter().fold(
        ForgeQueryGroupedContributionInput::new(declaration),
        |input, (member_index, contribution)| {
            input.with_member_contribution(member_index, contribution)
        },
    )
}
