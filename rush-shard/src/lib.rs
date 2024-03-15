use libshards::ShardsAst;

/// Entry point for parsing code shell code.
///
/// # Safety
/// The values passed to this function must be a valid &[`str`].
#[no_mangle]
pub unsafe extern "C" fn parse(c: *const u8, len: usize) -> ShardsAst {
    let Ok(s) = std::str::from_utf8(std::slice::from_raw_parts(c, len)) else {
        eprintln!("Failed to parse string");
        return todo!();
    };
    dbg!(&s);
    todo!()
}
