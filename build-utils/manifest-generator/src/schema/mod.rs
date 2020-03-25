mod schema_drive;
mod schema_app;

pub use schema_drive::*;
pub use schema_app::*;

#[cfg(test)]
mod tests {
    use std::path::Path;
    use structopt::StructOpt;
    use tokio::runtime::Runtime;
    use crate::schema::*;
    use crate::loader::*;
    use crate::config::*;

    #[test]
    fn test_schema() {
        let path = UrlOrPath::Path(Path::new("./mock/small.json"));
        //let path = UrlOrPath::Path(Path::new("./mock/sample.json"));
        let config = Config::from_args();

        let mut rt = Runtime::new().unwrap();
        //let creation_packs = rt.block_on(load_tinytap_packs(path));

    }
}