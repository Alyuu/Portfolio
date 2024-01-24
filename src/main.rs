use axum::{
    routing::get, 
    response::IntoResponse,
    Router
};
use askama::Template;
use tower_http::services::ServeDir;

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .route("/", get(base))
        .route("/home", get(home))
        .route("/experience", get(experience))
        .route("/projects", get(projects))
        .route("/blog", get(blog))
        .route("/contact", get(contact))
        .nest_service("/assets", ServeDir::new("assets"));

    Ok(router.into())
}

#[derive(Template)]
#[template(path = "base.html")]
struct BaseTemplate {
    title: String,
}

async fn base() -> impl IntoResponse {
    BaseTemplate { title : String::from("Alejandro Yuste") }
}

#[derive(Template)]
#[template(path = "home.html")]
struct HomeTemplate;

async fn home() -> impl IntoResponse {
    HomeTemplate
}

#[derive(Template)]
#[template(path = "experience.html")]
struct ExperienceTemplate;

async fn experience() -> impl IntoResponse {
    ExperienceTemplate
}

#[derive(Template)]
#[template(path = "projects.html")]
struct ProjectsTemplate;

async fn projects() -> impl IntoResponse {
    ProjectsTemplate
}

#[derive(Template)]
#[template(path = "blog.html")]
struct BlogTemplate;

async fn blog() -> impl IntoResponse {
    BlogTemplate
}

#[derive(Template)]
#[template(path = "contact.html")]
struct ContactTemplate;

async fn contact() -> impl IntoResponse {
    ContactTemplate
}
