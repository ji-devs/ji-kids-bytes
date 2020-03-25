import {LitElement, customElement, property, css} from "lit-element";
import {nothing, html, svg} from "lit-html";
import {styleMap} from 'lit-html/directives/style-map';
import {classMap} from 'lit-html/directives/class-map';
import {repeat} from 'lit-html/directives/repeat';
import list_css from "../list-section.css";
import common_css from "@components/common/common.css";
import {SelectSectionEvent, Section} from "@events/events";
import {Path} from "@settings/settings";

type SelectHandler = (section:Section) => any;

@customElement("section-discover")
export class _ extends LitElement {
    static styles = [common_css,list_css];

    @property( { type : String }  ) topic = "";
    @property( { type : String }  ) discovers_json = "";
    @property( { type : String }  ) topic_id = "";

    @property( { type : Array }  ) discovers  = [] as Array<Link>; 

    firstUpdated() {
        this.discovers = JSON.parse(this.discovers_json);
    }

    render() {

        let discovers = this.discovers;

        //for testing scrolling
        //discovers = [...discovers, ...discovers, ...discovers, ...discovers, ...discovers, ...discovers];


        return html`
            <div class="scroller">
                <ul>
                    ${discovers.map(link_li(this.topic_id))}
                </ul>
            </div>
        `;
    }
}



const link_li= (topic:string) => ({link, image_filename, link_label, title}:Link) => {
    const src = Path.topic(topic) (`discover/${image_filename}`);
    return html`
    <li>
        <img src=${src}>
        <div class="title">${title}</div>
        <a href=${link} target="_blank" >
            <div class="button">${link_label}</div>
        </a>
    </li>
    `;
}