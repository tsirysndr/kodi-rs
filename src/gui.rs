use surf::Client;

pub struct GuiService {
  client: Client,
}

impl GuiService {
  pub fn new(client: &Client) -> Self {
    Self {
      client: client.clone(),
    }
  }

  pub fn activate_window(&self) {}

  pub fn get_properties(&self) {}

  pub fn get_stereoscopic_modes(&self) {}

  pub fn set_fullscreen(&self) {}

  pub fn set_stereoscopic_mode(&self) {}

  pub fn show_notification(&self) {}
}
