import {LitElement, customElement, property, css} from "lit-element";
import {nothing, html, svg} from "lit-html";
import {styleMap} from 'lit-html/directives/style-map';
import {classMap} from 'lit-html/directives/class-map';
import {repeat} from 'lit-html/directives/repeat';
import games_css from "./games.css";
import common_css from "@components/common/common.css";
import {SelectSectionEvent, Section} from "@events/events";
import {Path} from "@settings/settings";
import {getScale} from "@settings/settings";

type SelectHandler = (section:Section) => any;

@customElement("section-games")
export class _ extends LitElement {
    static styles = [common_css, games_css];

    @property( { type : String }  ) ids = "";
    render() {

        const ids = JSON.parse(this.ids) as Array<string>;

        console.log(ids);

        return html`
            <ul>
                ${ids.map(id => html`<li>${embed(id)}</li>`)}
            </ul>
        `;
    }
}

const embed = (id:string) => html`
    <div class="container">
        <iframe  src='https://jitap.net/activities/${id}/player/' webkitallowfullscreen='' mozallowfullscreen='' allowfullscreen='' style='border:0'></iframe>
    </div>
`