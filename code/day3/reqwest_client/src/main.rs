use authentication::User;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    const URL: &str = "http://localhost:3001/all_users";
    let users= reqwest::get(URL)
        .await?
        .json::<Vec<User>>()
        .await?;
    println!("{users:?}");
    Ok(())
}
