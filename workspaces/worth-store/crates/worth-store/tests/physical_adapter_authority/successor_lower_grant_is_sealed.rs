use worth_store::physical_runtime::BlobPhysicalAllocation;

fn extract(allocation: BlobPhysicalAllocation) {
    let _lower_grant = allocation.into_grant();
}

fn main() {}
