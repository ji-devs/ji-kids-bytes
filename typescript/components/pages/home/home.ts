import {LitElement, customElement, property, css} from "lit-element";
import {classMap} from 'lit-html/directives/class-map';
import {nothing, html} from "lit-html";
import {unsafeHTML} from 'lit-html/directives/unsafe-html';
import {repeat} from 'lit-html/directives/repeat';
import common_css from "@components/common/common.css";
import all_css from "./home-all.css";
import body_css from "./home-body.css";
import header_css from "./home-header.css";
import {Path, MEDIA_URL} from "@settings/settings";
import {startResizer} from "@utils/scale";

import "./home-header";
import "./home-footer";
import "./home-help";
import "./home-partners";

export enum InnerPage {
    Main,
    Help,
    Partners
}
@customElement("home-landing")
export class Home extends LitElement {
    static styles = [common_css, all_css, header_css, body_css];

    @property( { type : Object }  ) topics = [] as Array<TopicMeta>;
    @property( { type : Number }  ) inner_page = InnerPage.Main; 
    @property( { type : Boolean }  ) showing_signup = true;

    firstUpdated() {
        startResizer("normal");
    }
    render() {
        const on_page_change = (page:InnerPage) => this.inner_page = page;
        const on_close_footer = () => this.showing_signup = false;

        const [featured_topic, ...topics] = this.topics; 
        //Just for testing overflow
        //topics = new Array(4).fill(null).reduce((acc, cur) => acc.concat(this.topics), []);


        return html`
            <main>
                <home-header .on_page_change=${on_page_change} .page=${this.inner_page}></home-header>
                <section>
                    ${  this.inner_page === InnerPage.Help ? html`<home-help></home-help>` 
                        : this.inner_page === InnerPage.Partners ? html`<home-partners></home-partners>` 
                        : main(featured_topic, topics)
                    }
                </section>
                <div class="footer">
                    <ji-footer></ji-footer>
                </div>
                <home-footer .on_close=${on_close_footer.bind(this)} .visible=${this.showing_signup}></home-footer>
            </main>
            
        `;
    }
}

/* MAIN */
const main = (featured_topic:TopicMeta, topics:Array<TopicMeta>) => html`
    ${featured(featured_topic)}
    ${list_section(topics)}
`;

const featured = (topic:TopicMeta) => html`
    <a href="/topic/${topic.id}">
        <div class="featured-container">
            <div class="featured">
            <img src=${Path.topic(topic.id)(`${topic.id}.svg`)} />
            <div class="text">${topic.title}</div>
            </div>
        </div>
    </a>
`
const list_section = (topics:Array<TopicMeta>) => html`
    <ul class="topics">
        ${topics.map(topic_cell)}
    </ul>
`;

const topic_cell = (topic:TopicMeta) => {
    const {id, title} = topic;

    return html`
            <li>
                <a href="/topic/${id}">
                    <img class="icon" src=${Path.topic(id)(`${id}.svg`)} />
                    <div class="label">${title}</div>
                </a>
            </li>
    `
}
