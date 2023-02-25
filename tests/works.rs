#[cfg(test)]
mod tests {
    use my_ectd::storage::Storage;

    #[test]
    fn it_works() {
        assert_eq!(true,true);
    }

    #[test]
    fn set_get(){
        let mut  s = Storage::new();
        s.set("foo","bar");
        let val = s.get("foo");
        assert_eq!(val,"bar");
    }

    #[test]
    fn save_load()
    {
        let mut  s = Storage::new();
        s.set("foo","bar");
        s.save();
        s.load();
        let value =s.get("foo");
        assert_eq!(value,"bar");

    }
}