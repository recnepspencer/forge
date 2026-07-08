use std::env;
use std::path::PathBuf;

pub(crate) enum Mode {
    Generate,
    Check,
}

pub(crate) struct CliConfig {
    pub(crate) mode: Mode,
    pub(crate) root: PathBuf,
    pub(crate) config: PathBuf,
}

pub(crate) fn parse_args() -> Result<CliConfig, String> {
    let mut mode = Mode::Generate;
    let mut root = PathBuf::from(".");
    let mut config = PathBuf::from("tools/boundary-check/config/road1.toml");
    let mut args = env::args().skip(1);

    if let Some(first) = args.next() {
        match first.as_str() {
            "generate" => mode = Mode::Generate,
            "check" => mode = Mode::Check,
            "--root" | "--config" => {
                apply_flag(first, &mut args, &mut root, &mut config)?;
            }
            other => return Err(format!("unknown argument: {other}")),
        }
    }

    while let Some(arg) = args.next() {
        apply_flag(arg, &mut args, &mut root, &mut config)?;
    }

    Ok(CliConfig { mode, root, config })
}

fn apply_flag(
    arg: String,
    args: &mut impl Iterator<Item = String>,
    root: &mut PathBuf,
    config: &mut PathBuf,
) -> Result<(), String> {
    match arg.as_str() {
        "--root" => *root = PathBuf::from(args.next().ok_or("missing --root value")?),
        "--config" => *config = PathBuf::from(args.next().ok_or("missing --config value")?),
        other => return Err(format!("unknown argument: {other}")),
    }
    Ok(())
}
