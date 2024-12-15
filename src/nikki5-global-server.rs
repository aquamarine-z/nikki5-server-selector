use crate::utils::{load_config, ServerType};

mod utils;


fn main() {
        load_config();
    utils::select_and_open(&ServerType::GLOBAL);
    return;
}
