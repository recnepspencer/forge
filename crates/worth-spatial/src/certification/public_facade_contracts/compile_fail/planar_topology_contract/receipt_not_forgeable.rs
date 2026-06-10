use worth_spatial::facade::planar_topology_contract::PlanarTopologyContractCompletenessReceipt;

fn main() {
    let _receipt = PlanarTopologyContractCompletenessReceipt {
        basis: fake(),
        declaration_digest: String::new(),
        envelope_digest: String::new(),
        fact_digest: String::new(),
        counters: fake(),
    };
}

fn fake<T>() -> T {
    unimplemented!()
}
