{ inputs,  pkgs, ... }:

{
  packages = [
    pkgs.openssl
    pkgs.pkg-config
    pkgs.vscode
  ];

  languages.rust = {
    enable = true;
    channel = "stable";
  };
  languages.javascript = {
    enable = true;
    bun.enable = true;
  };

  pre-commit.hooks.rustfmt.enable = true;

  scripts.update-ha-docs-src.exec = ''
    set -e
    rm -rf $DEVENV_ROOT/generator/input
    mkdir -p $DEVENV_ROOT/generator/input $DEVENV_ROOT/generator/input/device_classes
    cp -r ${inputs.homeassistant-docs}/source/_integrations/*.mqtt.markdown $DEVENV_ROOT/generator/input/
    cp $(grep -lri '^###* Device Class' ${inputs.homeassistant-docs}/source/_integrations/) $DEVENV_ROOT/generator/input/device_classes/
    chmod -R +w generator/input/
  '';

  scripts.generate-types.exec = ''
    bun run generator/src/index.ts
    find src -type f -exec rustfmt --edition 2024 {} \+
  '';
}
