extern mod syntax;
extern mod rustc;

use syntax::parse;
use syntax::ast;

use std::os;
use std::path;

fn get_ast_and_resolve(crate: &Path) -> (@ast::crate, rustc::middle::resolve::CrateMap) {
    use rustc::{front, metadata, driver, middle};
    let parsesess = parse::new_parse_sess(None);
    let sessopts = @driver::session::options {
        binary: @"rustdoc",
        ..copy *rustc::driver::session::basic_options()
    };

    let sess = driver::driver::build_session(sessopts, syntax::diagnostic::emit);

    let mut crate = parse::parse_crate_from_file(crate, ~[], parsesess);
    // XXX: these need to be kept in sync with the pass order in rustc::driver::compile_rest
    crate = front::config::strip_unconfigured_items(crate);
    crate = syntax::ext::expand::expand_crate(parsesess, ~[], crate);
    crate = front::config::strip_unconfigured_items(crate);
    crate = front::std_inject::maybe_inject_libstd_ref(sess, crate);
    let ast_map = syntax::ast_map::map_crate(sess.diagnostic(), crate);
    metadata::creader::read_crates(sess.diagnostic(), crate, sess.cstore,
                                          sess.filesearch,
                                          driver::session::sess_os_to_meta_os(sess.targ_cfg.os),
                                          sess.opts.is_static,
                                          parse::token::get_ident_interner());
    let lang_items = middle::lang_items::collect_language_items(crate, sess);
    let cmap = middle::resolve::resolve_crate(sess, lang_items, crate);
    (crate, cmap)
}

fn main() {
    let cratename = Path(os::args()[1]);
    println(fmt!("%?", get_ast_and_resolve(&cratename)));
}
