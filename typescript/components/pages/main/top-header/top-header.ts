import {LitElement, customElement, property, css} from "lit-element";
import {nothing, html, svg} from "lit-html";
import {styleMap} from 'lit-html/directives/style-map';
import {classMap} from 'lit-html/directives/class-map';
import {repeat} from 'lit-html/directives/repeat';
import top_header_css from "./top-header.css";
import common_css from "@components/common/common.css";
import {SelectSectionEvent, Section} from "@events/events";
import {Path} from "@settings/settings";
import {getScale} from "@settings/settings";

type SelectHandler = (section:Section) => any;

@customElement("top-header")
export class TopHeader extends LitElement {
    static styles = [common_css, top_header_css];

    @property( { type : String }  ) "title" = "" as Section;
    @property( { type : String }  ) "color" = "" as Section;
    render() {

        const on_select = (section:Section) => (evt:any) => this.dispatchEvent(new SelectSectionEvent(section));

        return html`
            <header>
                <div class="buttons">
                    <div class="button">
                        <img class="home" src=${Path.ui("top-header-home.svg")} />
                        <div class="label">Home</div>
                    </div>
                    <div class="button">
                        <img class="help" src=${Path.ui("top-header-help.svg")} />
                        <div class="label">Help</div>
                    </div>
                </div>
                <div class="title">${this.title}</div>
            </header>
        `;
    }
}