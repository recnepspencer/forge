use std::sync::OnceLock;

use worth_foundational::facade::{
    AbsenceLaw, AspectContract, AspectContractRevision, AspectEvolutionPolicy, AspectIdentity,
    AspectKey, AspectMask, CanonicalFieldPath, FieldDeclaration, FieldKey, FieldRequirement,
    ProjectionMask, ScalarAspectType, StructAspectShape,
};
use worth_query::facade::consumer_kit::{in_memory_test_runtime, WorthQueryTestBackendSchema};
use worth_query::facade::{domain, foundation, read, runtime};
use worth_query_declaration::facade::authoring::{
    AspectFieldSelector, AuthoredQueryBundleRequest, AuthoredResultShapeField,
    CollectionQueryBuilder, CollectionResultShapeBuilder, RootEntityKey,
};
use worth_query_declaration::facade::binding::QueryBindingDescriptor;
use worth_query_declaration::facade::canonicalization::canonicalize_request;

use super::samples::{matrix_aspect_key, matrix_contract, matrix_value, MATRIX_ASPECT};
use super::world_scale::add_unrelated_domains;
use crate::suite::installed_operation_fixture::{semantic_closure, GeometryDomain, ReadFamily};

pub(super) const COLLECTION: &str = "NativeMatrix";

#[derive(Clone, Copy, Debug)]
pub(super) struct NativeMatrixRead;

impl domain::WorthQueryExecutableDomainOperation<GeometryDomain, ReadFamily> for NativeMatrixRead {
    type Input = ();
    type Output = read::WorthQueryReadCompletion;
    type Publication = domain::WorthQueryPublishingOperation;
    type Execution = domain::WorthQueryDirectOperation;
}

#[derive(Clone, Copy)]
pub(super) struct NativeMatrixExecutor;

impl domain::WorthQueryDomainOperationExecutor<GeometryDomain, NativeMatrixRead, ReadFamily>
    for NativeMatrixExecutor
{
    const LOWERING_FAMILY: &'static str = "native-matrix-read-v1";
    const DETERMINISTIC: bool = true;
    const EXECUTION_COST: domain::WorthQueryOperationCostClass =
        domain::WorthQueryOperationCostClass::DeclaredWidth;
    const RESULT_WIDTH_COST: domain::WorthQueryOperationCostClass =
        domain::WorthQueryOperationCostClass::DeclaredWidth;

    fn installed_read_declaration(&self) -> Option<&read::WorthQueryReadDeclaration> {
        Some(matrix_read_declaration())
    }

    fn execute(
        &self,
        _: (),
        context: &domain::WorthQueryOperationExecutionContext<'_>,
        workspace: &mut domain::WorthQueryOperationWorkspace<'_>,
    ) -> Result<
        domain::WorthQueryOperationExecutionMaterial<read::WorthQueryReadCompletion>,
        domain::WorthQueryOperationExecutorFailure,
    > {
        Ok(domain::WorthQueryOperationExecutionMaterial::new(
            context.execute_installed_read(workspace)?,
            domain::WorthQueryOperationResultState::Ready,
        ))
    }
}

pub(super) fn matrix_workspace(
    name: &str,
    row_count: usize,
    unrelated_domains: bool,
) -> runtime::WorthQueryWorkspace {
    let values = (0..row_count)
        .map(|row| matrix_value(row as u64))
        .collect::<Vec<_>>();
    matrix_workspace_with_values(name, &values, unrelated_domains)
}

