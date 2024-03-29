use crate::version_history::LAST_TEMPLATE_VERSION;

pub enum RepoVersion {
    Master,
    Tag(String),
}

impl RepoVersion {
    pub fn url(&self) -> String {
        match self {
            RepoVersion::Master => {
                "https://github.com/multiversx/mx-sdk-rs/archive/refs/heads/master.zip".to_string()
            },
            RepoVersion::Tag(tag) => {
                format!("https://github.com/multiversx/mx-sdk-rs/archive/refs/tags/v{tag}.zip")
            },
        }
    }

    pub fn temp_dir_name(&self) -> String {
        match self {
            RepoVersion::Master => "mx-sdk-rs-master".to_string(),
            RepoVersion::Tag(tag) => {
                format!("mx-sdk-rs-{tag}")
            },
        }
    }

    pub fn get_tag(&self) -> String {
        match self {
            RepoVersion::Master => LAST_TEMPLATE_VERSION.to_string(),
            RepoVersion::Tag(tag) => tag.to_string(),
        }
    }
}
