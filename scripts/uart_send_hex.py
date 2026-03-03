#!/usr/bin/env python3
import argparse
import re
import sys

import serial
from serial.tools import list_ports


HEX_RE = re.compile(r"^[0-9a-fA-F]{1,2}$")


def parse_hex_byte(value: str) -> int:
    value = value.strip().lower().removeprefix("0x")
    if not HEX_RE.match(value):
        raise ValueError(f"Invalid hex byte: {value!r}")
    return int(value, 16)


def choose_port(port: str | None) -> str:
    if port:
        return port
    ports = list(list_ports.comports())
    if not ports:
        raise RuntimeError("No serial ports found.")
    print("Available serial ports:")
    for idx, p in enumerate(ports, start=1):
        desc = f"{p.device} - {p.description}"
        if p.hwid:
            desc += f" ({p.hwid})"
        print(f"  {idx}. {desc}")
    while True:
        choice = input("Select port number: ").strip()
        if not choice.isdigit():
            print("Enter a number.")
            continue
        idx = int(choice)
        if 1 <= idx <= len(ports):
            return ports[idx - 1].device
        print("Out of range.")


def main() -> int:
    parser = argparse.ArgumentParser(
        description="Interactively send hex bytes over serial."
    )
    parser.add_argument(
        "-p",
        "--port",
        help="Serial port (e.g. /dev/ttyUSB0 or COM3). If omitted, you'll be prompted.",
    )
    parser.add_argument(
        "-b",
        "--baud",
        type=int,
        default=115200,
        help="Baud rate (default: 115200).",
    )
    args = parser.parse_args()

    try:
        port = choose_port(args.port)
    except RuntimeError as exc:
        print(exc, file=sys.stderr)
        return 2

    with serial.Serial(port, args.baud, timeout=1) as ser:
        print("Enter hex bytes to send (e.g. AF, 0xBE, 7). Press Enter on empty line or Ctrl-D to exit.")
        while True:
            try:
                line = input("> ")
            except EOFError:
                print()
                break
            if not line.strip():
                break
            try:
                value = parse_hex_byte(line)
            except ValueError as exc:
                print(exc, file=sys.stderr)
                continue
            ser.write(bytes([value]))
            ser.flush()
            print(f"Sent 0x{value:02X}")

    print(f"Done. Port: {port} @ {args.baud} baud.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
