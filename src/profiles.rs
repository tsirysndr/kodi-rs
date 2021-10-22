use surf::Client;

pub struct ProfilesService {
  client: Client,
}

impl ProfilesService {
  pub fn new(client: &Client) -> Self {
    Self {
      client: client.clone(),
    }
  }

  pub fn get_current_profile(&self) {}

  pub fn get_profiles(&self) {}

  pub fn load_profile(&self) {}
}
