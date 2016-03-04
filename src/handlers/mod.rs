mod head;
mod patch;
mod options;

pub use self::head::handle_head_method;
pub use self::patch::handle_patch_method;
pub use self::options::handle_options_method;
