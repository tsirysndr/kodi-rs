use surf::Client;

pub struct JsonRpcService {
  client: Client,
}

impl JsonRpcService {
  pub fn new(client: &Client) -> Self {
    Self {
      client: client.clone(),
    }
  }

  pub fn get_configuration(&self) {}

  pub fn introspect(&self) {}

  pub fn notify_all(&self) {}

  pub fn permission(&self) {}

  pub fn ping(&self) {}

  pub fn set_configuration(&self) {}

  pub fn version(&self) {}
}
