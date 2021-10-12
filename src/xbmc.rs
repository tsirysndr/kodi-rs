use surf::Client;

pub struct XbmcService {
  client: Client,
}

impl XbmcService {
  pub fn new(client: &Client) -> Self {
    Self {
      client: client.clone() ,
    }
  }
}
