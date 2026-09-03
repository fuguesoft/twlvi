{
  rustPlatform,
  lib,
}:
rustPlatform.buildRustPackage (final: {
  pname = "twlvi";
  version = "0.0.0";

  src = ./.;

  cargoHash = "sha256-/H2HVPFd4s3q9icVFKU78Y/4V6Y7BjEihWFcC63PC04=";

  meta = {
    description = "Print the lyrics to The Twelve Days of Christmas in sequence";
    homepage = "https://github.com/fuguesoft/twlvi";
    license = lib.licenses.gpl3;
    maintainers = with lib.maintainers; [ fuguesoft ];
  };
})
