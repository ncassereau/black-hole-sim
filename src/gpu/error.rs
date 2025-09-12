use wgpu::{RequestAdapterError, RequestDeviceError};

#[derive(Debug)]
pub enum GPUBackendError {
    RequestAdapter(RequestAdapterError),
    NoComputeShaders,
    Device(RequestDeviceError),
}

impl std::fmt::Display for GPUBackendError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::RequestAdapter(e) => write!(f, "Failed to request adapter: {}", e),
            Self::NoComputeShaders => write!(f, "Adapter does not support compute shaders"),
            Self::Device(e) => write!(f, "Failed to request device: {}", e),
        }
    }
}

impl std::error::Error for GPUBackendError {}

impl From<RequestAdapterError> for GPUBackendError {
    fn from(err: RequestAdapterError) -> Self {
        Self::RequestAdapter(err)
    }
}

impl From<RequestDeviceError> for GPUBackendError {
    fn from(err: RequestDeviceError) -> Self {
        Self::Device(err)
    }
}
