use handlebars::Handlebars;
use std::sync::Arc;

pub fn register_templates() -> Arc<Handlebars<'static>> {
    let mut hb = Handlebars::new();

    hb.register_template_file("facebook-pixel", "./markup/templates/facebook-pixel.hbs").expect("unable to parse facebook pixel template");
    hb.register_template_file("hubspot-tracking", "./markup/templates/hubspot-tracking.hbs").expect("unable to parse hubspot tracking template");
    hb.register_template_file("google-analytics", "./markup/templates/google-analytics.hbs").expect("unable to parse google analytics template");

    hb.register_template_file("home", "./markup/templates/home.hbs").expect("unable to parse home template");
    hb.register_template_file("home-topics", "./markup/templates/home/topics.hbs").expect("unable to parse home-topics template");
    hb.register_template_file("home-help", "./markup/templates/home/help.hbs").expect("unable to parse home-help template");
    hb.register_template_file("home-partners", "./markup/templates/home/partners.hbs").expect("unable to parse home-partners template");

    hb.register_template_file("player", "./markup/templates/player.hbs").expect("unable to parse player template");
    hb.register_template_file("player-left-menu", "./markup/templates/player/left-menu.hbs").expect("unable to parse player left-menu template");
    hb.register_template_file("player-top-header", "./markup/templates/player/top-header.hbs").expect("unable to parse player top-header template");
    hb.register_template_file("player-section-media", "./markup/templates/player/section-media.hbs").expect("unable to parse player section-media template");
    hb.register_template_file("player-section-links", "./markup/templates/player/section-links.hbs").expect("unable to parse player section-links template");
    hb.register_template_file("player-section-create", "./markup/templates/player/section-create.hbs").expect("unable to parse player section-create template");
    hb.register_template_file("player-section-help", "./markup/templates/player/section-help.hbs").expect("unable to parse player section-help template");

    hb.register_template_file("sitemap", "./markup/templates/sitemap.hbs").expect("unable to parse sitemap template");

    Arc::new(hb)
}