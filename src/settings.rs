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

  pub fn get_categories(&self) {}

  pub fn get_sections(&self) {}

  pub fn get_setting_value(&self) {}

  pub fn get_settings(&self) {}

  pub fn reset_setting_value(&self) {}

  pub fn set_setting_value(&self) {}
}
