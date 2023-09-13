use rand::distributions::Alphanumeric;
use rand::Rng;
use crate::spotify::spotify_error::SpotifyError;

pub const SPOTIFY_AUTH_URL: &str = "https://accounts.spotify.com/authorize";
pub const SPOTIFY_TOKEN_URL: &str = "https://accounts.spotify.com/api/token";

pub fn generate_random_string(length: usize) -> String {
    rand::thread_rng().sample_iter(Alphanumeric).take(length).collect()
}

pub type SpotifyResult<T, E = SpotifyError> = Result<T, E>;