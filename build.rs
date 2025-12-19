use std::path::{Path, PathBuf};
use std::{fs, process::Command};
use std::io::Result;


fn set_executable_permission(path: &Path) -> Result<()> {
    // Extract file metadata
    let metadata = fs::metadata(path)?;
    let mut permissions = metadata.permissions();

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        
        // Octal (r-xr-x---)
        let executable_mode: u32 = 0o550;
        
        let current_mode = permissions.mode();
        permissions.set_mode(current_mode | executable_mode);
    }

    // Apply mode 
    fs::set_permissions(path, permissions)?;

    Ok(())
}

fn main() -> Result<()> {
    // Rerun in every change
    let watch_dirs = ["src", "public", "assets", "templates"];
    for dir in watch_dirs {
        println!("cargo:rerun-if-changed={}", dir);
    }

    // Check tailwind existance
    let file_path = Path::new("assets/tailwindcss");
    // check file existance and set permissions
    if file_path.exists() {
        if let Err(e) = set_executable_permission(file_path) {
            println!("cargo:warning=Failed to set executable permissions for {}: {}", file_path.display(), e);
            return Err(e);
        }
    }

    // Exec tailwind script
    match Command::new(file_path).args(&["-i", "assets/input.css", "-o", "public/output.css", "--minify"]).status() {
        Ok(status) if status.success() => println!("cargo:warning=tailwindcss produced public/output.css"),
        Ok(status) => println!("cargo:warning=tailwindcss exited with status: {}", status),
        Err(e) => println!("cargo:warning=Failed to execute tailwindcss: {}", e),
    }

    Ok(())
}
