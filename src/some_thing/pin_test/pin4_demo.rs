use std::marker::PhantomPinned;
use std::pin::Pin;
use std::ptr::NonNull;

struct SelfReferential {
    value: String,
    pointer_to_value: Option<NonNull<String>>,
    _marker: PhantomPinned, // 标记类型，以确保它不会实现 Unpin
}

impl SelfReferential {
    fn new(val: String) -> Self {
        Self {
            value: val,
            pointer_to_value: None,
            _marker: PhantomPinned,
        }
    }

    fn init_pointer(self: Pin<&mut Self>) {
        let self_ref = unsafe { self.get_unchecked_mut() };
        self_ref.pointer_to_value = Some(NonNull::from(&self_ref.value));
    }

    fn get_value<'a>(self: Pin<&'a Self>) -> &'a String {
        unsafe { self.pointer_to_value.as_ref().unwrap().as_ref() }
    }
}

fn main() {
    // 创建一个堆分配的 SelfReferential 实例以确保它不会移动
    let mut my_struct = Box::pin(SelfReferential::new("Hello, Rust!".to_string()));

    // 初始化指针
    SelfReferential::init_pointer(my_struct.as_mut());

    // 确认指针有效
    assert_eq!(SelfReferential::get_value(my_struct.as_ref()), "Hello, Rust!");

    // 输出指针内容
    println!("Value: {}", SelfReferential::get_value(my_struct.as_ref()));
}
