fn require_deserialize<T: serde::de::DeserializeOwned>() {}

fn main() {
    require_deserialize::<forge_store::MaintenanceWorkDescriptor>();
}
