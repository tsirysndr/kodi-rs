use surf::Client;

pub struct ApplicationService {
  client: Client,
}

impl ApplicationService {
  pub fn new(client: &Client) -> Self {
    Self {
      client: client.clone(),
    }
  }

  pub fn get_properties(&self) {}

  pub fn quit(&self) {}

  pub fn set_mute(&self) {}

  pub fn set_volume(&self) {}
}
