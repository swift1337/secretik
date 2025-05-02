use crate::crypt::*;

#[test]
fn test_encrypt() {
    for tt in vec![
        (
            "helloThereAbc123",
            "Lorem ipsum dolor sit amet, consectetur adipiscing elit.",
        ),
        (
            "randomPassword123!",
            "This is a multiline\ntext example that spans\nmultiple lines.",
        ),
        (
            "yetAnotherPass789#",
            "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about",
        ),
        (
            "complexPass!@#123",
            "Here is another example\nof a multiline text\nwith more lines.",
        ),
        (
            "ethKeyPass123",
            "0xe0f09faa90fb670dbbce4894319408c8847c8623aea374f6684c38bc96710c95",
        ),
        (
            "seedPhrasePass456",
            "legal winner thank year wave sausage worth useful legal winner thank yellow",
        ),
    ] {
        let (password, content) = tt;

        let encrypted = encrypt(content.as_bytes(), password).unwrap();
        println!("[{}, {}] -> {}", password, content, encrypted.to_string());
    }
}
