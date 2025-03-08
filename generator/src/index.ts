import { readFileSync, writeFileSync, readdirSync } from "fs";
import Handlebars from "handlebars";
import { allAbbreviations } from "./abbretiations";
import { extractDeviceClassesEnums } from "./device-class";
import { generateMqttEntityModel } from "./entity";
import { toPascalCase } from "./strings";

const BASEDIR = process.env.DEVENV_ROOT;

Handlebars.registerHelper("abbreviation", (name: string) => {
  const abbreviation = Object.entries(allAbbreviations).find(
    ([_shortName, fullName]) => name === fullName
  );
  return new Handlebars.SafeString(abbreviation ? abbreviation[0] : name);
});

Handlebars.registerHelper("comment", (text: string) => {
  return text?.replaceAll("\n", "\n/// ");
});

Handlebars.registerHelper("toPascalCase", toPascalCase);

// generate entities types
const entities = readdirSync(`${BASEDIR}/generator/input/`)
    .filter(fileName => fileName.endsWith('.md'))
    .map(fileName => fileName.replace(/\.md/, ''));
for (const entityName of entities) {
  const model = generateMqttEntityModel(`${BASEDIR}/generator/input/`, entityName);
  const template = readFileSync(
    `${BASEDIR}/generator/src/rust_model.mustache`
  ).toString();
  const output = Handlebars.compile(template)(model);
  writeFileSync(`${BASEDIR}/src/mqtt/${entityName}.rs`, output);
}

// generate device class types
const enumsModels = readdirSync(
  `${BASEDIR}/generator/input/device_classes`
).map((deviceDocFile) => {
  const name = deviceDocFile.replace(".markdown", "");
  return extractDeviceClassesEnums(
    name,
    `${BASEDIR}/generator/input/device_classes/${deviceDocFile}`
  );
});
const template = readFileSync(
  `${BASEDIR}/generator/src/rust_device_classes.mustache`
).toString();
const output = Handlebars.compile(template)(enumsModels);
writeFileSync(`${BASEDIR}/src/mqtt/device_classes.rs`, output);

// generate mod.rs
const templateMod = readFileSync(
  `${BASEDIR}/generator/src/rust_mod.mustache`
).toString();
const outputMod = Handlebars.compile(templateMod)(entities);
writeFileSync(`${BASEDIR}/src/mqtt/mod.rs`, outputMod);