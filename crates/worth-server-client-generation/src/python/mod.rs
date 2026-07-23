mod protocol;
mod remote_client;

pub use protocol::render_python_product_protocol;
pub use remote_client::render_python_remote_client;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthServerPythonClientPackage {
    protocol_module: String,
    remote_client_module: String,
}

impl WorthServerPythonClientPackage {
    pub(crate) fn new(protocol_module: String, remote_client_module: String) -> Self {
        Self {
            protocol_module,
            remote_client_module,
        }
    }

    pub fn protocol_module(&self) -> &str {
        &self.protocol_module
    }

    pub fn remote_client_module(&self) -> &str {
        &self.remote_client_module
    }
}
