use url::Url;
use crate::spotify::util::SpotifyResult;

#[derive(Debug, PartialEq)]
pub struct SpotifyAuthorizationCallback {
    code: Option<String>,

    error: Option<String>,

    state: String,
}


impl SpotifyAuthorizationCallback {
    #[allow(dead_code)]
    fn new(code: Option<String>, error: Option<String>, state: String) -> Self {
        Self {
            code,
            state,
            error,
        }
    }


    pub async fn convert_into_token(&self, client_id: String, client_secret: String, redirect_url: Url) -> SpotifyResult<String> {
        todo!()
    }
}