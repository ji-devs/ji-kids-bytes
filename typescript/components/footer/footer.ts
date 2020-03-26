import {LitElement, customElement, property, css} from "lit-element";
import {classMap} from 'lit-html/directives/class-map';
import {nothing, html} from "lit-html";
import {unsafeHTML} from 'lit-html/directives/unsafe-html';
import {repeat} from 'lit-html/directives/repeat';
import common_css from "@components/common/common.css";
import footer_css from "./footer.css";
import {Path, MEDIA_URL} from "@settings/settings";
import {startResizer} from "@utils/scale";

@customElement("ji-footer")
export class Home extends LitElement {
    static styles = [common_css, footer_css];


    render() {

        return html`
            <div class="main-footer">
                <div>
                    Copyright ${new Date().getFullYear()} Jewish Interactive Inc | All Rights Reserved
                </div>
                <div>
                    <a href="https://www.jewishinteractive.org/privacy-policy/" target="_blank">Privacy Policy</a>
                </div>
                <div>
                    <a href="https://www.jewishinteractive.org/terms-and-conditions/" target="_blank">Terms & Conditions</a> 
                </div>
                <div>
                    <a href="https://www.jewishinteractive.org/jewish-interactive-child-protection-policy/" target="_blank">Child Protection Policy</a>
                </div>
            </div>
        `
    }
}
