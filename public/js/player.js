const STAGE_WIDTH = 1920;
const STAGE_HEIGHT = 1080;
const STAGE_PADDING_Y_PERC = 0.05; // in percentage
const STAGE_PADDING_X_PERC = 0.05; // in percentage

let help_toggled = false;

function init_page(section) {
    startResizer();
}

function toggle_help() {
    
    help_toggled = !help_toggled;

    const help_button = document.querySelector("#help-button");
    const help_section = document.querySelector("#help-section");
    const sections = document.querySelector("#sections");

    if(help_toggled) {
        help_button.classList.add("selected");
        help_section.classList.remove("invisible");
        sections.classList.add("invisible");
    } else {
        help_button.classList.remove("selected");
        help_section.classList.add("invisible");
        sections.classList.remove("invisible");
    }

}

function startResizer() {
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

        //document.documentElement.style.setProperty('font-size', `calc(62.5% * ${scale})`);

        //help fix safari, use hard pixel values
        document.documentElement.style.setProperty('font-size', `calc(10px * ${scale})`);
        document.documentElement.style.setProperty('--scale', `${scale}`);
        document.documentElement.style.setProperty('--x', `${x}px`);
        document.documentElement.style.setProperty('--y', `${y}px`);
        document.documentElement.style.setProperty('--width', `${width}px`);
        document.documentElement.style.setProperty('--height', `${height}px`);
        document.documentElement.style.setProperty('--content-x', `${(STAGE_PADDING_X_PERC/2) * width}px`);
        document.documentElement.style.setProperty('--content-y', `${(STAGE_PADDING_Y_PERC/2) * height}px`);
        document.documentElement.style.setProperty('--content-width', `${width - (STAGE_PADDING_X_PERC * width)}px`);
        document.documentElement.style.setProperty('--content-height', `${height - (STAGE_PADDING_Y_PERC * height)}px`);
    }

    window.onresize = resizeFit;
    resizeFit();

}