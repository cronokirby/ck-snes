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
    # VHDL formatting.
    find hdl -name '*.vhd' -print0 | while IFS= read -r -d '' f; do \
        ghdl fmt --std=08 "$f" > "$f".fmt && mv "$f".fmt "$f"; \
    done
    # Python formatting.
    ruff format

# Run tests
test:
    just check
    pytest hdl/test_suite.py

# Build a bitstream for a target board.
# Usage: just build ulx3s-12f | ulx3s-85f
build target:
    case "{{target}}" in \
        ulx3s-12f) chip="--12k" ;; \
        ulx3s-85f) chip="--85k" ;; \
        *) \
            echo "Unsupported target: {{target}}"; \
            echo "Supported targets: ulx3s-12f ulx3s-85f"; \
            exit 1; \
            ;; \
    esac; \
    outdir="target/{{target}}"; \
    mkdir -p "$outdir"; \
    yosys -p "ghdl --std=08 hdl/src/Test.vhd hdl/src/Uart.vhd platform/ulx3s/Top.vhd -e Top; synth_ecp5 -top Top -json $outdir/top.json"; \
    nextpnr-ecp5 --json $outdir/top.json --lpf platform/ulx3s/ulx3s_v20_min.lpf --textcfg $outdir/top.config "$chip" --package CABGA381 --freq 25; \
    ecppack $outdir/top.config $outdir/Top.bit

# Build + load a bitstream onto a target board.
# Usage: just load ulx3s-12f | ulx3s-85f
load target:
    case "{{target}}" in \
        ulx3s-12f|ulx3s-85f) ;; \
        *) \
            echo "Unsupported target: {{target}}"; \
            echo "Supported targets: ulx3s-12f ulx3s-85f"; \
            exit 1; \
            ;; \
    esac; \
    just build {{target}}
    openFPGALoader -b ulx3s target/{{target}}/Top.bit
