use dotenv;
use tide;

mod moderation;
mod spongemock;

#[async_std::main]
async fn main() -> tide::Result<()> {
    dotenv::dotenv().ok();

    let mut app = tide::new();

    app.at("/utils/spongemock/:text")
        .get(spongemock::get_sponge_mock);
    app.at("/utils/spongemock").get(spongemock::get_sponge_mock);

    app.at("/utils/moderation")
        .post(moderation::get_recommendation);

    app.listen("127.0.0.1:8080").await?;
    println!("server running on localhost:8080");

    Ok(())
}
