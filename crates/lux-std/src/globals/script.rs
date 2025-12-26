//! Script global - provides context about the currently executing script
//!
//! Provides Roblox-like instance semantics for the filesystem:
//! - script.Name - The file or folder name
//! - script.Parent - The parent directory
//! - script.ClassName - "Script", "RustFile", "JsonFile", "ModuleScript", or "Folder"
//! - script:FindFirstChild(name) - Find a child (works inside folders)
//! - script:GetChildren() - List items in folder (empty if script is a file)
//!
//! AUTO-DETECTION: Automatically detects the current script path using debug.getinfo

use mlua::prelude::*;
use std::fs;
use std::path::{Path, PathBuf};

/// Key for storing the current script context in Lua registry
pub const SCRIPT_CONTEXT_KEY: &str = "__lux_script_context";

/// Represents a script or instance context
#[derive(Debug, Clone)]
pub struct ScriptContext {
    pub path: PathBuf,
    pub is_file: bool,
    pub class_name: String,
}

impl ScriptContext {
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        let path = path.as_ref().to_path_buf();

        // Determine if it is a file or directory.
        let is_file = path
            .metadata()
            .map(|m| m.is_file())
            .unwrap_or_else(|_| path.extension().is_some());

        let class_name = if is_file {
            Self::classify_file(&path)
        } else {
            Self::classify_directory(&path)
        };

