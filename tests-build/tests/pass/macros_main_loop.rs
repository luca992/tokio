use tests_build::tokio;

#[tokio::main]
async fn main() -> Result<(), ()> {
    loop {
        if !never() {
            return Ok(());
        }
    }
}

fn never() -> bool {
    web_time::Instant::now() > web_time::Instant::now()
}
