import {LitElement, customElement, property, css} from "lit-element";
import {nothing, html, svg} from "lit-html";
import {styleMap} from 'lit-html/directives/style-map';
import {classMap} from 'lit-html/directives/class-map';
import {repeat} from 'lit-html/directives/repeat';
import media_css from "./media.css";
import common_css from "@components/common/common.css";
import {SelectSectionEvent, Section} from "@events/events";
import {Path} from "@settings/settings";
import {gameIdToAlbumId} from "@utils/jitap";

type SelectHandler = (section:Section) => any;

@customElement("section-media")
export class _ extends LitElement {
    static styles = [common_css,media_css];

    @property( { type : String }  ) topic = "";
    @property( { type : String }  ) section = "";
    @property( { type : Number }  ) current_index = 0; 
    @property( { type : String }  ) topic_id = "";
    @property( { type : Array }  ) medias = [] as Array<Media>; 

    render() {
        const on_select = (index:number) => {
            this.current_index = index;
        }

        return html`
            <section>
                <div class="player widescreen">
                    ${player(this.medias[this.current_index])}
                </div>
                <div class="thumbnails">
                    <ul>
                        ${this.medias.map((media, index) => thumb(this.topic_id, media, this.current_index === index, () => on_select(index)))}
                    </ul>
                </div>
            </section>
        `;
    }
}

const thumb = (topic: string, media:Media, selected: boolean, on_select: () => any) => {

    const {id} = media;

    let thumb_src = '';

    if(media.player === "jitap") {
        const album_id = gameIdToAlbumId(id);
        thumb_src = `https://jitap.net/store/album/${album_id}/cover_image/`;
    } else {
        thumb_src = `https://img.youtube.com/vi/${id}/0.jpg`;
    }

    return html`
        <li class="standard" @click=${on_select}>
            <img class=${classMap({selected})} src=${thumb_src} />
        </li>
    `;
}

const player = (media:Media) => {
    const {id} = media;

    if(media.player === "jitap") {
        return html`<iframe  src='https://jitap.net/activities/${id}/player/' webkitallowfullscreen='' mozallowfullscreen='' allowfullscreen='' style='border:0'></iframe>`;
    } else {
        return html`
            <iframe src="https://www.youtube.com/embed/${id}" frameborder="0" allow="accelerometer; autoplay; encrypted-media; gyroscope; picture-in-picture" allowfullscreen></iframe>
        `
    }
}