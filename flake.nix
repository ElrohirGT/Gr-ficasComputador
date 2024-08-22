{
  description = "Flake para el curso de gráficas por computador";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = {
    self,
    nixpkgs,
    rust-overlay,
    crane,
    ...
  }: let
    forAllSystems = {
      pkgs ? nixpkgs,
      function,
    }:
      nixpkgs.lib.genAttrs [
        "x86_64-linux"
        "x86_64-macos"
        "aarch64-linux"
        "aarch64-darwin"
      ]
      (system:
        function {
          pkgs = import pkgs {
            inherit system;
            config.allowUnfree = true;
            overlays = [
              rust-overlay.overlays.default
            ];
          };
          inherit system;
        });
  in {
    packages = forAllSystems {
      function = {pkgs, ...}: let
        craneLib = crane.mkLib pkgs;
        src = craneLib.cleanCargoSource ./.;
        commonArgs = {
          inherit src;
          # strictDeps = true;
        };
        cargoArtifacts = craneLib.buildDepsOnly commonArgs;
        hornysteinPkg = craneLib.buildPackage (commonArgs
          // {
            inherit cargoArtifacts;
            cargoLock = ./Hornystein/Cargo.lock;
            cargoToml = ./Hornystein/Cargo.toml;
            postUnpack = ''
              cd $sourceRoot/Hornystein
              sourceRoot="."
            '';
          });

        mazeFile = ./Hornystein/maze;
        assets = ./Hornystein/night_assets/.;
      in {
        # Use: nix run '.?submodules=1#hornystein'
        hornystein = pkgs.writeShellApplication {
          name = "hornystein";
          runtimeInputs = [hornysteinPkg pkgs.pkg-config];
          text = ''
            hornystein-bin ${mazeFile} ${assets}
          '';
        };
      };
    };

    devShells = forAllSystems {
      function = {
        system,
        pkgs,
      }: {
        default = pkgs.mkShell {
          packages = with pkgs; [
            rust-bin.stable.latest.default
            openssl
            pkg-config
            # For Hornystein in Linux
            xdotool
            xorg.libXcursor
          ];

          shellHook = ''
            alias gs="git status"
            alias e="exit"
          '';
        };
      };
    };
  };
}
