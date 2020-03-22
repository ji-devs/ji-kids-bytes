//this will cause all the components to be registered
//note there is no module name
import "./components";

//to get the typescript without needing to go through the webpack dance
import * as _WasmCore from "../_static/wasm/app/pkg/my_app_bg";
type WasmCore = typeof _WasmCore;


//see index.html
(window as any).load_wasm((wasm:WasmCore) => {
    wasm.init_app();
});

/*
import {STAGE_WIDTH, STAGE_HEIGHT} from "@settings/settings";
export const resizeApp = () => {
    const targetRatio = STAGE_WIDTH / STAGE_HEIGHT;
    let width = window.innerWidth;
    let height = window.innerHeight;
    const windowRatio = width / height;

    if (windowRatio > targetRatio ) {
        width = height * targetRatio;
    } else {
        height = width / targetRatio;
    }
   
    const scale = width / STAGE_WIDTH;
    const x = (window.innerWidth - width) / 2;
    const y = (window.innerHeight- height) / 2;
    
    const rootElement = document.documentElement;
    rootElement.style.setProperty('font-size', `${17 * scale}px`);
    rootElement.style.setProperty('--scale', `${scale}`);
    rootElement.style.setProperty('--stage-x', `${x}px`);
    rootElement.style.setProperty('--stage-y', `${y}px`);
    rootElement.style.setProperty('--stage-width', `${width}px`);
    rootElement.style.setProperty('--stage-height', `${height}px`);

}


window.onresize = resizeApp;
resizeApp();
*/