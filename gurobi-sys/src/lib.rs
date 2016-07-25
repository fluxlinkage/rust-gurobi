#![allow(improper_ctypes)]

extern crate libc;

pub use libc::{c_int, c_double, c_schar};

#[repr(C)]
pub struct GRBenv;

#[repr(C)]
pub struct GRBmodel;

extern "C" {
  pub fn GRBloadenv(envP: *mut *mut GRBenv,
                    logfilename: *const c_schar)
                    -> c_int;
  pub fn GRBgeterrormsg(env: *mut GRBenv) -> *const c_schar;
  pub fn GRBfreeenv(env: *mut GRBenv);
  pub fn GRBnewmodel(env: *mut GRBenv,
                     modelP: *mut *mut GRBmodel,
                     Pname: *const c_schar,
                     numvars: c_int,
                     obj: *mut c_double,
                     lb: *mut c_double,
                     ub: *mut c_double,
                     vtype: *mut c_schar,
                     varnames: *mut *mut c_schar)
                     -> c_int;
  pub fn GRBfreemodel(model: *mut GRBmodel) -> c_int;
  pub fn GRBoptimize(model: *mut GRBmodel) -> c_int;
  pub fn GRBgetintattr(model: *mut GRBmodel,
                       attrname: *const c_schar,
                       valueP: *mut c_int)
                       -> c_int;
  pub fn GRBwrite(model: *mut GRBmodel, filename: *const c_schar) -> c_int;

  pub fn GRBaddvar(model: *mut GRBmodel,
                   numnz: c_int,
                   vind: *mut c_int,
                   vval: *mut c_double,
                   obj: f64,
                   lb: f64,
                   ub: f64,
                   vtype: c_schar,
                   name: *const c_schar)
                   -> c_int;

  pub fn GRBupdatemodel(model: *mut GRBmodel) -> c_int;
  pub fn GRBcopymodel(model: *mut GRBmodel) -> *mut GRBmodel;

  pub fn GRBgetstrparam(env: *mut GRBenv,
                        paramname: *const c_schar,
                        value: *mut c_schar)
                        -> c_int;
}
