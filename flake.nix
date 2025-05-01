{
  description = "Onigiri";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
        
        onigiri = pkgs.rustPlatform.buildRustPackage {
          pname = "onigiri";
          version = "0.1.0";
          src = ./.;
          
          cargoLock = {
            lockFile = ./Cargo.lock;
            # Allow Nix to use the Cargo.lock as-is
            allowBuiltinFetchGit = true;
          };
          
          nativeBuildInputs = with pkgs; [ pkg-config ];
          buildInputs = with pkgs; [ 
            xorg.libX11 
            xorg.libXrandr
          ];
          
          # Enable the nix_config feature
          cargoBuildFlags = "--features nix_config";
          
          # Don't check for now to speed up development
          doCheck = false;
        };
      in {
        packages.default = onigiri;

        devShell = pkgs.mkShell {
          packages = with pkgs; [
            rustc
            cargo
            pkg-config
            xorg.libX11
            xorg.libXrandr
          ];
        };
      }
    );
}
