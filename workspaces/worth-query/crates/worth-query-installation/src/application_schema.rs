use std::marker::PhantomData;

use worth_query_declaration::facade::application_schema::{
    ApplicationEntityRef, ApplicationOperationRef, ApplicationSchema,
    ApplicationSchemaAuthoringContext, ApplicationSchemaBindingIdentity,
    ApplicationSchemaDeclaration, ErasedApplicationSchemaDeclaration, TypedEffectIntentBuilder,
    TypedOperationBuilder, TypedReadDeclarationBuilder,
};

use crate::installed_index::WorthQueryInstalledPackageAuthority;
use crate::package::WorthQueryPortableDomainPackageIdentity;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryInstalledApplicationSchemaDenialKind {
    DomainNotInstalled,
    SchemaNotInstalled,
    SchemaMeaningChanged,
    ForeignRuntime,
    StaleGeneration,
    PackageIdentityChanged,
    AdmissionIdentityChanged,
    AuthorityMismatch,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryInstalledApplicationSchemaDenial {
    kind: WorthQueryInstalledApplicationSchemaDenialKind,
    subject: String,
}

impl WorthQueryInstalledApplicationSchemaDenial {
    pub(crate) fn new(
        kind: WorthQueryInstalledApplicationSchemaDenialKind,
        subject: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            subject: subject.into(),
        }
    }

    pub const fn kind(&self) -> WorthQueryInstalledApplicationSchemaDenialKind {
        self.kind
    }

    pub fn subject(&self) -> &str {
        &self.subject
    }
}

impl std::fmt::Display for WorthQueryInstalledApplicationSchemaDenial {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter,
            "installed application schema denied: {:?} ({})",
            self.kind, self.subject
        )
    }
}

impl std::error::Error for WorthQueryInstalledApplicationSchemaDenial {}

/// Opaque proof that one typed schema declaration belongs to an exact
/// installed package, runtime, and generation.
pub struct WorthQueryInstalledApplicationSchema<Schema> {
    pub(crate) package_authority: WorthQueryInstalledPackageAuthority,
    pub(crate) schema_name: String,
    pub(crate) schema_identity:
        worth_query_declaration::facade::application_schema::ApplicationSchemaIdentity,
    pub(crate) schema: ErasedApplicationSchemaDeclaration,
    _schema: PhantomData<fn() -> Schema>,
}

impl<Schema> WorthQueryInstalledApplicationSchema<Schema>
where
    Schema: ApplicationSchema,
{
    pub(crate) fn new(
        package_authority: WorthQueryInstalledPackageAuthority,
        declaration: &ApplicationSchemaDeclaration<Schema>,
    ) -> Self {
        Self {
            schema_name: declaration.erased().name().to_string(),
            schema_identity: declaration.identity().clone(),
            schema: declaration.erased().clone(),
            package_authority,
            _schema: PhantomData,
        }
    }

    fn authoring_context(&self) -> ApplicationSchemaAuthoringContext {
        ApplicationSchemaAuthoringContext::from_installed_declaration(
            self.binding_identity(),
            &self.schema,
        )
    }

    pub fn owner(&self) -> &str {
        self.package_authority.owner()
    }

    pub fn schema_name(&self) -> &str {
        &self.schema_name
    }

    pub fn package_identity(&self) -> &WorthQueryPortableDomainPackageIdentity {
        self.package_authority.package_identity()
    }

    pub fn binding_identity(&self) -> ApplicationSchemaBindingIdentity {
        ApplicationSchemaBindingIdentity::from_installed_parts(
            self.package_authority.runtime_ordinal,
            self.package_authority.generation.ordinal(),
            self.package_authority.package_identity.as_str(),
            self.schema_identity.clone(),
        )
    }

    pub fn query<Entity>(
        &self,
        entity: ApplicationEntityRef<Schema, Entity>,
    ) -> TypedReadDeclarationBuilder<Schema, Entity> {
        TypedReadDeclarationBuilder::new(entity).with_installed_context(self.authoring_context())
    }

    pub fn operation<Operation, Input>(
        &self,
        operation: ApplicationOperationRef<Schema, Operation, Input>,
    ) -> TypedOperationBuilder<Schema, Operation, Input> {
        TypedOperationBuilder::new(operation).with_installed_context(self.authoring_context())
    }

    pub fn effects<Operation, Input>(
        &self,
        operation: ApplicationOperationRef<Schema, Operation, Input>,
    ) -> TypedEffectIntentBuilder<Schema, Operation, Input> {
        TypedEffectIntentBuilder::new(operation).with_installed_context(self.authoring_context())
    }
}

impl<Schema> std::fmt::Debug for WorthQueryInstalledApplicationSchema<Schema> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("WorthQueryInstalledApplicationSchema")
            .field("owner", &self.package_authority.owner())
            .field("schema_name", &self.schema_name)
            .field("schema_identity", &self.schema_identity)
            .finish_non_exhaustive()
    }
}
