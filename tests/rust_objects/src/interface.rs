/* generated by rust_qt_binding_generator */
#![allow(unknown_lints)]
#![allow(mutex_atomic, needless_pass_by_value)]
use libc::{c_int, c_void, uint8_t, uint16_t};
use std::slice;

use std::sync::{Arc, Mutex};
use std::ptr::null;

use implementation::*;


#[repr(C)]
pub struct QString {
    data: *const uint8_t,
    len: c_int,
}

#[repr(C)]
pub struct QStringIn {
    data: *const uint16_t,
    len: c_int,
}

impl QStringIn {
    fn convert(&self) -> String {
        let data = unsafe { slice::from_raw_parts(self.data, self.len as usize) };
        String::from_utf16_lossy(data)
    }
}

impl<'a> From<&'a str> for QString {
    fn from(string: &'a str) -> QString {
        QString {
            len: string.len() as c_int,
            data: string.as_ptr(),
        }
    }
}

impl<'a> From<&'a String> for QString {
    fn from(string: &'a String) -> QString {
        QString {
            len: string.len() as c_int,
            data: string.as_ptr(),
        }
    }
}

pub struct GroupQObject {}

#[derive(Clone)]
pub struct GroupEmitter {
    qobject: Arc<Mutex<*const GroupQObject>>,
}

unsafe impl Send for GroupEmitter {}

impl GroupEmitter {
    fn clear(&self) {
        *self.qobject.lock().unwrap() = null();
    }
}

pub trait GroupTrait {
    fn create(emit: GroupEmitter,
        person: Person) -> Self;
    fn emit(&self) -> &GroupEmitter;
    fn person(&self) -> &Person;
    fn get_mut_person(&mut self) -> &mut Person;
}

#[no_mangle]
pub extern "C" fn group_new(
    group: *mut GroupQObject,
    person: *mut PersonQObject,
    object: *mut InnerObjectQObject,
    description_changed: fn(*const InnerObjectQObject),
) -> *mut Group {
    let object_emit = InnerObjectEmitter {
        qobject: Arc::new(Mutex::new(object)),
        description_changed: description_changed,
    };
    let d_object = InnerObject::create(object_emit);
    let person_emit = PersonEmitter {
        qobject: Arc::new(Mutex::new(person)),
    };
    let d_person = Person::create(person_emit,
        d_object);
    let group_emit = GroupEmitter {
        qobject: Arc::new(Mutex::new(group)),
    };
    let d_group = Group::create(group_emit,
        d_person);
    Box::into_raw(Box::new(d_group))
}

#[no_mangle]
pub unsafe extern "C" fn group_free(ptr: *mut Group) {
    Box::from_raw(ptr).emit().clear();
}

#[no_mangle]
pub unsafe extern "C" fn group_person_get(ptr: *mut Group) -> *mut Person {
    (&mut *ptr).get_mut_person()
}

pub struct InnerObjectQObject {}

#[derive(Clone)]
pub struct InnerObjectEmitter {
    qobject: Arc<Mutex<*const InnerObjectQObject>>,
    description_changed: fn(*const InnerObjectQObject),
}

unsafe impl Send for InnerObjectEmitter {}

impl InnerObjectEmitter {
    fn clear(&self) {
        *self.qobject.lock().unwrap() = null();
    }
    pub fn description_changed(&self) {
        let ptr = *self.qobject.lock().unwrap();
        if !ptr.is_null() {
            (self.description_changed)(ptr);
        }
    }
}

pub trait InnerObjectTrait {
    fn create(emit: InnerObjectEmitter) -> Self;
    fn emit(&self) -> &InnerObjectEmitter;
    fn description(&self) -> &str;
    fn set_description(&mut self, value: String);
}

#[no_mangle]
pub extern "C" fn inner_object_new(
    inner_object: *mut InnerObjectQObject,
    description_changed: fn(*const InnerObjectQObject),
) -> *mut InnerObject {
    let inner_object_emit = InnerObjectEmitter {
        qobject: Arc::new(Mutex::new(inner_object)),
        description_changed: description_changed,
    };
    let d_inner_object = InnerObject::create(inner_object_emit);
    Box::into_raw(Box::new(d_inner_object))
}

#[no_mangle]
pub unsafe extern "C" fn inner_object_free(ptr: *mut InnerObject) {
    Box::from_raw(ptr).emit().clear();
}

#[no_mangle]
pub unsafe extern "C" fn inner_object_description_get(
    ptr: *const InnerObject,
    p: *mut c_void,
    set: fn(*mut c_void, QString),
) {
    let data = (&*ptr).description();
    set(p, data.into());
}

#[no_mangle]
pub unsafe extern "C" fn inner_object_description_set(ptr: *mut InnerObject, v: QStringIn) {
    (&mut *ptr).set_description(v.convert());
}

pub struct PersonQObject {}

#[derive(Clone)]
pub struct PersonEmitter {
    qobject: Arc<Mutex<*const PersonQObject>>,
}

unsafe impl Send for PersonEmitter {}

impl PersonEmitter {
    fn clear(&self) {
        *self.qobject.lock().unwrap() = null();
    }
}

pub trait PersonTrait {
    fn create(emit: PersonEmitter,
        object: InnerObject) -> Self;
    fn emit(&self) -> &PersonEmitter;
    fn object(&self) -> &InnerObject;
    fn get_mut_object(&mut self) -> &mut InnerObject;
}

#[no_mangle]
pub extern "C" fn person_new(
    person: *mut PersonQObject,
    object: *mut InnerObjectQObject,
    description_changed: fn(*const InnerObjectQObject),
) -> *mut Person {
    let object_emit = InnerObjectEmitter {
        qobject: Arc::new(Mutex::new(object)),
        description_changed: description_changed,
    };
    let d_object = InnerObject::create(object_emit);
    let person_emit = PersonEmitter {
        qobject: Arc::new(Mutex::new(person)),
    };
    let d_person = Person::create(person_emit,
        d_object);
    Box::into_raw(Box::new(d_person))
}

#[no_mangle]
pub unsafe extern "C" fn person_free(ptr: *mut Person) {
    Box::from_raw(ptr).emit().clear();
}

#[no_mangle]
pub unsafe extern "C" fn person_object_get(ptr: *mut Person) -> *mut InnerObject {
    (&mut *ptr).get_mut_object()
}