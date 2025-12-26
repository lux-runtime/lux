fn main() {
    // Only run on Windows
    #[cfg(target_os = "windows")]
    {
        let mut res = winres::WindowsResource::new();
        res.set_icon("../../assets/icon.ico");
        println!("cargo:rerun-if-changed=../../assets/icon.ico");
        res.set("ProductName", "Lux Runtime");
        res.set("FileDescription", "Lux - Luau Runtime");
        res.set("LegalCopyright", "Copyright © 2025 Lux Runtime");

        if let Err(e) = res.compile() {
            eprintln!("Warning: Failed to compile Windows resources: {}", e);
            // Don't fail the build if icon is missing
        }
    }
}
