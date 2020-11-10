pub mod route;

pub fn run(route: &str) {
    route::api::dir_exists(route);
}