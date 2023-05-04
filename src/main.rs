use tide::Request;
// use tide::prelude::*;

#[async_std::main]
async fn main() -> tide::Result<()> {
    let mut app = tide::new();

    app.at("/utils/spongemock/:text").get(sponge_mock);
    app.at("/utils/spongemock").get(sponge_mock);

    app.listen("127.0.0.1:8080").await?;
    println!("server running on localhost:8080");

    Ok(())
}

async fn sponge_mock(req: Request<()>) -> tide::Result<String> {
    println!("got here!");
    let input = req.param("text").unwrap_or("missing text parameter").replace("%20", " ");

    let result: String = input
        .chars()
        .enumerate()
        .map(|(index, ch)| {
            if index % 2 == 0 {
                ch.to_uppercase().to_string()
            } else {
                ch.to_string()
            }
        })
        .collect();

    Ok(format!("{}", result))
}
