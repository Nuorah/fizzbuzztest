use actix_web::{get, web, App, HttpServer, Result};
use serde::Deserialize;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use actix_web::{App, HttpServer};

    HttpServer::new(|| App::new().service(fizzbuzz))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}

#[derive(Deserialize)]
struct FizzBuzzParams {
    int1: u32,
    int2: u32,
    limit: u32,
    str1: String,
    str2: String,
}

fn fizzbuzz_generator(int1: u32, int2: u32, limit: u32, str1: &str, str2: &str) -> Vec<String> {
    ((1..limit+1).map(|i| {
                match (i.checked_rem(int1), i.checked_rem(int2)) {
                    (Some(0), Some(0)) => str1.to_owned() + &str2,
                    (Some(0), _) => str1.to_owned(),
                    (_, Some(0)) => str2.to_owned(),
                    (_, _) => i.to_string()
                }
            })).collect()
    }

#[get("/fizzbuzz")]
async fn fizzbuzz(fizzbuzzparams: web::Query<FizzBuzzParams>) -> Result<web::Json<Vec<String>>> {
    Ok(web::Json(
        fizzbuzz_generator(
            fizzbuzzparams.int1,
            fizzbuzzparams.int2,
            fizzbuzzparams.limit,
            &fizzbuzzparams.str1,
            &fizzbuzzparams.str2)
    ))
}

#[cfg(test)]
mod tests {
    use actix_web::{http::header::ContentType, test, web, App, body};
    use actix_web::rt::pin;

    use super::*;

    #[test]
    async fn test_fizzbuzz_generator() {
        let result = vec![
            "1", "2", "fizz", "4", "buzz", "fizz", "7", "8", "fizz", "buzz",
            "11", "fizz", "13", "14", "fizzbuzz", "16", "17", "fizz", "19", "buzz"
        ];
        assert_eq!(result, fizzbuzz_generator(3, 5, 20, "fizz", "buzz"))
    }

    #[actix_web::test]
    async fn test_fizzbuzz_get() {
        let app = test::init_service(App::new().service(fizzbuzz)).await;
        let req = test::TestRequest::get()
            .uri("/?int1=3&int2=5&limit=15&str1=fish&str2=bush")
            .to_request();
        let result: Vec<String> = test::call_and_read_body_json(&app, req).await;
        assert_eq!(result, vec![
            "1", "2", "fish", "4", "bush", "fish", "7", "8", "fish", "bush",
            "11", "fish", "13", "14", "fishbush"])
    }

    #[actix_web::test]
    async fn test_fizzbuzz_missing_param() {
        let app = test::init_service(App::new().service(fizzbuzz)).await;
        let req = test::TestRequest::get()
            .uri("/fizzbuzz/?int1=3&int2=5&limit=15&str2=bush")
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_client_error())
    }

    #[actix_web::test]
    async fn test_fizzbuzz_invalid_param() {
        let app = test::init_service(App::new().service(fizzbuzz)).await;
        let req = test::TestRequest::get()
            .uri("/fizzbuzz/?int1=fizz&int2=5&limit=15&str1=fizz&str2=bush")
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_client_error())
    }
}