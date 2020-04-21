import "@components/root";
import {load_wasm} from "@utils/wasm";

(async () => {
    await load_wasm();

    let root_el = document.createElement('app-root');
    root_el.id = "root";

    document.body.appendChild(root_el);
})();