        Self {
            path,
            is_file,
            class_name,
        }
    }

    /// Classifies a file based on its extension - all possible types
    fn classify_file(path: &Path) -> String {
        match path.extension().and_then(|e| e.to_str()) {
            // Luau/Lua Script files
            Some("luau") | Some("lua") => "Script".to_string(),

            // === Programming Languages ===
            Some("rs") => "RustFile".to_string(),
            Some("py") | Some("pyw") | Some("pyi") => "PythonFile".to_string(),
            Some("js") | Some("mjs") | Some("cjs") => "JavaScriptFile".to_string(),
            Some("ts") | Some("mts") | Some("cts") => "TypeScriptFile".to_string(),
            Some("jsx") => "ReactFile".to_string(),
            Some("tsx") => "ReactTSFile".to_string(),
            Some("vue") => "VueFile".to_string(),
            Some("svelte") => "SvelteFile".to_string(),
            Some("c") => "CFile".to_string(),
            Some("cpp") | Some("cc") | Some("cxx") | Some("c++") => "CppFile".to_string(),
            Some("h") => "CHeaderFile".to_string(),
            Some("hpp") | Some("hxx") | Some("h++") | Some("hh") => "CppHeaderFile".to_string(),
            Some("go") => "GoFile".to_string(),
            Some("java") => "JavaFile".to_string(),
            Some("class") => "JavaClassFile".to_string(),
            Some("jar") => "JavaArchive".to_string(),
            Some("cs") => "CSharpFile".to_string(),
            Some("fs") | Some("fsx") => "FSharpFile".to_string(),
            Some("vb") => "VisualBasicFile".to_string(),
            Some("rb") | Some("rake") => "RubyFile".to_string(),
            Some("php") | Some("phtml") => "PHPFile".to_string(),
            Some("swift") => "SwiftFile".to_string(),
            Some("kt") | Some("kts") => "KotlinFile".to_string(),
            Some("dart") => "DartFile".to_string(),
            Some("r") | Some("R") => "RFile".to_string(),
            Some("m") => "ObjectiveCFile".to_string(),
            Some("mm") => "ObjectiveCppFile".to_string(),
            Some("zig") => "ZigFile".to_string(),
            Some("nim") => "NimFile".to_string(),
            Some("v") => "VlangFile".to_string(),
            Some("d") => "DFile".to_string(),
            Some("cr") => "CrystalFile".to_string(),
            Some("ex") | Some("exs") => "ElixirFile".to_string(),
            Some("erl") | Some("hrl") => "ErlangFile".to_string(),
            Some("hs") | Some("lhs") => "HaskellFile".to_string(),
            Some("ml") | Some("mli") => "OCamlFile".to_string(),
            Some("clj") | Some("cljs") | Some("cljc") => "ClojureFile".to_string(),
            Some("scala") | Some("sc") => "ScalaFile".to_string(),
            Some("groovy") | Some("gvy") => "GroovyFile".to_string(),
            Some("pl") | Some("pm") => "PerlFile".to_string(),
            Some("tcl") => "TclFile".to_string(),
            Some("jl") => "JuliaFile".to_string(),
            Some("f90") | Some("f95") | Some("f03") | Some("f") | Some("for") => {
                "FortranFile".to_string()
            }
            Some("cob") | Some("cbl") => "CobolFile".to_string(),
            Some("pas") | Some("pp") => "PascalFile".to_string(),
            Some("ada") | Some("adb") | Some("ads") => "AdaFile".to_string(),
            Some("lisp") | Some("lsp") | Some("cl") => "LispFile".to_string(),
            Some("scm") | Some("ss") => "SchemeFile".to_string(),
            Some("rkt") => "RacketFile".to_string(),
            Some("coffee") => "CoffeeScriptFile".to_string(),
            Some("elm") => "ElmFile".to_string(),
            Some("purs") => "PureScriptFile".to_string(),
            Some("sol") => "SolidityFile".to_string(),
            Some("wasm") => "WebAssemblyFile".to_string(),
            Some("wat") => "WebAssemblyTextFile".to_string(),
            Some("asm") | Some("s") | Some("S") => "AssemblyFile".to_string(),
            Some("nasm") => "NasmFile".to_string(),

            // === Shader Languages ===
            Some("glsl") | Some("vert") | Some("frag") | Some("geom") | Some("comp")
            | Some("tesc") | Some("tese") => "GlslFile".to_string(),
            Some("hlsl") | Some("fx") => "HlslFile".to_string(),
            Some("wgsl") => "WgslFile".to_string(),
            Some("metal") => "MetalFile".to_string(),
            Some("cg") => "CgFile".to_string(),

            // === Data/Config formats ===
            Some("json") | Some("jsonc") | Some("json5") => "JsonFile".to_string(),
            Some("toml") => "TomlFile".to_string(),
            Some("yaml") | Some("yml") => "YamlFile".to_string(),
            Some("xml") | Some("xsd") | Some("xsl") | Some("xslt") => "XmlFile".to_string(),
            Some("ini") => "IniFile".to_string(),
            Some("env") => "EnvFile".to_string(),
            Some("cfg") | Some("config") | Some("conf") => "ConfigFile".to_string(),
            Some("properties") => "PropertiesFile".to_string(),
            Some("plist") => "PlistFile".to_string(),
            Some("csv") => "CsvFile".to_string(),
            Some("tsv") => "TsvFile".to_string(),
            Some("ndjson") | Some("jsonl") => "JsonLinesFile".to_string(),

            // === Documentation ===
            Some("md") | Some("markdown") | Some("mdown") => "MarkdownFile".to_string(),
            Some("txt") | Some("text") => "TextFile".to_string(),
            Some("rst") => "RestructuredTextFile".to_string(),
            Some("adoc") | Some("asciidoc") => "AsciiDocFile".to_string(),
            Some("org") => "OrgFile".to_string(),
            Some("tex") | Some("latex") => "LaTeXFile".to_string(),
            Some("rtf") => "RichTextFile".to_string(),
            Some("man") => "ManPageFile".to_string(),

            // === Web ===
            Some("html") | Some("htm") | Some("xhtml") => "HtmlFile".to_string(),
            Some("css") => "CssFile".to_string(),
            Some("scss") => "ScssFile".to_string(),
            Some("sass") => "SassFile".to_string(),
            Some("less") => "LessFile".to_string(),
            Some("styl") | Some("stylus") => "StylusFile".to_string(),
            Some("svg") => "SvgFile".to_string(),
            Some("woff") | Some("woff2") => "WebFontFile".to_string(),
            Some("webmanifest") => "WebManifestFile".to_string(),

            // === Databases ===
            Some("sql") => "SqlFile".to_string(),
            Some("sqlite") | Some("sqlite3") | Some("db") | Some("db3") => {
                "SqliteDatabase".to_string()
            }
            Some("mdb") | Some("accdb") => "AccessDatabase".to_string(),
            Some("graphql") | Some("gql") => "GraphQLFile".to_string(),
            Some("prisma") => "PrismaFile".to_string(),

            // === Shell/Scripts ===
            Some("sh") => "ShellScript".to_string(),
            Some("bash") => "BashScript".to_string(),
            Some("zsh") => "ZshScript".to_string(),
            Some("fish") => "FishScript".to_string(),
            Some("bat") | Some("cmd") => "BatchFile".to_string(),
            Some("ps1") | Some("psm1") | Some("psd1") => "PowerShellScript".to_string(),
            Some("nu") => "NushellScript".to_string(),
            Some("ahk") => "AutoHotkeyScript".to_string(),
            Some("applescript") | Some("scpt") => "AppleScript".to_string(),
            Some("vbs") | Some("vbe") => "VBScript".to_string(),
            Some("awk") => "AwkScript".to_string(),
            Some("sed") => "SedScript".to_string(),

            // === Build/DevOps ===
            Some("dockerfile") => "Dockerfile".to_string(),
            Some("containerfile") => "Containerfile".to_string(),
            Some("vagrantfile") => "Vagrantfile".to_string(),
            Some("tf") | Some("tfvars") => "TerraformFile".to_string(),
            Some("hcl") => "HclFile".to_string(),
            Some("mk") => "MakefileInclude".to_string(),
            Some("cmake") => "CMakeFile".to_string(),
            Some("ninja") => "NinjaFile".to_string(),
            Some("gradle") => "GradleFile".to_string(),
            Some("sbt") => "SbtFile".to_string(),
            Some("ant") => "AntFile".to_string(),
            Some("pom") => "PomFile".to_string(),
            Some("nix") => "NixFile".to_string(),
            Some("bazel") | Some("bzl") => "BazelFile".to_string(),

            // === Images ===
            Some("png") => "PngImage".to_string(),
            Some("jpg") | Some("jpeg") => "JpegImage".to_string(),
            Some("gif") => "GifImage".to_string(),
            Some("bmp") => "BmpImage".to_string(),
            Some("webp") => "WebpImage".to_string(),
            Some("ico") | Some("icon") => "IconFile".to_string(),
            Some("tiff") | Some("tif") => "TiffImage".to_string(),
            Some("psd") => "PhotoshopFile".to_string(),
            Some("ai") => "IllustratorFile".to_string(),
            Some("eps") => "EpsFile".to_string(),
            Some("raw") | Some("cr2") | Some("nef") | Some("arw") => "RawImage".to_string(),
            Some("heic") | Some("heif") => "HeicImage".to_string(),
            Some("avif") => "AvifImage".to_string(),

            // === Audio ===
            Some("mp3") => "Mp3Audio".to_string(),
            Some("wav") => "WavAudio".to_string(),
            Some("ogg") | Some("oga") => "OggAudio".to_string(),
            Some("flac") => "FlacAudio".to_string(),
            Some("aac") => "AacAudio".to_string(),
            Some("m4a") => "M4aAudio".to_string(),
            Some("wma") => "WmaAudio".to_string(),
            Some("aiff") | Some("aif") => "AiffAudio".to_string(),
            Some("opus") => "OpusAudio".to_string(),
            Some("mid") | Some("midi") => "MidiFile".to_string(),

            // === Video ===
            Some("mp4") | Some("m4v") => "Mp4Video".to_string(),
            Some("avi") => "AviVideo".to_string(),
            Some("mkv") => "MkvVideo".to_string(),
            Some("mov") => "MovVideo".to_string(),
            Some("wmv") => "WmvVideo".to_string(),
            Some("flv") => "FlvVideo".to_string(),
            Some("webm") => "WebmVideo".to_string(),
            Some("mpeg") | Some("mpg") => "MpegVideo".to_string(),
            Some("3gp") => "3gpVideo".to_string(),
            Some("ogv") => "OgvVideo".to_string(),

            // === Fonts ===
            Some("ttf") => "TrueTypeFont".to_string(),
            Some("otf") => "OpenTypeFont".to_string(),
            Some("eot") => "EmbeddedOpenTypeFont".to_string(),

            // === Archives ===
            Some("zip") => "ZipArchive".to_string(),
            Some("tar") => "TarArchive".to_string(),
            Some("gz") | Some("gzip") => "GzipArchive".to_string(),
            Some("bz2") | Some("bzip2") => "Bzip2Archive".to_string(),
            Some("xz") => "XzArchive".to_string(),
            Some("7z") => "SevenZipArchive".to_string(),
            Some("rar") => "RarArchive".to_string(),
            Some("lz") | Some("lzma") => "LzmaArchive".to_string(),
            Some("zst") | Some("zstd") => "ZstdArchive".to_string(),
            Some("cab") => "CabArchive".to_string(),
            Some("iso") => "IsoImage".to_string(),
            Some("dmg") => "DmgImage".to_string(),
            Some("deb") => "DebPackage".to_string(),
            Some("rpm") => "RpmPackage".to_string(),
            Some("apk") => "ApkPackage".to_string(),
            Some("appimage") => "AppImagePackage".to_string(),
            Some("flatpak") => "FlatpakPackage".to_string(),
            Some("snap") => "SnapPackage".to_string(),
            Some("msi") => "MsiInstaller".to_string(),
            Some("pkg") => "PkgInstaller".to_string(),

            // === Documents ===
            Some("pdf") => "PdfDocument".to_string(),
            Some("doc") | Some("docx") => "WordDocument".to_string(),
            Some("xls") | Some("xlsx") => "ExcelDocument".to_string(),
            Some("ppt") | Some("pptx") => "PowerPointDocument".to_string(),
            Some("odt") => "OpenDocumentText".to_string(),
            Some("ods") => "OpenDocumentSpreadsheet".to_string(),
            Some("odp") => "OpenDocumentPresentation".to_string(),
            Some("epub") => "EpubDocument".to_string(),
            Some("mobi") => "MobiDocument".to_string(),
            Some("djvu") => "DjvuDocument".to_string(),
            Some("xps") => "XpsDocument".to_string(),

            // === Binary/Executable ===
            Some("exe") => "WindowsExecutable".to_string(),
            Some("dll") => "WindowsDynamicLibrary".to_string(),
            Some("so") => "SharedObject".to_string(),
            Some("dylib") => "MacOSDynamicLibrary".to_string(),
            Some("a") | Some("lib") => "StaticLibrary".to_string(),
            Some("o") | Some("obj") => "ObjectFile".to_string(),
            Some("pdb") => "ProgramDatabase".to_string(),
            Some("app") => "MacOSApp".to_string(),
            Some("elf") => "ElfBinary".to_string(),
            Some("bin") => "BinaryFile".to_string(),
            Some("com") => "DosExecutable".to_string(),
            Some("sys") => "SystemDriver".to_string(),

            // === 3D/CAD/Game Assets ===
            Some("fbx") => "FbxModel".to_string(),
            Some("gltf") | Some("glb") => "GltfModel".to_string(),
            Some("stl") => "StlModel".to_string(),
            Some("dae") => "ColladaModel".to_string(),
            Some("blend") => "BlenderFile".to_string(),
            Some("max") | Some("3ds") => "3dsMaxFile".to_string(),
            Some("maya") | Some("mb") | Some("ma") => "MayaFile".to_string(),
            Some("c4d") => "Cinema4DFile".to_string(),
            Some("uasset") | Some("umap") => "UnrealAsset".to_string(),
            Some("unity") | Some("prefab") => "UnityAsset".to_string(),
            Some("rbxm") | Some("rbxmx") => "RobloxModel".to_string(),
            Some("rbxl") | Some("rbxlx") => "RobloxPlace".to_string(),

            // === Data Science / ML ===
            Some("ipynb") => "JupyterNotebook".to_string(),
            Some("parquet") => "ParquetFile".to_string(),
            Some("feather") => "FeatherFile".to_string(),
            Some("pickle") | Some("pkl") => "PickleFile".to_string(),
            Some("h5") | Some("hdf5") => "Hdf5File".to_string(),
            Some("onnx") => "OnnxModel".to_string(),
            Some("pt") | Some("pth") => "PyTorchModel".to_string(),
            Some("ckpt") => "TensorflowCheckpoint".to_string(),
            Some("safetensors") => "SafeTensorsFile".to_string(),

            // === Lock/Package files ===
            Some("lock") => "LockFile".to_string(),
            Some("sum") => "ChecksumFile".to_string(),
            Some("crate") => "CrateFile".to_string(),
            Some("whl") => "PythonWheel".to_string(),
            Some("egg") => "PythonEgg".to_string(),
            Some("gem") => "RubyGem".to_string(),
            Some("nupkg") => "NuGetPackage".to_string(),

            // === Certificate/Security ===
            Some("pem") | Some("crt") | Some("cer") => "Certificate".to_string(),
            Some("key") => "PrivateKey".to_string(),
            Some("pub") => "PublicKey".to_string(),
            Some("pfx") | Some("p12") => "PKCS12File".to_string(),
            Some("gpg") | Some("asc") => "GpgFile".to_string(),
            Some("sig") => "SignatureFile".to_string(),

            // === Game-specific ===
            Some("pak") => "PackedAsset".to_string(),
            Some("vpk") => "ValvePackFile".to_string(),
            Some("bsp") => "BspMap".to_string(),
            Some("wad") => "WadFile".to_string(),
            Some("rom") | Some("nes") | Some("snes") | Some("gba") | Some("nds") => {
                "RomFile".to_string()
            }
            Some("sav") => "SaveFile".to_string(),

            // === Miscellaneous ===
            Some("log") => "LogFile".to_string(),
            Some("tmp") | Some("temp") => "TempFile".to_string(),
            Some("bak") | Some("backup") => "BackupFile".to_string(),
            Some("cache") => "CacheFile".to_string(),
            Some("swp") | Some("swo") => "SwapFile".to_string(),
            Some("pid") => "PidFile".to_string(),
            Some("socket") | Some("sock") => "SocketFile".to_string(),
            Some("fifo") => "FifoFile".to_string(),
            Some("lnk") => "ShortcutFile".to_string(),
            Some("url") => "UrlShortcut".to_string(),
            Some("desktop") => "DesktopEntry".to_string(),

            // === Special filename matches (commonly without extension) ===
            // These are handled via filename in classify_by_filename if needed

            // Default for unknown extensions
            _ => "File".to_string(),
        }
    }

    /// Classifies a directory
    fn classify_directory(path: &Path) -> String {
        // Check if it acts as a ModuleScript (contains init.luau or init.lua)
        if path.join("init.luau").exists() || path.join("init.lua").exists() {
            "ModuleScript".to_string()
        } else {
            "Folder".to_string()
        }
    }

    pub fn name(&self) -> String {
        if self.is_file {
            self.path
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("Script")
                .to_string()
        } else {
            self.path
                .file_name()
                .and_then(|s| s.to_str())
                .unwrap_or("Folder")
                .to_string()
        }
    }

    pub fn parent(&self) -> Option<Self> {
        self.path.parent().map(|p| ScriptContext::new(p))
    }

    pub fn get_full_name(&self) -> String {
        self.path.to_string_lossy().replace('\\', "/")
    }

    pub fn get_children(&self) -> Vec<Self> {
        // Files (Scripts) do not have children.
        // Only Folders can have children.
        if self.is_file {
            return Vec::new();
        }

        let mut children = Vec::new();
        if let Ok(entries) = fs::read_dir(&self.path) {
            for entry in entries.flatten() {
                let path = entry.path();
                if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                    // Skip hidden files
                    if !name.starts_with('.') {
                        children.push(ScriptContext::new(path));
                    }
                }
            }
        }
        children
    }

    pub fn find_first_child(&self, name: &str) -> Option<Self> {
        // Files cannot have children
        if self.is_file {
            return None;
        }

        let dir_path = &self.path;

        // 1. Exact match (File or Folder)
        let exact = dir_path.join(name);
        if exact.exists() {
            return Some(ScriptContext::new(exact));
        }

        // 2. Match with .luau extension
        let with_luau = dir_path.join(format!("{}.luau", name));
        if with_luau.exists() {
            return Some(ScriptContext::new(with_luau));
        }

        // 3. Match with .lua extension
        let with_lua = dir_path.join(format!("{}.lua", name));
        if with_lua.exists() {
            return Some(ScriptContext::new(with_lua));
        }

        // 4. Match as init.luau inside folder (returns the Folder/ModuleScript context)
        let folder = dir_path.join(name);
        if folder.is_dir() {
            let init_luau = folder.join("init.luau");
            let init_lua = folder.join("init.lua");
            if init_luau.exists() || init_lua.exists() {
                return Some(ScriptContext::new(folder));
            }
        }

        None
    }

    pub fn source(&self) -> Option<String> {
        // Only files have source code. Directories (Folders/ModuleScripts) do not.
        if self.is_file {
            fs::read_to_string(&self.path).ok()
        } else {
            None
        }
    }

    pub fn find_first_ancestor(&self, name: &str) -> Option<Self> {
        let mut current = self.parent()?;
        loop {
            if current.name() == name {
                return Some(current);
            }
            match current.parent() {
                Some(p) => current = p,
                None => return None,
            }
        }
    }

    pub fn is_descendant_of(&self, ancestor: &Self) -> bool {
        self.path.starts_with(&ancestor.path) && self.path != ancestor.path
    }

    pub fn is_ancestor_of(&self, descendant: &Self) -> bool {
        descendant.is_descendant_of(self)
    }
}

