from pathlib import Path

from runner.generation import ScaffoldRequest, generate_scaffold


def run_generate_command(args) -> int:
    result = generate_scaffold(
        ScaffoldRequest(
            args.kind,
            args.name,
            Path(args.project_root).resolve(),
            args.spec,
            args.force,
            args.telegram,
        )
    )
    print(result.config_path)
    return 0
