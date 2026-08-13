use std::marker::PhantomData;

use crate::authority_cryptography::{AuthoritySeal, AuthoritySealDomain, AuthorityTranscript};
use crate::domain_operation::WorthQueryConditionalNodeRef;
use crate::installed_domain_operation::WorthQueryInstalledDomainOperationAuthority;

use super::{
    WorthQueryInstalledApplicationOperation,
    WorthQueryPortableApplicationConditionalOperationBinding,
};

type ConditionalOperationMarker<Schema, ApplicationOperation, Input, D, O, F> =
    fn(Input) -> (Schema, ApplicationOperation, D, O, F);
type ConditionalNodeMarker<Schema, ApplicationOperation, Input, D, O, F, N> =
    fn(Input) -> (Schema, ApplicationOperation, D, O, F, N);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryConditionalApplicationOperationDenialKind {
    ForeignRuntime,
    StaleGeneration,
    ApplicationOperationChanged,
    BindingNotInstalled,
    BindingMeaningChanged,
    DomainOperationNotInstalled,
    DomainOperationChanged,
    NodeNotDeclared,
    NodeMeaningChanged,
    ProviderIdentityInvalid,
    NodeNotTemporal,
    HostClockNotRequired,
    ClockIdentityInvalid,
    TemporalIntentQueryForeign,
    TemporalIntentProjectorInvalid,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryConditionalApplicationOperationDenial {
    kind: WorthQueryConditionalApplicationOperationDenialKind,
    subject: String,
}

impl WorthQueryConditionalApplicationOperationDenial {
    pub(crate) fn new(
        kind: WorthQueryConditionalApplicationOperationDenialKind,
        subject: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            subject: subject.into(),
        }
    }

    pub fn kind(&self) -> WorthQueryConditionalApplicationOperationDenialKind {
        self.kind
    }

    pub fn subject(&self) -> &str {
        &self.subject
    }
}

/// Move-only installed binding between one exact application operation and
/// the domain operation that owns its conditional declarations.
pub struct WorthQueryInstalledApplicationConditionalOperation<
    Schema,
    ApplicationOperation,
    Input,
    D,
    O,
    F,
> {
    application_operation:
        WorthQueryInstalledApplicationOperation<Schema, ApplicationOperation, Input>,
    domain_operation: WorthQueryInstalledDomainOperationAuthority,
    binding: WorthQueryPortableApplicationConditionalOperationBinding,
    authority_identity: AuthoritySeal,
    marker: PhantomData<ConditionalOperationMarker<Schema, ApplicationOperation, Input, D, O, F>>,
}

impl<Schema, ApplicationOperation, Input, D, O, F>
    WorthQueryInstalledApplicationConditionalOperation<Schema, ApplicationOperation, Input, D, O, F>
{
    pub(crate) fn new(
        application_operation: WorthQueryInstalledApplicationOperation<
            Schema,
            ApplicationOperation,
            Input,
        >,
        domain_operation: WorthQueryInstalledDomainOperationAuthority,
        binding: WorthQueryPortableApplicationConditionalOperationBinding,
    ) -> Self {
        let authority_identity =
            conditional_operation_seal(&application_operation, &domain_operation, &binding);
        Self {
            application_operation,
            domain_operation,
            binding,
            authority_identity,
            marker: PhantomData,
        }
    }

    pub fn application_operation(
        &self,
    ) -> &WorthQueryInstalledApplicationOperation<Schema, ApplicationOperation, Input> {
        &self.application_operation
    }

    pub fn domain_operation(&self) -> &WorthQueryInstalledDomainOperationAuthority {
        &self.domain_operation
    }

    pub fn binding(&self) -> &WorthQueryPortableApplicationConditionalOperationBinding {
        &self.binding
    }

    pub fn authority_identity(&self) -> &str {
        self.authority_identity.as_str()
    }

    pub fn bind_node<N>(
        self,
        reference: WorthQueryConditionalNodeRef<D, O, F, N>,
    ) -> Result<
        WorthQueryInstalledApplicationConditionalNode<
            Schema,
            ApplicationOperation,
            Input,
            D,
            O,
            F,
            N,
        >,
        WorthQueryConditionalApplicationOperationDenial,
    > {
        let declaration = self
            .domain_operation
            .conditional_node_declaration(reference.location())
            .map_err(|_| {
                WorthQueryConditionalApplicationOperationDenial::new(
                    WorthQueryConditionalApplicationOperationDenialKind::NodeNotDeclared,
                    reference.node_identity(),
                )
            })?;
        let authority_identity = conditional_node_seal(&self, reference.location(), &declaration);
        Ok(WorthQueryInstalledApplicationConditionalNode {
            operation: self,
            location: reference.location().clone(),
            declaration,
            authority_identity,
            marker: PhantomData,
        })
    }
}

