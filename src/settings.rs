use surf::Client;

pub struct SettingsService {
  client: Client,
}

impl SettingsService {
  pub fn new(client: &Client) -> Self {
    Self {
      client: client.clone(),
    }
  }
}
