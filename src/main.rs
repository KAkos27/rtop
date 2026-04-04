use crate::app::App;

mod app;
mod system_information;

fn main() -> color_eyre::Result<()> {
    let mut app: App = App::init();
    return app.run();
}
