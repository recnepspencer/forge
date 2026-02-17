#!/usr/bin/env bash
# OCC Test Runner
# Creates a Python venv (if needed), installs dependencies, and runs pytest.

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
VENV_DIR="$SCRIPT_DIR/.venv"

if [ ! -d "$VENV_DIR" ]; then
    echo "Creating Python venv at $VENV_DIR ..."
    python3 -m venv "$VENV_DIR"
fi

source "$VENV_DIR/bin/activate"

echo "Installing requirements ..."
pip install -q -r "$SCRIPT_DIR/requirements.txt"

echo "Running OCC tests ..."
pytest "$SCRIPT_DIR" -v --tb=short "$@"