impl LuaUserData for ScriptContext {
    fn add_fields<F: LuaUserDataFields<Self>>(fields: &mut F) {
        fields.add_field_method_get("Name", |_, this| Ok(this.name()));
        fields.add_field_method_get("ClassName", |_, this| Ok(this.class_name.clone()));
        fields.add_field_method_get("Enabled", |_, _| Ok(true));
        fields.add_field_method_get("Parent", |lua, this| match this.parent() {
            Some(parent) => lua.create_userdata(parent).map(LuaValue::UserData),
            None => Ok(LuaValue::Nil),
        });
        fields.add_field_method_get("Source", |_, this| Ok(this.source()));
    }

    fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
        methods.add_method("GetFullName", |_, this, ()| Ok(this.get_full_name()));

        methods.add_method("GetChildren", |lua, this, ()| {
            let children = this.get_children();
            let table = lua.create_table()?;
            for (i, child) in children.into_iter().enumerate() {
                table.set(i + 1, lua.create_userdata(child)?)?;
            }
            Ok(table)
        });

        methods.add_method("FindFirstChild", |lua, this, name: String| {
            match this.find_first_child(&name) {
                Some(child) => lua.create_userdata(child).map(LuaValue::UserData),
                None => Ok(LuaValue::Nil),
            }
        });

