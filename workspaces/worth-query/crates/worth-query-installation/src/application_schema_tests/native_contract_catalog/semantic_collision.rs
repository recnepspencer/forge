use worth_foundational::facade::{AspectContractRevision, AspectIdentity};
use worth_query_declaration::facade::application_schema::{
    ApplicationAspectMarkerIdentity, ApplicationAspectRef, ApplicationFieldMarkerIdentity,
    ApplicationFieldPresence, ApplicationFieldRef, ApplicationSchema, ApplicationSchemaDeclaration,
    ApplicationSchemaDeclarationBuilder, DeclaredApplicationFieldValue,
};

use super::{assert_installation_denial, WorthQueryInstalledPackageIndexDenialKind};

struct DuplicateAspectLocusSchema;
struct DuplicateFieldLocusSchema;
struct DuplicateIdentityAcrossEntitiesSchema;
struct SameNameAcrossEntitiesSchema;
worth_query_declaration::worth_query_entity!(AspectEntity in DuplicateAspectLocusSchema);
worth_query_declaration::worth_query_entity!(FieldEntity in DuplicateFieldLocusSchema);
worth_query_declaration::worth_query_entity!(FirstIdentityEntity in DuplicateIdentityAcrossEntitiesSchema);
worth_query_declaration::worth_query_entity!(SecondIdentityEntity in DuplicateIdentityAcrossEntitiesSchema);
worth_query_declaration::worth_query_entity!(FirstNamedEntity in SameNameAcrossEntitiesSchema);
worth_query_declaration::worth_query_entity!(SecondNamedEntity in SameNameAcrossEntitiesSchema);
worth_query_declaration::worth_query_aspect!(
    FieldAspect in DuplicateFieldLocusSchema, FieldEntity;
    identity = AspectIdentity(21),
    revision = AspectContractRevision(1),
);

struct FirstAspect;
struct SecondAspect;
struct FirstField;
struct SecondField;
struct FirstIdentityAspect;
struct SecondIdentityAspect;
struct FirstNamedAspect;
struct SecondNamedAspect;
struct FirstNamedField;
struct SecondNamedField;

macro_rules! duplicate_aspect_marker {
    ($marker:ty, $identity:expr) => {
        impl ApplicationAspectMarkerIdentity for $marker {
            type Schema = DuplicateAspectLocusSchema;
            type Entity = AspectEntity;
            const IDENTIFIER: &'static str = "SameAspect";
            const ASPECT_IDENTITY: AspectIdentity = AspectIdentity($identity);
            const CONTRACT_REVISION: AspectContractRevision = AspectContractRevision(1);
        }
    };
}

duplicate_aspect_marker!(FirstAspect, 11);
duplicate_aspect_marker!(SecondAspect, 12);

macro_rules! duplicate_field_marker {
    ($marker:ty, $value:ty) => {
        impl ApplicationFieldMarkerIdentity for $marker {
            type Schema = DuplicateFieldLocusSchema;
            type Entity = FieldEntity;
            type Aspect = FieldAspect;
            const IDENTIFIER: &'static str = "SameField";
        }

        impl DeclaredApplicationFieldValue for $marker {
            type Value = $value;
            const PRESENCE: ApplicationFieldPresence = ApplicationFieldPresence::Required;
        }
    };
}

duplicate_field_marker!(FirstField, u64);
duplicate_field_marker!(SecondField, String);

macro_rules! aspect_marker {
    ($marker:ty, $schema:ty, $entity:ty, $name:literal, $identity:expr) => {
        impl ApplicationAspectMarkerIdentity for $marker {
            type Schema = $schema;
            type Entity = $entity;
            const IDENTIFIER: &'static str = $name;
            const ASPECT_IDENTITY: AspectIdentity = AspectIdentity($identity);
            const CONTRACT_REVISION: AspectContractRevision = AspectContractRevision(1);
        }
    };
}

aspect_marker!(
    FirstIdentityAspect,
    DuplicateIdentityAcrossEntitiesSchema,
    FirstIdentityEntity,
    "FirstAspect",
    31
);
aspect_marker!(
    SecondIdentityAspect,
    DuplicateIdentityAcrossEntitiesSchema,
    SecondIdentityEntity,
    "SecondAspect",
    31
);
aspect_marker!(
    FirstNamedAspect,
    SameNameAcrossEntitiesSchema,
    FirstNamedEntity,
    "SameAspect",
    41
);
aspect_marker!(
    SecondNamedAspect,
    SameNameAcrossEntitiesSchema,
    SecondNamedEntity,
    "SameAspect",
    42
);

macro_rules! named_field_marker {
    ($marker:ty, $entity:ty, $aspect:ty) => {
        impl ApplicationFieldMarkerIdentity for $marker {
            type Schema = SameNameAcrossEntitiesSchema;
            type Entity = $entity;
            type Aspect = $aspect;
            const IDENTIFIER: &'static str = "SameField";
        }
        impl DeclaredApplicationFieldValue for $marker {
            type Value = u64;
            const PRESENCE: ApplicationFieldPresence = ApplicationFieldPresence::Required;
        }
    };
}

