#[cfg(test)]
mod tests
{
 use si_unit_prefix::SiUnitPrefix;

 #[test]
 fn parse()
 {
  assert_eq!(SiUnitPrefix::parse_from_str("Q"), Some(SiUnitPrefix::Q));
  assert_eq!(SiUnitPrefix::parse_from_str("R"), Some(SiUnitPrefix::R));
  assert_eq!(SiUnitPrefix::parse_from_str("Y"), Some(SiUnitPrefix::Y));
  assert_eq!(SiUnitPrefix::parse_from_str("Z"), Some(SiUnitPrefix::Z));
  assert_eq!(SiUnitPrefix::parse_from_str("E"), Some(SiUnitPrefix::E));
  assert_eq!(SiUnitPrefix::parse_from_str("P"), Some(SiUnitPrefix::P));
  assert_eq!(SiUnitPrefix::parse_from_str("T"), Some(SiUnitPrefix::T));
  assert_eq!(SiUnitPrefix::parse_from_str("G"), Some(SiUnitPrefix::G));
  assert_eq!(SiUnitPrefix::parse_from_str("㌐"), Some(SiUnitPrefix::G));
  assert_eq!(SiUnitPrefix::parse_from_str("M"), Some(SiUnitPrefix::M));
  assert_eq!(SiUnitPrefix::parse_from_str("㍋"), Some(SiUnitPrefix::M));
  assert_eq!(SiUnitPrefix::parse_from_str("k"), Some(SiUnitPrefix::k));
  assert_eq!(SiUnitPrefix::parse_from_str("㌔"), Some(SiUnitPrefix::k));
  assert_eq!(SiUnitPrefix::parse_from_str("h"), Some(SiUnitPrefix::h));
  assert_eq!(SiUnitPrefix::parse_from_str("da"), Some(SiUnitPrefix::da));
  assert_eq!(SiUnitPrefix::parse_from_str(""), None);
  assert_eq!(SiUnitPrefix::parse_from_str("d"), Some(SiUnitPrefix::d));
  assert_eq!(SiUnitPrefix::parse_from_str("㌥"), Some(SiUnitPrefix::d));
  assert_eq!(SiUnitPrefix::parse_from_str("c"), Some(SiUnitPrefix::c));
  assert_eq!(SiUnitPrefix::parse_from_str("㌢"), Some(SiUnitPrefix::c));
  assert_eq!(SiUnitPrefix::parse_from_str("m"), Some(SiUnitPrefix::m));
  assert_eq!(SiUnitPrefix::parse_from_str("㍉"), Some(SiUnitPrefix::m));
  assert_eq!(SiUnitPrefix::parse_from_str("u"), Some(SiUnitPrefix::u));
  assert_eq!(SiUnitPrefix::parse_from_str("μ"), Some(SiUnitPrefix::u));
  assert_eq!(SiUnitPrefix::parse_from_str("㍃"), Some(SiUnitPrefix::u));
  assert_eq!(SiUnitPrefix::parse_from_str("n"), Some(SiUnitPrefix::n));
  assert_eq!(SiUnitPrefix::parse_from_str("㌨"), Some(SiUnitPrefix::n));
  assert_eq!(SiUnitPrefix::parse_from_str("p"), Some(SiUnitPrefix::p));
  assert_eq!(SiUnitPrefix::parse_from_str("㌰"), Some(SiUnitPrefix::p));
  assert_eq!(SiUnitPrefix::parse_from_str("f"), Some(SiUnitPrefix::f));
  assert_eq!(SiUnitPrefix::parse_from_str("a"), Some(SiUnitPrefix::a));
  assert_eq!(SiUnitPrefix::parse_from_str("z"), Some(SiUnitPrefix::z));
  assert_eq!(SiUnitPrefix::parse_from_str("y"), Some(SiUnitPrefix::y));
  assert_eq!(SiUnitPrefix::parse_from_str("r"), Some(SiUnitPrefix::r));
  assert_eq!(SiUnitPrefix::parse_from_str("q"), Some(SiUnitPrefix::q));
 }

 #[test]
 fn f64()
 {
  assert_eq!(SiUnitPrefix::Q.as_f64(), 1e+30f64);
  assert_eq!(SiUnitPrefix::R.as_f64(), 1e+27f64);
  assert_eq!(SiUnitPrefix::Y.as_f64(), 1e+24f64);
  assert_eq!(SiUnitPrefix::Z.as_f64(), 1e+21f64);
  assert_eq!(SiUnitPrefix::E.as_f64(), 1e+18f64);
  assert_eq!(SiUnitPrefix::P.as_f64(), 1e+15f64);
  assert_eq!(SiUnitPrefix::T.as_f64(), 1e+12f64);
  assert_eq!(SiUnitPrefix::G.as_f64(), 1e+9f64);
  assert_eq!(SiUnitPrefix::M.as_f64(), 1e+6f64);
  assert_eq!(SiUnitPrefix::k.as_f64(), 1e+3f64);
  assert_eq!(SiUnitPrefix::h.as_f64(), 1e+2f64);
  assert_eq!(SiUnitPrefix::da.as_f64(), 1e+1f64);
  assert_eq!(SiUnitPrefix::d.as_f64(), 1e-1f64);
  assert_eq!(SiUnitPrefix::c.as_f64(), 1e-2f64);
  assert_eq!(SiUnitPrefix::m.as_f64(), 1e-3f64);
  assert_eq!(SiUnitPrefix::u.as_f64(), 1e-6f64);
  assert_eq!(SiUnitPrefix::n.as_f64(), 1e-9f64);
  assert_eq!(SiUnitPrefix::p.as_f64(), 1e-12f64);
  assert_eq!(SiUnitPrefix::f.as_f64(), 1e-15f64);
  assert_eq!(SiUnitPrefix::a.as_f64(), 1e-18f64);
  assert_eq!(SiUnitPrefix::z.as_f64(), 1e-21f64);
  assert_eq!(SiUnitPrefix::y.as_f64(), 1e-24f64);
  assert_eq!(SiUnitPrefix::r.as_f64(), 1e-27f64);
  assert_eq!(SiUnitPrefix::q.as_f64(), 1e-30f64);
 }

