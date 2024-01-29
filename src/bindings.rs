// Generated by `wit-bindgen` 0.16.0. DO NOT EDIT!
#[derive(Clone)]
pub struct Options {
  pub environment: wit_bindgen::rt::vec::Vec::<(wit_bindgen::rt::string::String,wit_bindgen::rt::string::String,)>,
}
impl ::core::fmt::Debug for Options {
  fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
    f.debug_struct("Options").field("environment", &self.environment).finish()
  }
}
const _: () = {
  
  #[doc(hidden)]
  #[export_name = "transform"]
  #[allow(non_snake_case)]
  unsafe extern "C" fn __export_transform(arg0: i32,arg1: i32,arg2: i32,arg3: i32,) -> i32 {
    #[allow(unused_imports)]
    use wit_bindgen::rt::{alloc, vec::Vec, string::String};
    
    // Before executing any other code, use this function to run all static
    // constructors, if they have not yet been run. This is a hack required
    // to work around wasi-libc ctors calling import functions to initialize
    // the environment.
    //
    // This functionality will be removed once rust 1.69.0 is stable, at which
    // point wasi-libc will no longer have this behavior.
    //
    // See
    // https://github.com/bytecodealliance/preview2-prototyping/issues/99
    // for more details.
    #[cfg(target_arch="wasm32")]
    wit_bindgen::rt::run_ctors_once();
    
    let len0 = arg1 as usize;
    let base7 = arg2;
    let len7 = arg3;
    let mut result7 = Vec::with_capacity(len7 as usize);
    for i in 0..len7 {
      let base = base7 + i * 16;
      let e7 = {
        let l1 = *((base + 0) as *const i32);
        let l2 = *((base + 4) as *const i32);
        let len3 = l2 as usize;
        let bytes3 = Vec::from_raw_parts(l1 as *mut _, len3, len3);
        let l4 = *((base + 8) as *const i32);
        let l5 = *((base + 12) as *const i32);
        let len6 = l5 as usize;
        let bytes6 = Vec::from_raw_parts(l4 as *mut _, len6, len6);
        
        (wit_bindgen::rt::string_lift(bytes3), wit_bindgen::rt::string_lift(bytes6))
      };
      result7.push(e7);
    }
    wit_bindgen::rt::dealloc(base7, (len7 as usize) * 16, 4);
    let result8 = <_GuestImpl as Guest>::transform(Vec::from_raw_parts(arg0 as *mut _, len0, len0), Options{
      environment: result7,
    });
    let ptr9 = _RET_AREA.0.as_mut_ptr() as i32;
    match result8 {
      Ok(e) => { {
        *((ptr9 + 0) as *mut u8) = (0i32) as u8;
        let vec10 = (e).into_boxed_slice();
        let ptr10 = vec10.as_ptr() as i32;
        let len10 = vec10.len() as i32;
        ::core::mem::forget(vec10);
        *((ptr9 + 8) as *mut i32) = len10;
        *((ptr9 + 4) as *mut i32) = ptr10;
      } },
      Err(e) => { {
        *((ptr9 + 0) as *mut u8) = (1i32) as u8;
        let vec11 = (e.into_bytes()).into_boxed_slice();
        let ptr11 = vec11.as_ptr() as i32;
        let len11 = vec11.len() as i32;
        ::core::mem::forget(vec11);
        *((ptr9 + 8) as *mut i32) = len11;
        *((ptr9 + 4) as *mut i32) = ptr11;
      } },
    };ptr9
  }
  
  const _: () = {
    #[doc(hidden)]
    #[export_name = "cabi_post_transform"]
    #[allow(non_snake_case)]
    unsafe extern "C" fn __post_return_transform(arg0: i32,) {
      let l0 = i32::from(*((arg0 + 0) as *const u8));
      match l0 {
        0 => {
          let l1 = *((arg0 + 4) as *const i32);
          let l2 = *((arg0 + 8) as *const i32);
          let base3 = l1;
          let len3 = l2;
          wit_bindgen::rt::dealloc(base3, (len3 as usize) * 1, 1);
        },
        _ => {
          let l4 = *((arg0 + 4) as *const i32);
          let l5 = *((arg0 + 8) as *const i32);
          wit_bindgen::rt::dealloc(l4, (l5) as usize, 1);
        },
      }
    }
  };
};
use super::Component as _GuestImpl;
pub trait Guest {
  fn transform(component: wit_bindgen::rt::vec::Vec::<u8>,options: Options,) -> Result<wit_bindgen::rt::vec::Vec::<u8>,wit_bindgen::rt::string::String>;
}

#[allow(unused_imports)]
use wit_bindgen::rt::{alloc, vec::Vec, string::String};

#[repr(align(4))]
struct _RetArea([u8; 12]);
static mut _RET_AREA: _RetArea = _RetArea([0; 12]);

#[cfg(target_arch = "wasm32")]
#[link_section = "component-type:transformer"]
#[doc(hidden)]
pub static __WIT_BINDGEN_COMPONENT_TYPE: [u8; 250] = [3, 0, 11, 116, 114, 97, 110, 115, 102, 111, 114, 109, 101, 114, 0, 97, 115, 109, 13, 0, 1, 0, 7, 117, 1, 65, 2, 1, 65, 8, 1, 111, 2, 115, 115, 1, 112, 0, 1, 114, 1, 11, 101, 110, 118, 105, 114, 111, 110, 109, 101, 110, 116, 1, 3, 0, 7, 111, 112, 116, 105, 111, 110, 115, 3, 0, 2, 1, 112, 125, 1, 106, 1, 4, 1, 115, 1, 64, 2, 9, 99, 111, 109, 112, 111, 110, 101, 110, 116, 4, 7, 111, 112, 116, 105, 111, 110, 115, 3, 0, 5, 4, 0, 9, 116, 114, 97, 110, 115, 102, 111, 114, 109, 1, 6, 4, 1, 21, 118, 105, 114, 116, 58, 119, 97, 115, 105, 47, 116, 114, 97, 110, 115, 102, 111, 114, 109, 101, 114, 4, 0, 11, 17, 1, 0, 11, 116, 114, 97, 110, 115, 102, 111, 114, 109, 101, 114, 3, 0, 0, 0, 16, 12, 112, 97, 99, 107, 97, 103, 101, 45, 100, 111, 99, 115, 0, 123, 125, 0, 70, 9, 112, 114, 111, 100, 117, 99, 101, 114, 115, 1, 12, 112, 114, 111, 99, 101, 115, 115, 101, 100, 45, 98, 121, 2, 13, 119, 105, 116, 45, 99, 111, 109, 112, 111, 110, 101, 110, 116, 6, 48, 46, 49, 56, 46, 50, 16, 119, 105, 116, 45, 98, 105, 110, 100, 103, 101, 110, 45, 114, 117, 115, 116, 6, 48, 46, 49, 54, 46, 48];

#[inline(never)]
#[doc(hidden)]
#[cfg(target_arch = "wasm32")]
pub fn __link_section() {}
