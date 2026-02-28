import cocotb
from cocotb.clock import Clock
from cocotb.triggers import ClockCycles, ReadOnly, RisingEdge


@cocotb.test()
async def test_led_blinks(d):
    cocotb.start_soon(Clock(d.clk, 10, unit="ns").start(start_high=False))
    # Wait for a rising edge so initialization settles before sampling.
    await RisingEdge(d.clk)
    await ReadOnly()
    # The LED should start off.
    assert not d.led.value
    # Wait 3 more rising edges so counter reaches 4 (MSB goes high with BITS=3).
    await ClockCycles(d.clk, 3)
    await ReadOnly()
    # The LED should now be on.
    assert d.led.value
    await ClockCycles(d.clk, 4)
    await ReadOnly()
    # The LED should now be off again.
    assert not d.led.value
