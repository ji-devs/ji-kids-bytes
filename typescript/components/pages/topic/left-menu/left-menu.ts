import {LitElement, customElement, property, css} from "lit-element";
import {nothing, html, svg} from "lit-html";
import {styleMap} from 'lit-html/directives/style-map';
import {classMap} from 'lit-html/directives/class-map';
import {repeat} from 'lit-html/directives/repeat';
import left_menu_css from "./left-menu.css";
import common_css from "@components/common/common.css";
import {SelectSectionEvent, Section} from "@events/events";
import {Path} from "@settings/settings";

type SectionChangeHandler = (section:Section) => any;

@customElement("left-menu")
export class LeftMenu extends LitElement {
    static styles = [common_css, left_menu_css];

    @property( { type : String }  ) section = "watch" as Section;

    @property() on_section_change:SectionChangeHandler;

    render() {

        return html`
            <div class="menu" >
                <a href="/"><img class="logo" src=${Path.ui("menu-logo.svg")} /></a>
                ${list(this.section, this.on_section_change)}
            </div>
        `;
    }
}

const list = (current_section:Section, on_section_change:SectionChangeHandler) => {

    return html`
        <ul>
            ${item("Watch", "watch", on_section_change, current_section)}
            ${item("Games", "games", on_section_change, current_section)}
            ${item("Discover", "discover", on_section_change, current_section)}
            ${item("Create", "create", on_section_change, current_section)}
            ${item("Craft", "craft", on_section_change, current_section)}
        </ul>
    `;
}

function item(label:string, section:Section, on_section_change: SectionChangeHandler, current_section:Section) {
    const selected = section === current_section 
        ? { [`${section}-selected`]: true }
        : {};

    return html`
        <li @click=${() => on_section_change(section)}>
            <div class="item">
                <div class=${classMap({rect: true, ...selected})}>
                    <div class="text">${label}</div>
                </div>
                <div class=${classMap({circle: true, ...selected})}>
                    <img class="icon" src=${Path.ui(`menu-icon-${section}.svg`)} />
                </div>
            </div>
        </li> 
    `;
}