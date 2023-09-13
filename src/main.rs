use crate::spotify::{
    spotify_auth::SpotifyAuth,
    spotify_scope::SpotifyClientScope,
};

mod spotify;

fn main() {
    let authorization_url = SpotifyAuth::from_env_variable("code".into(),
                                                           vec![SpotifyClientScope::UserReadPrivate]);
    let url = authorization_url.authorize_url().unwrap();
    open::that(url).unwrap();
}
