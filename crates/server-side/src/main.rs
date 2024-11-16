use actix_web::{web, App, HttpServer};
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
async fn main() {
    dotenv().ok();

    check_for_env_vars();

    let database_pool = database_access::init_db_pool()
        .await
        .expect("Should create database pool");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(database_pool.clone()))
            .service(web::scope("/api/v1/chadocar"))
    })
    .bind(ADDRS)
    .expect("Should bind to address")
    .run()
    .await
    .expect("Should run server");
}

fn check_for_env_vars() {
    let env_vars_needed = vec!["DATABASE_URL"];

    for env_var in env_vars_needed {
        if std::env::var(env_var).is_err() {
            panic!("{} environment variable not set", env_var);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use actix_web::http::StatusCode;
    use actix_web::{test, App};

    #[tokio::test]
    async fn test_hello_world() {
        let app = test::init_service(App::new().service(controllers::example::hello_world)).await;

        let request = test::TestRequest::get().uri("/hello_world").to_request();

        let response = test::call_service(&app, request).await;

        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(
            test_utils::response_to_string(response).unwrap(),
            r#"{"message":"Hello, world!"}"#
        );
    }
}
