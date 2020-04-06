use crate::config::Config;

pub struct SocialTags {
    pub title:String,
    pub description: String,
    pub image: String,
    pub url: String,
}

impl SocialTags {
    pub fn new_home(config:&Config) -> Self {
        Self::new(config, None, None, None, None)
    }
    
    pub fn new(config:&Config, title:Option<&str>, description: Option<&str>, image: Option<&str>, uri: Option<&str> ) -> Self {

        let description = match description {
            None => {
                match title {
                    None => config.base_description.to_string(),
                    Some(title) => format!("{} - {}", title, config.base_description)
                }
            },
            Some(description) => {
                match title {
                    None => format!("{} - {}", description, config.base_description),
                    Some(title) => format!("{} - {} - {}", title, description, config.base_description)
                }
            }
        };
        
        let title =  match title {
            None => config.base_title.to_string(),
            Some(title) => format!("{} - {}", title, config.base_title)
        };


        let image= format!("{}/{}", config.base_media_url, match image {
            None => &config.default_image,
            Some(image) => image
        });

        let url= match uri {
            None => config.base_url.to_string(),
            Some(url) => format!("{}/{}", config.base_url, url)
        };


        Self {
            title,
            description,
            image,
            url
        }
    }
}