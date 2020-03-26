import {LitElement, customElement, property, css} from "lit-element";
import {classMap} from 'lit-html/directives/class-map';
import {nothing, html} from "lit-html";
import {unsafeHTML} from 'lit-html/directives/unsafe-html';
import {repeat} from 'lit-html/directives/repeat';
import common_css from "@components/common/common.css";
import all_css from "./home-all.css";
import partners_css from "./home-partners.css";
import {Path, MEDIA_URL} from "@settings/settings";
import {startResizer} from "@utils/scale";

const PARTNERS:Array<[string, string]> = 
    [
        ["Behrman-house.jpg", "https://www.behrmanhouse.com/"],
        ["Jewish-Journeys.jpg", "https://jccmanhattan.org/children-families/jewish-life/jewish-journeys"],
        ["Leo-Baeck.jpg", "https://lbc.ac.uk/"],
        ["NLI.jpg", "https://web.nli.org.il/sites/nli/english/pages/default.aspx"],
        ["PJ-Library.jpg", "https://pjlibrary.org"],
        ["GIT-girls-in-trouble.jpg", "https://www.girlsintroublemusic.com/"],
        ["Eliana-light.jpg", "https://elianalight.com/"],
        ["Jewish-News.jpg", "https://jewishnews.timesofisrael.com/"],
        ["PaJeS.jpg", "https://www.pajes.org.uk/"],
    ];

@customElement("home-partners")
export class _ extends LitElement {
    static styles = [common_css, all_css, partners_css];
    @property( { type : String }  ) contents = ""; 

    render() {
        return html`

            <ul class="partners">
                ${PARTNERS.map(partner_cell)}
            </ul>
            `;
    }
}

const partner_cell = ([image_src, url]:[string, string]) => {
    return html`
            <li>
                <a href="${url}" target="_blank">
                    <img class="icon" src=${Path.app(`partners/${image_src}`)} />
                </a>
            </li>
    `
}