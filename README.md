# ck-snes

I'm trying to write an SNES emulator.

# Development

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

## Suggested Editor Config

### Helix

You can create a local `.helix` directory. A suggested `languages.toml` to put
there would be:

```toml
[language-server.ruff]
command = "ruff"
args = ["server"]

[language-server.vhdl-ls]
command = "vhdl_ls"

[[language]]
name = "python"
language-servers = ["ruff"]
auto-format = true
formatter = { command = "ruff", args = ["format", "--stdin-filename", "tmp.py", "-"] }

[[language]]
name = "vhdl"
file-types = ["vhd", "vhdl"]
language-servers = ["vhdl-ls"]
auto-format = true
formatter = { command = "sh", args = ["-c", "tmp=$(mktemp); cat > \"$tmp\"; ghdl fmt --std=08 \"$tmp\"; rc=$?; rm -f \"$tmp\"; exit $rc"] }
```

The VHDL formatting requires a bit of a hack, because `ghdl` doesn't support stdin input.
