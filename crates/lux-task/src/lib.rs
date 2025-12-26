#![allow(clippy::cargo_common_metadata)]

//! Task library - spawn, defer, delay, wait, cancel
//!
//! Provides Roblox-compatible task scheduling functions that integrate
//! with the mlua-luau-scheduler.

use std::time::{Duration, Instant};

use async_io::Timer;
use futures_lite::future::yield_now;

use mlua::prelude::*;
use mlua_luau_scheduler::Functions;

use lux_utils::TableBuilder;

const TYPEDEFS: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/types.d.luau"));

/// Returns type definitions for the task library.
#[must_use]
pub fn typedefs() -> String {
    TYPEDEFS.to_string()
}

const DELAY_IMPL_LUA: &str = r"
return defer(function(...)
    wait(select(1, ...))
    spawn(select(2, ...))
end, ...)
";

/// Creates the task global table
pub fn create(lua: Lua) -> LuaResult<LuaValue> {
    let fns = Functions::new(lua.clone())?;

    // Create wait & delay functions
    let task_wait = lua.create_async_function(wait)?;
    let task_delay_env = TableBuilder::new(lua.clone())?
        .with_value("select", lua.globals().get::<LuaFunction>("select")?)?
        .with_value("spawn", fns.spawn.clone())?
        .with_value("defer", fns.defer.clone())?
        .with_value("wait", task_wait.clone())?
        .build_readonly()?;
    let task_delay = lua
        .load(DELAY_IMPL_LUA)
        .set_name("task.delay")
        .set_environment(task_delay_env)
        .into_function()?;

    TableBuilder::new(lua)?
        .with_value("cancel", fns.cancel)?
        .with_value("defer", fns.defer)?
        .with_value("delay", task_delay)?
        .with_value("spawn", fns.spawn)?
        .with_value("wait", task_wait)?
        .build_readonly()
        .map(LuaValue::Table)
}

async fn wait(lua: Lua, secs: Option<f64>) -> LuaResult<f64> {
    // Guarantee that task.wait always yields from Lua perspective
    yield_now().await;
    wait_inner(lua, secs).await
}

async fn wait_inner(_: Lua, secs: Option<f64>) -> LuaResult<f64> {
    let duration = Duration::from_secs_f64(secs.unwrap_or_default());
    let duration = duration.max(Duration::from_millis(1));
    yield_now().await;
    let before = Instant::now();
    let after = Timer::after(duration).await;
    Ok((after - before).as_secs_f64())
}