named_field_marker!(FirstNamedField, FirstNamedEntity, FirstNamedAspect);
named_field_marker!(SecondNamedField, SecondNamedEntity, SecondNamedAspect);

macro_rules! schema_identity {
    ($schema:ty, $name:literal) => {
        impl ApplicationSchema for $schema {
            const OWNER: &'static str = "native-contract-semantic-collision";
            const NAME: &'static str = $name;
            const MAJOR: u32 = 1;
            const MINOR: u32 = 0;

            fn declaration() -> Result<
                ApplicationSchemaDeclaration<Self>,
                worth_query_declaration::facade::application_schema::ApplicationSchemaDeclarationDenial,
            > {
                unreachable!("the collision tests author exact declarations")
            }
        }
    };
}

schema_identity!(DuplicateAspectLocusSchema, "DuplicateAspectLocusSchema");
schema_identity!(DuplicateFieldLocusSchema, "DuplicateFieldLocusSchema");
schema_identity!(
    DuplicateIdentityAcrossEntitiesSchema,
    "DuplicateIdentityAcrossEntitiesSchema"
);
schema_identity!(SameNameAcrossEntitiesSchema, "SameNameAcrossEntitiesSchema");

#[test]
fn duplicate_semantic_aspect_locus_denies_instead_of_overwriting_meaning() {
    let entity = AspectEntity::reference();
    let declaration = ApplicationSchemaDeclarationBuilder::for_schema()
        .entity(entity)
        .aspect(
            entity,
            ApplicationAspectRef::<_, _, FirstAspect>::from_schema_identifier("SameAspect"),
        )
        .aspect(
            entity,
            ApplicationAspectRef::<_, _, SecondAspect>::from_schema_identifier("SameAspect"),
        )
        .build()
        .unwrap();
    assert_installation_denial(
        declaration,
        WorthQueryInstalledPackageIndexDenialKind::ApplicationSchemaDuplicateAspectLocus,
    );
}

#[test]
fn duplicate_semantic_field_locus_denies_instead_of_overwriting_shape() {
    let entity = FieldEntity::reference();
    let declaration = ApplicationSchemaDeclarationBuilder::for_schema()
        .entity(entity)
        .aspect(entity, FieldAspect::reference())
        .field(
            entity,
            ApplicationFieldRef::<_, _, _, FirstField, u64>::from_schema_types(),
        )
        .field(
            entity,
            ApplicationFieldRef::<_, _, _, SecondField, String>::from_schema_types(),
        )
        .build()
        .unwrap();
    assert_installation_denial(
        declaration,
        WorthQueryInstalledPackageIndexDenialKind::ApplicationSchemaDuplicateFieldLocus,
    );
}

#[test]
fn duplicate_identity_denies_across_distinct_entities() {
    let first = FirstIdentityEntity::reference();
    let second = SecondIdentityEntity::reference();
    let declaration = ApplicationSchemaDeclarationBuilder::for_schema()
        .entity(first)
        .entity(second)
        .aspect(
            first,
            ApplicationAspectRef::<_, _, FirstIdentityAspect>::from_schema_identifier(
                "FirstAspect",
            ),
        )
        .aspect(
            second,
            ApplicationAspectRef::<_, _, SecondIdentityAspect>::from_schema_identifier(
                "SecondAspect",
            ),
        )
        .build()
        .unwrap();
    assert_installation_denial(
        declaration,
        WorthQueryInstalledPackageIndexDenialKind::ApplicationSchemaDuplicateAspectIdentity,
    );
}

#[test]
fn same_rendered_aspect_and_field_names_remain_distinct_across_entities() {
    let first = FirstNamedEntity::reference();
    let second = SecondNamedEntity::reference();
    let declaration = ApplicationSchemaDeclarationBuilder::for_schema()
        .entity(first)
        .entity(second)
        .aspect(
            first,
            ApplicationAspectRef::<_, _, FirstNamedAspect>::from_schema_identifier("SameAspect"),
        )
        .aspect(
            second,
            ApplicationAspectRef::<_, _, SecondNamedAspect>::from_schema_identifier("SameAspect"),
        )
        .field(
            first,
            ApplicationFieldRef::<_, _, _, FirstNamedField, u64>::from_schema_types(),
        )
        .field(
            second,
            ApplicationFieldRef::<_, _, _, SecondNamedField, u64>::from_schema_types(),
        )
        .build()
        .unwrap();
    let index = crate::facade::WorthQueryInstalledPackageIndex::build(
        crate::facade::WorthQueryInstallationRuntimeIdentity::fresh(),
        crate::facade::WorthQueryInstallationGeneration::initial(),
        [super::admitted_package(declaration.clone())],
    )
    .unwrap();
    let installed = index.bind_application_schema(declaration).unwrap();
    assert_eq!(
        installed
            .native_contracts()
            .aspect("FirstNamedEntity", "SameAspect")
            .unwrap()
            .contract()
            .identity(),
        AspectIdentity(41)
    );
    assert_eq!(
        installed
            .native_contracts()
            .aspect("SecondNamedEntity", "SameAspect")
            .unwrap()
            .contract()
            .identity(),
        AspectIdentity(42)
    );
}
