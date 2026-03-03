ULX3S LED blink bring-up

Files
- Top.vhd: top-level wiring for ULX3S (clk_25mhz, led[7:0])
- ulx3s_v20_min.lpf: minimal constraints for ULX3S v2.x.x / v3.0.x

What this does
- Instantiates hdl/src/Test.vhd
- Drives led[0] with a ~1.5 Hz blink (25 MHz / 2^25)

How to use
- Include these sources in your ECP5 build:
  - hdl/src/Test.vhd
  - platform/ulx3s/Top.vhd
  - platform/ulx3s/ulx3s_v20_min.lpf
- Set top entity to Top
- Map clk_25mhz to the ULX3S 25 MHz clock and led[7:0] to LEDs
- Build a bitstream:
  - just build ulx3s-12f
  - just build ulx3s-85f
- Program the resulting bitstream with openFPGALoader:
  - just load ulx3s-12f
  - just load ulx3s-85f

Notes
- If your board revision is newer than v3.0.x, use the matching constraints file.
- This repo does not auto-detect your board. Provide the exact bitstream path.
- The build recipe supports 12F (LFE5U-12F) and 85F (LFE5U-85F) devices in the CABGA381 package.
- Bitstreams are emitted to target/ulx3s-12f/Top.bit or target/ulx3s-85f/Top.bit.
