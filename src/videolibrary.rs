use surf::Client;

pub struct VideoLibraryService {
  client: Client,
}

impl VideoLibraryService {
  pub fn new(client: *const Client) -> Self {
    Self {
      client: unsafe { (*client).clone() },
    }
  }
}
