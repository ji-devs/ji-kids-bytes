import {LitElement, customElement, property, css} from "lit-element";
import {nothing, html} from "lit-html";
import {repeat} from 'lit-html/directives/repeat';
import common_css from "@components/common/common.css";
import not_found_css from "@components/pages/not-found/not-found.css";
import {startResizer} from "@utils/scale";

@customElement("not-found")
export class _ extends LitElement {
    static styles = [common_css, not_found_css];

    firstUpdated() {
        startResizer("normal");
    }
    render() {
        return html`
            <div class="text">Not found!</div>
        `;
    }
}