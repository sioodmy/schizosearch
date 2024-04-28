{buildGoModule}:
buildGoModule {
  pname = "schizosearch";
  version = "0.0.1";

  src = ./.;

  vendorHash = "sha256-59BE6A2REG9EPeTM5sTkrySDW/otM0Bt6xt6G3U72R8=";

  ldflags = ["-s" "-w"];
}
