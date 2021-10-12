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
}
