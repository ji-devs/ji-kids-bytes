import {LitElement, customElement, property, css} from "lit-element";
import {nothing, html} from "lit-html";
import {styleMap} from 'lit-html/directives/style-map';
import {repeat} from 'lit-html/directives/repeat';
import topic_css from "./topic.css";
import common_css from "@components/common/common.css";
import {STAGE_WIDTH, STAGE_HEIGHT} from "@settings/settings";
import {Section} from "@events/events";
import "./left-menu/left-menu";
import "./top-header/top-header";
import "./sections/watch/watch";
import "./sections/games/games";
import "./sections/discover/discover";
import "./sections/create/create";
import "./sections/crafts/crafts";
import "./sections/help/help";
import { startResizer } from "@utils/scale";

@customElement("topic-landing")
export class Main extends LitElement {
    static styles = [common_css, topic_css];

    @property( { type : String }  ) section = "";
    @property( { type : String }  ) title = "";
    @property( { type : String }  ) id = "";

    firstUpdated() {
        startResizer("fit");
    }
    render() {
        return html`
            <div class="container">
                <div class="wrapper">
                    <div class="content">
                        <left-menu section=${this.section}></left-menu>
                        <top-header title=${this.title} section=${this.section} ></top-header>
                        <slot name="section"></slot>
                    </div>
                </div>
            </div>
        `;
    }
}
