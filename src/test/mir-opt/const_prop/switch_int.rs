fn main() {
    match 1 {
        1 => std::process::exit(0),
        _ => std::process::exit(-1),
    }
}

// END RUST SOURCE
// START rustc.main.ConstProp.before.mir
//  bb0: {
//      ...
//      _1 = const 1i32;
//      switchInt(_1) -> [1i32: bb1, otherwise: bb2];
//  }
// END rustc.main.ConstProp.before.mir
// START rustc.main.ConstProp.after.mir
//  bb0: {
//      ...
//      switchInt(const 1i32) -> [1i32: bb1, otherwise: bb2];
//  }
// END rustc.main.ConstProp.after.mir
