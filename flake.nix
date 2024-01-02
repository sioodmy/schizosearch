{
  description = "Privacy respecting search engine written in Rust";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-parts.url = "github:hercules-ci/flake-parts";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    crane = {
      url = "github:ipetkov/crane";
      inputs = {
        nixpkgs.follows = "nixpkgs";
      };
    };
    home-manager = {
      url = "github:nix-community/home-manager";
      inputs.nixpkgs.follows = "nixpkgs";
    };

  };

  outputs = inputs @ {flake-parts, self, ...}:
    flake-parts.lib.mkFlake {inherit inputs;} {
      systems = ["x86_64-linux" "aarch64-linux" "aarch64-darwin" "x86_64-darwin"];
      perSystem = {
        config,
        self',
        inputs',
        pkgs,
        system,
        ...
      }: let
        libs = with pkgs; [
          openssl
          pkg-config
        ];
      in {
        formatter = pkgs.alejandra;

        packages.default = let
          craneLib =
            inputs.crane.lib.${system}.overrideToolchain
            inputs.fenix.packages.${system}.minimal.toolchain;
        in
          craneLib.buildPackage {
            src = ./.;
            buildInputs = libs;
          };


        
        devShells.default = pkgs.mkShell {
          RUST_LOG = "info";
          buildInputs = with pkgs;
            [
              inputs.fenix.packages.${system}.complete.toolchain
              clippy
              rustc
            ]
            ++ libs;

          LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath libs;
        };
      };
    };
}
