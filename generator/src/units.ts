import {readFileSync} from "fs";

type UnitGroup = {
    name: string;
    units: UnitModel[];
}

type UnitModel = {
    unitName: string;
    unitAbbreviation: string;
}

export function extractUnitsEnums(unitsFile: string): UnitGroup[] {
    console.log("Units", unitsFile);
    const unitsContent = readFileSync(unitsFile).toString();
    return unitsContent
        .split('\u0000')
        .map((unitContent) =>({
            name: (unitContent.match(/.*class UnitOf(\w+).*/) ?? [])[1],
            units: unitContent
                .matchAll(/    (?<name>\S+) = "(?<unit>[^"]+)"/g)
                .map((unit) => ({
                    unitName: fixUnitName(unit.groups.name),
                    unitAbbreviation: unit.groups.unit,
                }))
                .toArray(),
        }))
        .filter((unit) => !!unit.name);
}

function fixUnitName(name: string) {
    switch (name) {
        case 'MILLIWATT':
            return 'MILLI_WATT';
        default:
            return name;
    }
}