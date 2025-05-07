use authentication::User;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let new_user = User {
        username: "fred".to_string(),
        password: "ReallySecret".to_string(),
        role: authentication::LoginRole::Admin,
    };

    let client = reqwest::Client::new();
    let res = client
        .post("http://localhost:3001/add_user")
        .json(&new_user)
        .send()
        .await?;
    println!("Response: {:?}", res);
    Ok(())
}
