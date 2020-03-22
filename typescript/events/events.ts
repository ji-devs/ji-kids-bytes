export type Section = "watch" | "games" | "create" | "look" | "hands-on";

export class SelectSectionEvent extends CustomEvent<{section: Section}> {
    constructor(section: Section) {
        super("select-section", { detail: {section}, composed: true});
    }
}