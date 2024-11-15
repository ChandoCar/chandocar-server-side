use actix_web::{get, web, App, HttpServer};
use database_access;
use dotenvy::dotenv;

mod controllers;
#[cfg(test)]
mod test_utils;
mod validation;

const ADDRS: (&str, u16) = {
    #[cfg(debug_assertions)]
    {
        ("127.0.0.1", 8000)
    }

    #[cfg(not(debug_assertions))]
    {
        ("0.0.0.0", 8000)
    }
};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    check_for_env_vars();

    database_access::init_db_pool().await.unwrap();

    HttpServer::new(|| App::new().service(web::scope("/api/v1/chadocar").service(hello_world)))
        .bind(ADDRS)?
        .run()
        .await
}

fn check_for_env_vars() {
    let env_vars_needed = vec!["DATABASE_URL"];

    for env_var in env_vars_needed {
        if std::env::var(env_var).is_err() {
            panic!("{} environment variable not set", env_var);
        }
    }
}

#[get("/hello_world")]
async fn hello_world() -> String {
    "Hello World".to_string()
}

#[cfg(test)]
mod test {
    use crate::{hello_world, test_utils};
    use actix_web::http::StatusCode;
    use actix_web::{test, App};

    #[tokio::test]
    async fn test_hello_world() {
        let app = test::init_service(App::new().service(hello_world)).await;

        let request = test::TestRequest::get().uri("/hello_world").to_request();

        let response = test::call_service(&app, request).await;

        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(
            test_utils::response_to_string(response).unwrap(),
            "Hello World"
        );
    }
}
