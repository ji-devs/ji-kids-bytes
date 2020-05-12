import {LitElement, customElement, property, css} from "lit-element";
import {classMap} from 'lit-html/directives/class-map';
import {nothing, html} from "lit-html";
import {unsafeHTML} from 'lit-html/directives/unsafe-html';
import {repeat} from 'lit-html/directives/repeat';
import common_css from "@components/common/common.css";
import all_css from "@components/pages/home/home-all.css";
import body_css from "@components/pages/home/home-body.css";
import header_css from "@components/pages/home/home-header.css";
import {Path, MEDIA_URL} from "@settings/settings";
import {startResizer} from "@utils/scale";
import {wasm} from "@utils/wasm";

import "@components/pages/home/home-header";
import "@components/pages/home/home-footer";
import "@components/pages/home/home-help";
import "@components/pages/home/home-partners";
import "@components/pages/home/home-donate";
import "@components/pages/home/home-body-series";

export enum HomeSection {
    Landing = "landing",
    Help = "help",
    Partners = "partners"
}

@customElement("app-home")
export class Home extends LitElement {
    static styles = [common_css, all_css, header_css, body_css];

    @property( { type : Object }  ) topics = [] as Array<TopicMeta>;
    @property( { type : String }  ) section = HomeSection.Landing; 
    @property( { type : Boolean }  ) showing_signup = true;

    firstUpdated() {
        startResizer("normal");
    }
    render() {
        const on_section_change = (section:HomeSection) => this.section = section;
        const on_close_footer = () => this.showing_signup = false;

        return html`
            <home-donate></home-donate>
            <main>
                <home-header .on_section_change=${on_section_change} .section=${this.section}></home-header>
                <section>
                    ${  this.section === HomeSection.Help ? html`<home-help></home-help>` 
                        : this.section === HomeSection.Partners ? html`<home-partners></home-partners>` 
                        : html`<home-topics></home-topics>` 
                    }
                </section>
                <home-footer .on_close=${on_close_footer.bind(this)} .visible=${this.showing_signup}></home-footer>
                <div class="footer">
                    <ji-footer></ji-footer>
                </div>
            </main>
            
        `;
    }
}

@customElement("home-topics")
export class _ extends LitElement {
    static styles = [common_css, all_css, header_css, body_css];

    @property() manifest:AppManifest

    async firstUpdated() {
        const {load_app_manifest} = wasm;
        const manifest:AppManifest = await load_app_manifest();

        this.manifest = manifest;
    }
    render() {
        const {manifest} = this;

        if(!manifest) {
            return html`<h1>Loading...</h1>`
        } else {
            //TODO - move to Rust
            const all_unlocked = window.location.hash === "#unlock";
            const series = 
                this.manifest.series
                    .map(series => ({
                        id: series.id,
                        title: series.title,
                        topics: all_unlocked ? series.topics 
                        : series.topics.filter(topic => !topic.locked)
                    }))
                    .filter(({topics}) => topics.length > 0);

            if(!series.length) {
                return html`<h1>No series?!</h1>`;
            }

            const featured_topic = series[0].topics.splice(0, 1)[0];

            //const topics = series.reduce((acc, curr) => acc.concat(curr.topics), []);

            return html`
                ${featured(featured_topic)}
                ${series.map(series => html`<home-series .series=${series}></home-series>`)}
            `;
                //TODO!${series.map(series_row)}
        }
    }
}

const featured = (topic:TopicMeta) => html`
    <a href="/topic/${topic.id}">
        <div class="featured">
            <img src=${Path.topic(topic.id)(`${topic.id}_medium.png`)} />
            <div class="text">${topic.title}</div>
        </div>
    </a>
`;
