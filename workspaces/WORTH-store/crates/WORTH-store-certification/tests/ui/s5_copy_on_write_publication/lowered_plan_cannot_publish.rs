use worth_store_physical_isolation::{
    LoweredCopyOnWritePublicationPlan, ReadCopyUpdateRootPublication,
};

fn misuse(plan: LoweredCopyOnWritePublicationPlan) {
    let _ = ReadCopyUpdateRootPublication::publish(plan);
}

fn main() {}
