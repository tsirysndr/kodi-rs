use surf::Client;

pub struct ProfilesService {
  client: Client,
}

impl ProfilesService {
  pub fn new(client: *const Client) -> Self {
    Self {
      client: unsafe { (*client).clone() },
    }
  }
}
