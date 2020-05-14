use std::sync::Arc;
use handlebars::Handlebars;
use serde_json::json;
use futures_util::future::TryFutureExt;
use serde::{Serialize, Deserialize};
use chrono::{Datelike, Timelike, Utc};
use crate::settings::SETTINGS;
use crate::reject::{CustomWarpRejection, RequiredData};
use crate::data::manifest::{AppManifest, Series, TopicMeta};
use crate::loader::{load_string, load_json};

#[derive(Eq, PartialEq, strum_macros::Display)]
#[strum(serialize_all = "lowercase")]
pub enum HomeSection {
    Help,
    Partners,
    Topics
}
pub async fn home_page(hb:Arc<Handlebars<'_>>, section:HomeSection) -> Result<impl warp::Reply, warp::Rejection> {
    let now = Utc::now();

    let year = format!("{}", now.year());

    let render = match section {
        HomeSection::Help => {
            hb.render("home", &json!({
                "PathUi": SETTINGS.path_ui(),
                "PathHelp": SETTINGS.path_help(),
                "page": "help",
                "local_dev": SETTINGS.local_dev,
                "year": year
            }))
        },

        HomeSection::Partners => {
            hb.render("home", &Partners::new(year))
        },

        HomeSection::Topics  => {
            let manifest:AppManifest = load_json(&SETTINGS.path_app_manifest()).await?;
            hb.render("home", &Topics::new(manifest, year))
        }
    }.unwrap_or_else(|err| err.to_string());

    Ok(warp::reply::html(render))
}

#[derive(Serialize)]
struct Topics {
    #[serde(rename="PathUi")]
    path_ui: String,
    #[serde(rename="PathTopics")]
    path_topics: String,
    page: String,
    local_dev: bool,
    series: Vec<Series>,
    featured: TopicMeta,
    year: String
}
impl Topics {
    pub fn new(manifest:AppManifest, year:String) -> Self {
        let featured:TopicMeta = {
            let first = &manifest.series[0].topics[0];
            first.clone()
        };

        Self{
            year,
            path_ui: SETTINGS.path_ui(),
            path_topics: SETTINGS.path_topics(),
            page: "topics".to_string(),
            local_dev: SETTINGS.local_dev,
            series: manifest.series,
            featured
        }
    }
}

#[derive(Serialize)]
struct Partners {
    #[serde(rename="PathUi")]
    path_ui: String,
    page: String,
    local_dev: bool,
    partners: Vec<Partner>,
    year: String,
}
#[derive(Serialize)]
struct Partner {
    src: String,
    href: &'static str,
    alt: &'static str
}

impl Partners {
    pub fn new(year: String) -> Self {
        Self{
            year,
            path_ui: SETTINGS.path_ui(),
            page: "partners".to_string(),
            local_dev: SETTINGS.local_dev,
            partners:
                crate::data::partners::list
                .into_iter()
                .map(|item| {
                    Partner {
                        src: SETTINGS.path_partners(item[0]),
                        href: item[1],
                        alt: item[2]
                    }
                })
                .collect()
        }
    }
}