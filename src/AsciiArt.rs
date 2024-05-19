use crate::Request::*;

pub async fn ascii_art(text: &str) -> Option<String> {
    let ascii_art_api = format!(
        "https://asciified.thelicato.io/api/v2/ascii?text={}&font=Ogre",
        text
    );
    let response = get_request(&ascii_art_api).await;
    if response.is_ok() {
        Some(response.unwrap())
    } else {
        None
    }
}
