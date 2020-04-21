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
import {HomeSection} from "@components/pages/home";

@customElement("home-header")
export class _ extends LitElement {
    static styles = [common_css, all_css, header_css, body_css];

    @property( { type : String }  ) section = HomeSection.Landing; 
    @property( { type : Function } ) on_section_change = (section:HomeSection) => {};
    render() {

        const on_select_help = () => this.on_section_change(this.section === HomeSection.Help ? HomeSection.Landing : HomeSection.Help); 
        const on_select_partners = () => this.on_section_change(this.section === HomeSection.Partners ? HomeSection.Landing : HomeSection.Partners); 

        return html`
            <header>
                <a href="/">
                    <div class="logo">
                        <img src=${Path.ui("top-header-logo.svg")} />
                        <div class="byline">
                            A taste of Jewish learning for your kids for an hour a day!
                        </div>
                    </div>
                </a>
                <div class="header-line"></div>
                <div class="right">
                    <div class=${classMap({top_header_button: true, selected: this.section === HomeSection.Help})} @click=${on_select_help.bind(this)}>
                        <div class="circle">
                            <img class="help" src=${Path.ui("top-header-help.svg")} />
                        </div>
                        <div class="label">Help</div>
                    </div>
                    <div class=${classMap({top_header_button: true, selected: this.section === HomeSection.Partners})} @click=${on_select_partners.bind(this)}>
                        <div>
                            <img class="partners" src=${Path.ui(this.section === HomeSection.Partners ? "top-header-partners-selected.svg" : "top-header-partners.svg")} />
                            <div class="label">Partners</div>
                        </div>
                    </div>
                    <a href="https://www.jewishinteractive.org/kids-learning-at-home" target="_blank">
                        <img class="home" src=${Path.ui("top-header-home.svg")} />
                        <div class="label home-offset">Home Learning</div>
                    </a>
                </div>
            </header>
                ${intro()}
            
        `;
    }
}


const intro = () => html`
    <div class="header-intro">
        <div class="container">
            <!--
                Ji Bytes gives your kids a taste of Jewish learning - for an hour a day!
                <br/>
                <div class="button">
                    Sign up for free updates
                </div>
            -->
            <!--
            Ji Bytes gives you a taste of Jewish learning
            <br/>that kids can really chew on (for an hour a day).
            -->
        </div>
    </div>
`