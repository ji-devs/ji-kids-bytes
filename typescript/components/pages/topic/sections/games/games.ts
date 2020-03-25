import {LitElement, customElement, property, css} from "lit-element";
import {nothing, html, svg} from "lit-html";
import {styleMap} from 'lit-html/directives/style-map';
import {classMap} from 'lit-html/directives/class-map';
import {repeat} from 'lit-html/directives/repeat';
import games_css from "./games.css";
import common_css from "@components/common/common.css";
import player_css from "@components/pages/topic/sections/player-section.css";
import {SelectSectionEvent, Section} from "@events/events";
import {Path} from "@settings/settings";
import {gameIdToAlbumId} from "@utils/jitap";
import {is_mobile} from "@utils/user-agent";

type SelectHandler = (section:Section) => any;

@customElement("section-games")
export class _ extends LitElement {
    static styles = [common_css,player_css, games_css];

    @property( { type : String }  ) topic = "";
    @property( { type : String }  ) ids_json = "";
    @property( { type : String }  ) current_id = "";
    @property( { type : String }  ) topic_id = "";

    @property( { type : Array }  ) ids = [] as Array<string>; 

    firstUpdated() {
        this.ids = JSON.parse(this.ids_json);

        this.current_id = this.ids[0];

        if(is_mobile()) {
            alert(`Games may not play on this device.\nFor best experience - use a regular computer instead`)
        }
    }

    render() {

        let ids = this.ids;

        //for testing scrolling
        //ids = [...ids, ...ids, ...ids, ...ids, ...ids, ...ids];


        const on_select = (id:string) => {
            this.current_id = id;
        }

        return html`
            <section>
                <div class="player standard">
                    ${game(this.current_id)}
                </div>
                <div class="thumbnails">
                    <ul>
                        ${ids.map(id => thumb(this.topic_id, id, this.current_id, on_select))}
                    </ul>
                </div>
            </section>
        `;

    }
}

const thumb = (topic: string, id:string, current_id: string, on_select: (id:string) => any) => {
    const album_id = gameIdToAlbumId(id);
    const img_src = () => 
        `https://jitap.net/store/album/${album_id}/cover_image/`;

    return html`
        <li class="standard" @click=${() => on_select(id)}>
            <img class=${classMap({selected: current_id === id})} src=${img_src()} />
        </li>
    `;
}


const game = (id:string) => html`
        <iframe  src='https://jitap.net/activities/${id}/player/' webkitallowfullscreen='' mozallowfullscreen='' allowfullscreen='' style='border:0'></iframe>
`