//this will cause all the components to be registered
//note there is no module name
import "./components";
import {STAGE_WIDTH, STAGE_HEIGHT} from "@settings/settings";

//to get the typescript without needing to go through the webpack dance
import * as _WasmCore from "../_static/wasm/app/pkg/my_app_bg";
type WasmCore = typeof _WasmCore;

//see index.html
(window as any).load_wasm((wasm:WasmCore) => {
    //wasm loaded
});
