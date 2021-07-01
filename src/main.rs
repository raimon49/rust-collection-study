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

        assert_eq!(words.get(2), Some(&"no"));
        assert_eq!(words.get(4), None);

        let mut slice = [0, 1, 2, 3];
        {
            let last = slice.last_mut().unwrap(); // &mut i32型として取得
            assert_eq!(*last, 3);
            // 参照を書き換える
            *last = 100;
            assert_eq!(slice, [0, 1, 2, 100]);
        }

        let v = [1, 2, 3, 4, 5, 6, 7, 8, 9];
        // to_vec()は例外で値をコピーして新しいベクタを返す
        assert_eq!(v.to_vec(),
                   vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
        assert_eq!(v[0..6].to_vec(),
                   vec![1, 2, 3, 4, 5, 6]);

        assert_eq!(v.len(), 9);
        assert!(!v[0..1].is_empty());

        // Vec::with_capacity()で確保したバッファより大きくはならないためvec.capacity() >= vec.len() は常に成り立つ
        assert!(v.to_vec().capacity() >= v.to_vec().len());

        let mut two_vec = vec!["a", "b"];
        assert_eq!(two_vec.pop(), Some("b"));
        assert_eq!(two_vec.pop(), Some("a"));
        assert_eq!(two_vec.pop(), None);

        two_vec.push("1");
        two_vec.push("2");
        two_vec.push("3");
        assert_eq!(two_vec.len(), 3);
        two_vec.truncate(2); // index 2..以降を全てドロップ
        assert_eq!(two_vec.len(), 2);
    }
}
