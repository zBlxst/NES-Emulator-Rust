
// We treat possible errors instead of letting the possibility to panic

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("CPU Error: {0}")]
    CpuError(String),

    //example to wrap other crates' errors
    #[error(transparent)]
    WindowError(#[from] sdl2::video::WindowBuildError),
}