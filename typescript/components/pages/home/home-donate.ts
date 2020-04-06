import {LitElement, customElement, property, css} from "lit-element";
import {classMap} from 'lit-html/directives/class-map';
import {nothing, html} from "lit-html";
import {unsafeHTML} from 'lit-html/directives/unsafe-html';
import {repeat} from 'lit-html/directives/repeat';
import common_css from "@components/common/common.css";
import all_css from "./home-all.css";
import donate_css from "./home-donate.css";
import {Path, MEDIA_URL} from "@settings/settings";
import {startResizer} from "@utils/scale";

@customElement("home-donate")
export class _ extends LitElement {
    static styles = [common_css, all_css, donate_css];

    render() {
        return html`

            <div class="donate">
                <span>
                    Enjoying using Ji? 
                    Help us continue to help you. 
                    We are a nonprofit. 
                    <a href="https://www.jewishinteractive.org/donate-to-ji-coronavirus/" target="_blank">Please click here to donate.</a>
                </span>
            </div>
            `;
    }
}