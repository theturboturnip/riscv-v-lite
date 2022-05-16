#!/usr/bin/env bash

# Tests if make failed or if it included "X errors remaining"
if (cd ./programs/ && make) then
    echo "Succeeded building tests"
else
    echo "Error building tests"
    exit 1
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
python3.8 ./test.py