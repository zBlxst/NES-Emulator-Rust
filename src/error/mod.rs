
// We treat possible errors instead of letting the possibility to panic

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("CPU Error: {0}")]
    CpuError(String),

    #[error(transparent)]
    WindowError(#[from] sdl2::video::WindowBuildError),

    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error("Rom Error: {0}")]
    RomError(String),
}