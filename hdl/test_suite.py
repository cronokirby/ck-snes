from pathlib import Path
from cocotb_tools.runner import get_runner


HDL_SRC = Path(__file__).resolve().parent / "src"


def test():
    runner = get_runner("ghdl")
    # We avoid passing `always=True`, to allow for caching build artifacts.
    runner.build(
        sources=[str(HDL_SRC / "Test.vhd")],
        hdl_toplevel="test",
    )
    runner.test(
        hdl_toplevel="test",
        test_module="test.Test",
        parameters={
            "BITS": 3
        }
    )
