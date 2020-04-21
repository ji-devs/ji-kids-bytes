import {LitElement, customElement, property, css} from "lit-element";
import {html} from "lit-html";
import {wasm} from "@utils/wasm";
import "@components/router";


@customElement("app-root")
export class _ extends LitElement {
    render() {
        return html`
            <app-router></app-router>
        `;

    }
}