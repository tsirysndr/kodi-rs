use surf::Client;

pub struct PlayerService {
  client: Client,
}

impl PlayerService {
  pub fn new(client: *const Client) -> Self {
    Self {
      client: unsafe { (*client).clone() },
    }
  }
}
