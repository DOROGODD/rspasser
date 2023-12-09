# RsPasser
> Как решить инвизибл капчу и не выстрелить себе в лицо.


```rust

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let token = RsPasser::new().solve_captcha("<URL>".to_string()).await;

    Ok(())
}
```