pub fn test() {
    let t = [1, 3, 5, 7, 0, 2, 4, 6, 1, 3, 5, 7]
        .iter()
        .zip([0, 0, 0, 0, 1, 1, 1, 1, 2, 2, 2, 2].iter());

    let collected: Vec<_> = t.collect();
}
