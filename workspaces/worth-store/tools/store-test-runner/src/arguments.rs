use std::path::PathBuf;

use crate::classification::CiTestLane;
use crate::product::TestProduct;

#[derive(Debug, Clone, Eq, PartialEq)]
pub(crate) struct Arguments {
    pub(crate) product: TestProduct,
    pub(crate) list: bool,
    pub(crate) target_root: Option<PathBuf>,
    pub(crate) report: Option<PathBuf>,
    pub(crate) mutant: Option<u8>,
    pub(crate) first_mutant: Option<u8>,
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
        let mut mutant = None;
        let mut first_mutant = None;

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
                "--mutant" => {
                    let raw = value(&mut arguments, &argument)?;
                    mutant = Some(raw.parse().map_err(|_| {
                        format!("--mutant requires an integer from 1 through 14, got `{raw}`")
                    })?)
                }
                "--from-mutant" => {
                    let raw = value(&mut arguments, &argument)?;
                    first_mutant = Some(raw.parse().map_err(|_| {
                        format!("--from-mutant requires an integer from 1 through 14, got `{raw}`")
                    })?)
                }
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
            "mutants" => TestProduct::Mutants,
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
        if !matches!(product, TestProduct::Mutants) && (mutant.is_some() || first_mutant.is_some())
        {
            return Err("mutation selectors are valid only for mutants".into());
        }
        if mutant.is_some() && first_mutant.is_some() {
            return Err("--mutant and --from-mutant are mutually exclusive".into());
        }
        if mutant
            .or(first_mutant)
            .is_some_and(|id| !(1..=14).contains(&id))
        {
            return Err("mutation selectors require an integer from 1 through 14".into());
        }

        Ok(Self {
            product,
            list,
            target_root,
            report,
            mutant,
            first_mutant,
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
    "usage: store-test-runner <owner -p PACKAGE|smoke|ui|mutants|ci --partition LANE> \
     [--shard-index N --shard-count N] [--mutant N|--from-mutant N] [--list] [--target-root PATH] [--report PATH]"
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

    #[test]
    fn parses_mutation_campaign() {
        let parsed = Arguments::parse(["mutants".into(), "--list".into()]).unwrap();
        assert!(parsed.list);
        assert_eq!(parsed.product, TestProduct::Mutants);
        assert_eq!(parsed.mutant, None);
        assert_eq!(parsed.first_mutant, None);
    }

    #[test]
    fn mutation_campaign_accepts_one_bounded_selector_mode() {
        let selected =
            Arguments::parse(["mutants".into(), "--mutant".into(), "14".into()]).unwrap();
        assert_eq!(selected.mutant, Some(14));
        assert_eq!(selected.first_mutant, None);

        let resumed =
            Arguments::parse(["mutants".into(), "--from-mutant".into(), "11".into()]).unwrap();
        assert_eq!(resumed.mutant, None);
        assert_eq!(resumed.first_mutant, Some(11));

        for invalid in [
            ["mutants", "--mutant", "0"],
            ["mutants", "--from-mutant", "15"],
        ] {
            assert!(Arguments::parse(invalid.map(str::to_owned)).is_err());
        }
        assert!(Arguments::parse([
            "mutants".into(),
            "--mutant".into(),
            "1".into(),
            "--from-mutant".into(),
            "2".into(),
        ])
        .is_err());
    }
}
