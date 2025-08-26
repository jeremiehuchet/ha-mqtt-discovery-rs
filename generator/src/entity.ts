import {readFileSync} from "fs";
import YAML from "yaml";
import {toPascalCase} from "./strings";

const IGNORED_ATTRS = [
  "availability",
  "availability_mode",
  "availability_template",
  "availability_topic",
  "payload_available",
  "payload_not_available",
  "platform",
  "expire_after",
  "device",
  "entity_category",
];

type FieldAttributes = {
  description: string;
  required: boolean;
  type:
    | "template"
    | "string"
    | "icon"
    | "float"
    | "integer"
    | "boolean"
    | "list"
    | "map"
    | ["string", "list"];
  rustType?: string;
  enumValues?: {[name: string]: string}
  import?: string;
  useInto?: boolean;
  iterable?: boolean;
  rustSafeName?: string;
  defaultValue? : string;

  keys?: any;
};

type MqttEntity = {
  entityName: string;
  entityDoc: string;
  imports: Set<string>;
  properties: object;
};

export function generateMqttEntityModel(
  basedir: string,
  entityName: string,
): MqttEntity {
  const docFile = `${basedir}/${entityName}.md`;
  const modelFile = `${basedir}/${entityName}.yml`;
  console.log(entityName, docFile, modelFile);
  const docContent = readFileSync(docFile).toString();
  const modelContent = readFileSync(modelFile).toString();

  try {
    const modelDescriptor = YAML.parse(modelContent);
    const entries = Object.entries(modelDescriptor)
      .filter(([name, _attrs]) => !IGNORED_ATTRS.includes(name));
    for (const [name, attrs] of entries) {
      const attrsFieldAttributes = attrs as FieldAttributes
      appendRustType(name, attrsFieldAttributes);
      if (name === "platform") {
        attrsFieldAttributes.defaultValue = entityName;
      }
    }

    return {
      entityName,
      entityDoc: docContent,
      imports: new Set(entries.map(([_name, attrs]) => attrs.import).filter(importInstruction => !!importInstruction)),
      properties: Object.fromEntries(entries),
    };
  } catch (e) {
    console.error(modelContent);
    throw e;
  }
}

function appendRustType(name: string, attrs: FieldAttributes) {
  if (name === "type") {
    attrs.rustSafeName = `r#${name}`;
  } else {
    attrs.rustSafeName = name;
  }
  switch (attrs.type) {
    case "template":
    case "string":
    case "icon":
      attrs.rustType = "String";
      attrs.useInto = true;
      break;
    case "float":
      attrs.rustType = "Decimal";
      attrs.import = "pub use rust_decimal::Decimal"
      break;
    case "integer":
      attrs.rustType = "i32";
      break;
    case "boolean":
      attrs.rustType = "bool";
      break;
    case "list":
        if (!attrs.keys) {
          attrs.rustType = "String";
          attrs.iterable = true;
          attrs.useInto = true;
        }
        break;
    default:
      if (attrs.type.includes("list")) {
        attrs.rustType = "String";
        attrs.iterable = true;
        attrs.useInto = true;
      }
  }
  switch (name) {
    case "device_class":
      const entityName = new RegExp(
        "/integrations/(?<name>[^/]*)/#device-class"
      ).exec(attrs.description)?.groups.name;
      if (entityName) {
        const deviceClassType = `${toPascalCase(entityName)}DeviceClass`;
        attrs.rustType = deviceClassType;
        attrs.import = `use crate::generated::${deviceClassType}`;
      }
      break;
    case "unit_of_measurement":
      attrs.rustType = "Unit";
      attrs.import = `use crate::generated::Unit`;
      break;
      case "state_class":
        attrs.rustType = "SensorStateClass";
        attrs.import = `use crate::common::SensorStateClass`;
        break;
    case "qos":
      attrs.rustType = "Qos";
      attrs.import = `use crate::common::Qos`;
      break;
    case "temperature_unit":
      attrs.rustType = "TemperatureUnit";
      attrs.import = `use crate::common::TemperatureUnit`;
      break;
    case "state_topic":
      attrs.required = false;
      break;
  }
}