        methods.add_method("WaitForChild", |lua, this, name: String| {
            // Synchronous environment: WaitForChild == FindFirstChild
            match this.find_first_child(&name) {
                Some(child) => lua.create_userdata(child).map(LuaValue::UserData),
                None => Ok(LuaValue::Nil),
            }
        });

        methods.add_method("FindFirstAncestor", |lua, this, name: String| {
            match this.find_first_ancestor(&name) {
                Some(ancestor) => lua.create_userdata(ancestor).map(LuaValue::UserData),
                None => Ok(LuaValue::Nil),
            }
        });

        methods.add_method(
            "IsDescendantOf",
            |_, this, ancestor: LuaUserDataRef<ScriptContext>| Ok(this.is_descendant_of(&ancestor)),
        );

        methods.add_method(
            "IsAncestorOf",
            |_, this, descendant: LuaUserDataRef<ScriptContext>| {
                Ok(this.is_ancestor_of(&descendant))
            },
        );

        methods.add_method("GetDescendants", |lua, this, ()| {
            fn get_all_descendants(path: &Path, results: &mut Vec<ScriptContext>) {
                if let Ok(entries) = fs::read_dir(path) {
                    for entry in entries.flatten() {
                        let p = entry.path();
                        if let Some(name) = p.file_name().and_then(|n| n.to_str()) {
                            if !name.starts_with('.') {
                                results.push(ScriptContext::new(&p));
                                if p.is_dir() {
                                    get_all_descendants(&p, results);
                                }
                            }
                        }
                    }
                }
            }

            // Files don't have descendants in the file system hierarchy
            if this.is_file {
                return Ok(lua.create_table()?);
            }

            let mut descendants = Vec::new();
            get_all_descendants(&this.path, &mut descendants);

            let table = lua.create_table()?;
            for (i, desc) in descendants.into_iter().enumerate() {
                table.set(i + 1, lua.create_userdata(desc)?)?;
            }
            Ok(table)
        });

