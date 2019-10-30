use failure::Context;
pub use failure::{bail, err_msg, Fail, Fallible, ResultExt};
pub use lazy_static::lazy_static;
pub use serde_derive::{Deserialize, Serialize};

pub trait FailExt {
    fn downcast_ctx<T: Fail>(&self) -> Option<&T>;
}

impl FailExt for dyn Fail {
    fn downcast_ctx<T: Fail>(&self) -> Option<&T> {
        if let Some(res) = self.downcast_ref::<T>() {
            Some(res)
        } else if let Some(ctx) = self.downcast_ref::<Context<T>>() {
            Some(ctx.get_context())
        } else {
            None
        }
    }
}
