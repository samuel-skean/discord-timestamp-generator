use std::sync::LazyLock;

use axum::{http::StatusCode, response::Html};
use chrono::Local;
use minijinja::{Environment, context, path_loader};
use minijinja_autoreload::AutoReloader;

static RELOADER: LazyLock<AutoReloader> = LazyLock::new(|| {
    AutoReloader::new(|notifier| {
        let templates_path = "templates";
        let mut env = Environment::new();
        env.set_loader(path_loader(templates_path));
        notifier.watch_path(templates_path, true);
        Ok(env)
    })
});

pub async fn app() -> (StatusCode, Html<String>) {
    let env = RELOADER
        .acquire_env()
        .expect("Unable to acquire minijinja environment.");
    let Ok(template) = env.get_template("app.html.jinja2") else {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Html(String::from("Internal error getting template")),
        );
    };

    let Ok(rendered) =
        template.render(context!(defaultTime => Local::now().format("%Y-%m-%dT%H:%M").to_string()))
    else {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Html(String::from("Internal error rendering template")),
        );
    };
    return (StatusCode::OK, Html(rendered));
}
