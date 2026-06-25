use topology::derived_invalidation_deletion_closeout::{
    DerivedInvalidationDeletionCloseout, DerivedInvalidationDeletionCounters,
    DerivedInvalidationDeletionAudit, DerivedInvalidationDeletionLedger,
    DerivedInvalidationDeletionSourceFirewall,
    DerivedInvalidationPhaseNineSeed, DerivedInvalidationResidueAudit,
};

fn main() {
    let _ = DerivedInvalidationDeletionCloseout {
        phase_eight_seed_digest: String::new(),
        migration_sweep_digest: String::new(),
        inventory_digest: String::new(),
        deletion_ledger: fake_deletion_ledger(),
        residue_audit: fake_residue_audit(),
        source_firewall: fake_source_firewall(),
        deletion_audit: fake_deletion_audit(),
        counters: fake_counters(),
        phase_nine_seed: fake_phase_nine_seed(),
        closeout_digest: String::new(),
    };
}

fn fake_deletion_ledger() -> DerivedInvalidationDeletionLedger {
    panic!("compile-fail fixture does not execute")
}

fn fake_residue_audit() -> DerivedInvalidationResidueAudit {
    panic!("compile-fail fixture does not execute")
}

fn fake_source_firewall() -> DerivedInvalidationDeletionSourceFirewall {
    panic!("compile-fail fixture does not execute")
}

fn fake_deletion_audit() -> DerivedInvalidationDeletionAudit {
    panic!("compile-fail fixture does not execute")
}

fn fake_counters() -> DerivedInvalidationDeletionCounters {
    panic!("compile-fail fixture does not execute")
}

fn fake_phase_nine_seed() -> DerivedInvalidationPhaseNineSeed {
    panic!("compile-fail fixture does not execute")
}
