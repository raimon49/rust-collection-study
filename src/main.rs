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

        let mut byte_vec = b"Misssssssissssippi".to_vec();
        byte_vec.dedup(); // 連続している「s」をドロップ
        assert_eq!(&byte_vec, b"Misisipi");

        byte_vec = b"Misssssssissssippi".to_vec();
        let mut seen = HashSet::new();
        byte_vec.retain(|r| seen.insert(*r)); // HashSetに入れて「s」の重複を完全に排除する
        assert_eq!(&byte_vec, b"Misp");

        // すべてのスライスをつなげた新しいベクタを返す
        assert_eq!([[1, 2], [3, 4], [5, 6]].concat(), vec![1, 2, 3, 4, 5, 6]);
        // スライスとスライスを連結する間にseparatorのコピーを挟む
        assert_eq!([[1, 2], [3, 4], [5, 6]].join(&0), vec![1, 2, 0, 3, 4, 0, 5, 6]);

        let mut nums = [4, 2, 3, 1];
        nums.sort();
        assert_eq!(nums.to_vec(), vec![1, 2, 3, 4]);
        nums.reverse();
        assert_eq!(nums.to_vec(), vec![4, 3, 2, 1]);

        let contain = "As a Service".contains(&"M");
        assert_eq!(contain, false);

        assert_eq!([1, 2, 3, 4].starts_with(&[1, 2]), true);
        assert_eq!([1, 2, 3, 4].starts_with(&[2, 3]), false);

        assert_eq!([1, 2, 3, 4].ends_with(&[3, 4]), true);
        assert_eq!([1, 2, 3, 4].ends_with(&[1, 2]), false);

        use rand::{thread_rng, Rng};
        let mut rng = thread_rng();
        let x: u32 = rng.gen();
        println!("{}", x);
        println!("{:?}", rng.gen::<(f64, bool)>());

        let mut my_vec = vec![1, 3, 5, 7, 9];
        my_vec.retain(|&val| val <= 4); // 4以下の要素をすべて削除
        println!("{:?}", my_vec);
    }
    {
        use std::collections::BinaryHeap;

        let mut heap = BinaryHeap::from(vec![2, 3, 8, 6, 9, 5, 4]);

        assert_eq!(heap.peek(), Some(&9)); // ヒープ最大値の参照
        assert_eq!(heap.pop(), Some(9));   // ヒープ最大値の取り出し

        // 取り出す度にBinaryHeapでは最大値がコレクションの先頭になる
        assert_eq!(heap.pop(), Some(8));
        assert_eq!(heap.pop(), Some(6));
        assert_eq!(heap.pop(), Some(5));

        // BinaryHeapは.iter()メソッドを持つためループ可能だが、必ずしも最大値順に取り出されることは保証されない
        while let Some(num) = heap.pop() {
            println!("BinaryHeap.pop() loop: {}", num);
        }
    }
    {
        use std::collections::HashMap;

        struct Student {
            name: String
        }

        let target_name = "taro".to_string();
        let mut student_map = HashMap::new();
        if !student_map.contains_key(&target_name) {
            student_map.insert(target_name.clone(), Student{name: target_name});
        }

        // "taro"は厳密にはStringではないが、StringがBorrow<&str>を実装してるためcontains_key()の引数にできる
        assert!(student_map.contains_key("taro"));
    }
}
