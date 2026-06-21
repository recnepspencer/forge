use serde::de::DeserializeOwned;
use topology::facade::TopologyTouchedGraphBasis;

fn requires_deserialize<T: DeserializeOwned>() {}

fn main() {
    requires_deserialize::<TopologyTouchedGraphBasis>();
}
