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
      in
      {
        devShells.default = pkgs.mkShell {
          packages = with pkgs; [
            # Command runner.
            just
            # Tooling for VHDL.
            ghdl
            # HDL synthesis and simulation with yosys.
            #
            # We also wrap it to include the necessary plugins for VHDL.
            (pkgs.writeShellScriptBin "yosys" ''
              exec ${pkgs.yosys}/bin/yosys \
                -m ${pkgs.yosys-ghdl}/share/yosys/plugins/ghdl.so \
                "$@"
            '')
          ]; 
        };
      }
    );
}
