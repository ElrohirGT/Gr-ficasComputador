{
  description = "Flake para el curso de gráficas por computador";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = {
    self,
    nixpkgs,
    rust-overlay,
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
        hornysteinPkg = pkgs.rustPlatform.buildRustPackage {
          pname = "hornystein-bin";
          version = "0.1";

          src = ./Hornystein/.;

          cargoHash = "";
          # postPatch = ''
          #   ln -s ${./Cargo.lock} Cargo.lock
          # '';

          # cargoLock.lockFile = ./Hornystein/Cargo.lock;
          # cargoLock = {
          #   lockFile = ./Hornystein/Cargo.lock;
          #   allowBuiltinFetchGit = true;
          # };

          meta = {
            description = "A Wolfstein look a like with lolis and more!";
            homepage = "https://github.com/ElrohirGT/Hornystein";
            license = pkgs.lib.licenses.mit;
            maintainers = [];
          };
        };
        mazeFile = builtins.path {
          path = ./Hornystein/maze;
          name = "mazeFile";
        };
        # assets = builtins.path {
        #   path = ./Hornystein/night_assets/.;
        #   name = "horny-stein_assets";
        # };

        assets = pkgs.stdenv.mkDerivation {
          # name of our derivation
          name = "assets_dir";

          # sources that will be used for our derivation.
          src = ./Hornystein/night_assets/.;

          # see https://nixos.org/nixpkgs/manual/#ssec-install-phase
          # $src is defined as the location of our `src` attribute above
          installPhase = ''
            # $out is an automatically generated filepath by nix,
            # but it's up to you to make it what you need. We'll create a directory at
            # that filepath, then copy our sources into it.
            mkdir $out
            cp -rv $src/* $out
          '';
        };
      in {
        hornystein = pkgs.writeShellApplication {
          name = "hornystein";
          runtimeInputs = [hornysteinPkg];
          text = ''
            # hornystein-bin ${mazeFile} ${assets}
            hornystein-bin ${assets}
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
