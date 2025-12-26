#![allow(clippy::cargo_common_metadata)]

//! Signal - High-performance event system for Lux
//!
//! Zero-GC optimized signal implementation.
//! Uses static storage and minimal allocations.
//!
//! Usage:
//! ```lua
//! local Signal = require("@lux/signal")
//! local mySignal = Signal.new()
//! local conn = mySignal:Connect(function(value)
//!     print("Got:", value)
//! end)
//! mySignal:Fire("Hello!")
//! conn:Disconnect()
//! ```

use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};

use lux_utils::TableBuilder;
use mlua::prelude::*;
use parking_lot::Mutex;

/// Global connection ID
static CONN_ID: AtomicU64 = AtomicU64::new(1);

/// Connection entry - minimal size
struct Conn {
    id: u64,
    func: LuaFunction,
    once: bool,
}

/// Signal internal state
struct State {
    conns: Vec<Conn>,
    to_remove: Vec<u64>,
    firing: bool,
}

/// The Signal type
#[derive(Clone)]
pub struct Signal(Arc<Mutex<State>>);

impl Signal {
    #[inline]
    pub fn new() -> Self {
        Self(Arc::new(Mutex::new(State {
            conns: Vec::with_capacity(2),
            to_remove: Vec::new(),
            firing: false,
        })))
    }

    #[inline]
    pub fn connect(&self, func: LuaFunction, once: bool) -> u64 {
        let id = CONN_ID.fetch_add(1, Ordering::Relaxed);
        self.0.lock().conns.push(Conn { id, func, once });
        id
    }

    #[inline]
    pub fn disconnect(&self, id: u64) {
        let mut s = self.0.lock();
        if s.firing {
            s.to_remove.push(id);
        } else {
            s.conns.retain(|c| c.id != id);
        }
    }

    pub fn fire(&self, _lua: &Lua, args: LuaMultiValue) -> LuaResult<()> {
        let funcs: Vec<(u64, LuaFunction, bool)> = {
            let mut s = self.0.lock();
            s.firing = true;
            s.conns
                .iter()
                .map(|c| (c.id, c.func.clone(), c.once))
                .collect()
        };

        let mut once_ids = Vec::new();
        for (id, func, once) in funcs {
            if once {
                once_ids.push(id);
            }
            let _ = func.call::<()>(args.clone());
        }

        let mut s = self.0.lock();
        s.firing = false;

        // Remove once connections
        for id in once_ids {
            s.conns.retain(|c| c.id != id);
        }

        // Process pending disconnects
        let pending: Vec<u64> = s.to_remove.drain(..).collect();
        for id in pending {
            s.conns.retain(|c| c.id != id);
        }
        Ok(())
    }

    #[inline]
    pub fn clear(&self) {
        self.0.lock().conns.clear();
    }

    #[inline]
    pub fn count(&self) -> usize {
        self.0.lock().conns.len()
    }
}

impl Default for Signal {
    fn default() -> Self {
        Self::new()
    }
}

/// Connection handle
#[derive(Clone)]
pub struct Connection {
    id: u64,
    sig: Signal,
}

impl LuaUserData for Connection {
    fn add_fields<F: LuaUserDataFields<Self>>(f: &mut F) {
        f.add_field_method_get("Connected", |_, this| {
            Ok(this.sig.0.lock().conns.iter().any(|c| c.id == this.id))
        });
    }

    fn add_methods<M: LuaUserDataMethods<Self>>(m: &mut M) {
        m.add_method("Disconnect", |_, this, ()| {
            this.sig.disconnect(this.id);
            Ok(())
        });
    }
}

impl LuaUserData for Signal {
    fn add_methods<M: LuaUserDataMethods<Self>>(m: &mut M) {
        m.add_method("Connect", |lua, this, func: LuaFunction| {
            let id = this.connect(func, false);
            lua.create_userdata(Connection {
                id,
                sig: this.clone(),
            })
        });

        m.add_method("Once", |lua, this, func: LuaFunction| {
            let id = this.connect(func, true);
            lua.create_userdata(Connection {
                id,
                sig: this.clone(),
            })
        });

        m.add_method("Fire", |lua, this, args: LuaMultiValue| {
            this.fire(&lua, args)
        });

        m.add_method("DisconnectAll", |_, this, ()| {
            this.clear();
            Ok(())
        });

        m.add_method("Destroy", |_, this, ()| {
            this.clear();
            Ok(())
        });

        m.add_method("GetConnections", |_, this, ()| Ok(this.count()));

        m.add_method("Wait", |_, _, ()| -> LuaResult<()> { Ok(()) });
    }
}

/// Create the module
pub fn module(lua: Lua) -> LuaResult<LuaTable> {
    TableBuilder::new(lua)?
        .with_function("new", |lua, ()| lua.create_userdata(Signal::new()))?
        .build_readonly()
}

/// Type definitions for this module
pub fn typedefs() -> String {
    include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/types.d.luau")).to_string()
}

pub use Signal as SignalType;
