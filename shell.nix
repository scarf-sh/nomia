let
  sources = import ./nix/sources.nix { };

  pkgs = import sources.nixpkgs {};

  texlive = pkgs.texlive.combine { inherit (pkgs.texlive) scheme-small ucs; };
in pkgs.mkShell {
  name = "nomia-dev-env";

  nativeBuildInputs = [ pkgs.haskellPackages.BNFC texlive pkgs.niv pkgs.cargo pkgs.rustc pkgs.flex pkgs.bison ];
}
