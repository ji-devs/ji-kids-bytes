export const buildMode = process.env['NODE_ENV'];
export const buildVersion =  process.env['BUILD_VERSION'];
export const isProduction = buildMode === "production" ? true : false;
export const isLocal = process.env['release_target'] === "local";

export const MEDIA_URL = isLocal 
    ? 'http://localhost:4102'
    : "https://storage.googleapis.com/bytes-ji-kids-eu";

export const Path = (() => {
    const MEDIA_APP = `${MEDIA_URL}/app`;
    const MEDIA_APP_UI = `${MEDIA_APP}/ui`;

    return {
        ui: (path:string) => `${MEDIA_APP_UI}/${path}`
    }
})();

export const STAGE_WIDTH = 1920;
export const STAGE_HEIGHT = 1080;

export function getScale() {
    const scaleHeight = window.innerHeight / STAGE_HEIGHT;
    const scaleWidth = window.innerWidth / STAGE_WIDTH;
    const scaleMax = scaleWidth > scaleHeight ? scaleWidth : scaleHeight;
    const scaleMin = scaleWidth < scaleHeight ? scaleWidth : scaleHeight;

    return {scaleHeight, scaleWidth, scaleMax, scaleMin};
}