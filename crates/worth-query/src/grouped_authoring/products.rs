use crate::application::{
    WorthQueryAdmittedConfiguredDomainHandle, WorthQueryDeclarationEnvelopeChecked,
    WorthQueryDeclarationEnvelopeOrchestrationTranscript, WorthQueryDeclarationInput,
    WorthQueryDeclarationReceiptChecked, WorthQueryDeclarationReceiptOrchestrationTranscript,
    WorthQueryDeclarationRouteOrchestrationTranscript, WorthQueryDeclarationRoutePlanChecked,
    WorthQueryDomainEntryMarker, WorthQueryDomainOperatingContext,
};

use super::artifact::{
    WorthQueryGroupedDeclarationArtifact, WorthQueryGroupedDeclarationAspectRecord,
};
use super::posture::WorthQueryGroupedMemberRole;

pub struct WorthQueryGroupedProductMember<P> {
    member_index: usize,
    role: WorthQueryGroupedMemberRole,
    aspect_record: WorthQueryGroupedDeclarationAspectRecord,
    product: P,
}

impl<P> WorthQueryGroupedProductMember<P> {
    fn new(
        member_index: usize,
        role: WorthQueryGroupedMemberRole,
        aspect_record: WorthQueryGroupedDeclarationAspectRecord,
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

    pub fn role(&self) -> WorthQueryGroupedMemberRole {
        self.role
    }

    pub fn aspect_record(&self) -> &WorthQueryGroupedDeclarationAspectRecord {
        &self.aspect_record
    }

    pub fn product(&self) -> &P {
        &self.product
    }
}

macro_rules! define_grouped_projection {
    ($name:ident, $member:ident, $product:ty) => {
        pub struct $name<D: WorthQueryDomainEntryMarker, I: WorthQueryDeclarationInput<D>> {
            declaration: WorthQueryGroupedDeclarationArtifact<D, I>,
            members: Vec<$member<$product>>,
        }

        impl<D: WorthQueryDomainEntryMarker, I: WorthQueryDeclarationInput<D>> $name<D, I> {
            fn new(
                declaration: WorthQueryGroupedDeclarationArtifact<D, I>,
                members: Vec<$member<$product>>,
            ) -> Self {
                Self {
                    declaration,
                    members,
                }
            }

            pub fn declaration(&self) -> &WorthQueryGroupedDeclarationArtifact<D, I> {
                &self.declaration
            }

            pub fn members(&self) -> &[$member<$product>] {
                &self.members
            }
        }
    };
}

pub type WorthQueryGroupedRouteMember<P> = WorthQueryGroupedProductMember<P>;
pub type WorthQueryGroupedReceiptMember<P> = WorthQueryGroupedProductMember<P>;
pub type WorthQueryGroupedEnvelopeMember<P> = WorthQueryGroupedProductMember<P>;

define_grouped_projection!(
    WorthQueryGroupedRouteChecked,
    WorthQueryGroupedRouteMember,
    WorthQueryDeclarationRoutePlanChecked<D, I>
);
define_grouped_projection!(
    WorthQueryGroupedRouteTranscript,
    WorthQueryGroupedRouteMember,
    WorthQueryDeclarationRouteOrchestrationTranscript<D, I>
);
define_grouped_projection!(
    WorthQueryGroupedReceiptChecked,
    WorthQueryGroupedReceiptMember,
    WorthQueryDeclarationReceiptChecked<D, I>
);
define_grouped_projection!(
    WorthQueryGroupedReceiptTranscript,
    WorthQueryGroupedReceiptMember,
    WorthQueryDeclarationReceiptOrchestrationTranscript<D, I>
);
define_grouped_projection!(
    WorthQueryGroupedEnvelopeChecked,
    WorthQueryGroupedEnvelopeMember,
    WorthQueryDeclarationEnvelopeChecked<D, I>
);
define_grouped_projection!(
    WorthQueryGroupedEnvelopeTranscript,
    WorthQueryGroupedEnvelopeMember,
    WorthQueryDeclarationEnvelopeOrchestrationTranscript<D, I>
);

pub(crate) fn worth_query_grouped_route_checked_on_handle<
    D: WorthQueryDomainEntryMarker,
    C: WorthQueryDomainOperatingContext<D>,
    I: WorthQueryDeclarationInput<D> + Clone,
>(
    handle: &WorthQueryAdmittedConfiguredDomainHandle<D, C>,
    declaration: WorthQueryGroupedDeclarationArtifact<D, I>,
) -> WorthQueryGroupedRouteChecked<D, I> {
    WorthQueryGroupedRouteChecked::new(
        declaration.clone(),
        declaration
            .members()
            .iter()
            .map(|member| {
                WorthQueryGroupedRouteMember::new(
                    member.member_index(),
                    member.role(),
                    member.aspect_record().clone(),
                    handle.orchestrate_routes_from_progressed_checked(member.progression().clone()),
                )
            })
            .collect(),
    )
}

pub(crate) fn worth_query_grouped_route_proof_on_handle<
    D: WorthQueryDomainEntryMarker,
    C: WorthQueryDomainOperatingContext<D>,
    I: WorthQueryDeclarationInput<D> + Clone,
>(
    handle: &WorthQueryAdmittedConfiguredDomainHandle<D, C>,
    declaration: WorthQueryGroupedDeclarationArtifact<D, I>,
) -> WorthQueryGroupedRouteTranscript<D, I> {
    WorthQueryGroupedRouteTranscript::new(
        declaration.clone(),
        declaration
            .members()
            .iter()
            .map(|member| {
                WorthQueryGroupedRouteMember::new(
                    member.member_index(),
                    member.role(),
                    member.aspect_record().clone(),
                    handle.orchestrate_routes_from_progressed_proof(member.progression().clone()),
                )
            })
            .collect(),
    )
}

pub(crate) fn worth_query_grouped_receipt_checked_on_handle<
    D: WorthQueryDomainEntryMarker,
    C: WorthQueryDomainOperatingContext<D>,
    I: WorthQueryDeclarationInput<D> + Clone,
>(
    handle: &WorthQueryAdmittedConfiguredDomainHandle<D, C>,
    declaration: WorthQueryGroupedDeclarationArtifact<D, I>,
) -> WorthQueryGroupedReceiptChecked<D, I> {
    WorthQueryGroupedReceiptChecked::new(
        declaration.clone(),
        declaration
            .members()
            .iter()
            .map(|member| {
                WorthQueryGroupedReceiptMember::new(
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

pub(crate) fn worth_query_grouped_receipt_proof_on_handle<
    D: WorthQueryDomainEntryMarker,
    C: WorthQueryDomainOperatingContext<D>,
    I: WorthQueryDeclarationInput<D> + Clone,
>(
    handle: &WorthQueryAdmittedConfiguredDomainHandle<D, C>,
    declaration: WorthQueryGroupedDeclarationArtifact<D, I>,
) -> WorthQueryGroupedReceiptTranscript<D, I> {
    WorthQueryGroupedReceiptTranscript::new(
        declaration.clone(),
        declaration
            .members()
            .iter()
            .map(|member| {
                WorthQueryGroupedReceiptMember::new(
                    member.member_index(),
                    member.role(),
                    member.aspect_record().clone(),
                    handle.orchestrate_receipt_from_progressed_proof(member.progression().clone()),
                )
            })
            .collect(),
    )
}

pub(crate) fn worth_query_grouped_envelope_checked_on_handle<
    D: WorthQueryDomainEntryMarker,
    C: WorthQueryDomainOperatingContext<D>,
    I: WorthQueryDeclarationInput<D> + Clone,
>(
    handle: &WorthQueryAdmittedConfiguredDomainHandle<D, C>,
    declaration: WorthQueryGroupedDeclarationArtifact<D, I>,
) -> WorthQueryGroupedEnvelopeChecked<D, I> {
    WorthQueryGroupedEnvelopeChecked::new(
        declaration.clone(),
        declaration
            .members()
            .iter()
            .map(|member| {
                WorthQueryGroupedEnvelopeMember::new(
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

pub(crate) fn worth_query_grouped_envelope_proof_on_handle<
    D: WorthQueryDomainEntryMarker,
    C: WorthQueryDomainOperatingContext<D>,
    I: WorthQueryDeclarationInput<D> + Clone,
>(
    handle: &WorthQueryAdmittedConfiguredDomainHandle<D, C>,
    declaration: WorthQueryGroupedDeclarationArtifact<D, I>,
) -> WorthQueryGroupedEnvelopeTranscript<D, I> {
    WorthQueryGroupedEnvelopeTranscript::new(
        declaration.clone(),
        declaration
            .members()
            .iter()
            .map(|member| {
                WorthQueryGroupedEnvelopeMember::new(
                    member.member_index(),
                    member.role(),
                    member.aspect_record().clone(),
                    handle.orchestrate_envelope_from_progressed_proof(member.progression().clone()),
                )
            })
            .collect(),
    )
}
