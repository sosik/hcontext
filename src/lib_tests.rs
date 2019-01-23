#[cfg(test)]
mod test {
    use crate::HContext;

    #[test]
    fn from_parent() {
        let val1 = String::from("aaa");
        let val2 = String::from("bbb");
        let val3 = String::from("ccc");
        let parent = HContext::new()
            .with("aaa", val1.clone())
            .with("bbb", val2.clone());

        assert_eq!(&val1, parent.get::<String>("aaa").unwrap());
        assert_eq!(&val2, parent.get::<String>("bbb").unwrap());

        let child = HContext::from(parent).with("aaa", val3.clone());
        assert_eq!(&val3, child.get::<String>("aaa").unwrap());
        assert_eq!(&val2, child.get::<String>("bbb").unwrap());

        let unwinded_parent = child.unwind().unwrap();

        assert_eq!(&val1, unwinded_parent.get::<String>("aaa").unwrap());
        assert_eq!(&val2, unwinded_parent.get::<String>("bbb").unwrap());
    }

}
