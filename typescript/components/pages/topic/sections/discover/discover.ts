import {LitElement, customElement, property, css} from "lit-element";
import {nothing, html, svg} from "lit-html";
import {styleMap} from 'lit-html/directives/style-map';
import {classMap} from 'lit-html/directives/class-map';
import {repeat} from 'lit-html/directives/repeat';
import discover_css from "./discover.css";
import common_css from "@components/common/common.css";
import links_css from "@components/pages/topic/sections/links-section.css";
import {SelectSectionEvent, Section} from "@events/events";
import {Path} from "@settings/settings";
import {getScale} from "@settings/settings";

type SelectHandler = (section:Section) => any;

@customElement("section-discover")
export class _ extends LitElement {
    static styles = [common_css,links_css, discover_css];

    @property( { type : String }  ) topic = "";
    @property( { type : String }  ) discovers_json = "";
    @property( { type : String }  ) topic_id = "";

    @property( { type : Array }  ) discovers  = [] as Array<Discover>; 

    firstUpdated() {
        this.discovers = JSON.parse(this.discovers_json);
    }

    render() {

        let discovers = this.discovers;

        //for testing scrolling
        //discovers = [...discovers, ...discovers, ...discovers, ...discovers, ...discovers, ...discovers];


        return html`
            <main>
                <ul>
                    ${discovers.map(link_li(this.topic_id))}
                </ul>
            </main>
        `;
    }
}


type LinkItem = {
  link: string,

  image_filename: string,

  title: string,

  desc: string,
}
const link_li= (topic:string) => ({link, image_filename, title, desc}:LinkItem) => {
    const src = Path.topic(topic) (`discover/${image_filename}`);
    return html`
    <li>
        <img src=${src}>
        <div class="info">
            <header>${title}</header>
            <div class="desc">${desc}</div>
            <a href=${link} target="_blank" >
                <div class="link">Link Here</div>
            </a>
        </div>
    </li>
    `;
}