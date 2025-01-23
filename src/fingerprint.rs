pub trait Fingerprint {
    fn to_spaced_hex(&self) -> String;
}

impl Fingerprint for String {
    fn to_spaced_hex(&self) -> String {
        self.chars()
            .collect::<Vec<_>>()
            .chunks(4)
            .map(|chunk| chunk.iter().collect::<String>())
            .collect::<Vec<_>>()
            .join(" ")
    }
}
