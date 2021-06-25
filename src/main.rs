fn main() {
    {
        let mut _numbers: Vec<i32> = vec![];
        let words = vec!["step", "on", "no", "pets"];
        let buffer = vec![0u8; 1024];

        // 他のコレクションからベクタに変換
        use std::collections::HashSet;
        let mut my_set = HashSet::new();
        my_set.insert("A Dance With Dragons".to_string());
        my_set.insert("To Kill a Mockingbird".to_string());
        let _converted_my_vec = my_set.into_iter().collect::<Vec<String>>();

        let _second_word = words[1];
        let _my_ref = &buffer[4..12];
        let _my_copy = buffer[4..12].to_vec();

        if let Some(item) = words.first() {
            println!("We got one! {}", item);
        }

        if let Some(item) = words.last() {
            println!("We got one! {}", item);
        }
    }
}
