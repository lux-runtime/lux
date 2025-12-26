use std::env::set_current_dir;
use std::path::PathBuf;
use std::process::ExitCode;

use anyhow::Result;
use console::set_colors_enabled;
use console::set_colors_enabled_stderr;

use lux_utils::path::clean_path;

use crate::Runtime;

const ARGS: &[&str] = &["Foo", "Bar"];

fn run_test(path: &str) -> Result<ExitCode> {
    async_io::block_on(async {
        // We need to change the current directory to the workspace root since
        // we are in a sub-crate and tests would run relative to the sub-crate
        let workspace_dir_str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../");
        let workspace_dir = clean_path(PathBuf::from(workspace_dir_str));
        set_current_dir(&workspace_dir)?;

        // Disable styling for stdout and stderr since
        // some tests rely on output not being styled
        set_colors_enabled(false);
        set_colors_enabled_stderr(false);

        // The rest of the test logic can continue as normal
        let mut rt = Runtime::new()?.with_args(ARGS).with_jit(true);

        let script_path = workspace_dir.join("tests").join(format!("{path}.luau"));
        let script_values = rt.run_file(script_path).await?;

        Ok(ExitCode::from(script_values.status()))
    })
}

macro_rules! create_tests {
    ($($name:ident: $value:expr,)*) => { $(
        #[test]
        fn $name() -> Result<ExitCode> {
        	run_test($value)
        }
    )* }
}

#[cfg(any(
    feature = "std-fs",
    feature = "std-luau",
    feature = "std-process",
    feature = "std-regex",
    feature = "std-serde",
    feature = "std-stdio",
    feature = "std-ffi",
    feature = "std-signal",
))]
create_tests! {
    require_aliases: "require/tests/aliases",
    require_async: "require/tests/async",
    require_async_concurrent: "require/tests/async_concurrent",
    require_async_sequential: "require/tests/async_sequential",
    require_builtins: "require/tests/builtins",
    require_children: "require/tests/children",
    require_init: "require/tests/init_files",
    require_invalid: "require/tests/invalid",
    require_multi_ext: "require/tests/multi_ext",
    require_nested: "require/tests/nested",
    require_parents: "require/tests/parents",
    require_siblings: "require/tests/siblings",
    require_state: "require/tests/state",

    global_g_table: "globals/_G",
    global_version: "globals/_VERSION",
    global_coroutine: "globals/coroutine",
    global_error: "globals/error",
    global_pcall: "globals/pcall",
    global_type: "globals/type",
    global_typeof: "globals/typeof",
    global_warn: "globals/warn",
}

#[cfg(feature = "std-fs")]
create_tests! {
    fs_files: "fs/files",
    fs_copy: "fs/copy",
    fs_dirs: "fs/dirs",
    fs_metadata: "fs/metadata",
    fs_move: "fs/move",
}

#[cfg(feature = "std-serde")]
create_tests! {
    serde_compression_files: "serde/compression/files",
    serde_compression_roundtrip: "serde/compression/roundtrip",
    serde_json_decode: "serde/json/decode",
    serde_json_encode: "serde/json/encode",
    serde_jsonc_decode: "serde/jsonc/decode",
    serde_jsonc_encode: "serde/jsonc/encode",
    serde_toml_decode: "serde/toml/decode",
    serde_toml_encode: "serde/toml/encode",
    serde_hashing_hash: "serde/hashing/hash",
    serde_hashing_hmac: "serde/hashing/hmac",
}

#[cfg(feature = "std-stdio")]
create_tests! {
    stdio_format: "stdio/format",
    stdio_color: "stdio/color",
    stdio_style: "stdio/style",
    stdio_write: "stdio/write",
    stdio_ewrite: "stdio/ewrite",
}
