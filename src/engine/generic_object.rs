pub trait GenericObject {
    //fn key_reader(&self, key: &str);
    fn get_type(&self) -> String;
    fn get_position(&self) -> (f64,f64,f64);
    fn get_name(&self) -> String;
}
