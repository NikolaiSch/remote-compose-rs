# from https://github.com/nix-community/naersk?tab=readme-ov-file#setup
{
  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    naersk.url = "github:nix-community/naersk";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
  };

  outputs =
    {
      self,
      flake-utils,
      naersk,
      nixpkgs,
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = (import nixpkgs) {
          inherit system;
        };

        naersk' = pkgs.callPackage naersk { };

      in
      rec {
        packages =
          let
            crates = pkgs.lib.attrNames (
              pkgs.lib.filterAttrs (n: v: v == "directory") (builtins.readDir ./crates)
            );
            mkPackage =
              name: mode:
              naersk'.buildPackage {
                src = ./.;
                inherit mode;
                cargoBuildOptions =
                  x:
                  x
                  ++ [
                    "-p"
                    "remote-compose-${name}"
                  ];
              };
          in
          pkgs.lib.genAttrs crates (name: {
            check = mkPackage name "check";
            test = mkPackage name "test";
            clippy = mkPackage name "clippy";
          });

        # For `nix develop` (optional, can be skipped):
        devShell = pkgs.mkShell {
          nativeBuildInputs = with pkgs; [
            rustc
            cargo
            rust-analyzer
          ];
        };
      }
    );
}
