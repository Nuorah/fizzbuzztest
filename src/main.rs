use actix_web::{get, web, App, HttpServer, Result};
use serde::Deserialize;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use actix_web::{App, HttpServer};

    HttpServer::new(|| App::new().service(index))
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

fn fizzbuzz(int1: u32, int2: u32, limit: u32, str1: &str, str2: &str) -> Vec<String> {
    (1..limit+1).map(|i| {
        match (i%int1, i%int2) {
            (0, 0) => str1.to_owned() + &str2,
            (0, _) => str1.to_owned(),
            (_, 0) => str2.to_owned(),
            (_, _) => i.to_string()
        }
    })
        .collect()
}

#[get("/")]
async fn index(fizzbuzzparams: web::Query<FizzBuzzParams>) -> Result<web::Json<Vec<String>>> {
    Ok(web::Json(
        fizzbuzz(
            fizzbuzzparams.int1,
            fizzbuzzparams.int2,
            fizzbuzzparams.limit,
            &fizzbuzzparams.str1,
            &fizzbuzzparams.str2)
    ))
}

#[cfg(test)]
mod tests {
    todo!();
}