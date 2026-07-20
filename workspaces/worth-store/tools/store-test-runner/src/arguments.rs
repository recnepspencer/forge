use std::path::PathBuf;

use crate::classification::CiTestLane;
use crate::product::TestProduct;

#[derive(Debug, Clone, Eq, PartialEq)]
pub(crate) struct Arguments {
    pub(crate) product: TestProduct,
    pub(crate) list: bool,
    pub(crate) target_root: Option<PathBuf>,
    pub(crate) report: Option<PathBuf>,
}

impl Arguments {
    pub(crate) fn parse(arguments: impl IntoIterator<Item = String>) -> Result<Self, String> {
        let mut arguments = arguments.into_iter();
        let command = arguments.next().ok_or_else(usage)?;
        let mut package = None;
        let mut partition = None;
        let mut shard_index = None;
        let mut shard_count = None;
        let mut list = false;
        let mut target_root = None;
        let mut report = None;

        while let Some(argument) = arguments.next() {
            match argument.as_str() {
                "-p" | "--package" => package = Some(value(&mut arguments, &argument)?),
                "--partition" => {
                    partition = Some(value(&mut arguments, &argument)?.parse::<CiTestLane>()?)
                }
                "--shard-index" => shard_index = Some(number(&mut arguments, &argument)?),
                "--shard-count" => shard_count = Some(number(&mut arguments, &argument)?),
                "--target-root" => {
                    target_root = Some(PathBuf::from(value(&mut arguments, &argument)?))
                }
                "--report" => report = Some(PathBuf::from(value(&mut arguments, &argument)?)),
                "--list" => list = true,
                unknown => return Err(format!("unknown argument `{unknown}`\n{}", usage())),
            }
        }

        let product = match command.as_str() {
            "owner" => TestProduct::Owner {
                package: package
                    .clone()
                    .ok_or_else(|| "owner requires -p <package>".to_owned())?,
            },
            "smoke" => TestProduct::Smoke,
            "ui" => TestProduct::Ui,
            "ci" => TestProduct::Ci {
                lane: partition.ok_or_else(|| "ci requires --partition <lane>".to_owned())?,
                shard: shard(shard_index, shard_count)?,
            },
            unknown => return Err(format!("unknown command `{unknown}`\n{}", usage())),
        };

        if !matches!(product, TestProduct::Owner { .. }) && package.is_some() {
            return Err("-p/--package is valid only for owner".into());
        }
        if !matches!(product, TestProduct::Ci { .. })
            && (partition.is_some() || shard_index.is_some() || shard_count.is_some())
        {
            return Err("partition and shard arguments are valid only for ci".into());
        }

        Ok(Self {
            product,
            list,
            target_root,
            report,
        })
    }
}

fn value(arguments: &mut impl Iterator<Item = String>, option: &str) -> Result<String, String> {
    arguments
        .next()
        .ok_or_else(|| format!("{option} requires a value"))
}

fn number(arguments: &mut impl Iterator<Item = String>, option: &str) -> Result<usize, String> {
    let raw = value(arguments, option)?;
    raw.parse()
        .map_err(|_| format!("{option} requires a non-negative integer, got `{raw}`"))
}

fn shard(index: Option<usize>, count: Option<usize>) -> Result<Option<(usize, usize)>, String> {
    match (index, count) {
        (None, None) => Ok(None),
        (Some(index), Some(count)) if count > 0 && index < count => Ok(Some((index, count))),
        (Some(_), Some(0)) => Err("--shard-count must be greater than zero".into()),
        (Some(index), Some(count)) => Err(format!(
            "--shard-index {index} is outside shard count {count}"
        )),
        _ => Err("--shard-index and --shard-count must be supplied together".into()),
    }
}

fn usage() -> String {
    "usage: store-test-runner <owner -p PACKAGE|smoke|ui|ci --partition LANE> \
     [--shard-index N --shard-count N] [--list] [--target-root PATH] [--report PATH]"
        .into()
}

#[cfg(test)]
mod tests {
    use super::Arguments;
    use crate::product::TestProduct;

    #[test]
    fn owner_requires_a_package() {
        let error = Arguments::parse(["owner".into()]).unwrap_err();
        assert!(error.contains("requires -p"));
    }

    #[test]
    fn shard_arguments_are_a_pair() {
        let error = Arguments::parse([
            "ci".into(),
            "--partition".into(),
            "scenario".into(),
            "--shard-index".into(),
            "0".into(),
        ])
        .unwrap_err();
        assert!(error.contains("supplied together"));
    }

    #[test]
    fn parses_owner_options() {
        let parsed = Arguments::parse([
            "owner".into(),
            "-p".into(),
            "worth-store".into(),
            "--list".into(),
        ])
        .unwrap();
        assert!(parsed.list);
        assert_eq!(
            parsed.product,
            TestProduct::Owner {
                package: "worth-store".into()
            }
        );
    }
}
