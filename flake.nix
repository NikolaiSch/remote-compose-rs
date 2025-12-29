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
      {
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

            mkExample =
              name: example:
              naersk'.buildPackage {
                src = ./.;
                cargoBuildOptions =
                  x:
                  x
                  ++ [
                    "-p"
                    "remote-compose-${name}"
                    "--example"
                    example
                  ];
              };

            genCrateTargets =
              name:
              let
                examplesDir = ./crates + "/${name}/examples";
                examples =
                  if builtins.pathExists examplesDir then
                    pkgs.lib.attrNames (
                      pkgs.lib.filterAttrs (n: v: v == "regular" && pkgs.lib.hasSuffix ".rs" n) (
                        builtins.readDir examplesDir
                      )
                    )
                  else
                    [ ];
                exampleNames = map (n: pkgs.lib.removeSuffix ".rs" n) examples;
              in
              pkgs.lib.listToAttrs (
                [
                  {
                    name = "${name}-check";
                    value = mkPackage name "check";
                  }
                  {
                    name = "${name}-test";
                    value = mkPackage name "test";
                  }
                  {
                    name = "${name}-clippy";
                    value = mkPackage name "clippy";
                  }
                ]
                ++ (map (example: {
                  name = "${name}-example-${example}";
                  value = mkExample name example;
                }) exampleNames)
              );
          in
          pkgs.lib.foldl' (acc: name: acc // (genCrateTargets name)) { } crates;

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
