use yagui::App;

fn main() {
    let app = App::from_yaml(include_str!("app.yml"));
    app.unwrap().run();
}
