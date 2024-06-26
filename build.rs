use std::env;
use std::fs;
use std::path::Path;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

/// The delimiter of the path.
/// It is `/` on Unix-like systems and `\` on Windows.
const DELIMITER: char = if cfg!(windows) { '\\' } else { '/' };

fn main() -> Result<()> {
    if cfg!(debug_assertions) {
        // if the build script is in debug mode, do nothing.
    } else {
        // if the build script is not in debug mode, build the project.
        // and copy the binary to the `~/.coppo/bin` directory.
        build()?;
    }

    Ok(())
}

fn build() -> Result<()> {
    // Get the home directory.
    let home_dir = dirs::home_dir().ok_or("Failed to get the home directory")?;

    // Create the `.coppo` directory in the home directory.
    let coppo_dir = home_dir.join(".coppo");
    if !coppo_dir.exists() {
        fs::create_dir(&coppo_dir)
            .map_err(|e| format!("Failed to create the `.coppo` directory: {}", e))?;
    }

    // Get the name of the binary.
    let mut bin_name = env::var("CARGO_PKG_NAME")
        .map_err(|e| format!("Failed to get the name of the binary: {}", e))?;

    if cfg!(windows) {
        // Add the `.exe` extension to the binary name on Windows.
        bin_name = format!("{}.exe", bin_name);
    }

    // Get the target directory.
    // take while the string is not equal to "target"
    // so the `target` string is not included in the path.
    let root_dir = env::var("OUT_DIR")
        .map_err(|e| format!("Failed to get the target directory: {}", e))?
        .split(DELIMITER)
        .take_while(|s| *s != "target")
        .collect::<Vec<_>>()
        .join(&DELIMITER.to_string());

    // Copy the binary to the target directory.
    let bin_path = Path::new(&root_dir)
        .join("target")
        .join("release")
        .join(&bin_name);

    if !coppo_dir.join("bin").exists() {
        fs::create_dir(coppo_dir.join("bin"))
            .map_err(|e| format!("Failed to create the `~/.coppo/bin` directory: {}", e))?;
    }

    fs::copy(&bin_path, coppo_dir.join("bin").join(&bin_name)).map_err(|e| {
        format!(
            "Failed to copy the binary to the `.coppo/bin` directory: {}",
            e
        )
    })?;

    // Write configuration file.
    let config_file = coppo_dir.join("config.toml");

    if !config_file.exists() {
        fs::write(
            &config_file,
            "\
            [linker]\n\
            compiler = \"clang++\"\n\
            ",
        )
        .map_err(|e| format!("Failed to write the configuration file: {}", e))?;
    }

    Ok(())
}
