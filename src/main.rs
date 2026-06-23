mod apps;
use apps::collisions_app::collision_app;
// use apps::fox_app::fox_app;
// use apps::shapes_app::shapes_app;

fn main() {
    let mut app = collision_app();
    app.run();
}
