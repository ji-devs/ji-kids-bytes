import {LitElement, customElement, property, css} from "lit-element";
import {classMap} from 'lit-html/directives/class-map';
import {nothing, html} from "lit-html";
import {unsafeHTML} from 'lit-html/directives/unsafe-html';
import {repeat} from 'lit-html/directives/repeat';
import common_css from "@components/common/common.css";
import all_css from "./home-all.css";
import body_css from "./home-body.css";
import header_css from "./home-header.css";
import {Path, MEDIA_URL} from "@settings/settings";
import {startResizer} from "@utils/scale";


/* HELP */
import help_css from "./home-help.css";
@customElement("home-help")
export class _ extends LitElement {
    static styles = [common_css, all_css, help_css];
    @property( { type : String }  ) contents = ""; 

    firstUpdated() {
        fetch(Path.help("help-main-snippet.html"))
            .then(contents => contents.text())
            .then(contents => {
                this.contents = contents.replace(/%HELP_MEDIA_URL%/g, Path.MEDIA_APP_HELP)
            })
    }
    render() {
        return html`
            <div class="help-contents" >
                ${unsafeHTML(this.contents)}
            </div>
            `;
    }
}