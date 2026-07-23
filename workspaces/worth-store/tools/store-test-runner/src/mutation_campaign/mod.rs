mod catalog;
mod evidence;
mod execution;
mod sandbox;

use std::path::Path;

pub(super) fn run(
    workspace_root: &Path,
    list: bool,
    selected: Option<u8>,
    first: Option<u8>,
) -> Result<(), String> {
    if list {
        for mutation in catalog::mutations()
            .iter()
            .filter(|mutation| selected.is_none_or(|id| mutation.id == id))
            .filter(|mutation| first.is_none_or(|id| mutation.id >= id))
        {
            println!(
                "{}\t{}\t{}\t{}",
                mutation.id, mutation.predicate, mutation.source, mutation.selector
            );
        }
        return Ok(());
    }
    let sandbox = sandbox::MutationSandbox::create(workspace_root)?;
    for mutation in catalog::mutations()
        .iter()
        .filter(|mutation| selected.is_none_or(|id| mutation.id == id))
        .filter(|mutation| first.is_none_or(|id| mutation.id >= id))
    {
        println!("mutate: {} ({})", mutation.id, mutation.predicate);
        let observation = execution::execute(&sandbox, mutation)?;
        println!("C5_MUTANT_EVIDENCE {}", evidence::encode(&observation)?);
    }
    Ok(())
}
