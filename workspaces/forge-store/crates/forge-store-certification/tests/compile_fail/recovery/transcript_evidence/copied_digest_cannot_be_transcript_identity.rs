use forge_store_physical_certification::PhysicalSimulationTranscriptIdentity;

fn requires_identity(_: PhysicalSimulationTranscriptIdentity) {}

fn main() {
    let copied_digest = [0_u8; 32];
    requires_identity(copied_digest);
}
