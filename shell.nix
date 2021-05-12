let
  sources = import ./nix/sources.nix { };

  pkgs = import sources.nixpkgs {
    overlays = [ (import sources.rust-overlay) ];
  };

  texlive = pkgs.texlive.combine { inherit (pkgs.texlive) scheme-small ucs; };

  # Don't use the upstream nix code to speed-up evaluation. Trade updating
  # cargoSha256 to avoid evaluating nixpkgs once again.
  treefmt = pkgs.rustPlatform.buildRustPackage {
    pname = "treefmt";
    version = "unstable";
    src = sources.treefmt;
    cargoSha256 = "0cpkw2jny3m654x6jg04ajfyhsf2mprxy5fy9s1bb0wid6y741b7";
  };

  BNFC = pkgs.haskell.lib.overrideCabal pkgs.haskellPackages.BNFC (orig: {
    patches = (orig.patches or [ ]) ++ [
      ./nix/bnfc-nil-define.patch
    ];
  });

  rust = pkgs.rust-bin.selectLatestNightlyWith (toolchain: toolchain.default);
in
pkgs.mkShell {
  name = "nomia-dev-env";

  nativeBuildInputs = [ BNFC texlive pkgs.niv pkgs.cargo rust pkgs.flex pkgs.bison treefmt pkgs.nixpkgs-fmt ];

  LIBCLANG_PATH = "${pkgs.llvmPackages.libclang}/lib";
}
