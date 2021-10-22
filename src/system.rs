use surf::Client;

pub struct SystemService {
  client: Client,
}

impl SystemService {
  pub fn new(client: &Client) -> Self {
    Self {
      client: client.clone(),
    }
  }

  pub fn eject_optical_drive(&self) {}

  pub fn get_properties(&self) {}

  pub fn hibernate(&self) {}

  pub fn reboot(&self) {}

  pub fn shutdown(&self) {}

  pub fn suspend(&self) {}
}