        methods.add_method("Clone", |lua, this, ()| lua.create_userdata(this.clone()));
        methods.add_method("Destroy", |_, _, ()| Ok(()));
        methods.add_method("GetAttribute", |_, _, _: String| -> LuaResult<LuaValue> {
            Ok(LuaValue::Nil)
        });

        // __tostring returns the relative path suitable for require()
        // This allows require(script.Parent.script2) to work directly
        methods.add_meta_method(LuaMetaMethod::ToString, |lua, this, ()| {
            // Get current script context to calculate relative path
            let current_ctx: LuaResult<LuaUserDataRef<ScriptContext>> =
                lua.named_registry_value(SCRIPT_CONTEXT_KEY);

            let target_path = this.path.clone();

            // Calculate relative path from current script to target
            if let Ok(current) = current_ctx {
                let current_dir = current.path.parent().unwrap_or(&current.path);
                Ok(calculate_relative_path(current_dir, &target_path))
            } else {
                let cwd = std::env::current_dir().unwrap_or_default();
                Ok(calculate_relative_path(&cwd, &target_path))
            }
        });

        methods.add_meta_method(
            LuaMetaMethod::Eq,
            |_, this, other: LuaUserDataRef<ScriptContext>| Ok(this.path == other.path),
        );

        // __index para FindFirstChild automatico - permite script.Parent.Script2
        methods.add_meta_method(LuaMetaMethod::Index, |lua, this, key: String| {
            // Tenta encontrar um filho com esse nome
            match this.find_first_child(&key) {
                Some(child) => lua.create_userdata(child).map(LuaValue::UserData),
                None => Ok(LuaValue::Nil),
            }
        });

