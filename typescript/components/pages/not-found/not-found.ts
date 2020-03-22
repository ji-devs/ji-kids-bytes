import {LitElement, customElement, property, css} from "lit-element";
import {nothing, html} from "lit-html";
import {repeat} from 'lit-html/directives/repeat';
import common_css from "@components/common/common.css";
import not_found_css from "./not-found.css";

@customElement("not-found")
export class NotFound extends LitElement {
    static styles = [common_css, not_found_css];

    render() {
        return html`
            <h1>Not found!</h1>
        `;
    }
}