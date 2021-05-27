use log::{error, info, debug};
use structopt::StructOpt;
use anyhow::{Context, Result};

/// Search for a pattern in a file and display the lines that contain it.
#[derive(StructOpt)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn find_matches(content: &str, pattern: &str, mut writer: impl std::io::Write) {
    for line in content.lines() {
        if line.contains(pattern) {
            let result = writeln!(writer, "{}", line);
            match result {
                Ok(_content) => { }
                Err(error) => { error!("Oh noes: {:?}", error); }
            }
        }
    }
}


fn main() -> Result<()> {
    env_logger::init();
    info!("starting up");
    let args = Cli::from_args();
    let content = std::fs::read_to_string(&args.path)
        .with_context(|| format!("could not read file `{}`", args.path.display()))?;

    find_matches(&content, &args.pattern, &mut std::io::stdout());
    debug!("Hello, world!");

    Ok(())
}

#[test]
fn check_answer_validity() {
    assert_eq!(6 * 7, 42);
}

#[test]
fn find_a_match() {
    let mut result = Vec::new();
    find_matches("lorem ipsum\ndolor sit amet", "lorem", &mut result);
    assert_eq!(result, b"lorem ipsum\n");
}
