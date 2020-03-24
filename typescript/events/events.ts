export type Section = "watch" | "games" | "discover" | "create" | "craft" | "help";

export class SelectSectionEvent extends CustomEvent<{section: Section}> {
    constructor(section: Section) {
        super("select-section", { detail: {section}, composed: true});
    }
}