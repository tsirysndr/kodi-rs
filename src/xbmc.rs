use surf::Client;

pub struct XbmcService {
  client: Client,
}

impl XbmcService {
  pub fn new(client: &Client) -> Self {
    Self {
      client: client.clone(),
    }
  }

  pub fn get_info_booleans(&self) {}

  pub fn get_info_labels(&self) {}
}
