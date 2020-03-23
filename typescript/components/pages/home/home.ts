import {LitElement, customElement, property, css} from "lit-element";
import {nothing, html} from "lit-html";
import {repeat} from 'lit-html/directives/repeat';
import common_css from "@components/common/common.css";
import home_css from "./home.css";
import {Path} from "@settings/settings";

@customElement("home-landing")
export class Home extends LitElement {
    static styles = [common_css, home_css];

    @property( { type : Object }  ) topics = [] as Array<TopicMeta>;

    render() {

        console.log(this.topics);

        return html`
            <h1>Welcome to Ji Kids Bytes!</h1>
            <h2>Choose a topic:</h2>
            <ul>
                ${this.topics.map(topic_cell)}
            </ul>
            
        `;
    }
}

const topic_cell = (topic:TopicMeta) => {
    const {id, title} = topic;

    return html`
        <li>
            <img class="icon" src=${Path.topic(id)(`${id}.svg`)} />
            <a href="/topic/${id}">${title}</a>
        </li>
    `
}