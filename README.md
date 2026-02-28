# ck-snes

I'm trying to write an SNES emulator.

## ULX3S USB permissions (Linux)

If `openFPGALoader` fails with a USB permission error, add a udev rule for the
FTDI device, then reload rules and replug the board.

Example rule (ULX3S FTDI 0403:6015):

```
SUBSYSTEM=="usb", ATTR{idVendor}=="0403", ATTR{idProduct}=="6015", MODE="0666"
```

Alternative (group-based):

```
SUBSYSTEM=="usb", ATTR{idVendor}=="0403", ATTR{idProduct}=="6015", GROUP="plugdev", MODE="0660"
```

Save as `/etc/udev/rules.d/99-ulx3s.rules`, then run:

```
sudo udevadm control --reload-rules
```
