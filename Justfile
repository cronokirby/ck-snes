# Run basic linting.
check:
    # Check all vhdl files, in isolation.
    ghdl -a --std=08 -Wall -Werror hdl/**/*.vhd
    # Lint python.
    ruff check
    # Check python formatting.
    ruff format --check

# Fix formatting.
fmt:
    # Python formatting.
    ruff format

# Run tests
test:
    just check
    pytest hdl/test_suite.py

# Build a bitstream for a target board.
# Usage: just build ulx3s-12f
build target:
    if [ "{{target}}" != "ulx3s-12f" ]; then \
        echo "Unsupported target: {{target}}"; \
        echo "Supported targets: ulx3s-12f"; \
        exit 1; \
    fi
    mkdir -p target/ulx3s
    yosys -p "ghdl --std=08 hdl/src/Test.vhd platform/ulx3s/Top.vhd -e Top; synth_ecp5 -top Top -json target/ulx3s/top.json"
    nextpnr-ecp5 --json target/ulx3s/top.json --lpf platform/ulx3s/ulx3s_v20_min.lpf --textcfg target/ulx3s/top.config --12k --package CABGA381 --freq 25
    ecppack target/ulx3s/top.config target/ulx3s/Top.bit

# Build + load a bitstream onto a target board.
# Usage: just load ulx3s-12f
load target:
    if [ "{{target}}" != "ulx3s-12f" ]; then \
        echo "Unsupported target: {{target}}"; \
        echo "Supported targets: ulx3s-12f"; \
        exit 1; \
    fi
    just build ulx3s-12f
    openFPGALoader -b ulx3s target/ulx3s/Top.bit
