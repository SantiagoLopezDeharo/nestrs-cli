pub struct DogDto {
    pub message: String,
}

impl DogDto {
    pub fn new(message: String) -> Self {
        Self { message }
    }
}
