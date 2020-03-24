import {LitElement, customElement, property, css} from "lit-element";
import {nothing, html, svg} from "lit-html";
import {styleMap} from 'lit-html/directives/style-map';
import {classMap} from 'lit-html/directives/class-map';
import {repeat} from 'lit-html/directives/repeat';
import create_css from "./create.css";
import common_css from "@components/common/common.css";
import {SelectSectionEvent, Section} from "@events/events";
import {Path} from "@settings/settings";
import {getScale} from "@settings/settings";

type SelectHandler = (section:Section) => any;

@customElement("section-create")
export class _ extends LitElement {
    static styles = [common_css,create_css];

    @property( { type : String }  ) create_json = "";
    @property( { type : String }  ) topic_id = "";

    @property( { type : Array }  ) create = {} as Create; 

    firstUpdated() {
        this.create = JSON.parse(this.create_json);
    }

    render() {

        const {tool, image_filename, header, body} = this.create;

        const src = Path.topic(this.topic_id) (`create/${image_filename}`);

        return html`
            <section>
                <div class="left">
                    <img src=${src} />
                </div>
                <div class="right">
                    <header>${header}</header>
                    <div class="description">
                        ${body}
                    </div>
                    ${tool_link(tool)}
            </section>
        `;
    }
}

enum CreationTool {
  JiTap = "jitap",
  JiStudio = "jistudio"
}
const tool_link = (tool:CreationTool) => {
    const links = {
        [CreationTool.JiStudio]: "https://jistudio.net",
        [CreationTool.JiTap]: "https://jitap.net",
    }

    return html`
        <a href=${links[tool]} target="_blank">
            <div class="button">Create Here</div>
        </a>
    `;
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