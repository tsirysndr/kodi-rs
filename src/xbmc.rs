use surf::Client;

pub struct XbmcService {
  client: Client,
}

impl XbmcService {
  pub fn new(client: *const Client) -> Self {
    Self {
      client: unsafe { (*client).clone() },
    }
  }
}
