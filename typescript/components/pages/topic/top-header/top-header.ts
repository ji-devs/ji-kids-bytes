import {LitElement, customElement, property, css} from "lit-element";
import {nothing, html, svg} from "lit-html";
import {styleMap} from 'lit-html/directives/style-map';
import {classMap} from 'lit-html/directives/class-map';
import {repeat} from 'lit-html/directives/repeat';
import top_header_css from "./top-header.css";
import common_css from "@components/common/common.css";
import {SelectSectionEvent, Section} from "@events/events";
import {Path} from "@settings/settings";

type SelectHandler = (section:Section) => any;

@customElement("top-header")
export class TopHeader extends LitElement {
    static styles = [common_css, top_header_css];

    @property( { type : String }  ) title = "";
    @property( { type : String }  ) color = "";
    @property( { type : String }  ) section = "" as Section;
    render() {
        const on_select = (section:Section) => (evt:any) => this.dispatchEvent(new SelectSectionEvent(section));

        const help_selected = this.section === "help";

        return html`
            <header>
                <div class="buttons">
                    <div class=${classMap({header_button: true, selected: help_selected})} @click=${on_select("help")}>
                        <div class="circle">
                            <img class="help" src=${Path.ui("top-header-help.svg")} />
                        </div>
                        <div class="label">Help for<br/>parents</div>
                    </div>
                </div>
                <div class="title">${this.title}</div>
            </header>
        `;
    }
}