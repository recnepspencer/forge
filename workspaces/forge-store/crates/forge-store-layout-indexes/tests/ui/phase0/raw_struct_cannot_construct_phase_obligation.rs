use forge_store_layout_indexes::S8PhaseSkeletonObligationRow;

fn main() {
    let _ = S8PhaseSkeletonObligationRow {
        phase_number: 0,
        owning_crate: "forged",
        owning_module_path: "forged",
        public_facade_path: "forged",
        consumed_authority: "forged",
        minted_authority: "forged",
        courtroom_boundary: "forged",
        shortcut_proof: "forged",
    };
}