        // __newindex para proteção e setters
        methods.add_meta_method_mut(
            LuaMetaMethod::NewIndex,
            |_, this, (key, value): (String, LuaValue)| {
                match key.as_str() {
                    // Name - rename the file/folder
                    "Name" => {
                        let new_name: String = match value {
                            LuaValue::String(s) => s.to_string_lossy().to_string(),
                            _ => return Err(LuaError::runtime("Name must be a string")),
                        };
                        // Keep parent path, change only the name
                        if let Some(parent) = this.path.parent() {
                            let ext = this.path.extension().map(|e| e.to_os_string());
                            let mut new_path = parent.join(&new_name);
                            if let Some(ext) = ext {
                                new_path = new_path.with_extension(ext);
                            }
                            this.path = new_path;
                        }
                        Ok(())
                    }
                    // Source - write content to file
                    "Source" => {
                        let content: String = match value {
                            LuaValue::String(s) => s.to_string_lossy().to_string(),
                            _ => return Err(LuaError::runtime("Source must be a string")),
                        };
                        std::fs::write(&this.path, content).map_err(|e| {
                            LuaError::runtime(format!("Failed to write Source: {}", e))
                        })
                    }
                    // Read-only properties
                    "ClassName" | "Enabled" => Err(LuaError::runtime(format!(
                        "'{}' is read-only. Use ChangeClass() to change file extension.",
                        key
                    ))),
                    // Parent - can be modified to move the script
                    "Parent" => match value {
                        LuaValue::UserData(ud) => {
                            if let Ok(parent_ctx) = ud.borrow::<ScriptContext>() {
                                let ext = this.path.extension().map(|e| e.to_os_string());
                                let mut new_path = parent_ctx.path.join(this.name());
                                if let Some(ext) = ext {
                                    new_path = new_path.with_extension(ext);
                                }
                                this.path = new_path;
                                Ok(())
                            } else {
                                Err(LuaError::runtime("Parent must be a ScriptContext"))
                            }
                        }
                        LuaValue::Nil => {
                            let cwd = std::env::current_dir().unwrap_or_default();
                            let ext = this.path.extension().map(|e| e.to_os_string());
                            let mut new_path = cwd.join(this.name());
                            if let Some(ext) = ext {
                                new_path = new_path.with_extension(ext);
                            }
                            this.path = new_path;
                            Ok(())
                        }
                        _ => Err(LuaError::runtime("Parent must be a ScriptContext or nil")),
                    },
                    _ => Err(LuaError::runtime(format!(
                        "'{}' is not a valid property of ScriptContext",
                        key
                    ))),
                }
            },
        );

