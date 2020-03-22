import {LitElement, customElement, property, css} from "lit-element";
import {nothing, html} from "lit-html";
import {repeat} from 'lit-html/directives/repeat';
import common_css from "@components/common/common.css";
import home_css from "./home.css";

@customElement("app-home")
export class Home extends LitElement {
    static styles = [common_css, home_css];

    render() {
        return html`
            <h1>Welcome to Ji Kids Bytes!</h1>
            <h2>Choose a topic:</h2>
            <ul>
                <li><a href="/moses">Moses</a></li>
            </ul>
            
        `;
    }
}