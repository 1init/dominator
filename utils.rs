
// this will hold the FETCH url also

use lazy_static::lazy_static;
use dominator::class;

lazy_static! {
  pub static ref CT_MAIN_OUTTR: String = class! {
    .style("border", "red 2px solid").style("height", "200vh")
    .style("width", "100%")
  };


  pub static ref AT_LINE_FLD: String = class! {
    .style("display", "flex").style("flex-direction", "row")
  };
}