{
  description = "CK's attempt at a SNES emulator";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-25.11";    
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };

        python = pkgs.python3.withPackages (pkgs': with pkgs'; [
          # Python testing framework for HDL simulation.
          cocotb
          # Needed as well.
          pytest
          # Not entirely sure if this is needed, but Nix being weird might require this?
          find-libpython
          # Serial support for UART helper script.
          pyserial
        ]);
      in
      {
        devShells.default = pkgs.mkShell {
          packages = with pkgs; [
            # Command runner.
            just
            # Tooling for VHDL.
            ghdl
            # VHDL language server (rust_hdl).
            vhdl-ls
            # HDL synthesis and simulation with yosys.
            #
            # We also wrap it to include the necessary plugins for VHDL.
            (pkgs.writeShellScriptBin "yosys" ''
              exec ${pkgs.yosys}/bin/yosys \
                -m ${pkgs.yosys-ghdl}/share/yosys/plugins/ghdl.so \
                "$@"
            '')
            # ECP5 place-and-route + bitstream tools.
            nextpnr
            (pkgs.ecppack or pkgs.trellis)
            # FPGA programmer.
            openfpgaloader
            # Python tooling, for testing HDL.
            python
            # Python linter/formatter.
            ruff
            # Rust tooling.
            cargo
            rustc
            rust-analyzer
            rustfmt
            clippy
            # Needed for the `serialport` library
            pkg-config
            # See above: provides libudev
            systemd
          ]; 
        };
      }
    );
}
