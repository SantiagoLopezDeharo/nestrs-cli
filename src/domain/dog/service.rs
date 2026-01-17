use super::repo::DogRepo;

pub struct DogService {
    _repo: DogRepo,
}

impl DogService {
    pub fn new(repo: DogRepo) -> Self {
        Self { _repo: repo }
    }

    pub fn speak(&self) -> String {
        "Woof".to_string()
    }
}
