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
        // ARRANGE
        // Given a password and a content
        let (password, content) = tt;

        // ACT
        // Encrypt the content
        let encrypted_result = encrypt(content.as_bytes(), password);

        // ASSERT
        // The encryption should be successful
        assert!(encrypted_result.is_ok());

        // Get the encrypted data
        let encrypted = encrypted_result.unwrap();

        let as_base64 = encrypted.to_base64();
        println!("[{}, {}] -> {}", password, content, as_base64);

        // ACT
        // Decrypt
        let decrypted_result = decrypt(&as_base64, password);

        // ASSERT
        assert!(decrypted_result.is_ok());

        let decrypted = decrypted_result.unwrap();
        let decrypted_str = String::from_utf8(decrypted).unwrap();

        assert_eq!(content, decrypted_str);
    }
}
