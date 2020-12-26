mod implementations;

#[cfg(test)]
mod tests {
    use super::*;
    use implementations::location::Location;
    #[test]
    fn location_init() {
        let _loc2 = Location::new(1, 2);
    }
}
