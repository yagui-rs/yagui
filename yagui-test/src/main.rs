// TODO: #[macro_use] extern crate yagui;

use anyhow::Result;
use yagui::App;

// TODO: #[button_clicked("start")]
fn start_clicked(app: &mut App) {
    dbg!("start_clicked");
}

fn run() -> Result<()> {
    let app = App::from_yaml(include_str!("app.yml"))?;
    // TODO: .events(events![start_clicked])
    app.run()
}

fn main() {
    if let Err(e) = run() {
        eprintln!("{}", e);
        e.chain().skip(1).for_each(|cause| eprintln!("{}", cause));
        std::process::exit(1);
    }
}
