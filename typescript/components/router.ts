import {LitElement, customElement, property, css} from "lit-element";
import {html} from "lit-html";
import {wasm} from "@utils/wasm";
import "@components/pages/not-found";
import "@components/pages/home";
import "@components/pages/topic";
import {HomeSection} from "@components/pages/home";

export type Page = 
    { type: "home", content: HomeSection }
    | { type: "topic", content: string }

@customElement("app-router")
export class _ extends LitElement {
    render() {
        const {get_page} = wasm;
        const page:Page = get_page();

        switch(page.type) {
            case "home": return html`<app-home .section=${page.content}></app-home>`
            case "topic": return html`<app-topic .topic=${page.content}></app-topic>`
            default: return html`<not-found></not-found>`
        }
    }
}