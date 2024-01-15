/// A service is an application that can be connected to via channels,
/// in order to identify a service it must fulfill the following requirements:
/// - must have a name or common ID that can be used to identify it.
/// - must explicitly define all channels that it wants to host.
pub struct Service {
    /// The name of the service. (e.g. "minecraft")
    pub name: String,
    /// A unique identifier for the service.
    pub id: String,
    /// A description of the service.
    /// This is used to describe what the service is.
    pub description: Option<String>,
}

impl Service {
    /// Creates a new service with the given name and id.
    pub fn new(name: String, id: String) -> Self {
        Self {
            name,
            id,
            description: None,
        }
    }
}
