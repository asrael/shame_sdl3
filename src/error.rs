use shame::results::VertexAttribFormat;
use shame::{EncodingErrors, Indexing, ShaderStage, ThreadIsAlreadyEncoding};

#[derive(Debug)]
pub enum ShameToSdlError {
    UnsupportedIndexBufferFormat(Indexing),
    UnsupportedTextureFormat(&'static str),
    UnsupportedShaderStage(ShaderStage),
    UnsupportedVertexAttribFormat(VertexAttribFormat),
    MustStartAtIndexZero(&'static str, u32),
    MustHaveConsecutiveIndices(&'static str),
    FragmentStageNeedsAttachmentInteraction,
    RuntimeSurfaceFormatNotProvided,
}

impl core::error::Error for ShameToSdlError {}
impl core::fmt::Display for ShameToSdlError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            ShameToSdlError::UnsupportedIndexBufferFormat(format) => {
                write!(
                    f,
                    "sdlgpu does not support the index buffer format {format:?}"
                )
            }
            ShameToSdlError::UnsupportedShaderStage(mask) => {
                write!(f, "mask `{mask}` contains stages unsupported by sdlgpu")
            }
            ShameToSdlError::UnsupportedTextureFormat(format) => {
                write!(f, "texture format `{format}` is unsupported by sdlgpu")
            }
            ShameToSdlError::UnsupportedVertexAttribFormat(format) => {
                write!(
                    f,
                    "sdlgpu does not support the vertex attribute format `{format:?}`"
                )
            }
            ShameToSdlError::MustStartAtIndexZero(thing, index) => {
                write!(
                    f,
                    "this first {thing} (index={index}) must have index zero in sdlgpu"
                )
            }
            ShameToSdlError::MustHaveConsecutiveIndices(thing) => {
                write!(f, "{thing}s must have consecutive indices in sdlgpu")
            }
            ShameToSdlError::FragmentStageNeedsAttachmentInteraction => {
                write!(
                    f,
                    "sdlgpu requires render pipelines to access at least one color or depth/stencil attachment"
                )
            }
            ShameToSdlError::RuntimeSurfaceFormatNotProvided => {
                write!(
                    f,
                    "trying to convert runtime surface format to sdlgpu, but `surface_format` is not available ('None') in this context"
                )
            }
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    ShameToSdl(#[from] ShameToSdlError),
    #[error(transparent)]
    ThreadIsAlreadyEncoding(#[from] ThreadIsAlreadyEncoding),
    #[error(transparent)]
    Encoding(#[from] EncodingErrors),
}