        // ChangeClass - change file extension (e.g., .txt -> .lua)
        methods.add_method_mut("ChangeClass", |_, this, new_ext: String| {
            // Remove leading dot if present
            let ext = new_ext.strip_prefix('.').unwrap_or(&new_ext);
            this.path = this.path.with_extension(ext);
            // Update class_name based on new extension
            this.class_name = Self::classify_file(&this.path);
            Ok(())
        });
    }
}

/// Calculate relative path from source directory to target file
fn calculate_relative_path(from_dir: &Path, to_file: &Path) -> String {
    // Remove Windows extended path prefix if present (\\?\)
    let from_str = from_dir.to_string_lossy();
    let to_str = to_file.to_string_lossy();

    let from_clean = if from_str.starts_with(r"\\?\") {
        PathBuf::from(&from_str[4..])
    } else {
        from_dir.to_path_buf()
    };

    let to_clean = if to_str.starts_with(r"\\?\") {
        PathBuf::from(&to_str[4..])
    } else {
        to_file.to_path_buf()
    };

    // Try to find common ancestor and compute relative path
    let from_components: Vec<_> = from_clean.components().collect();
    let to_components: Vec<_> = to_clean.components().collect();

    // Find common prefix length
    let common_len = from_components
        .iter()
        .zip(to_components.iter())
        .take_while(|(a, b)| a == b)
        .count();

    // Build relative path
    let mut result = String::new();

    // Add ../ for each remaining component in from_dir
    let ups = from_components.len() - common_len;
    if ups == 0 {
        result.push_str("./");
    } else {
        for _ in 0..ups {
            result.push_str("../");
        }
    }

    // Add remaining components from target
    let remaining: Vec<_> = to_components[common_len..]
        .iter()
        .map(|c| c.as_os_str().to_string_lossy().to_string())
        .collect();
    result.push_str(&remaining.join("/"));

    // Remove file extension for require
    result
        .strip_suffix(".luau")
        .or_else(|| result.strip_suffix(".lua"))
        .unwrap_or(&result)
        .to_string()
}

/// Sets the script context in the Lua registry.
/// This should be called BEFORE executing a script so that `script` global works correctly.
pub fn set_script_context<P: AsRef<Path>>(lua: &Lua, path: P) -> LuaResult<()> {
    // Make sure we have an absolute, canonicalized path
    let path = path.as_ref();
    let absolute_path = if path.is_absolute() {
        path.to_path_buf()
    } else {
        std::env::current_dir()
            .unwrap_or_else(|_| PathBuf::from("."))
            .join(path)
    };

    // Canonicalize to resolve any .. or symlinks
    let canonical_path = absolute_path.canonicalize().unwrap_or(absolute_path);

    let ctx = ScriptContext::new(canonical_path);
    let ud = lua.create_userdata(ctx)?;
    lua.set_named_registry_value(SCRIPT_CONTEXT_KEY, ud)
}