pub(super) fn matrix_workspace_with_values(
    name: &str,
    values: &[worth_foundational::facade::StructAspectValue],
    unrelated_domains: bool,
) -> runtime::WorthQueryWorkspace {
    let contract = matrix_contract(1);
    let semantics = matrix_semantics(contract.clone());
    let operation = domain::WorthQueryDomainOperationDefinition::<
        GeometryDomain,
        NativeMatrixRead,
        ReadFamily,
    >::new(
        domain::WorthQueryDomainOperationIdentity::new("native-matrix-read", 1),
        semantics,
    );
    let package =
        domain::WorthQueryDomainPackage::declare(GeometryDomain, domain_identity("geometry"))
            .operation(operation);
    let schema = WorthQueryTestBackendSchema::single_collection(COLLECTION)
        .aspect_contract(contract)
        .unwrap()
        .aspect_contract(identity_contract())
        .unwrap()
        .native_aspect_mapping(
            runtime::WorthQueryAspectTouch::whole_aspect(matrix_aspect_key()),
            CanonicalFieldPath::single(super::samples::sample_field(0)),
        )
        .unwrap()
        .native_aspect_mapping(
            runtime::WorthQueryAspectTouch::aspect_field_path(
                AspectKey::new("identity").unwrap(),
                CanonicalFieldPath::single(FieldKey::new("id").unwrap()),
            ),
            CanonicalFieldPath::single(FieldKey::new("identity_storage").unwrap()),
        )
        .unwrap();
    let mut builder = in_memory_test_runtime()
        .with_schema(schema)
        .domain_package(package)
        .domain_operation_executor(
            GeometryDomain,
            NativeMatrixRead,
            ReadFamily,
            NativeMatrixExecutor,
        );
    if unrelated_domains {
        builder = add_unrelated_domains(builder);
    }
    let mut workspace = builder.workspace(name).unwrap();
    for (row, value) in values.iter().enumerate() {
        workspace
            .insert(COLLECTION, |mutation| {
                mutation
                    .set_aspect(
                        runtime::WorthQueryAspectTouch::whole_aspect(matrix_aspect_key()),
                        value.clone(),
                    )
                    .set_aspect(
                        runtime::WorthQueryAspectTouch::aspect_field_path(
                            AspectKey::new("identity").unwrap(),
                            CanonicalFieldPath::single(FieldKey::new("id").unwrap()),
                        ),
                        format!("matrix-row-{row:04}"),
                    )
            })
            .unwrap();
    }
    workspace
}

pub(super) fn bind(
    workspace: &runtime::WorthQueryWorkspace,
) -> domain::WorthQueryBoundDomainOperation<
    GeometryDomain,
    NativeMatrixRead,
    ReadFamily,
    foundation::ObservationLaneWitness,
> {
    let installed = workspace.domain(GeometryDomain).unwrap();
    workspace
        .operating_world(observation_basis())
        .family(ReadFamily)
        .bind(&installed, NativeMatrixRead)
        .unwrap()
}

pub(super) fn observation_basis(
) -> foundation::AdmittedBasisCapability<foundation::ObservationLaneWitness> {
    foundation::basis_lifecycle()
        .current_head()
        .for_observation()
        .unwrap()
        .admit()
        .unwrap()
        .capability()
        .clone()
}

pub(super) fn matrix_read_declaration() -> &'static read::WorthQueryReadDeclaration {
    static DECLARATION: OnceLock<read::WorthQueryReadDeclaration> = OnceLock::new();
    DECLARATION.get_or_init(|| {
        read::declare(|builder| {
            builder.local_collection(
                COLLECTION,
                read::QuerySchemaView::new("native-matrix-v1", schema_fields(), []),
                |query| {
                    schema_fields()
                        .into_iter()
                        .fold(query, |query, field| {
                            query.project(
                                read::AspectFieldSelector::new(
                                    MATRIX_ASPECT,
                                    field.field_name().as_str(),
                                )
                                .unwrap(),
                            )
                        })
                        .order_by(read::OrderingSelector::ascending(MATRIX_ASPECT, "f15").unwrap())
                },
                |shape| {
                    schema_fields().into_iter().fold(shape, |shape, field| {
                        shape.field(
                            read::AuthoredResultShapeField::new(
                                MATRIX_ASPECT,
                                field.field_name().as_str(),
                                field.field_name().as_str(),
                            )
                            .unwrap(),
                        )
                    })
                },
            )
        })
        .unwrap()
    })
}

