#!/usr/bin/env python3
import argparse
import shutil
from pathlib import Path


def main():
    parser = argparse.ArgumentParser(description="Promote a relational perf summary JSONL to the committed baseline path")
    parser.add_argument("--input", required=True, help="Summary JSONL to promote")
    parser.add_argument("--baseline", required=True, help="Baseline JSONL destination")
    parser.add_argument("--markdown-input", help="Optional markdown summary to promote alongside the JSONL")
    parser.add_argument("--markdown-output", help="Optional markdown baseline destination")
    args = parser.parse_args()

    input_path = Path(args.input)
    baseline_path = Path(args.baseline)
    baseline_path.parent.mkdir(parents=True, exist_ok=True)
    shutil.copyfile(input_path, baseline_path)

    if args.markdown_input and args.markdown_output:
        markdown_input = Path(args.markdown_input)
        markdown_output = Path(args.markdown_output)
        markdown_output.parent.mkdir(parents=True, exist_ok=True)
        shutil.copyfile(markdown_input, markdown_output)

    print(f"[relational-perf-baseline] promoted {input_path} -> {baseline_path}")


if __name__ == "__main__":
    main()
