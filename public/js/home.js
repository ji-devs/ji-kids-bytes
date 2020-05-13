init_signup();
init_carousel();

function init_signup() {
    const signup_el = document.getElementById("signup");
    const signup_close_btn_el = document.getElementById("signup_close_btn");

    const STORAGE_KEY = "ji-kids-bytes-signup-closed";

    signup_close_btn_el.onclick = () => {
        signup_el.classList.remove("visible");

        localStorage.setItem(STORAGE_KEY, Date.now());
    }


    const signupClosed = localStorage.getItem(STORAGE_KEY);

    if(signupClosed != null) {
        try {
            const dateClosed = parseInt(signupClosed);
            if(dateClosed) {
                signup_el.remove();
            }
        } catch(e) {
            console.error(e);
            localStorage.removeItem(STORAGE_KEY);
        }
    }
}

function init_carousel() {
    document.querySelectorAll('.main-carousel')
        .forEach(elem => {
            const flkty = new Flickity( elem, {
                groupCells: true,
                cellAlign: 'left',

            });
        });
}