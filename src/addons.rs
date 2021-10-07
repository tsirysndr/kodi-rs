use surf::Client;

pub struct AddonsService {
  client: Client,
}

impl AddonsService {
  pub fn new(client: *const Client) -> Self {
    Self {
      client: unsafe { (*client).clone() },
    }
  }
}
