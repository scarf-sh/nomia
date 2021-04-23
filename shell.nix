let
  sources = import ./nix/sources.nix { };

  pkgs = import sources.nixpkgs { };

  texlive = pkgs.texlive.combine { inherit (pkgs.texlive) scheme-small ucs; };

  # Don't use the upstream nix code to speed-up evaluation. Trade updating
  # cargoSha256 to avoid evaluating nixpkgs once again.
  treefmt = pkgs.rustPlatform.buildRustPackage {
    pname = "treefmt";
    version = "unstable";
    src = sources.treefmt;
    cargoSha256 = "0cpkw2jny3m654x6jg04ajfyhsf2mprxy5fy9s1bb0wid6y741b7";
  };
in
pkgs.mkShell {
  name = "nomia-dev-env";

  nativeBuildInputs = [ pkgs.haskellPackages.BNFC texlive pkgs.niv pkgs.cargo pkgs.rustc pkgs.flex pkgs.bison treefmt pkgs.nixpkgs-fmt ];
}
