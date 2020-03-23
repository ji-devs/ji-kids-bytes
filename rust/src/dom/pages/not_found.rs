use dominator::{Dom, html};

pub struct NotFound {
}

impl NotFound {
    pub fn render() -> Dom {
        html!("not-found-landing")
    }
}
