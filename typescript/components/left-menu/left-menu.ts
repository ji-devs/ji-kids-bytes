import {LitElement, customElement, property, css} from "lit-element";
import {nothing, html, svg} from "lit-html";
import {styleMap} from 'lit-html/directives/style-map';
import {classMap} from 'lit-html/directives/class-map';
import {repeat} from 'lit-html/directives/repeat';
import left_menu_css from "./left-menu.css";
import common_css from "@components/common/common.css";
import {SelectSectionEvent, Section} from "@events/events";
import {Path} from "@settings/settings";
import {getScale} from "@settings/settings";

type SelectHandler = (section:Section) => any;

@customElement("left-menu")
export class LeftMenu extends LitElement {
    static styles = [common_css, left_menu_css];

    @property( { type : Number}  ) amount = 0;
    @property( { type : Number}  ) scale = getScale();

    @property( { type : String }  ) section = "watch" as Section;

    render() {

        const on_select = (section:Section) => (evt:any) => this.dispatchEvent(new SelectSectionEvent(section));

        return html`
            <div class="menu" >
                <img class="logo" src=${Path.ui("menu-logo.svg")} />
                ${list(this.section, on_select)}
            </div>
        `;
    }
}

const list = (current_section:Section, on_select:SelectHandler) => {

    return html`
        <ul>
            ${item("Watch", "watch", on_select, current_section)}
            ${item("Games", "games", on_select, current_section)}
            ${item("Discover", "look", on_select, current_section)}
            ${item("Make", "create", on_select, current_section)}
            ${item("Craft", "hands-on", on_select, current_section)}
        </ul>
    `;
}
/*
                */
function item(label:string, section:Section, on_select: SelectHandler, current_section:Section) {
    const selected = section === current_section 
        ? { [`${section}-selected`]: true }
        : {};

    return html`
        <li @click=${on_select(section)}>
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