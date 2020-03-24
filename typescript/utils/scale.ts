import {STAGE_HEIGHT,STAGE_WIDTH} from "@settings/settings";

export const startResizer = (style:"normal" | "fit") => {
    const resizeFit = () => {
        const targetRatio = STAGE_WIDTH / STAGE_HEIGHT;
        let width = window.innerWidth;
        let height = window.innerHeight;
        const windowRatio = width / height;

        if (windowRatio > targetRatio ) {
            width = height * targetRatio;
        } else {
            height = width / targetRatio;
        }

        const x = (window.innerWidth - width) / 2;
        const y = (window.innerHeight- height) / 2;
        const scale = width / STAGE_WIDTH;

        document.documentElement.style.setProperty('font-size', `calc(62.5% * ${scale})`);
        document.documentElement.style.setProperty('--scale', `${scale}`);
        document.documentElement.style.setProperty('--x', `${x}px`);
        document.documentElement.style.setProperty('--y', `${y}px`);
        document.documentElement.style.setProperty('--width', `${width}px`);
        document.documentElement.style.setProperty('--height', `${height}px`);
    }

    const resizeNormal = () => {
        document.documentElement.style.setProperty('font-size', `62.5%`);
    }

    if(style == "fit") {
        window.onresize = resizeFit;
        resizeFit();
    } else {
        window.onresize = resizeNormal;
        resizeNormal();
    }
}