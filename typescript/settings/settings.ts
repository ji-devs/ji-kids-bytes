export const buildMode = process.env['NODE_ENV'];
export const buildVersion =  process.env['BUILD_VERSION'];
export const isProduction = buildMode === "production" ? true : false;
export const isLocal = process.env['release_target'] === "local";

export const MEDIA_URL = isLocal 
    ? 'http://localhost:4102'
    : "https://storage.googleapis.com/bytes-ji-kids-eu";

export const Path = (() => {
    const MEDIA_APP = `${MEDIA_URL}/app`;
    const MEDIA_APP_HELP = `${MEDIA_URL}/app/help`;
    const MEDIA_APP_UI = `${MEDIA_APP}/ui`;
    const MEDIA_TOPIC = `${MEDIA_URL}/topics`;

    return {
        MEDIA_APP,
        MEDIA_APP_HELP,
        MEDIA_APP_UI,
        MEDIA_TOPIC,
        help: (path:string) => `${MEDIA_APP_HELP}/${path}`,
        ui: (path:string) => `${MEDIA_APP_UI}/${path}`,
        topic: (topic:string) => (path:string) => `${MEDIA_TOPIC}/${topic}/${path}`
    }
})();

export const STAGE_WIDTH = 1920;
export const STAGE_HEIGHT = 1080;
export const STAGE_PADDING_Y_PERC = 0.05; // in percentage
export const STAGE_PADDING_X_PERC = 0.05; // in percentage