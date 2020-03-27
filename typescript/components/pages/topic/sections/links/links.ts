import {LitElement, customElement, property, css} from "lit-element";
import {nothing, html, svg} from "lit-html";
import {styleMap} from 'lit-html/directives/style-map';
import {classMap} from 'lit-html/directives/class-map';
import {repeat} from 'lit-html/directives/repeat';
import links_css from "./links.css";
import common_css from "@components/common/common.css";
import {SelectSectionEvent, Section} from "@events/events";
import {Path} from "@settings/settings";

type SelectHandler = (section:Section) => any;

@customElement("section-links")
export class _ extends LitElement {
    static styles = [common_css,links_css];

    @property( { type : String }  ) topic = "";
    @property( { type : String }  ) section = "";
    @property( { type : String }  ) topic_id = "";

    @property( { type : Array }  ) links = [] as Array<Link>; 


    render() {

        console.log(this.links);

        //for testing scrolling
        //discovers = [...discovers, ...discovers, ...discovers, ...discovers, ...discovers, ...discovers];


        return html`
            <div class="scroller">
                <ul>
                    ${this.links.map(link_li(this.topic_id, this.section))}
                </ul>
            </div>
        `;
    }
}



const link_li= (topic:string, section:string) => ({link, image_filename, link_label, title}:Link) => {
    const image_base = section === "craft" ? "crafts" : section;

    const src = Path.topic(topic) (`${image_base}/${image_filename}`);
    return html`
    <li>
        <img src=${src}>
        <div class="title">${title}</div>
        <a href=${link} target="_blank" >
            <div class="button">${get_link_label(link_label)}</div>
        </a>
    </li>
    `;
}

const get_link_label = (link_label?:string):string => 
    link_label == null || link_label === ""
        ? "Show me how"
        : link_label;