//! Glue code with rustc
#![feature(rustc_private)]
#![feature(custom_mir)]
#![feature(box_patterns)]

use mircallgraph::rustc_cb_entry;
use rustc_driver::Compilation;


extern crate rustc_ast;
extern crate rustc_ast_pretty;
extern crate rustc_driver;
extern crate rustc_error_codes;
extern crate rustc_errors;
extern crate rustc_hash;
extern crate rustc_hir;
extern crate rustc_index;
extern crate rustc_interface;
extern crate rustc_middle;
extern crate rustc_mir_dataflow;
extern crate rustc_session;
extern crate rustc_span;

fn main() {
    // Print internal compiler error on panic
    rustc_driver::install_ice_hook();
    rustc_driver::init_rustc_env_logger();

    // ensure --sysroot is included in args
    let mut args: Vec<String> = std::env::args().collect();
    let home = env!("RUSTUP_HOME");
    let toolchain = env!("RUSTUP_TOOLCHAIN");
    let sysroot = format!("{home}/toolchains/{toolchain}");
    if !args.iter().any(|e| e == "--sysroot") {
        args.push("--sysroot".to_owned());
        args.push(sysroot);
    }

    // invoke compiler with taint analysis callback
    let exit_code = rustc_driver::catch_with_exit_code(move || {
        rustc_driver::RunCompiler::new(&args, &mut CallgraphCallbacks).run()
    });
    std::process::exit(exit_code)
}

struct CallgraphCallbacks;

impl rustc_driver::Callbacks for CallgraphCallbacks {
    /// All the work we do happens after analysis, so that we can make assumptions about the validity of the MIR.
    fn after_analysis<'tcx>(
        &mut self,
        compiler: &rustc_interface::interface::Compiler,
        queries: &'tcx rustc_interface::Queries<'tcx>,
    ) -> Compilation {
        compiler.session().abort_if_errors();
        queries
            .global_ctxt()
            .unwrap()
            .get_mut()
            .enter(rustc_cb_entry);
        compiler.session().abort_if_errors();
        Compilation::Stop
    }
}
