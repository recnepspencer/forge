use worth_query::facade::domain;

use super::{read_vertex_definition, FederatedRead, GeometryDomain, ReadFamily};

pub fn federated_touch_package<G1: 'static, G2: 'static>(
    compensated: bool,
    touches_remote_b: bool,
) -> domain::WorthQueryDomainPackage<GeometryDomain> {
    let mut semantics = read_vertex_definition(domain::WorthQuerySupportRequirement::Required)
        .semantics()
        .clone();
    semantics.graph_reads = domain::WorthQueryOperationGraphReadContract::Declared {
        roles: vec![
            domain::WorthQueryOperationGraphReadRole {
                role: "remote-a".into(),
                participation: domain::WorthQueryOperationGraphParticipation::SeparateAuthority {
                    role: "remote-a".into(),
                },
                access: domain::WorthQueryOperationGraphAccess::Project,
                semantic_reads: Vec::new(),
            },
            domain::WorthQueryOperationGraphReadRole {
                role: "remote-b".into(),
                participation: domain::WorthQueryOperationGraphParticipation::SeparateAuthority {
                    role: "remote-b".into(),
                },
                access: domain::WorthQueryOperationGraphAccess::Observe,
                semantic_reads: Vec::new(),
            },
        ],
    };
    semantics.touches = domain::WorthQueryOperationTouchContract::Declared {
        graph_roles: if touches_remote_b {
            vec!["remote-a".into(), "remote-b".into()]
        } else {
            vec!["remote-a".into()]
        },
        scopes: vec!["vertex".into()],
    };
    semantics.effects = domain::WorthQueryOperationEffectContract::Declared {
        effect_families: vec![domain::WorthQueryOperationEffectFamily::Mutation],
    };
    if compensated {
        semantics.reversal = domain::WorthQueryOperationReversalContract::Compensation {
            operation: domain::WorthQueryDomainOperationIdentity::new(
                "compensate-federated-touch",
                1,
            ),
        };
    }
    semantics.cost.execution = domain::WorthQueryOperationCostClass::ExternalBoundary;
    let operation = domain::WorthQueryDomainOperationDefinition::<
        GeometryDomain,
        FederatedRead,
        ReadFamily,
    >::new(
        domain::WorthQueryDomainOperationIdentity::new("federated-touch", 1),
        semantics,
    );
    domain::WorthQueryDomainPackage::declare(
        GeometryDomain,
        domain::WorthQueryDomainIdentityDeclaration::new(
            domain::WorthQueryDomainIdentityNamespace::new("WORTH.tests").unwrap(),
            domain::WorthQueryDomainIdentityName::new("geometry").unwrap(),
            domain::WorthQueryDomainSemanticVersion::new(1, 0),
        ),
    )
    .operation(operation)
    .operation_graph_participation::<FederatedRead, ReadFamily, G1>("remote-a")
    .operation_graph_participation::<FederatedRead, ReadFamily, G2>("remote-b")
}
