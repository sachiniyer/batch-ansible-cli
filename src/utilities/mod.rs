pub mod parse;
pub use parse::contents;
pub use parse::unwrap;
pub use parse::unwrap_envs;
pub use parse::unwrap_name;

pub mod args;
pub use args::arg_parse;
pub use args::arg_parse_env;
pub use args::map_files;
pub use args::map_name;
pub use args::map_num;
