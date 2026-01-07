// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    // Fix for white screen on some Linux setups (rendering issues)
    #[cfg(target_os = "linux")]
    {
        // Disable DMABUF renderer (often helps with NVIDIA/new WebKitGTK)
        std::env::set_var("WEBKIT_DISABLE_DMABUF_RENDERER", "1");
        // Often needed for AppImages on some distros to prevent white screen
        // std::env::set_var("WEBKIT_DISABLE_COMPOSITING_MODE", "1");
    }

    let _ = fix_path_env::fix();
    codexia_lib::run()
}
