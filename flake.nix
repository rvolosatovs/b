{
  description = "random";

  inputs.crane.url = github:ipetkov/crane;
  inputs.flake-utils.url = github:numtide/flake-utils;

  outputs = {
    self,
    crane,
    flake-utils,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (
      system: let
        craneLib = crane.lib.${system};

        commonArgs = {
          src = craneLib.cleanCargoSource ./.;
        };

        cargoFmt = craneLib.cargoFmt commonArgs;
        pkg = craneLib.buildPackage commonArgs;
      in {
        checks.fmt = cargoFmt;

        packages.default = pkg;
      }
    );
}
