**Fizzbuzz**

Fizzbuzz for LeBonCoin interview.
You need Rust and Cargo.

*To run on localhost:8080:*

`cargo run`

*Build:*

`cargo build --release`
then deploy the binary located in target/release/

*Tests:*

`cargo test`

**Endpoints:**

*URL* : `/fizzbuzz/?`

*Method* : `GET`

*Query params*

```
    int1: int,
    int2: int,
    lim: int,    
    str1: string,
    str2: string
```

**example**

`curl -get "http://localhost:8080/fizzbuzz/?int1=3&int2=5&limit=10&str1=fizz&str2=buzz`
