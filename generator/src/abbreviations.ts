import {readFileSync} from "fs";

type Abbreviation = {
    shortName: string;
    fullName: string;
};

export function loadAbbreviations(abbreviationsFile: string): Abbreviation[] {
    console.log("Abbreviations", abbreviationsFile);

    const abbreviationsSections = readFileSync(abbreviationsFile)
        .toString()
        .replaceAll(/^""".*/g, '')
        // remove trailing comma
        .replaceAll(/,$\n}/gm, '\n}')
        // split sections
        .split(/^\w*\s*=/gm)
        .filter((section) => section.trim().length > 0);

    return abbreviationsSections
        .map((section) => JSON.parse(section))
        .flatMap((section) => Object.entries(section))
        .map(([shortName, fullName]) => ({shortName, fullName}) as Abbreviation);
}