/// Create the script global - returns a lazy proxy that reads from registry when accessed
pub fn create(lua: Lua) -> LuaResult<LuaValue> {
    // Create a proxy table with __index that reads from registry each time
    let proxy = lua.create_table()?;

    // Store the lua reference for the metatable
    let lua_clone = lua.clone();

    let index_fn = lua.create_function(move |inner_lua, (_, key): (LuaTable, String)| {
        // Get the current ScriptContext from registry (set by runtime before execution)
        let result: LuaResult<LuaUserDataRef<ScriptContext>> =
            lua_clone.named_registry_value(SCRIPT_CONTEXT_KEY);

        let ctx: ScriptContext = match result {
            Ok(ctx_ref) => ScriptContext {
                path: ctx_ref.path.clone(),
                is_file: ctx_ref.is_file,
                class_name: ctx_ref.class_name.clone(),
            },
            Err(_) => {
                // Fallback: current directory (shouldn't happen in normal usage)
                let cwd = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
                ScriptContext::new(cwd)
            }
        };

        // Handle property access
        match key.as_str() {
            "Name" => Ok(LuaValue::String(inner_lua.create_string(&ctx.name())?)),
            "ClassName" => Ok(LuaValue::String(inner_lua.create_string(&ctx.class_name)?)),
            "Enabled" => Ok(LuaValue::Boolean(true)),
            "Parent" => match ctx.parent() {
                Some(parent) => inner_lua.create_userdata(parent).map(LuaValue::UserData),
                None => Ok(LuaValue::Nil),
            },
            "Source" => match ctx.source() {
                Some(src) => Ok(LuaValue::String(inner_lua.create_string(&src)?)),
                None => Ok(LuaValue::Nil),
            },
            // Methods - return the underlying userdata method
            "GetFullName" => {
                let ud = inner_lua.create_userdata(ctx)?;
                ud.get::<LuaFunction>("GetFullName").map(LuaValue::Function)
            }
            "GetChildren" => {
                let ud = inner_lua.create_userdata(ctx)?;
                ud.get::<LuaFunction>("GetChildren").map(LuaValue::Function)
            }
            "FindFirstChild" => {
                let ud = inner_lua.create_userdata(ctx)?;
                ud.get::<LuaFunction>("FindFirstChild")
                    .map(LuaValue::Function)
            }
            "WaitForChild" => {
                let ud = inner_lua.create_userdata(ctx)?;
                ud.get::<LuaFunction>("WaitForChild")
                    .map(LuaValue::Function)
            }
            "FindFirstAncestor" => {
                let ud = inner_lua.create_userdata(ctx)?;
                ud.get::<LuaFunction>("FindFirstAncestor")
                    .map(LuaValue::Function)
            }
            "IsDescendantOf" => {
                let ud = inner_lua.create_userdata(ctx)?;
                ud.get::<LuaFunction>("IsDescendantOf")
                    .map(LuaValue::Function)
            }
            "IsAncestorOf" => {
                let ud = inner_lua.create_userdata(ctx)?;
                ud.get::<LuaFunction>("IsAncestorOf")
                    .map(LuaValue::Function)
            }
            "GetDescendants" => {
                let ud = inner_lua.create_userdata(ctx)?;
                ud.get::<LuaFunction>("GetDescendants")
                    .map(LuaValue::Function)
            }
            "Clone" => {
                let ud = inner_lua.create_userdata(ctx)?;
                ud.get::<LuaFunction>("Clone").map(LuaValue::Function)
            }
            "Destroy" => {
                let ud = inner_lua.create_userdata(ctx)?;
                ud.get::<LuaFunction>("Destroy").map(LuaValue::Function)
            }
            "GetAttribute" => {
                let ud = inner_lua.create_userdata(ctx)?;
                ud.get::<LuaFunction>("GetAttribute")
                    .map(LuaValue::Function)
            }
            "GetAttributes" => {
                let ud = inner_lua.create_userdata(ctx)?;
                ud.get::<LuaFunction>("GetAttributes")
                    .map(LuaValue::Function)
            }
            _ => Ok(LuaValue::Nil),
        }
    })?;

    // Create metatable with __index
    let metatable = lua.create_table()?;
    metatable.set("__index", index_fn)?;

    // Also need __tostring for the proxy
    let lua_clone2 = lua.clone();
    let tostring_fn = lua.create_function(move |_, _: LuaTable| {
        let result: LuaResult<LuaUserDataRef<ScriptContext>> =
            lua_clone2.named_registry_value(SCRIPT_CONTEXT_KEY);
        match result {
            Ok(ctx) => Ok(format!("{}(\"{}\")", ctx.class_name, ctx.name())),
            Err(_) => Ok("Script(\"unknown\")".to_string()),
        }
    })?;
    metatable.set("__tostring", tostring_fn)?;

    proxy.set_metatable(Some(metatable))?;

    Ok(LuaValue::Table(proxy))
}
