pub trait GenericObject {
    //fn key_reader(&self, key: &str);
    fn get_type() -> &str,
    fn get_position() -> (f64,f64,f64),
    fn get_name() -> &str,
}
