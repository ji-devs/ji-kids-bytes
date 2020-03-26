import {LitElement, customElement, property, css} from "lit-element";
import {classMap} from 'lit-html/directives/class-map';
import {nothing, html} from "lit-html";
import {unsafeHTML} from 'lit-html/directives/unsafe-html';
import {repeat} from 'lit-html/directives/repeat';
import common_css from "@components/common/common.css";
import all_css from "./home-all.css";
import footer_css from "./home-footer.css";
import {Path, MEDIA_URL} from "@settings/settings";
import {startResizer} from "@utils/scale";
import {InnerPage} from "./home";

@customElement("home-footer")
export class _ extends LitElement {
    static styles = [common_css, all_css, footer_css];

    @property( { type : Boolean }  ) visible = false; 
    @property( { type : Function }  ) on_close = () => {}; 
    render() {

        return html`
            <footer class=${classMap({visible: this.visible})}>
                <div class="bg">
                    <a href="https://share.hsforms.com/1p8ZRFmUCS-ijx6_QyHrTUw1kii1" target=_blank>
                        <img class="bg" src=${Path.ui(`bottom-footer-signup.svg`)} />
                    </a>
                    <div class="close" @click=${this.on_close}>
                        <img src=${Path.ui(`bottom-footer-signup-close.svg`)} />
                    </a>
                </div>
            </footer>
        `
    }
}