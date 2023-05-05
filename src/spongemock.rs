use tide::prelude::json;
use tide::{Request, Response};

pub async fn get_sponge_mock(req: Request<()>) -> tide::Result {
    let input = req
        .param("text")
        .unwrap_or("missing text parameter")
        .replace("%20", " ");

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

    Ok(Response::from(json!({ "data": result })))
}
