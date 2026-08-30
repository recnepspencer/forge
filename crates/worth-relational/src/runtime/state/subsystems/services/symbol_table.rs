use std::sync::{Arc, RwLock};

use crate::symbols::data::{StringInterner, Symbol, SymbolTableSnapshot};

#[derive(Debug)]
pub(crate) struct RuntimeSymbolTable {
    state: Arc<RwLock<StringInterner>>,
    configuration_snapshot: Arc<RwLock<SymbolTableSnapshot>>,
}

impl Default for RuntimeSymbolTable {
    fn default() -> Self {
        Self {
            state: Arc::new(RwLock::new(StringInterner::default())),
            configuration_snapshot: Arc::new(RwLock::new(SymbolTableSnapshot::default())),
        }
    }
}

impl Clone for RuntimeSymbolTable {
    fn clone(&self) -> Self {
        Self {
            state: Arc::clone(&self.state),
            configuration_snapshot: Arc::clone(&self.configuration_snapshot),
        }
    }
}

impl PartialEq for RuntimeSymbolTable {
    fn eq(&self, other: &Self) -> bool {
        self.interner_snapshot() == other.interner_snapshot()
            && self.configuration_snapshot() == other.configuration_snapshot()
    }
}

impl Eq for RuntimeSymbolTable {}

impl RuntimeSymbolTable {
    pub(crate) fn detached_owner_snapshot(&self) -> Self {
        Self {
            state: Arc::new(RwLock::new(self.interner_snapshot())),
            configuration_snapshot: Arc::new(RwLock::new(self.configuration_snapshot())),
        }
    }
    pub(crate) fn normalize_client_keys(
        &self,
        normalize: impl FnOnce(&mut StringInterner) -> Vec<(Symbol, String)>,
    ) {
        let new_entries = self.with_write(normalize);
        self.configuration_snapshot
            .write()
            .expect("runtime symbol configuration snapshot lock poisoned")
            .merge_new_entries(new_entries);
    }

    pub(crate) fn interner_snapshot(&self) -> StringInterner {
        self.state
            .read()
            .expect("runtime symbol table lock poisoned")
            .clone()
    }

    pub(crate) fn snapshot(&self) -> SymbolTableSnapshot {
        self.state
            .read()
            .expect("runtime symbol table lock poisoned")
            .snapshot()
    }

    pub(crate) fn configuration_snapshot(&self) -> SymbolTableSnapshot {
        self.configuration_snapshot
            .read()
            .expect("runtime symbol configuration snapshot lock poisoned")
            .clone()
    }

    pub(crate) fn initialize_configuration_snapshot(&self, snapshot: SymbolTableSnapshot) {
        *self
            .configuration_snapshot
            .write()
            .expect("runtime symbol configuration snapshot lock poisoned") = snapshot;
    }

    pub(crate) fn resolve(&self, symbol: Symbol) -> Option<String> {
        self.state
            .read()
            .expect("runtime symbol table lock poisoned")
            .resolve(symbol)
            .map(str::to_owned)
    }

    pub(crate) fn replace(&self, interner: StringInterner) {
        let snapshot = interner.snapshot();
        *self
            .state
            .write()
            .expect("runtime symbol table lock poisoned") = interner;
        self.initialize_configuration_snapshot(snapshot);
    }

    pub(crate) fn with_read<T>(&self, read: impl FnOnce(&StringInterner) -> T) -> T {
        let guard = self
            .state
            .read()
            .expect("runtime symbol table lock poisoned");
        read(&guard)
    }

    pub(crate) fn with_write<T>(&self, write: impl FnOnce(&mut StringInterner) -> T) -> T {
        let mut guard = self
            .state
            .write()
            .expect("runtime symbol table lock poisoned");
        write(&mut guard)
    }
}
