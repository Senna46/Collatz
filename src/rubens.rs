pub fn hoge(start: u64, func: impl Fn(u64) -> u64) -> bool {
    let mut value = start;
    let mut count = 0;
    loop {
        value = func(value);
        if value == 1 {
            break;
        }

        count += 1;

        if count > 10 {
            return false;
        };
    }
    true
}
