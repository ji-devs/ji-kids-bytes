import {LitElement, customElement, property, css} from "lit-element";
import {nothing, html, svg} from "lit-html";
import {styleMap} from 'lit-html/directives/style-map';
import {classMap} from 'lit-html/directives/class-map';
import {repeat} from 'lit-html/directives/repeat';
import {unsafeHTML} from 'lit-html/directives/unsafe-html';
import create_css from "./create.css";
import common_css from "@components/common/common.css";
import {SelectSectionEvent, Section} from "@events/events";
import {Path} from "@settings/settings";

type SelectHandler = (section:Section) => any;

@customElement("section-create")
export class _ extends LitElement {
    static styles = [common_css,create_css];

    @property( { type : String }  ) create_json = "";
    @property( { type : String }  ) topic_id = "";

    @property( { type : Object }  ) create = {} as Create; 

    render() {
        const {tool, image_filename, title, body} = this.create;
        const src = Path.topic(this.topic_id) (`create/${image_filename}`);

        return html`
            <section>
                <img src=${src} />
                <div class="right">
                    <header>${title}</header>
                    <div class="body-text">
                        ${unsafeHTML(body)}
                    </div>
                    ${tool_link(tool)}
                </div>
            </section>
        `;
    }
}

const tool_link = (tool:CreationTool) => {

    return html`
        <a href=${get_link(tool)} target="_blank">
            <div class="button">Create Here</div>
        </a>
    `;
}


//eh... enums in declaration don't seem to be usable as strings directly
//probably a way to make this less verbose but at least it typechecks
const get_link = (tool:CreationTool) => {
    switch(tool) {
        case "jistudio" as CreationTool.JiStudio: return "https://jistudio.net";
        case "jitap" as CreationTool.JiTap: return "https://jitap.net";
        case "spark" as CreationTool.Spark: return "https://spark.adobe.com/make/video-maker/";
        case "sketchpad" as CreationTool.Sketchpad: return "https://sketch.io/sketchpad/";
        case "autodraw" as CreationTool.Autodraw: return "https://www.autodraw.com/";
        default: 
            const _unreachable:never = tool;
            throw new Error("unknown creation tool " + tool);
    }
}