use crate::{private, GLScalar};

pub mod vecn;

pub trait Parameter2: private::Seal {}

impl<T: GLScalar> Parameter2 for T {}

pub trait Parameter3: private::Seal {}
impl<T: Parameter2> Parameter3 for T {}

pub trait Parameter4: private::Seal {}
impl<T: Parameter3> Parameter4 for T {}