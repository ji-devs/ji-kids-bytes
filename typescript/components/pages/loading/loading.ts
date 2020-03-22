import {LitElement, customElement, property, css} from "lit-element";
import {nothing, html} from "lit-html";
import {repeat} from 'lit-html/directives/repeat';
import loading_css from "./loading.css";

@customElement("app-loading")
export class Loading extends LitElement {
    static styles = loading_css;
    render() {
        return html`
            <section class="container">
                <h1>Loading...</h1>
            </section>
        `;
    }
}