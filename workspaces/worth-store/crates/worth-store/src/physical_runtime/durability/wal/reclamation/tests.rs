use super::*;

#[test]
fn reclamation_authority_is_move_owned() {
    fn consumes(_: EligiblePhysicalWalSegmentReclamation) {}
    let _ = consumes;
}
