use worth_query::facade::domain::WorthQueryInstalledDomainLiveHandle;
use worth_query::facade::runtime::WorthQueryWorkspace;

fn cannot_revive_after_close<D: 'static>(
    handle: WorthQueryInstalledDomainLiveHandle<D>,
    workspace: &mut WorthQueryWorkspace,
) {
    let _closed = handle.close(workspace);
    let _revived_name = handle.name();
}

fn main() {}
