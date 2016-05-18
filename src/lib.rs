mod main;

#[cfg(test)]
mod philosphers_tests {
    use main::Philosopher;

    #[test]
    fn it_has_a_constructor() {
        let socrates: Philosopher = Philosopher::new("Socreates (one name, like Cher)", 1, 2);
        assert_eq!(socrates.name, "Socreates (one name, like Cher)");
        assert_eq!(socrates.left, 1);
        assert_eq!(socrates.right, 2);
    }
}
