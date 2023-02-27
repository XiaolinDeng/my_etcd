#[cfg(test)]
mod tests {
    use my_ectd::storage::Storage;

    #[test]
    fn it_works() {
        assert_eq!(true, true);
    }

    #[test]
    fn set_get() {
        let mut s = Storage::new();
        s.set("foo", "bar");
        let val = s.get("foo");
        assert_eq!(val, "bar");
    }

    #[test]
    fn return_old_val()
    {
        let mut s = Storage::new();
        let old_val = s.set("foo", "bar");
        assert_eq!(old_val, Option::None);
        let old_val = s.set("foo", "bar_2");
        assert_eq!(old_val, Option::Some(std::string::String::from("bar")));
    }

    #[test]
    fn save_load()
    {
        let mut s = Storage::new();
        s.set("foo", "bar");
        s.save();
        s.load();
        let value = s.get("foo");
        assert_eq!(value, "bar");
    }
}