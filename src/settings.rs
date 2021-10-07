use surf::Client;

pub struct SettingsService {
  client: Client,
}

impl SettingsService {
  pub fn new(client: *const Client) -> Self {
    Self {
      client: unsafe { (*client).clone() },
    }
  }
}
