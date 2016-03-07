extern crate libc;
extern crate tendril;
extern crate html5ever;

use std::io;
use std::io::Write;
use libc::{c_int, c_char, size_t};
use tendril::TendrilSink;
use html5ever::driver::ParseOpts;
use html5ever::tree_builder::TreeBuilderOpts;
use html5ever::rcdom::RcDom;

const C_RES_OK: c_int = 0;
const C_RES_ERR_INVALID_PARAMETER: c_int = 1;
const C_RES_ERR_PARSE_DOCUMENT: c_int = 2;
const C_RES_ERR_DOCTYPE_WRITE: c_int = 3;
const C_RES_ERR_SERIALIZE: c_int = 4;

#[no_mangle]
pub extern fn parse_document(html: *const c_char, html_size: size_t, print_tree: c_int) -> c_int {
    if html.is_null() {
        return C_RES_ERR_INVALID_PARAMETER;
    }

    let mut html_slice = unsafe { std::slice::from_raw_parts(html as *const u8, html_size as usize) };

    let opts = ParseOpts {
        tree_builder: TreeBuilderOpts {
            drop_doctype: true,
            ..Default::default()
        },
        ..Default::default()
    };

    match html5ever::parse_document(RcDom::default(), opts)
        .from_utf8()
        .read_from(&mut html_slice) {
            Ok(dom) => if print_tree == 0 {
                C_RES_OK
            } else {
                // The validator.nu HTML2HTML always prints a doctype at the very beginning.
                if let Err(_) = io::stdout().write_all(b"<!DOCTYPE html>\n") {
                    return C_RES_ERR_DOCTYPE_WRITE;
                }

                match html5ever::serialize(&mut io::stdout(), &dom.document, Default::default()) {
                    Ok(()) =>
                        C_RES_OK,
                    Err(_) =>
                        C_RES_ERR_SERIALIZE
                }
            },
            Err(_) =>
                C_RES_ERR_PARSE_DOCUMENT
        }
}
