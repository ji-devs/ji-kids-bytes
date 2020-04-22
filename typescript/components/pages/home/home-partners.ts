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
        ["ShalomLearning.jpg", "https://www.shalomlearning.org/"],
        ["Judaism-unbound.jpg", "https://www.judaismunbound.com/"],
        ["Jewish-Live.jpg", "https://www.jewishlive.org/"],
        ["ORT-UK.jpg", "https://ortuk.org/"],
        ["CIJE.jpg", "https://www.thecije.org/"],
        ["jkidla.jpg", "https://jkidla.com/"],
        ["JEP.jpg", "https://www.jewishedproject.org/"],
        ["Olam-Shalem.jpg", "http://www.olamshalem.org/Index.aspx"],
        ["JLGB.jpg","https://www.jlgb.org/"],
        ["HebrewHelpers.jpg","https://www.hebrewhelpers.com/"],
        ["cojds.jpg","https://www.cojds.org/"],
        ["LSJS.jpg","https://www.lsjs.ac.uk/"],
        ["mjlogo.jpg","https://masorti.org.uk/"],
        //["hop.jpg","https://www.hop.co.il/"],
        ["WZO.jpg","https://www.wzo.org.il/education/"],
        ["beit-issie.jpg","https://en.beitissie.org.il/"],
        ["IAC.jpg","https://www.israeliamerican.org/home/kids"],
        ["Hadassah.jpg","http://hadassahinternational.org/"],
        ["IAC-home.jpg","https://www.israeliamerican.org/home/kids"],
        ["TheJewishAgency.jpg","https://www.jewishagency.org/"],
        ["opendor.jpg","https://opendormedia.org/"],
        ["unpacked.jpg","https://opendormedia.org/unpacked/"],
        ["ujia.jpg","https://ujia.org/"],
        ["icenter.jpg","https://www.theicenter.org/"],
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