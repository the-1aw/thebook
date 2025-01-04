#[macro_export]
macro_rules! simple_vec {
    ( $( $x: expr ), *) => {
        {
        let mut tmp_vec = Vec::new();
        $(
            tmp_vec.push($x);
        )*
        tmp_vec
        }
    };
}

pub trait MacroDemo {
    fn demo_macro() {}
}

#[cfg(test)]
mod tests {

    #[test]
    fn create_vec() {
        let v = vec![1, 2, 3];
        let v2 = simple_vec![1, 2, 3];
        assert_eq!(v, v2);
    }
}
