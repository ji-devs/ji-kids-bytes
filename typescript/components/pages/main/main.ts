import {LitElement, customElement, property, css} from "lit-element";
import {nothing, html} from "lit-html";
import {styleMap} from 'lit-html/directives/style-map';
import {repeat} from 'lit-html/directives/repeat';
import main_css from "./main.css";
import common_css from "@components/common/common.css";
import {getScale} from "@settings/settings";
import {Section} from "@events/events";
import "./left-menu/left-menu";
import "./top-header/top-header";
import "./contents/watch/watch";
import "./contents/games/games";

@customElement("app-main")
export class Main extends LitElement {
    static styles = [common_css, main_css];

    @property( { type : String }  ) section = "watch" as Section;
    @property( { type : String }  ) "meta-color" = "" as Section;
    @property( { type : String }  ) "meta-title" = "" as Section;

    firstUpdated(changedProperties) {
        window.onresize = () => this.requestUpdate();
    }
    render() {
        return html`
            <div class="main">
                <div class="left">
                    <left-menu section=${this.section}></left-menu>
                </div>
                <div class="right">
                    <top-header title=${this["meta-title"]} color=${this["meta-color"]} ></top-header>
                    <div class="contents">
                        <slot name="contents"></slot>
                    </div>
                </div>
            </div>
        `;
    }
}
