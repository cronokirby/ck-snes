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
            # HDL synthesis and simulation tooling.
            yosys
            # Tooling for VHDL.
            ghdl
            # Plugin to integrate ghdl into yosys.
            yosys-ghdl
          ]; 
        };
      }
    );
}
