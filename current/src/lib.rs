pub struct Robj;
pub trait FromRobj: Sized {
    fn from_robj(robj: &Robj) -> Result<Self, &'static str>;
}
impl FromRobj for bool {
    fn from_robj(_robj: &Robj) -> Result<Self, &'static str> {
        Ok(true)
    }
}
