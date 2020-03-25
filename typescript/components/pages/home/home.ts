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

enum InnerPage {
    Main,
    Help,
    Partners
}
@customElement("home-landing")
export class Home extends LitElement {
    static styles = [common_css, all_css, header_css, body_css];

    @property( { type : Object }  ) topics = [] as Array<TopicMeta>;
    @property( { type : Number }  ) inner_page = InnerPage.Main; 

    firstUpdated() {
        startResizer("normal");
    }
    render() {

        const on_select_help = () => this.inner_page = this.inner_page === InnerPage.Help ? InnerPage.Main : InnerPage.Help; 
        const on_select_partners = () => this.inner_page = this.inner_page === InnerPage.Partners ? InnerPage.Main : InnerPage.Partners; 

        let topics = this.topics;
        //Just for testing overflow
        //topics = new Array(4).fill(null).reduce((acc, cur) => acc.concat(this.topics), []);

        return html`
            <main>
                <header>
                    <a href="/">
                        <img class="logo" src=${Path.ui("top-header-logo.svg")} />
                    </a>
                    <div class="header-line"></div>
                    <div class="right">
                        <div class=${classMap({top_header_button: true, selected: this.inner_page === InnerPage.Help})} @click=${on_select_help.bind(this)}>
                            <div class="circle">
                                <img class="help" src=${Path.ui("top-header-help.svg")} />
                            </div>
                            <div class="label">Help</div>
                        </div>
                        <div class=${classMap({top_header_button: true, selected: this.inner_page === InnerPage.Partners})} @click=${on_select_partners.bind(this)}>
                            <div>
                                <img class="partners" src=${Path.ui(this.inner_page === InnerPage.Partners ? "top-header-partners-selected.svg" : "top-header-partners.svg")} />
                                <div class="label">Partners</div>
                            </div>
                        </div>
                        <a href="https://www.jewishinteractive.org/kids-learning-at-home" target="_blank">
                            <img class="home" src=${Path.ui("top-header-home.svg")} />
                            <div class="label">Home Learning</div>
                        </a>
                    </div>
                </header>
                <section>
                    ${  this.inner_page === InnerPage.Help ? html`<home-help></home-help>` 
                        : this.inner_page === InnerPage.Partners ? partners()
                        : main(topics)
                    }
                </section>
                <div class="footer">
                    <ji-footer></ji-footer>
                </div>
            </main>
            
        `;
    }
}

/* MAIN */
const main = (topics:Array<TopicMeta>) => html`
    <div class="intro-text">
        JiBytes gives you a taste of Jewish learning that kids can really chew on (for an hour a day).
        <br/>Come back for frequent updates of new JiBytes.
    </div>
    <!--${featured(topics[0])}-->
    ${list_section(topics)}
`;

const featured = (topic:TopicMeta) => html`
    <a href="/topic/${topic.id}">
        <div class="featured-container">
            <div class="featured">
            <img src=${Path.topic(topic.id)(`${topic.id}.svg`)} />
            <div class="text">FEATURED!</div>
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

/* MAIN */
const partners = () => html`
    <ul class="partners">
        ${[
            ["Behrman-house.jpg", "https://www.behrmanhouse.com/"],
            ["Jewish-Journeys.jpg", "https://jccmanhattan.org/children-families/jewish-life/jewish-journeys"],
            ["Leo-Baeck.jpg", "https://lbc.ac.uk/"],
            ["NLI.jpg", "https://web.nli.org.il/sites/nli/english/pages/default.aspx"],
            ["PJ-Library.jpg", "https://pjlibrary.org"],
            ["GIT-girls-in-trouble.jpg", "https://www.girlsintroublemusic.com/"],
            ["Eliana-light.jpg", "https://elianalight.com/"],
            ["Jewish-News.jpg", "https://jewishnews.timesofisrael.com/"],
            ["PaJeS.jpg", "https://www.pajes.org.uk/"],
        ].map(partner_cell)}
    </ul>
`;

const partner_cell = ([image_src, url]:[string, string]) => {
    return html`
            <li>
                <a href="${url}" target="_blank">
                    <img class="icon" src=${Path.app(`partners/${image_src}`)} />
                </a>
            </li>
    `
}

/* HELP */
import help_css from "./home-help.css";
@customElement("home-help")
export class _ extends LitElement {
    static styles = [common_css, all_css, help_css];
    @property( { type : String }  ) contents = ""; 

    firstUpdated() {
        fetch(Path.help("help-main-snippet.html"))
            .then(contents => contents.text())
            .then(contents => {
                this.contents = contents.replace(/%HELP_MEDIA_URL%/g, Path.MEDIA_APP_HELP)
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