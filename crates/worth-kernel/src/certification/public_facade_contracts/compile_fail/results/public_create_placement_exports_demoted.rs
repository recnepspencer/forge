use worth_kernel::facade::CreateSpatialIntent;
use worth_kernel::facade::authoring::intents::CreateSpatialIntent as IntentCreateSpatialIntent;

fn main() {
    let _ = std::any::type_name::<CreateSpatialIntent<()>>();
    let _ = std::any::type_name::<IntentCreateSpatialIntent<()>>();
}
