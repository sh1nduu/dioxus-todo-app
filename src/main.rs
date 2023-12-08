use dioxus_fullstack::prelude::*;
use log::LevelFilter;
use todolib::Route;

fn main() {
    dioxus_logger::init(LevelFilter::Info).expect("failed to init logger");

    let config = LaunchBuilder::<FullstackRouterConfig<Route>>::router();

    #[cfg(feature = "ssr")]
    let config = config.incremental(
        IncrementalRendererConfig::default().invalidate_after(std::time::Duration::from_secs(120)),
    );

    config.launch();
}
