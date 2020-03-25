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

@customElement("home-landing")
export class Home extends LitElement {
    static styles = [common_css, all_css, header_css, body_css];

    @property( { type : Object }  ) topics = [] as Array<TopicMeta>;
    @property( { type : Boolean }  ) help_selected = false; 

    firstUpdated() {
        startResizer("normal");
    }
    render() {

        const on_select_help = () => this.help_selected = !this.help_selected; 

        let topics = this.topics;
        //Just for testing overflow
        //topics = new Array(12).fill(null).reduce((acc, cur) => acc.concat(this.topics), []);

        return html`
            <main>
                <header>
                    <a href="/">
                        <img class="logo" src=${Path.ui("top-header-logo.svg")} />
                    </a>
                    <div class="header-line"></div>
                    <div class="right">
                        <div class=${classMap({top_home_button: true, selected: this.help_selected})} @click=${on_select_help.bind(this)}>
                            <div class="circle">
                                <img class="help" src=${Path.ui("top-header-help.svg")} />
                            </div>
                            <div class="label">Help</div>
                        </div>
                        <div>
                            <img class="partners" src=${Path.ui("top-header-partners.svg")} />
                            <div class="label">Partners</div>
                        </div>
                        <a href="https://www.jewishinteractive.org/kids-learning-at-home">
                            <img class="home" src=${Path.ui("top-header-home.svg")} />
                            <div class="label">Ji Home Learning</div>
                        </a>
                    </div>
                </header>
                <section>
                    ${this.help_selected ? html`<home-help></home-help>` : list_section(topics)}
                </section>
            </main>
            
        `;
    }
}

import help_css from "./home-help.css";
@customElement("home-help")
export class _ extends LitElement {
    static styles = [common_css, all_css, help_css];
    @property( { type : String }  ) contents = ""; 

    firstUpdated() {
        fetch(Path.help("help-main-snippet.html"))
            .then(contents => contents.text())
            .then(contents => {
                this.contents = contents.replace(/%HELP_MEDIA_URL%/g, Path.help(""));
            })
    }
    render() {
        return html`
            <div class="help-contents" >
                ${unsafeHTML(this.contents)}
            </div>
            `;
    }
}

const list_section = (topics:Array<TopicMeta>) => html`
    <ul>
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