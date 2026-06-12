use hadwiger_research::facade::MotifSeedDeclaration;

fn main() {
    let request = MotifSeedDeclaration::new("motif-a");
    let _ = request.certify_geometry();
}
