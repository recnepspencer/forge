mod change_set;
mod denial;

pub use change_set::{
    apply_installed_scoped_changes, InstalledSignalScopedChange, InstalledSignalScopedChangeSet,
    InstalledSignalScopedChangeView,
};
pub use denial::{SignalInstalledScopedChangeDenial, SignalInstalledScopedChangeOutcome};

#[cfg(test)]
mod tests;
