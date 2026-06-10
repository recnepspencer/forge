use worth_spatial::facade::planar_structural_identity::PlanarStructuralIdentityReceipt;

fn main() {
    let _receipt = PlanarStructuralIdentityReceipt {
        basis: fake(),
        declaration_digest: String::new(),
        envelope_digest: String::new(),
        structural_identity_digest: String::new(),
        canonical_transform_basis_digest: String::new(),
        counters: fake(),
    };
}

fn fake<T>() -> T {
    unimplemented!()
}
