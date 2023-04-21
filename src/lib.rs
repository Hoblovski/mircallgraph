#![feature(rustc_private)]
#![feature(box_patterns)]

use rustc_hir::def_id::DefId;
use rustc_middle::{
    mir::{ConstantKind, Operand, Terminator, TerminatorKind},
    ty::{self, TyCtxt},
};

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
extern crate rustc_type_ir;

use lazy_static::lazy_static;
lazy_static! {
    pub static ref PRINT_ONLY_LOCALS: bool =
        std::env::var("ONLY_LOCAL").unwrap_or("".to_string()) == "y";
}

pub fn rustc_cb_entry(tcx: TyCtxt) {
    for f in tcx.hir_crate_items(()).definitions().map(|x| x.to_def_id()) {
        if is_fn_with_mir(tcx, f) {
            handle_fn(tcx, f);
        }
    }
}

fn handle_fn(tcx: TyCtxt, f: DefId) {
    let caller = tcx.def_path_str(f);
    println!("Called by {caller}:");
    let body = tcx.optimized_mir(f);
    for (_bb, data) in body.basic_blocks.iter_enumerated() {
        if let Some(g) = terminator_to_callee(data.terminator()) {
            let callee = tcx.def_path_str(g);
            if !*PRINT_ONLY_LOCALS || g.krate == f.krate {
                println!("        {callee}");
            }
        }
    }
}

fn is_fn_with_mir<'tcx>(tcx: TyCtxt<'tcx>, f: DefId) -> bool {
    if !tcx.is_mir_available(f) {
        return false;
    }
    // to filter out constants
    match tcx.hir().body_const_context(f.as_local().unwrap()) {
        Some(rustc_hir::ConstContext::ConstFn) => true,
        None => true,
        _ => return false,
    }
}

fn terminator_to_callee(tmnt: &Terminator) -> Option<DefId> {
    if let TerminatorKind::Call { func, .. } = &tmnt.kind {
        if let Operand::Constant(box c) = func {
            if let ConstantKind::Val(_cv, ty) = c.literal {
                if let ty::FnDef(id, _) = ty.kind() {
                    return Some(*id);
                }
            }
        }
    }
    return None;
}
