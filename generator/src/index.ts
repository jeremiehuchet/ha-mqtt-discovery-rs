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

const entitiesModels = entities.map(entityName => generateMqttEntityModel(`${BASEDIR}/generator/input/`, entityName));

const imports = entitiesModels.flatMap((entityModel) => Array.from(entityModel.imports));
const templateImports = readFileSync(
    `${BASEDIR}/generator/src/rust_model_imports.mustache`
).toString();
const importsOutput = Handlebars.compile(templateImports)(new Set(imports));

const templateContent = readFileSync(
    `${BASEDIR}/generator/src/rust_model.mustache`
).toString();
const compiledTemplateContent = Handlebars.compile(templateContent);
const entitiesOutput = entitiesModels
    .map(entity => compiledTemplateContent(entity))
    .join("\n");

writeFileSync(`${BASEDIR}/src/generated/entities.rs`, `
${importsOutput}

${entitiesOutput}
`);

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

// generate mod.rs
const templateMod = readFileSync(
  `${BASEDIR}/generator/src/rust_mod.mustache`
).toString();
const unsupportedEntities = ['tag', 'device_trigger'];
const outputMod = Handlebars.compile(templateMod)({
  unsupportedEntities,
  supportedEntities: entities.filter(entity => !unsupportedEntities.includes(entity)),
});

// with units
const unitsTemplate = readFileSync(
    `${BASEDIR}/generator/src/rust_units.mustache`
).toString();
const unitsModel = extractUnitsEnums(`${BASEDIR}/generator/input/units.py`);
const unitsOutput = Handlebars.compile(unitsTemplate)(unitsModel);

// and device classes
const deviceClassesTemplate = readFileSync(
    `${BASEDIR}/generator/src/rust_device_classes.mustache`
).toString();
const output = Handlebars.compile(deviceClassesTemplate)(enumsModels);

writeFileSync(`${BASEDIR}/src/generated/mod.rs`, `
${outputMod}

${unitsOutput}

${output}`);