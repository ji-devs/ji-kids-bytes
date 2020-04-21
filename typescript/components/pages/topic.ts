import {LitElement, customElement, property, css} from "lit-element";
import {nothing, html} from "lit-html";
import {styleMap} from 'lit-html/directives/style-map';
import {repeat} from 'lit-html/directives/repeat';
import topic_css from "./topic/topic.css";
import common_css from "@components/common/common.css";
import {STAGE_WIDTH, STAGE_HEIGHT} from "@settings/settings";
import {Section} from "@events/events";
import {wasm} from "@utils/wasm";
import "./topic/left-menu/left-menu";
import "./topic/top-header/top-header";
import "./topic/sections/media/media";
import "./topic/sections/links/links";
import "./topic/sections/create/create";
import "./topic/sections/help/help";
import { startResizer } from "@utils/scale";

@customElement("app-topic")
export class Main extends LitElement {
    static styles = [common_css, topic_css];

    @property( { type : String }  ) section:Section = "watch";
    @property( { type : String }  ) topic = "";
    @property( { type : Object }  ) manifest:TopicManifest;

    async firstUpdated() {
        startResizer("fit");

        const {load_topic_manifest} = wasm;
        const manifest:TopicManifest = await load_topic_manifest(this.topic);

        this.manifest = manifest;
    }
    render() {
        const {manifest, section} = this;

        const on_section_change = (section:Section) => this.section = section;

        return html`
            <div class="container">
                <div class="wrapper">
                    <div class="content">
                        ${!manifest ? show_loading : show_manifest({manifest, section, on_section_change})}
                    </div>
                </div>
            </div>
            <ji-footer></ji-footer>
        `;
    }
}

const show_manifest = ({manifest, section, on_section_change}:{manifest:TopicManifest, section: string, on_section_change: (section:Section) => any}) => html`
    <left-menu .section=${section} .on_section_change=${on_section_change}></left-menu>
    <top-header .title=${manifest.meta.title} .section=${section} .on_section_change=${on_section_change}></top-header>
    ${section === "watch" ? html`<section-media .topic_id=${manifest.meta.id} .section=${section} .medias=${manifest.videos}></section-media>`
        : section === "games" ? html`<section-media .topic_id=${manifest.meta.id} .section=${section} .medias=${manifest.games}></section-media>`
        : section === "discover" ? html`<section-links .topic_id=${manifest.meta.id} .section=${section} .links=${manifest.discovers}></section-links>`
        : section === "create" ? html`<section-create .topic_id=${manifest.meta.id} .section=${section} .create=${manifest.create}></section-create>`
        : section === "craft" ? html`<section-links .topic_id=${manifest.meta.id} .section=${section} .links=${manifest.crafts}></section-links>`
        : section === "help" ? html`<section-help .topic_id=${manifest.meta.id} .section=${section}></section-help>`
        : html`${nothing}`
    }
`;

const show_loading = () => html`
    <h1>Loading</h1>
`;