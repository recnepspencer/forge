use crate::application::{
    ForgeQueryAdmittedConfiguredDomainHandle, ForgeQueryDeclarationEnvelopeChecked,
    ForgeQueryDeclarationEnvelopeOrchestrationTranscript, ForgeQueryDeclarationInput,
    ForgeQueryDeclarationReceiptChecked, ForgeQueryDeclarationReceiptOrchestrationTranscript,
    ForgeQueryDeclarationRouteOrchestrationTranscript, ForgeQueryDeclarationRoutePlanChecked,
    ForgeQueryDomainEntryMarker, ForgeQueryDomainOperatingContext,
};

use super::artifact::{
    ForgeQueryGroupedDeclarationArtifact, ForgeQueryGroupedDeclarationAspectRecord,
};
use super::posture::ForgeQueryGroupedMemberRole;

pub struct ForgeQueryGroupedProductMember<P> {
    member_index: usize,
    role: ForgeQueryGroupedMemberRole,
    aspect_record: ForgeQueryGroupedDeclarationAspectRecord,
    product: P,
}

impl<P> ForgeQueryGroupedProductMember<P> {
    fn new(
        member_index: usize,
        role: ForgeQueryGroupedMemberRole,
        aspect_record: ForgeQueryGroupedDeclarationAspectRecord,
        product: P,
    ) -> Self {
        Self {
            member_index,
            role,
            aspect_record,
            product,
        }
    }

    pub fn member_index(&self) -> usize {
        self.member_index
    }

    pub fn role(&self) -> ForgeQueryGroupedMemberRole {
        self.role
    }

    pub fn aspect_record(&self) -> &ForgeQueryGroupedDeclarationAspectRecord {
        &self.aspect_record
    }

    pub fn product(&self) -> &P {
        &self.product
    }
}

macro_rules! define_grouped_projection {
    ($name:ident, $member:ident, $product:ty) => {
        pub struct $name<D: ForgeQueryDomainEntryMarker, I: ForgeQueryDeclarationInput<D>> {
            declaration: ForgeQueryGroupedDeclarationArtifact<D, I>,
            members: Vec<$member<$product>>,
        }

        impl<D: ForgeQueryDomainEntryMarker, I: ForgeQueryDeclarationInput<D>> $name<D, I> {
            fn new(
                declaration: ForgeQueryGroupedDeclarationArtifact<D, I>,
                members: Vec<$member<$product>>,
            ) -> Self {
                Self {
                    declaration,
                    members,
                }
            }

            pub fn declaration(&self) -> &ForgeQueryGroupedDeclarationArtifact<D, I> {
                &self.declaration
            }

            pub fn members(&self) -> &[$member<$product>] {
                &self.members
            }
        }
    };
}

pub type ForgeQueryGroupedRouteMember<P> = ForgeQueryGroupedProductMember<P>;
pub type ForgeQueryGroupedReceiptMember<P> = ForgeQueryGroupedProductMember<P>;
pub type ForgeQueryGroupedEnvelopeMember<P> = ForgeQueryGroupedProductMember<P>;

define_grouped_projection!(
    ForgeQueryGroupedRouteChecked,
    ForgeQueryGroupedRouteMember,
    ForgeQueryDeclarationRoutePlanChecked<D, I>
);
define_grouped_projection!(
    ForgeQueryGroupedRouteTranscript,
    ForgeQueryGroupedRouteMember,
    ForgeQueryDeclarationRouteOrchestrationTranscript<D, I>
);
define_grouped_projection!(
    ForgeQueryGroupedReceiptChecked,
    ForgeQueryGroupedReceiptMember,
    ForgeQueryDeclarationReceiptChecked<D, I>
);
define_grouped_projection!(
    ForgeQueryGroupedReceiptTranscript,
    ForgeQueryGroupedReceiptMember,
    ForgeQueryDeclarationReceiptOrchestrationTranscript<D, I>
);
define_grouped_projection!(
    ForgeQueryGroupedEnvelopeChecked,
    ForgeQueryGroupedEnvelopeMember,
    ForgeQueryDeclarationEnvelopeChecked<D, I>
);
define_grouped_projection!(
    ForgeQueryGroupedEnvelopeTranscript,
    ForgeQueryGroupedEnvelopeMember,
    ForgeQueryDeclarationEnvelopeOrchestrationTranscript<D, I>
);

pub(crate) fn forge_query_grouped_route_checked_on_handle<
    D: ForgeQueryDomainEntryMarker,
    C: ForgeQueryDomainOperatingContext<D>,
    I: ForgeQueryDeclarationInput<D> + Clone,
>(
    handle: &ForgeQueryAdmittedConfiguredDomainHandle<D, C>,
    declaration: ForgeQueryGroupedDeclarationArtifact<D, I>,
) -> ForgeQueryGroupedRouteChecked<D, I> {
    ForgeQueryGroupedRouteChecked::new(
        declaration.clone(),
        declaration
            .members()
            .iter()
            .map(|member| {
                ForgeQueryGroupedRouteMember::new(
                    member.member_index(),
                    member.role(),
                    member.aspect_record().clone(),
                    handle.orchestrate_routes_from_progressed_checked(member.progression().clone()),
                )
            })
            .collect(),
    )
}

