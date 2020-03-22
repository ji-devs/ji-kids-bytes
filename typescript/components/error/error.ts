import {LitElement, customElement, property, css} from "lit-element";
import {nothing, html} from "lit-html";
import {repeat} from 'lit-html/directives/repeat';
import error_css from "./error.css";

@customElement("app-error")
export class Error extends LitElement {
    static styles = error_css;

    @property( { type : Number}  ) amount = 0;

    render() {


        return html`
            <h1>Error...</h1>
        `;
    }
}