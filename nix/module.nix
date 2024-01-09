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

    settings = {
      addr = mkOption {
        type = types.str;
        default = "0.0.0.0";
        example = "127.0.0.1";
        description = "IP Address used for Schizoserach";
      };
      port = mkOption {
        type = types.int;
        default = 3000;
        example = 2137;
        description = "Port used for Schizosearch";
      };
    };
  };
  config = mkIf cfg.enable {
    networking.firewall.allowedTCPPorts = [cfg.settings.port];
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
        ExecStart = ''${cfg.package}/bin/schizosearch --listener ${cfg.settings.addr}:${builtins.toString cfg.settings.port}'';
        Restart = "always";
      };
    };
  };
}
