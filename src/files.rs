use surf::Client;

pub struct FilesService {
    client: Client,
}

impl FilesService {
  pub fn new(client: *const Client) -> Self {
    Self {
      client: unsafe { (*client).clone() },
    }
  }
}
