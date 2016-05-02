
pub trait GenericObject {
    //fn key_reader(&self, key: &str);
    fn get_type(&self) -> &str;
    fn get_position(&self) -> (f32,f32,f32);
    fn get_name(&self) -> &str;
}
