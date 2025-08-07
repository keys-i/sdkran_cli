use sdkran::utils::{
    constants::VAR_DIR,
    directory_utils::infer_sdkman_dir,
    file_utils::{check_file_exists, read_file_content},
};
use std::{io::Write, path::PathBuf, process};
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

const CLI_VERSION_FILE: &str = "version";
const NATIVE_VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    // StandardStream to handle color ouptut
    let mut stdout = StandardStream::stdout(ColorChoice::Always);

    // Attempt to infer SDKMAN dir
    let sdkman_dir = match infer_sdkman_dir() {
        Ok(dir) => dir,
        Err(e) => {
            print_error(
                &mut stdout,
                &format!("Failed to infer SDKMAN directory: {}", e),
            );
            process::exit(1);
        }
    };

    let cli_version_file: PathBuf = sdkman_dir.join(VAR_DIR).join(CLI_VERSION_FILE);

    let cli_version = match check_file_exists(&cli_version_file) {
        Ok(true) => match read_file_content(&cli_version_file) {
            Ok(content) => content,
            Err(e) => {
                print_error(&mut stdout, &format!("Failed to read file content: {}", e));
                process::exit(1);
            }
        },
        Ok(false) => {
            print_error(&mut stdout, "CLI version file not found.");
            process::exit(1);
        }
        Err(e) => {
            print_error(
                &mut stdout,
                &format!("Failed to check file existence: {}", e),
            );
            process::exit(1);
        }
    };
    print_info(&mut stdout, "SDKRAN!".to_string());
    println!(
        "script: {}\nnative: {} ({} {})\n",
        cli_version,
        NATIVE_VERSION,
        std::env::consts::OS,
        std::env::consts::ARCH
    );
}

/// Function to print colored info text
fn print_info(stdout: &mut StandardStream, message: String) {
    let mut color_spec = ColorSpec::new();
    color_spec.set_bold(true).set_fg(Some(Color::Yellow));
    stdout.set_color(&color_spec).unwrap();
    writeln!(stdout, "{}", message).unwrap();
    stdout.reset().unwrap();
}

/// Function to print colored error text
fn print_error(stdout: &mut StandardStream, message: &str) {
    let mut color_spec = ColorSpec::new();
    color_spec.set_fg(Some(Color::Red));
    stdout.set_color(&color_spec).unwrap();
    writeln!(stdout, "Error: {}", message).unwrap();
    stdout.reset().unwrap();
}
