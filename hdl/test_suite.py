from pathlib import Path
from cocotb_tools.runner import get_runner


HDL_SRC = Path(__file__).resolve().parent / "src"


def test():
    runner = get_runner("ghdl")
    runner.build(
        sources=[str(HDL_SRC / "Test.vhd")],
        hdl_toplevel="test",
        always=True
    )
    runner.test(
        hdl_toplevel="test",
        test_module="test.Test",
        parameters={
            "BITS": 3
        }
    )
