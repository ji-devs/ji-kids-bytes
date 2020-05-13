use warp::{
    http::Method,
    Filter,
    path
};
use crate::settings::{SETTINGS, Settings};
use crate::reject::handle_rejection;
use crate::pages::{
    home::{HomeSection, home_page},
    player::{TopicSection, player_page, player_page_section_str},
    sitemap::sitemap_page,
    epoch::epoch_page
};
use ji_cloud_shared::{
    user::{UserCreateRequest, UserCreateError},
};
use super::cors::get_cors;
use crate::templates::register_templates;
use crate::reject::{CustomWarpRejection, RequiredData};
use std::net::SocketAddr;

pub fn get_routes() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {

    let hb = register_templates();
    

    path::end()
        .and_then({ 
            let hb = hb.clone(); 
            move || home_page(hb.clone(), HomeSection::Topics)
        })
        .or(path!("sitemap.xml").and_then({ 
            let hb = hb.clone(); 
            move || sitemap_page(hb.clone())
        }))
        .or(path!("help").and_then({ 
            let hb = hb.clone(); 
            move || home_page(hb.clone(), HomeSection::Help)
        }))
        .or(path!("partners").and_then({ 
            let hb = hb.clone(); 
            move || home_page(hb.clone(), HomeSection::Partners)
        }))
        .or(path!("topic" / String).and_then({ 
            let hb = hb.clone(); 
            move |topic_name| player_page(hb.clone(), None, topic_name)
        }))
        .or(path!("topic" / String / String).and_then({ 
            let hb = hb.clone(); 
            move |topic_name, section_name| player_page_section_str(hb.clone(), section_name, topic_name)
        }))
        .or(warp::fs::dir("./public/"))
        .or(path!("epoch").map(epoch_page))
        .recover(handle_rejection)
        .with(get_cors())
}