#!/usr/bin/env bash

REBUILD_TESTS="0"
if [ $# -gt 0 ]; then
    case "$1" in 
        "--rebuild-tests")
            REBUILD_TESTS="1";;
        *)
            echo "Usage: $0 [--rebuild-tests]"
            exit 1;;
    esac
fi

# Tests if make failed or if it included "X errors remaining"
if [ "$REBUILD_TESTS" -eq 1 ]; then
    if (cd ./programs/ && make) then
        echo "Succeeded building tests"
    else
        echo "Error building tests"
        exit 1
    fi
fi

if (cd ./rsim/ && cargo build) then
    echo "Succeeded building emulator"
else
    echo "Error building emulator"
    exit 1
fi

if (cd ./rsim/ && cargo test --doc) then
    echo "Succeeded doc-testing emulator"
else
    echo "Error doc-testing emulator"
    exit 1
fi

# Now that the emulator and tests are ready, run the tests
mkdir -p ./test_results/
python3.8 ./test.py