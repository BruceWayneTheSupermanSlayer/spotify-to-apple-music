use std::env;

use dotenv::dotenv;
use snafu::ResultExt;
use url::Url;

use crate::spotify::spotify_error::{MissingEnvVariableSnafu, UrlSnafu};
use crate::spotify::spotify_scope::SpotifyClientScope;
use crate::spotify::util::{generate_random_string, SPOTIFY_AUTH_URL, SpotifyResult};

pub struct SpotifyAuth {
    client_id: String,
    client_secret: String,
    #[allow(dead_code)]
    redirect_url: String,
    state: String,
    scope: Vec<SpotifyClientScope>,
    response_type: String,
}


impl Default for SpotifyAuth {
    fn default() -> Self {
        dotenv().ok();

        Self {
            client_id: env::var("CLIENT_ID").context(MissingEnvVariableSnafu).unwrap(),
            client_secret: env::var("CLIENT_SECRET").context(MissingEnvVariableSnafu).unwrap(),
            redirect_url: env::var("REDIRECT_URL").context(MissingEnvVariableSnafu).unwrap(),
            scope: vec![],
            state: generate_random_string(20),
            response_type: "code".to_owned(),
        }
    }
}


impl SpotifyAuth {
    #[allow(dead_code)]
    pub fn new(client_id: String, client_secret: String, redirect_url: String, scope: Vec<SpotifyClientScope>, response_type: String) -> Self {
        Self {
            response_type,
            state: generate_random_string(20),
            client_secret,
            client_id,
            redirect_url,
            scope,
        }
    }

    pub fn from_env_variable(response_type: String, scope: Vec<SpotifyClientScope>) -> Self {
        dotenv().ok();

        Self {
            scope,
            response_type,
            client_id: env::var("CLIENT_ID").context(MissingEnvVariableSnafu).unwrap(),
            client_secret: env::var("CLIENT_SECRET").context(MissingEnvVariableSnafu).unwrap(),
            redirect_url: env::var("REDIRECT_URL").context(MissingEnvVariableSnafu).unwrap(),
            state: generate_random_string(20),
        }
    }


    fn scope_into_strings(&self) -> String {
        self.scope
            .iter()
            .map(|x| x.clone().to_string())
            .collect::<Vec<String>>()
            .join(" ")
    }


    pub fn authorize_url(&self) -> SpotifyResult<String> {
        let mut url = Url::parse(SPOTIFY_AUTH_URL).context(UrlSnafu)?;

        url.query_pairs_mut()
            .append_pair("client_id", &self.client_id)
            .append_pair("client_secret", &self.client_secret)
            .append_pair("response_type", &self.response_type)
            .append_pair("state", &self.state)
            .append_pair("scope", &self.scope_into_strings());

        Ok(url.to_string())
    }
}
