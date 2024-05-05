#[macro_export]
macro_rules! read_modify_write {
    ($fn_name: ident, $register: ident, $setting_type: ty) => {
        pub fn $fn_name(&mut self, value: $setting_type) {
            unsafe {
                let read = ptr::read_volatile(&(*self.registers).$register);
                ptr::write_volatile(&mut (*self.registers).$register, value.modify(read));
            }
        }
    };
}
