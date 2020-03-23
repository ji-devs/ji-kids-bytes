use dominator::{Dom, html};

pub struct Loading {
}

impl Loading {
    pub fn render() -> Dom {
        html!("loading-landing")
    }
}