fn matrix_canonical_bundle(
) -> worth_query_declaration::facade::canonicalization::CanonicalQueryBundle {
    let query = schema_fields()
        .into_iter()
        .fold(
            CollectionQueryBuilder::new(RootEntityKey::new(COLLECTION).unwrap()),
            |query, field| {
                query.project(
                    AspectFieldSelector::new(MATRIX_ASPECT, field.field_name().as_str()).unwrap(),
                )
            },
        )
        .order_by(
            worth_query_declaration::facade::authoring::OrderingSelector::ascending(
                MATRIX_ASPECT,
                "f15",
            )
            .unwrap(),
        );
    let shape =
        schema_fields()
            .into_iter()
            .fold(CollectionResultShapeBuilder::new(), |shape, field| {
                shape.field(
                    AuthoredResultShapeField::new(
                        MATRIX_ASPECT,
                        field.field_name().as_str(),
                        field.field_name().as_str(),
                    )
                    .unwrap(),
                )
            });
    canonicalize_request(
        AuthoredQueryBundleRequest::for_ordinary_read(
            query.build().unwrap().into_raw(),
            shape.build().unwrap().into_raw(),
            QueryBindingDescriptor::new(),
        )
        .unwrap(),
    )
    .unwrap()
}

pub(super) fn matrix_semantics(
    contract: AspectContract,
) -> domain::WorthQueryDomainOperationSemanticClosure {
    let native_projection = domain::WorthQueryOperationNativeProjectionContract::new(
        contract,
        AspectMask::<ProjectionMask>::whole_aspect(),
    )
    .unwrap();
    let mut semantics = semantic_closure(
        matrix_canonical_bundle(),
        domain::WorthQuerySupportRequirement::Required,
        true,
    );
    semantics.native_projection = native_projection.clone();
    semantics.graph_reads = domain::WorthQueryOperationGraphReadContract::Declared {
        roles: vec![domain::WorthQueryOperationGraphReadRole {
            role: "matrix".into(),
            participation: domain::WorthQueryOperationGraphParticipation::PrimaryLogicalGraph,
            access: domain::WorthQueryOperationGraphAccess::Project,
            semantic_reads: vec![native_projection],
        }],
    };
    semantics.collection =
        domain::WorthQueryOperationCollectionContract::Collection {
            row_identity_field: domain::WorthQueryOperationCollectionField::from_dotted(&format!(
                "{MATRIX_ASPECT}.f15"
            ))
            .expect("valid collection field"),
            ordering_fields: vec![domain::WorthQueryOperationCollectionField::from_dotted(
                &format!("{MATRIX_ASPECT}.f15"),
            )
            .expect("valid collection field")],
            grouping: domain::WorthQueryOperationGroupingContract::Ungrouped,
            window: domain::WorthQueryOperationWindowPolicy::CompleteCollection,
            continuation: domain::WorthQueryOperationContinuationPosture::NotRequired,
        };
    semantics.lowering.family = "native-matrix-read-v1".into();
    semantics
}

fn schema_fields() -> Vec<read::SchemaFieldView> {
    let worth_foundational::facade::AspectShape::Struct(shape) = matrix_contract(1).shape().clone()
    else {
        unreachable!()
    };
    shape
        .fields()
        .iter()
        .map(|field| {
            read::SchemaFieldView::new(
                read::AspectName::new(MATRIX_ASPECT).unwrap(),
                read::FieldName::new(field.key().as_str()).unwrap(),
                field.value_type(),
            )
        })
        .collect()
}

fn domain_identity<D>(name: &str) -> domain::WorthQueryDomainIdentityDeclaration<D> {
    domain::WorthQueryDomainIdentityDeclaration::new(
        domain::WorthQueryDomainIdentityNamespace::new("WORTH.tests").unwrap(),
        domain::WorthQueryDomainIdentityName::new(name).unwrap(),
        domain::WorthQueryDomainSemanticVersion::new(1, 0),
    )
}

fn identity_contract() -> AspectContract {
    AspectContract::struct_aspect(
        AspectKey::new("identity").unwrap(),
        AspectIdentity(0x9150_1002),
        AspectContractRevision(1),
        StructAspectShape::new([FieldDeclaration::new(
            FieldKey::new("id").unwrap(),
            ScalarAspectType::String,
            FieldRequirement::Required,
            AbsenceLaw::Required,
            AspectEvolutionPolicy::ExplicitBreakRequired,
        )
        .unwrap()])
        .unwrap(),
    )
}
