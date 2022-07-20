use text_to_png::{TextPng, TextRenderer};
use worker::*;

mod utils;

#[event(fetch)]
pub async fn main(req: Request, env: Env, _ctx: worker::Context) -> Result<Response> {
    // Optionally, get more helpful error messages written to the console in the case of a panic.
    utils::set_panic_hook();

    let router = Router::new();

    router
        .get_async("/", |req, _| async move {
            if let Some(text) = req.url()?.query() {
                handle_slash(text.into()).await
            } else {
                handle_slash("What's up Cloudflare".into()).await
            }
        })
        .run(req, env)
        .await
}

async fn handle_slash(text: String) -> Result<Response> {
    let renderer = TextRenderer::default();

    let text = if text.len() > 128 {
        "Nope".into()
    } else {
        text
    };

    let text = urlencoding::decode(&text).map_err(|_| worker::Error::BadEncoding)?;

    let text_png: TextPng = renderer.render_text_to_png_data(text.replace("+", " "), 64, "red").unwrap();

    let mut headers = Headers::new();
    headers.set("content-type", "image/png")?;

    Ok(Response::from_bytes(text_png.data)?.with_headers(headers))
}
