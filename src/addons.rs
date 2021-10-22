use surf::Client;

pub struct AddonsService {
  client: Client,
}

impl AddonsService {
  pub fn new(client: &Client) -> Self {
    Self {
      client: client.clone(),
    }
  }

  pub fn execute_addon(&self) {}

  pub fn get_addon_details(&self) {}

  pub fn get_addons(&self) {}

  pub fn set_addon_enabled(&self) {}
}
