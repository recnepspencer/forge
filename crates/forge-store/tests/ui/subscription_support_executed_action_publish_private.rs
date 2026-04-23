use forge_store::ExecutedSupportAction;

fn attempt(action: ExecutedSupportAction) {
    let _ = action.publish();
}

fn main() {}
