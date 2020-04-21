export let wasm;

export const load_wasm = async () => {
    const _wasm = await import("../../_static/wasm/app/pkg/my_app");
    _wasm.init();

    wasm = _wasm;
}