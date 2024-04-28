inputs: {
  config,
  lib,
  pkgs,
  ...
}: let
  inherit (pkgs.stdenv.hostPlatform) system;
  cfg = config.services.schizosearch;

  package = inputs.self.packages.${system}.default;
  inherit (lib) mkOption mkEnableOption types mkIf;
in {
  options.services.schizosearch = {
    enable = mkEnableOption "Schizoserach meta search engine";
    package = mkOption {
      type = types.package;
      default = package;
      example = package;
      description = "Schizosearch package";
    };
    openFirewall = mkEnableOption "Open ports required for schizosearch";

  };
  config = mkIf cfg.enable {
    systemd.services.schizosearch= {
      description = "Privacy friendly meta search engine";
      wantedBy = ["multi-user.target"];
      wants = ["network.target"];
      after = [
        "network-online.target"
        "NetworkManager.service"
        "systemd-resolved.service"
      ];
      serviceConfig = {
        ExecStart = ''${cfg.package}/bin/schizosearch'';
        Restart = "always";
      };
    };
  };
}
