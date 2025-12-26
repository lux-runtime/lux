#![allow(clippy::cargo_common_metadata)]

//! Perlin and Simplex noise generation for Lux

use lux_utils::TableBuilder;
use mlua::prelude::*;
use noise::{Fbm, NoiseFn, Perlin, Simplex};
use std::sync::atomic::{AtomicU32, Ordering};

const TYPEDEFS: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/types.d.luau"));

static SEED: AtomicU32 = AtomicU32::new(0);

#[must_use]
pub fn typedefs() -> String {
    TYPEDEFS.to_string()
}

fn get_seed() -> u32 {
    SEED.load(Ordering::Relaxed)
}

/// Perlin 2D noise
fn perlin2(_: &Lua, (x, y): (f64, f64)) -> LuaResult<f64> {
    let perlin = Perlin::new(get_seed());
    Ok(perlin.get([x, y]))
}

/// Perlin 3D noise
fn perlin3(_: &Lua, (x, y, z): (f64, f64, f64)) -> LuaResult<f64> {
    let perlin = Perlin::new(get_seed());
    Ok(perlin.get([x, y, z]))
}

/// Simplex 2D noise
fn simplex2(_: &Lua, (x, y): (f64, f64)) -> LuaResult<f64> {
    let simplex = Simplex::new(get_seed());
    Ok(simplex.get([x, y]))
}

/// Simplex 3D noise
fn simplex3(_: &Lua, (x, y, z): (f64, f64, f64)) -> LuaResult<f64> {
    let simplex = Simplex::new(get_seed());
    Ok(simplex.get([x, y, z]))
}

/// Fractal Brownian Motion 2D
fn fbm2(
    _: &Lua,
    (x, y, octaves, lacunarity, gain): (f64, f64, Option<u32>, Option<f64>, Option<f64>),
) -> LuaResult<f64> {
    let mut fbm: Fbm<Perlin> = Fbm::new(get_seed());
    fbm.octaves = octaves.unwrap_or(6) as usize;
    fbm.lacunarity = lacunarity.unwrap_or(2.0);
    fbm.persistence = gain.unwrap_or(0.5);
    Ok(fbm.get([x, y]))
}

/// Fractal Brownian Motion 3D
fn fbm3(
    _: &Lua,
    (x, y, z, octaves, lacunarity, gain): (f64, f64, f64, Option<u32>, Option<f64>, Option<f64>),
) -> LuaResult<f64> {
    let mut fbm: Fbm<Perlin> = Fbm::new(get_seed());
    fbm.octaves = octaves.unwrap_or(6) as usize;
    fbm.lacunarity = lacunarity.unwrap_or(2.0);
    fbm.persistence = gain.unwrap_or(0.5);
    Ok(fbm.get([x, y, z]))
}

/// Set global seed
fn set_seed(_: &Lua, seed: u32) -> LuaResult<()> {
    SEED.store(seed, Ordering::Relaxed);
    Ok(())
}

/// Create the noise module
pub fn module(lua: Lua) -> LuaResult<LuaTable> {
    TableBuilder::new(lua)?
        .with_function("perlin2", perlin2)?
        .with_function("perlin3", perlin3)?
        .with_function("simplex2", simplex2)?
        .with_function("simplex3", simplex3)?
        .with_function("fbm2", fbm2)?
        .with_function("fbm3", fbm3)?
        .with_function("setSeed", set_seed)?
        .build_readonly()
}
