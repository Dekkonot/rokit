mod discovery;
mod id_or_spec;
mod prompts;
mod sources;
mod tracing;

pub use self::discovery::{
    discover_aftman_manifest_dir, discover_aftman_manifest_dirs, discover_closest_tool_spec,
};
pub use self::id_or_spec::ToolIdOrSpec;
pub use self::prompts::prompt_for_install_trust;
pub use self::sources::github_tool_source;
pub use self::tracing::init as init_tracing;
