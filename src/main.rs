mod apps;
use apps::collisions_app::collision_app;

fn main() {
    let mut app = collision_app();
    app.run();
}
