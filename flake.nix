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
        # rustPlatform = pkgs.makeRustPlatform {
        #   inherit (pkgs.rust-bin.stable.latest) cargo rustc rust-std;
        # };
        hornysteinPkg =
          pkgs.rustPlatform.buildRustPackage
          {
            pname = "hornystein-bin";
            version = "0.1";

            src = ./Hornystein/.;

            # nativeBuildInputs = with pkgs; [pkg-config];
            nativeBuildInputs = [
              pkgs.pkg-config
            ];

            # cargoHash = "";
            # postPatch = ''
            #   ln -s ${./Cargo.lock} Cargo.lock
            # '';

            # cargoLock.lockFile = ./Hornystein/Cargo.lock;
            cargoLock = {
              lockFile = ./Hornystein/Cargo.lock;
              allowBuiltinFetchGit = true;
            };

            meta = {
              description = "A Wolfstein look a like with lolis and more!";
              homepage = "https://github.com/ElrohirGT/Hornystein";
              license = pkgs.lib.licenses.mit;
              maintainers = [];
              platforms = pkgs.lib.platforms.unix;
            };
          };
        # mazeFile = builtins.path {
        #   path = ./Hornystein/maze;
        #   name = "mazeFile";
        # };
        # assets = builtins.path {
        #   path = ./Hornystein/night_assets/.;
        #   name = "horny-stein_assets";
        # };

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
