#[derive(Debug, serde::Deserialize, serde::Serialize, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[allow(non_camel_case_types)]
pub enum SiUnitPrefix
{
 Q = 30isize,
 R = 27isize,
 Y = 24isize,
 Z = 21isize,
 E = 18isize,
 P = 15isize,
 T = 12isize,
 G = 9isize,
 M = 6isize,
 k = 3isize,
 h = 2isize,
 da = 1isize,
 d = -1isize,
 c = -2isize,
 m = -3isize,
 u = -6isize,
 n = -9isize,
 p = -12isize,
 f = -15isize,
 a = -18isize,
 z = -21isize,
 y = -24isize,
 r = -27isize,
 q = -30isize
}

impl SiUnitPrefix
{
 pub fn as_f64(&self) -> f64
 {
  match self
  {
   SiUnitPrefix::Q => 1e+30f64,
   SiUnitPrefix::R => 1e+27f64,
   SiUnitPrefix::Y => 1e+24f64,
   SiUnitPrefix::Z => 1e+21f64,
   SiUnitPrefix::E => 1e+18f64,
   SiUnitPrefix::P => 1e+15f64,
   SiUnitPrefix::T => 1e+12f64,
   SiUnitPrefix::G => 1e+9f64,
   SiUnitPrefix::M => 1e+6f64,
   SiUnitPrefix::k => 1e+3f64,
   SiUnitPrefix::h => 1e+2f64,
   SiUnitPrefix::da => 1e+1f64,
   SiUnitPrefix::d => 1e-1f64,
   SiUnitPrefix::c => 1e-2f64,
   SiUnitPrefix::m => 1e-3f64,
   SiUnitPrefix::u => 1e-6f64,
   SiUnitPrefix::n => 1e-9f64,
   SiUnitPrefix::p => 1e-12f64,
   SiUnitPrefix::f => 1e-15f64,
   SiUnitPrefix::a => 1e-18f64,
   SiUnitPrefix::z => 1e-21f64,
   SiUnitPrefix::y => 1e-24f64,
   SiUnitPrefix::r => 1e-27f64,
   SiUnitPrefix::q => 1e-30f64
  }
 }

 pub fn as_f32(&self) -> f32
 {
  match self
  {
   SiUnitPrefix::Q => 1e+30f32,
   SiUnitPrefix::R => 1e+27f32,
   SiUnitPrefix::Y => 1e+24f32,
   SiUnitPrefix::Z => 1e+21f32,
   SiUnitPrefix::E => 1e+18f32,
   SiUnitPrefix::P => 1e+15f32,
   SiUnitPrefix::T => 1e+12f32,
   SiUnitPrefix::G => 1e+9f32,
   SiUnitPrefix::M => 1e+6f32,
   SiUnitPrefix::k => 1e+3f32,
   SiUnitPrefix::h => 1e+2f32,
   SiUnitPrefix::da => 1e+1f32,
   SiUnitPrefix::d => 1e-1f32,
   SiUnitPrefix::c => 1e-2f32,
   SiUnitPrefix::m => 1e-3f32,
   SiUnitPrefix::u => 1e-6f32,
   SiUnitPrefix::n => 1e-9f32,
   SiUnitPrefix::p => 1e-12f32,
   SiUnitPrefix::f => 1e-15f32,
   SiUnitPrefix::a => 1e-18f32,
   SiUnitPrefix::z => 1e-21f32,
   SiUnitPrefix::y => 1e-24f32,
   SiUnitPrefix::r => 1e-27f32,
   SiUnitPrefix::q => 1e-30f32
  }
 }

 pub fn as_exp(&self) -> i8
 {
  match self
  {
   SiUnitPrefix::Q => 30,
   SiUnitPrefix::R => 27,
   SiUnitPrefix::Y => 24,
   SiUnitPrefix::Z => 21,
   SiUnitPrefix::E => 18,
   SiUnitPrefix::P => 15,
   SiUnitPrefix::T => 12,
   SiUnitPrefix::G => 9,
   SiUnitPrefix::M => 6,
   SiUnitPrefix::k => 3,
   SiUnitPrefix::h => 2,
   SiUnitPrefix::da => 1,
   SiUnitPrefix::d => -1,
   SiUnitPrefix::c => -2,
   SiUnitPrefix::m => -3,
   SiUnitPrefix::u => -6,
   SiUnitPrefix::n => -9,
   SiUnitPrefix::p => -12,
   SiUnitPrefix::f => -15,
   SiUnitPrefix::a => -18,
   SiUnitPrefix::z => -21,
   SiUnitPrefix::y => -24,
   SiUnitPrefix::r => -27,
   SiUnitPrefix::q => -30
  }
 }

 pub fn parse_from_str(unit_str: &str) -> Option<SiUnitPrefix>
 {
  match unit_str
  {
   v if v.starts_with("Q") => Some(SiUnitPrefix::Q),
   v if v.starts_with("R") => Some(SiUnitPrefix::R),
   v if v.starts_with("Y") => Some(SiUnitPrefix::Y),
   v if v.starts_with("Z") => Some(SiUnitPrefix::Z),
   v if v.starts_with("E") => Some(SiUnitPrefix::E),
   v if v.starts_with("P") => Some(SiUnitPrefix::P),
   v if v.starts_with("T") => Some(SiUnitPrefix::T),
   v if v.starts_with("G") || v.starts_with("㌐") => Some(SiUnitPrefix::G),
   v if v.starts_with("M") || v.starts_with("㍋") => Some(SiUnitPrefix::M),
   v if v.starts_with("k") || v.starts_with("㌔") => Some(SiUnitPrefix::k),
   v if v.starts_with("h") => Some(SiUnitPrefix::h),
   v if v.starts_with("da") => Some(SiUnitPrefix::da),
   v if v.starts_with("d") || v.starts_with("㌥") => Some(SiUnitPrefix::d),
   v if v.starts_with("c") || v.starts_with("㌢") => Some(SiUnitPrefix::c),
   v if v.starts_with("m") || v.starts_with("㍉") => Some(SiUnitPrefix::m),
   v if v.starts_with("u") || v.starts_with("㍃") || v.starts_with("μ") => Some(SiUnitPrefix::u),
   v if v.starts_with("n") || v.starts_with("㌨") => Some(SiUnitPrefix::n),
   v if v.starts_with("p") || v.starts_with("㌰") => Some(SiUnitPrefix::p),
   v if v.starts_with("f") => Some(SiUnitPrefix::f),
   v if v.starts_with("a") => Some(SiUnitPrefix::a),
   v if v.starts_with("z") => Some(SiUnitPrefix::z),
   v if v.starts_with("y") => Some(SiUnitPrefix::y),
   v if v.starts_with("r") => Some(SiUnitPrefix::r),
   v if v.starts_with("q") => Some(SiUnitPrefix::q),
   _ => None
  }
 }
}