pub(crate) fn forge_query_grouped_route_proof_on_handle<
    D: ForgeQueryDomainEntryMarker,
    C: ForgeQueryDomainOperatingContext<D>,
    I: ForgeQueryDeclarationInput<D> + Clone,
>(
    handle: &ForgeQueryAdmittedConfiguredDomainHandle<D, C>,
    declaration: ForgeQueryGroupedDeclarationArtifact<D, I>,
) -> ForgeQueryGroupedRouteTranscript<D, I> {
    ForgeQueryGroupedRouteTranscript::new(
        declaration.clone(),
        declaration
            .members()
            .iter()
            .map(|member| {
                ForgeQueryGroupedRouteMember::new(
                    member.member_index(),
                    member.role(),
                    member.aspect_record().clone(),
                    handle.orchestrate_routes_from_progressed_proof(member.progression().clone()),
                )
            })
            .collect(),
    )
}

pub(crate) fn forge_query_grouped_receipt_checked_on_handle<
    D: ForgeQueryDomainEntryMarker,
    C: ForgeQueryDomainOperatingContext<D>,
    I: ForgeQueryDeclarationInput<D> + Clone,
>(
    handle: &ForgeQueryAdmittedConfiguredDomainHandle<D, C>,
    declaration: ForgeQueryGroupedDeclarationArtifact<D, I>,
) -> ForgeQueryGroupedReceiptChecked<D, I> {
    ForgeQueryGroupedReceiptChecked::new(
        declaration.clone(),
        declaration
            .members()
            .iter()
            .map(|member| {
                ForgeQueryGroupedReceiptMember::new(
                    member.member_index(),
                    member.role(),
                    member.aspect_record().clone(),
                    handle
                        .orchestrate_receipt_from_progressed_checked(member.progression().clone()),
                )
            })
            .collect(),
    )
}

pub(crate) fn forge_query_grouped_receipt_proof_on_handle<
    D: ForgeQueryDomainEntryMarker,
    C: ForgeQueryDomainOperatingContext<D>,
    I: ForgeQueryDeclarationInput<D> + Clone,
>(
    handle: &ForgeQueryAdmittedConfiguredDomainHandle<D, C>,
    declaration: ForgeQueryGroupedDeclarationArtifact<D, I>,
) -> ForgeQueryGroupedReceiptTranscript<D, I> {
    ForgeQueryGroupedReceiptTranscript::new(
        declaration.clone(),
        declaration
            .members()
            .iter()
            .map(|member| {
                ForgeQueryGroupedReceiptMember::new(
                    member.member_index(),
                    member.role(),
                    member.aspect_record().clone(),
                    handle.orchestrate_receipt_from_progressed_proof(member.progression().clone()),
                )
            })
            .collect(),
    )
}

pub(crate) fn forge_query_grouped_envelope_checked_on_handle<
    D: ForgeQueryDomainEntryMarker,
    C: ForgeQueryDomainOperatingContext<D>,
    I: ForgeQueryDeclarationInput<D> + Clone,
>(
    handle: &ForgeQueryAdmittedConfiguredDomainHandle<D, C>,
    declaration: ForgeQueryGroupedDeclarationArtifact<D, I>,
) -> ForgeQueryGroupedEnvelopeChecked<D, I> {
    ForgeQueryGroupedEnvelopeChecked::new(
        declaration.clone(),
        declaration
            .members()
            .iter()
            .map(|member| {
                ForgeQueryGroupedEnvelopeMember::new(
                    member.member_index(),
                    member.role(),
                    member.aspect_record().clone(),
                    handle
                        .orchestrate_envelope_from_progressed_checked(member.progression().clone()),
                )
            })
            .collect(),
    )
}

pub(crate) fn forge_query_grouped_envelope_proof_on_handle<
    D: ForgeQueryDomainEntryMarker,
    C: ForgeQueryDomainOperatingContext<D>,
    I: ForgeQueryDeclarationInput<D> + Clone,
>(
    handle: &ForgeQueryAdmittedConfiguredDomainHandle<D, C>,
    declaration: ForgeQueryGroupedDeclarationArtifact<D, I>,
) -> ForgeQueryGroupedEnvelopeTranscript<D, I> {
    ForgeQueryGroupedEnvelopeTranscript::new(
        declaration.clone(),
        declaration
            .members()
            .iter()
            .map(|member| {
                ForgeQueryGroupedEnvelopeMember::new(
                    member.member_index(),
                    member.role(),
                    member.aspect_record().clone(),
                    handle.orchestrate_envelope_from_progressed_proof(member.progression().clone()),
                )
            })
            .collect(),
    )
}
