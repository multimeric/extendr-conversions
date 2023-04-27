pub struct Robj;
pub trait FromRobj<Marker>: Sized {
    fn from_robj(robj: &Robj) -> Result<Self, &'static str>;
}
pub struct CoreConversion;
impl FromRobj<CoreConversion> for bool {
    fn from_robj(_robj: &Robj) -> Result<Self, &'static str> {
        return Ok(true);
    }
}
