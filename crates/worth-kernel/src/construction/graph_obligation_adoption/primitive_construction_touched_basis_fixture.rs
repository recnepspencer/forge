use topology::facade::TopologyPrimitiveConstructionBirthDeclaredTouchedBasis;

use super::family_execution_matrix::representative_primitive_construction_intent;
use crate::construction::admitted_scaffold::prepare_primitive_construction_topology_query_admitted_handoff_from_request;
use crate::construction::request::PrimitiveConstructionFamily;

pub(super) fn primitive_construction_touched_basis_for_family(
    family: PrimitiveConstructionFamily,
) -> TopologyPrimitiveConstructionBirthDeclaredTouchedBasis {
    let intent = representative_primitive_construction_intent(family);
    let handoff = prepare_primitive_construction_topology_query_admitted_handoff_from_request(
        intent.request(),
    )
    .expect("representative primitive request should admit to topology query handoff");

    TopologyPrimitiveConstructionBirthDeclaredTouchedBasis::from_admitted_handoff(&handoff)
        .expect("admitted topology handoff should lower to declared touched basis")
}
