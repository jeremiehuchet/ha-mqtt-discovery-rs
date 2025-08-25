import { readFileSync, writeFileSync, readdirSync } from "fs";
import Handlebars from "handlebars";
import { loadAbbreviations } from "./abbreviations.ts";
import { extractDeviceClassesEnums } from "./device-class";
import { generateMqttEntityModel } from "./entity";
import { extractUnitsEnums } from "./units";
import { toPascalCase } from "./strings";

const BASEDIR = process.env.DEVENV_ROOT;

const ABBREVIATIONS = loadAbbreviations(`${BASEDIR}/generator/input/abbreviations.py`);

Handlebars.registerHelper("abbreviation", (name: string) => {
  const abbreviation = ABBREVIATIONS.find((item) => item.fullName === name);
  return new Handlebars.SafeString(abbreviation ? abbreviation.shortName : name);
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
const deviceClassesTemplate = readFileSync(
  `${BASEDIR}/generator/src/rust_device_classes.mustache`
).toString();
const output = Handlebars.compile(deviceClassesTemplate)(enumsModels);
writeFileSync(`${BASEDIR}/src/mqtt/device_classes.rs`, output);

// generate units
const unitsTemplate = readFileSync(
    `${BASEDIR}/generator/src/rust_units.mustache`
).toString();
const unitsModel = extractUnitsEnums(`${BASEDIR}/generator/input/units.py`);
const unitsOutput = Handlebars.compile(unitsTemplate)(unitsModel);
writeFileSync(`${BASEDIR}/src/mqtt/units.rs`, unitsOutput);

// generate mod.rs
const templateMod = readFileSync(
  `${BASEDIR}/generator/src/rust_mod.mustache`
).toString();
const outputMod = Handlebars.compile(templateMod)(entities);
writeFileSync(`${BASEDIR}/src/mqtt/mod.rs`, outputMod);