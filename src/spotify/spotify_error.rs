use std::env;

use snafu::Snafu;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub (crate)))]
pub enum SpotifyError {
    #[snafu(display("No value found for {} in the environment variable", source))]
    MissingEnvVariable { source: env::VarError },

    #[snafu(display("Unable to Parse URL : {}", source))]
    UrlError { source: url::ParseError },
}
