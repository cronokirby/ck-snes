# Run basic linting.
check:
    # Check all vhdl files, in isolation.
    ghdl -a --std=08 -Wall -Werror hdl/**/*.vhd

# Run tests
test:
    pytest hdl/test_suite.py
