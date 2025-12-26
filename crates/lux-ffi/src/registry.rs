//! FFI Registry
//!
//! Stores registered C types, structs, and functions.

use crate::types::*;
use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};

/// Global registry
pub struct Registry {
    structs: HashMap<String, StructDef>,
    enums: HashMap<String, HashMap<String, i64>>,
    typedefs: HashMap<String, CType>,
    funcs: HashMap<String, FuncSig>,
}

impl Registry {
    pub fn get() -> std::sync::MutexGuard<'static, Registry> {
        static INSTANCE: OnceLock<Mutex<Registry>> = OnceLock::new();
        let instance = INSTANCE.get_or_init(|| {
            Mutex::new(Registry {
                structs: HashMap::new(),
                enums: HashMap::new(),
                typedefs: HashMap::new(),
                funcs: HashMap::new(),
            })
        });
        instance.lock().unwrap()
    }

    pub fn add_struct(&mut self, def: StructDef) {
        self.structs.insert(def.name.clone(), def);
    }

    pub fn add_enum(&mut self, name: &str, values: HashMap<String, i64>) {
        self.enums.insert(name.to_string(), values);
    }

    pub fn add_typedef(&mut self, name: &str, ctype: CType) {
        self.typedefs.insert(name.to_string(), ctype);
    }

    pub fn add_func(&mut self, sig: FuncSig) {
        self.funcs.insert(sig.name.clone(), sig);
    }

    pub fn get_struct(&self, name: &str) -> Option<&StructDef> {
        self.structs.get(name)
    }

    pub fn has_struct(&self, name: &str) -> bool {
        self.structs.contains_key(name)
    }

    pub fn get_enum(&self, name: &str) -> Option<&HashMap<String, i64>> {
        self.enums.get(name)
    }

    pub fn has_enum(&self, name: &str) -> bool {
        self.enums.contains_key(name)
    }

    pub fn get_typedef(&self, name: &str) -> Option<CType> {
        self.typedefs.get(name).cloned()
    }

    pub fn get_func(&self, name: &str) -> Option<FuncSig> {
        self.funcs.get(name).cloned()
    }

    pub fn struct_size(&self, name: &str) -> Option<usize> {
        self.structs.get(name).map(|s| s.size)
    }

    pub fn struct_align(&self, name: &str) -> Option<usize> {
        self.structs.get(name).map(|s| s.align)
    }

    // Exports for introspection
    pub fn all_typedefs(&self) -> HashMap<String, CType> {
        self.typedefs.clone()
    }

    pub fn all_structs(&self) -> HashMap<String, StructDef> {
        self.structs.clone()
    }

    pub fn all_enums(&self) -> HashMap<String, HashMap<String, i64>> {
        self.enums.clone()
    }

    pub fn all_funcs(&self) -> HashMap<String, FuncSig> {
        self.funcs.clone()
    }
}
