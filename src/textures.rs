use surf::Client;

pub struct TexturesService {
  client: Client,
}

impl TexturesService {
  pub fn new(client: *const Client) -> Self {
    Self {
      client: unsafe { (*client).clone() },
    }
  }
}
