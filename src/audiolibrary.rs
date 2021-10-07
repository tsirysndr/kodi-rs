use surf::Client;

pub struct AudioLibraryService {
  client: Client,
}

impl AudioLibraryService {
  pub fn new(client: *const Client) -> Self {
    Self {
      client: unsafe { client.as_ref().unwrap().clone() },
    }
  }
}
