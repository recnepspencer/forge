use worth_query_host::facade::domain::{
    ApplicationCapabilityRef, WorthQueryInstalledApplicationCapability,
};

struct Schema;
struct Capability;
struct Operation;

fn requires_installed(
    _: WorthQueryInstalledApplicationCapability<Schema, Capability, Operation, ()>,
) {
}

fn main() {
    requires_installed(
        ApplicationCapabilityRef::<Schema, Capability>::from_schema_identifier("Capability"),
    );
}
