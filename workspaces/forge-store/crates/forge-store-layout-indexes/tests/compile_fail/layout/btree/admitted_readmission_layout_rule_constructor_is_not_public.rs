use forge_store_recovery_physics::PartialPublicationReplayReadWitness;

fn main() {
    let _ = PartialPublicationReplayReadWitness {
        replay_read_identity: String::from("forged"),
        source: todo!(),
        operation_digest: String::new(),
    };
}
