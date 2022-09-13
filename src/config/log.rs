use fast_log::Config;
use fast_log::plugin::file_split::RollingType;
use fast_log::plugin::packer::LogPacker;
use fast_log::consts::LogSize;

pub fn init_logger() {
    fast_log::init(Config::new().console().file_split(
        "target/logs/",
        LogSize::MB(1),
        RollingType::All,
        LogPacker {},
    ))
    .unwrap();
}