 #[test]
 fn f32()
 {
  assert_eq!(SiUnitPrefix::Q.as_f32(), 1e+30f32);
  assert_eq!(SiUnitPrefix::R.as_f32(), 1e+27f32);
  assert_eq!(SiUnitPrefix::Y.as_f32(), 1e+24f32);
  assert_eq!(SiUnitPrefix::Z.as_f32(), 1e+21f32);
  assert_eq!(SiUnitPrefix::E.as_f32(), 1e+18f32);
  assert_eq!(SiUnitPrefix::P.as_f32(), 1e+15f32);
  assert_eq!(SiUnitPrefix::T.as_f32(), 1e+12f32);
  assert_eq!(SiUnitPrefix::G.as_f32(), 1e+9f32);
  assert_eq!(SiUnitPrefix::M.as_f32(), 1e+6f32);
  assert_eq!(SiUnitPrefix::k.as_f32(), 1e+3f32);
  assert_eq!(SiUnitPrefix::h.as_f32(), 1e+2f32);
  assert_eq!(SiUnitPrefix::da.as_f32(), 1e+1f32);
  assert_eq!(SiUnitPrefix::d.as_f32(), 1e-1f32);
  assert_eq!(SiUnitPrefix::c.as_f32(), 1e-2f32);
  assert_eq!(SiUnitPrefix::m.as_f32(), 1e-3f32);
  assert_eq!(SiUnitPrefix::u.as_f32(), 1e-6f32);
  assert_eq!(SiUnitPrefix::n.as_f32(), 1e-9f32);
  assert_eq!(SiUnitPrefix::p.as_f32(), 1e-12f32);
  assert_eq!(SiUnitPrefix::f.as_f32(), 1e-15f32);
  assert_eq!(SiUnitPrefix::a.as_f32(), 1e-18f32);
  assert_eq!(SiUnitPrefix::z.as_f32(), 1e-21f32);
  assert_eq!(SiUnitPrefix::y.as_f32(), 1e-24f32);
  assert_eq!(SiUnitPrefix::r.as_f32(), 1e-27f32);
  assert_eq!(SiUnitPrefix::q.as_f32(), 1e-30f32);
 }

 #[test]
 fn exp()
 {
  assert_eq!(SiUnitPrefix::Q.as_exp(), 30);
  assert_eq!(SiUnitPrefix::R.as_exp(), 27);
  assert_eq!(SiUnitPrefix::Y.as_exp(), 24);
  assert_eq!(SiUnitPrefix::Z.as_exp(), 21);
  assert_eq!(SiUnitPrefix::E.as_exp(), 18);
  assert_eq!(SiUnitPrefix::P.as_exp(), 15);
  assert_eq!(SiUnitPrefix::T.as_exp(), 12);
  assert_eq!(SiUnitPrefix::G.as_exp(), 9);
  assert_eq!(SiUnitPrefix::M.as_exp(), 6);
  assert_eq!(SiUnitPrefix::k.as_exp(), 3);
  assert_eq!(SiUnitPrefix::h.as_exp(), 2);
  assert_eq!(SiUnitPrefix::da.as_exp(), 1);
  assert_eq!(SiUnitPrefix::d.as_exp(), -1);
  assert_eq!(SiUnitPrefix::c.as_exp(), -2);
  assert_eq!(SiUnitPrefix::m.as_exp(), -3);
  assert_eq!(SiUnitPrefix::u.as_exp(), -6);
  assert_eq!(SiUnitPrefix::n.as_exp(), -9);
  assert_eq!(SiUnitPrefix::p.as_exp(), -12);
  assert_eq!(SiUnitPrefix::f.as_exp(), -15);
  assert_eq!(SiUnitPrefix::a.as_exp(), -18);
  assert_eq!(SiUnitPrefix::z.as_exp(), -21);
  assert_eq!(SiUnitPrefix::y.as_exp(), -24);
  assert_eq!(SiUnitPrefix::r.as_exp(), -27);
  assert_eq!(SiUnitPrefix::q.as_exp(), -30);
 }

 #[test]
 fn ord()
 {
  assert!(SiUnitPrefix::T > SiUnitPrefix::G);
  assert!(SiUnitPrefix::m > SiUnitPrefix::u);
 }

 #[test]
 fn clone()
 {
  let mut pre0 = SiUnitPrefix::G;
  let pre1 = pre0.clone();
  pre0 = SiUnitPrefix::M;
  assert_ne!(pre0, pre1)
 }

 #[test]
 fn copy()
 {
  let mut pre0 = SiUnitPrefix::G;
  let pre1 = pre0;
  pre0 = SiUnitPrefix::M;
  assert_ne!(pre0, pre1)
 }

 #[test]
 fn serde()
 {
  let pre = SiUnitPrefix::G;
  let serialized = serde_json::to_string(&pre).unwrap();
  let deserialized = serde_json::from_str(&serialized).unwrap();
  assert_eq!(pre, deserialized)
 }
}
