use crate::social::*;

pub fn replace_social_tags(template:&str, social:&SocialTags) -> String {
    template
        .replace("%SOCIAL_TITLE%", &social.title)
        .replace("%SOCIAL_DESCRIPTION%", &social.description)
        .replace("%SOCIAL_IMAGE%", &social.image)
        .replace("%SOCIAL_URL%", &social.url)
}