import {LitElement, customElement, property, css} from "lit-element";
import {classMap} from 'lit-html/directives/class-map';
import {styleMap} from 'lit-html/directives/style-map';
import {nothing, html} from "lit-html";
import {unsafeHTML} from 'lit-html/directives/unsafe-html';
import {repeat} from 'lit-html/directives/repeat';
import swiper_css from "swiper/css/swiper.css";
import common_css from "@components/common/common.css";
import all_css from "@components/pages/home/home-all.css";
import series_css from "@components/pages/home/home-body-series.css";
import {Path, MEDIA_URL} from "@settings/settings";
import {startResizer} from "@utils/scale";
import {wasm} from "@utils/wasm";

@customElement("home-series")
export class Home extends LitElement {
    static styles = [common_css, all_css, series_css];

    @property() series:Series;
    @property() page:number = 0;
    @property() arrow_right_visible:boolean = false;
    @property() arrow_left_visible:boolean = false;

    @property() row_width:number = 0;

    @property() scale_factor:number;

    private set_row_width = () => {
        const scale_factor = (() => {
            if(window.innerWidth <= 350) {
                return .3;
            }
            if(window.innerWidth <= 500) {
                return .4;
            }
            if(window.innerWidth <= 750) {
                return .6;
            }
            if(window.innerWidth <= 1150) {
                return .8;
            }

            return 1.0;
        })();

        const main_width = window.innerWidth - (86 * scale_factor);

        const row_width = main_width - (300 * scale_factor);

        this.row_width = row_width;
        this.scale_factor = scale_factor;

    }

    connectedCallback() {
        super.connectedCallback();

        window.addEventListener("resize", this.set_row_width);
    }
    disconnectedCallback() {
        super.disconnectedCallback();
        window.removeEventListener("resize", this.set_row_width);
    }

    firstUpdated() {
        this.set_row_width();
    }


    updated(changedProperties) {

        const scrollWidth = this.page * this.row_width;
        const totalWidth = this.series.topics.length * (350 * this.scale_factor);

        if(changedProperties.arrow_left_visible == null) {
            if(this.series.topics.length === 1) {
                return;
            }
            if(Math.abs(scrollWidth - this.row_width) > totalWidth) {
                this.arrow_left_visible = false;
            } else {
                this.arrow_left_visible = true;
            }
        }
        if(changedProperties.arrow_right_visible == null) {
            if(this.series.topics.length === 1) {
                return;
            }

            if(scrollWidth === 0) {
                this.arrow_right_visible = false;
            } else {
                this.arrow_right_visible = true;
            }
        }
    }

    get_style() {
        const {page, series} = this;

        const arrow_right = this.shadowRoot.getElementById("arrow-right");
        const first_item = this.shadowRoot.getElementById("first");
        const last_item = this.shadowRoot.getElementById("last");

        if(!arrow_right) {
            return null;
        }

        const targetX = page * this.row_width;

        return styleMap({
            transform: `translate(${targetX}px)`
        })
    }
    render() {
        const {series} = this;

        return html`

            <div class="row"> 
                <div class="title">${series.title}</div>
                <div class="items">
                    <div class="topics" style=${this.get_style()}>
                        ${series.topics.map((topic, index) => topic_cell(topic))}
                    </div>
                    <div id="arrow-left" class=${classMap({"arrow-container": true, left: true, disabled: !this.arrow_left_visible})} @click=${() => this.page = this.page - 1}>
                        <img src=${Path.ui("arrow-left.svg")} />
                    </div>
                    <div id="arrow-right" class=${classMap({"arrow-container": true, right: true, disabled: !this.arrow_right_visible})} @click=${() => this.page = this.page + 1}>
                        <img src=${Path.ui("arrow-right.svg")} />
                    </div>
                    
                </div>
            </div>
            
        `;
    }
}

const topic_cell = (topic:TopicMeta) => {
    const {id, title} = topic;

    return html`
            <div class="cell">
                <a href="/topic/${id}">
                    <img class="icon" src=${Path.topic(id)(`${id}_small.png`)} />
                    <div class="label">${title}</div>
                </a>
            </div>
    `
}