/// Move-only exact installed conditional node. It retains its complete
/// application/domain operation binding and exposes no lower-runtime node or
/// scheduling capability.
pub struct WorthQueryInstalledApplicationConditionalNode<
    Schema,
    ApplicationOperation,
    Input,
    D,
    O,
    F,
    N,
> {
    operation: WorthQueryInstalledApplicationConditionalOperation<
        Schema,
        ApplicationOperation,
        Input,
        D,
        O,
        F,
    >,
    location: crate::domain_operation::WorthQueryConditionalNodeLocation,
    declaration: crate::domain_operation::WorthQueryPortableConditionalNodeDeclaration,
    authority_identity: AuthoritySeal,
    marker: PhantomData<ConditionalNodeMarker<Schema, ApplicationOperation, Input, D, O, F, N>>,
}

impl<Schema, ApplicationOperation, Input, D, O, F, N>
    WorthQueryInstalledApplicationConditionalNode<Schema, ApplicationOperation, Input, D, O, F, N>
{
    pub fn operation(
        &self,
    ) -> &WorthQueryInstalledApplicationConditionalOperation<
        Schema,
        ApplicationOperation,
        Input,
        D,
        O,
        F,
    > {
        &self.operation
    }

    pub fn location(&self) -> &crate::domain_operation::WorthQueryConditionalNodeLocation {
        &self.location
    }

    pub fn declaration(
        &self,
    ) -> &crate::domain_operation::WorthQueryPortableConditionalNodeDeclaration {
        &self.declaration
    }

    pub fn authority_identity(&self) -> &str {
        self.authority_identity.as_str()
    }
}

fn conditional_operation_seal<Schema, ApplicationOperation, Input>(
    application_operation: &WorthQueryInstalledApplicationOperation<
        Schema,
        ApplicationOperation,
        Input,
    >,
    domain_operation: &WorthQueryInstalledDomainOperationAuthority,
    binding: &WorthQueryPortableApplicationConditionalOperationBinding,
) -> AuthoritySeal {
    let mut transcript = AuthorityTranscript::new(
        &domain_operation.package_authority_key,
        AuthoritySealDomain::InstalledConditionalApplicationOperation,
    );
    transcript.u64("runtime", domain_operation.runtime_ordinal());
    transcript.u64("generation", domain_operation.generation().ordinal());
    transcript.bytes(
        "application-operation-authority",
        &application_operation.authority_identity_bytes(),
    );
    transcript.text("schema", binding.schema_name());
    transcript.text("application-operation", binding.application_operation());
    transcript.text("domain-operation-slot", binding.domain_operation_slot());
    transcript.text(
        "domain-operation-identity",
        binding.domain_operation_canonical_identity(),
    );
    transcript.finish()
}

fn conditional_node_seal<Schema, ApplicationOperation, Input, D, O, F>(
    operation: &WorthQueryInstalledApplicationConditionalOperation<
        Schema,
        ApplicationOperation,
        Input,
        D,
        O,
        F,
    >,
    location: &crate::domain_operation::WorthQueryConditionalNodeLocation,
    declaration: &crate::domain_operation::WorthQueryPortableConditionalNodeDeclaration,
) -> AuthoritySeal {
    let mut transcript = AuthorityTranscript::new(
        &operation.domain_operation.package_authority_key,
        AuthoritySealDomain::InstalledConditionalNode,
    );
    transcript.bytes(
        "conditional-operation",
        operation.authority_identity.bytes(),
    );
    transcript.optional_text("stage", location.stage_identity());
    transcript.text("node", location.node_identity());
    transcript.text("declaration", &declaration.canonical_token());
    transcript.finish()
}
