pub struct Guess {
    value: i32,
}

#[derive(Debug)]
pub struct ErrorResponse {
    message: String,
}

impl Guess {
    pub fn new(input: String) -> Result<Guess, ErrorResponse> {

        let guess: i32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                return Err(ErrorResponse{message: format!("Invalid input, only numbers, got {}", input)})
            },
        };

        if guess < 1 || guess > 100 {
            return Err(ErrorResponse{message: format!("Guess value must be between 1 and 100, got {}.",guess)})
        }

        Ok(Guess { value: guess })
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}
