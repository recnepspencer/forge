use crate::facade::NodeContract;
use crate::schema::data::{
    SignalSchemaDescriptor, SignalSchemaId, SignalSchemaName, SignalSchemaRegistration,
    SignalSchemaRegistry, SignalSchemaVersion,
};

pub(in crate::logic::transaction::tests) fn demo_schema_registry() -> SignalSchemaRegistry {
    SignalSchemaRegistry::from_registrations(vec![SignalSchemaRegistration::new(
        SignalSchemaDescriptor::new(
            SignalSchemaId(1),
            SignalSchemaName::new("signal.demo.gear"),
            SignalSchemaVersion::new(1, 0),
            NodeContract::wildcard(),
        ),
    )
    .expect("valid schema registration")])
    .expect("valid schema registry")
}

pub(in crate::logic::transaction::tests) fn contract_backed_schema_registry(
    contract: NodeContract,
) -> SignalSchemaRegistry {
    SignalSchemaRegistry::from_registrations(vec![SignalSchemaRegistration::new(
        SignalSchemaDescriptor::new(
            SignalSchemaId(7),
            SignalSchemaName::new("signal.demo.schema-bound"),
            SignalSchemaVersion::new(1, 0),
            contract,
        ),
    )
    .expect("valid schema registration")])
    .expect("valid schema registry")
}
