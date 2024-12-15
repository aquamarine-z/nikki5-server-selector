use crate::utils::ServerType;
mod utils;
fn main() {
    utils::select_and_open(&ServerType::CHINA);
    return;
}
