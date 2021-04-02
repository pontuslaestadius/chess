#[derive(Clone, Copy, PartialEq, Debug)]
pub struct OptSq {
    pub digit: Option<usize>,
    pub letter: Option<usize>,
}

impl OptSq {
    pub fn new() -> Self {
        OptSq {
            digit: None,
            letter: None,
        }
    }
    #[allow(dead_code)]
    pub fn overwrite(&mut self, other: OptSq) {
        #[cfg(test)]
        println!("[place/optsq]: {:?} ", other);
        if let Some(digit) = other.digit {
            self.digit = Some(digit);
        }
        if let Some(letter) = other.letter {
            self.letter = Some(letter);
        }
    }
}
