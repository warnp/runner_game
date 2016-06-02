
pub trait GenericObject {
    //fn key_reader(&self, key: &str);
    fn get_type(&self) -> String;
    fn get_position(&self) -> (f32,f32,f32);
    fn get_name(&self) -> String;
    fn get_description(&self) -> String;
    fn get_texture_id(&self)->i32;
}

impl <F: ?Sized> GenericObject for Box<F>
    where F: GenericObject
{
    fn get_type(&self) -> String {
        (**self).get_type()
    }

    fn get_position(&self) -> (f32,f32,f32) {
        (**self).get_position()
    }

    fn get_name(&self) -> String {
        (**self).get_name()
    }

    fn get_description(&self) -> String {
        (**self).get_description()
    }
    fn get_texture_id(&self)->i32 {
        (**self).get_texture_id()
    }
}
