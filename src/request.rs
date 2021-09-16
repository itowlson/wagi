use std::{collections::HashMap, net::SocketAddr, path::PathBuf};

pub struct RequestContext {
    pub client_addr: SocketAddr,
}

pub struct RequestRouteContext {
    pub entrypoint: String,
}

pub struct RequestGlobalContext {
    pub cache_config_path: PathBuf,
    pub module_cache_dir: PathBuf,
    pub base_log_dir: PathBuf,
    pub default_host: String,
    pub use_tls: bool,
    pub global_env_vars: HashMap<String, String>,
}
