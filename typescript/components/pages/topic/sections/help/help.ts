import {LitElement, customElement, property, css} from "lit-element";
import {nothing, html, svg} from "lit-html";
import {styleMap} from 'lit-html/directives/style-map';
import {classMap} from 'lit-html/directives/class-map';
import {repeat} from 'lit-html/directives/repeat';
import help_css from "./help.css";
import common_css from "@components/common/common.css";
import {SelectSectionEvent, Section} from "@events/events";
import {Path} from "@settings/settings";
import {getScale} from "@settings/settings";

type SelectHandler = (section:Section) => any;

@customElement("section-help")
export class _ extends LitElement {
    static styles = [common_css,help_css];

    @property( { type : String }  ) create_json = "";
    @property( { type : String }  ) topic_id = "";

    render() {


        return html`
            <section>
                ${help_iframe()}
            </section>
        `;
    }
}

const help_iframe = () => html`
    <iframe src="${Path.help("help-topic-iframe.html")}" frameborder="0" allow="accelerometer; autoplay; encrypted-media; gyroscope; picture-in-picture" allowfullscreen></iframe>
`