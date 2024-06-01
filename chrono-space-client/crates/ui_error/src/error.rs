#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error(transparent)]
    GlooNet(#[from] gloo_net::Error),
}
