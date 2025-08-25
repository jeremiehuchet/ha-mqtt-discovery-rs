{ inputs,  pkgs, ... }:

{
  packages = [
    pkgs.json2yaml
    pkgs.openssl
    pkgs.pkg-config
    pkgs.yaml2json
  ];

  languages.rust = {
    enable = true;
    channel = "stable";
  };
  languages.javascript = {
    enable = true;
    bun.enable = true;
  };

  git-hooks.hooks.rustfmt.enable = true;

  scripts.update-ha-docs-src.exec = ''
    set -e
    rm -rf "$DEVENV_ROOT/generator/input"
    mkdir -p "$DEVENV_ROOT/generator/input" "$DEVENV_ROOT/generator/input/device_classes"

    for entity_doc in ${inputs.homeassistant-docs}/source/_integrations/*.mqtt.markdown ; do
      entity_name=$(basename "''${entity_doc%.mqtt.markdown}")
      cat "$entity_doc" \
          | sed '/^{% configuration %}$/,/^{% endconfiguration %}$/d' \
          | sed 's/{% term "[\\"]*" %}/\\1/g' \
          | sed 's/^{% caution %}$/🚨 Caution\\/g' \
          | sed 's/^{% important %}$/⚠ Important\\/g' \
          | sed 's/^{% note %}$/🛈 Note\\/g' \
          > "$DEVENV_ROOT/generator/input/''${entity_name}.md"
      for tag in caution important note raw ; do
          sed -i "/^{% $tag %}$/d" "$DEVENV_ROOT/generator/input/''${entity_name}.md"
          sed -i "/^{% end$tag %}$/d" "$DEVENV_ROOT/generator/input/''${entity_name}.md"
      done
      cat "$entity_doc" \
          | sed '1,/^{% configuration %}$/d' \
          | sed '/^{% endconfiguration %}$/,$d' \
          | yaml2json | json2yaml \
          > "$DEVENV_ROOT/generator/input/''${entity_name}.yml"
    done

    cp $(grep -lri '^###* Device Class' '${inputs.homeassistant-docs}/source/_integrations/') "$DEVENV_ROOT/generator/input/device_classes/"

    cat ${inputs.homeassistant-core}/homeassistant/const.py | grep -v '^$' | grep -Pzo '\nclass UnitOf(\w+).*\n(   .*\n)*' > "$DEVENV_ROOT/generator/input/units.py"

    chmod -R +w generator/input/
  '';

  scripts.generate-types.exec = ''
    bun run $DEVENV_ROOT/generator/src/index.ts
    find $DEVENV_ROOT/src -type f -exec rustfmt --edition 2024 {} \+
  '';
}
