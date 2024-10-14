use thiserror::Error;

/// Error that could happen during connecting to LND
///
/// This error may be returned by the `connect()` function if connecting failed.
/// It is currently opaque because it's unclear how the variants will look long-term.
/// Thus you probably only want to display it.
#[derive(Debug, Error)]
pub enum ConnectError {
    #[error("Invalid address: {address}")]
    InvalidAddress {
        address: String,
        #[source]
        error: Box<dyn std::error::Error + Send + Sync>,
    },

    #[error("Failed to connect to {address}")]
    Connect {
        address: String,
        #[source]
        error: tonic::transport::Error,
    },

    #[error("TLS configuration error")]
    TlsConfig(#[source] tonic::transport::Error),

    #[error("Invalid certificate")]
    InvalidCertificate,

    #[error(transparent)]
    Other(#[from] Box<dyn std::error::Error + Send + Sync>),
}
