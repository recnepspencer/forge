#!/bin/bash
set -o pipefail

FAIL_DIR="test_failures"
rm -rf "$FAIL_DIR"
mkdir -p "$FAIL_DIR"

echo "=== Running all tests (including ignored) ==="
echo ""

# Run all tests including ignored, single-threaded for clean output
OUTPUT=$(cargo test --workspace -- --include-ignored --test-threads=1 2>&1)
EXIT_CODE=$?

echo "$OUTPUT"
echo ""
echo "==============================="
echo "Exit code: $EXIT_CODE"
echo "==============================="

if [ $EXIT_CODE -ne 0 ]; then
    echo ""
    echo "=== Extracting individual failure logs ==="

    # Parse the output to find failed test names from the "failures:" section
    FAILED_TESTS=$(echo "$OUTPUT" | awk '/^failures:$/,/^$/' | grep -E '^\s+\S' | sed 's/^[[:space:]]*//')

    if [ -z "$FAILED_TESTS" ]; then
        # Fallback: try stdout sections
        FAILED_TESTS=$(echo "$OUTPUT" | grep "^---- .* stdout ----" | sed 's/^---- //; s/ stdout ----$//')
    fi

    if [ -z "$FAILED_TESTS" ]; then
        echo "Could not parse individual test names. Saving full output."
        echo "$OUTPUT" > "$FAIL_DIR/full_output.log"
    else
        echo "Failed tests found:"
        echo "$FAILED_TESTS"
        echo ""

        # For each failed test, re-run it individually and capture output
        while IFS= read -r test_name; do
            [ -z "$test_name" ] && continue
            # Create a safe filename
            safe_name=$(echo "$test_name" | tr ':' '_' | tr '/' '_')
            log_file="$FAIL_DIR/${safe_name}.log"

            echo "  Re-running: $test_name -> $log_file"
            FORGE_LOG=full cargo test --workspace -- --include-ignored --exact "$test_name" --test-threads=1 --nocapture > "$log_file" 2>&1
        done <<< "$FAILED_TESTS"

        echo ""
        echo "=== Failure logs written to $FAIL_DIR/ ==="
        ls -la "$FAIL_DIR/"
    fi
else
    echo ""
    echo "All tests passed!"
fi
