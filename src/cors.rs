use warp::http::Method;

pub fn get_cors() -> warp::filters::cors::Builder {
    warp::cors()
        .allow_methods(&[Method::GET, Method::POST, Method::DELETE, Method::OPTIONS])
        .allow_headers(vec!["Authorization", "Content-Type"])
        .allow_origins(vec!["http://localhost:8080", "https://bytes.jikids.org"])
}