extern crate assert_cli;

#[cfg(test)]
mod tests {
    use assert_cli::Assert;

    #[test]
    fn test_small_input(){
        Assert::main_binary()
            .with_args(&[
                "COF",
                "--metafrost-file",
                "tests/small.metafrost"]).succeeds()
            .stdout().contains("1\t1\n")
            .stdout().contains("1,2\t2\n")
            .stdout().contains("-\t4\n").unwrap();
    }
}
