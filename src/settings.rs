use std::fmt;
use cfg_if::cfg_if;
use lazy_static::lazy_static;
use strum_macros::{Display, EnumString};
use jsonwebtoken::EncodingKey;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
lazy_static! {
    pub static ref SETTINGS:Settings = Settings::new();
}

pub const META_BASE_URL:&'static str = "https://bytes.jikids.org";
pub const META_BASE_DESCRIPTION:&'static str = "A taste of Jewish learning for your kids for an hour a day!";
pub const META_BASE_TITLE:&'static str = "Ji Bytes";

pub struct Settings {
    pub api_target: RemoteTarget,
    pub path_target: RemoteTarget,
    pub port: u16,
    pub epoch: Duration,
    pub local_dev: bool,
}

impl Settings {
    pub fn api(&self) -> &'static str {
        match self.api_target {
            RemoteTarget::Local => "http://localhost:8082",
            RemoteTarget::Release=> "https://api.jicloud.org",
        }
    }
    pub fn path_base(&self) -> &'static str {
        match self.path_target {
            RemoteTarget::Local => "http://localhost:4102",
            RemoteTarget::Release => "https://storage.googleapis.com/bytes-ji-kids-eu",
        }
    }
    pub fn path_social(&self) -> String {
        format!("{}/social", self.path_base())
    }

    pub fn path_ui(&self) -> String {
        format!("{}/app/ui", self.path_base())
    }
    pub fn path_help(&self, file:&str) -> String {
        format!("{}/app/help/{}", self.path_base(), file)
    }
    pub fn path_partners(&self, file:&str) -> String {
        format!("{}/app/partners/{}", self.path_base(), file)
    }
    pub fn path_app_manifest(&self) -> String {
        format!("{}/manifests/app.json", self.path_base())
    }
    pub fn path_topic_manifest(&self, topic_name:&str) -> String {
        format!("{}/manifests/topics/{}.json", self.path_base(), topic_name)
    }
    pub fn path_topics(&self) -> String {
        format!("{}/topics", self.path_base())
    }
    pub fn path_topic(&self, topic:&str, file:&str) -> String {
        format!("{}/{}", self.path_topics(), topic)
    }
}

impl fmt::Debug for Settings {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "api_target is [{}] and port is [{}]", self.api_target, self.port)
    }
}

fn get_epoch() -> Duration {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
}

#[derive(Display, EnumString)]
#[strum(serialize_all = "lowercase")]
pub enum RemoteTarget {
    Local,
    Release,
}
impl Settings {
    pub fn new_local() -> Self {
        Self {
            api_target: RemoteTarget::Local,
            path_target: RemoteTarget::Local,
            port: 8081,
            epoch: get_epoch(),
            local_dev: true,
        }
    }
    pub fn new_release() -> Self {
        Self {
            api_target: RemoteTarget::Release,
            path_target: RemoteTarget::Release,
            port: 8080,
            epoch: get_epoch(),
            local_dev: false,
        }
    }
    
    cfg_if! {
        if #[cfg(feature = "local")] {
            pub fn new() -> Self { Self::new_local() }
        } else if #[cfg(feature = "release")] {
            pub fn new() -> Self { Self::new_release() }
        } else {
            pub fn new() -> Self { unimplemented!() }
        } 
    }
}
