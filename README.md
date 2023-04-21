# Callgraph
## 编译
```bash
$ cargo build
$ file ./target/debug/mircallgraph
```

## 单文件
```bash
$ export LD_LIBRARY_PATH=$(rustc --print sysroot)/lib:$LD_LIBRARY_PATH
$ ./target/debug/mircallgraph examples/e1.rs
Called by i:
Called by h:
Called by g:
        i
        h
Called by f:
        g
Called by main:
        f
        h
```

## cargo 项目
假设想知道 mircallgraph/src/lib.rs 的调用关系

先编译
```bash
$ touch src/lib.rs && cargo build --verbose
# ...
     Running `rustc --crate-name mircallgraph --edition=2021 src/lib.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --diagnostic-width=192 --crate-type lib --emit=dep-info,metadata,link -C embed-bitcode=no -C debuginfo=2 -C metadata=96391199dd068655 -C extra-filename=-96391199dd068655 --out-dir /home/hob/Programs/mircallgraph/target/debug/deps -C incremental=/home/hob/Programs/mircallgraph/target/debug/incremental -L dependency=/home/hob/Programs/mircallgraph/target/debug/deps --extern lazy_static=/home/hob/Programs/mircallgraph/target/debug/deps/liblazy_static-b33cd6b8ee13db74.rmeta`
# ...
```

把 `Running ``...`` ` 里面的 rustc 换成 ./target/debug/mircallgraph，再跑
```bash
$ ./target/debug/mircallgraph --crate-name mircallgraph --edition=2021 src/lib.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --diagnostic-width=192 --crate-type lib --emit=dep-info,metadata,link -C embed-bitcode=no -C debuginfo=2 -C metadata=96391199dd068655 -C extra-filename=-96391199dd068655 --out-dir /home/hob/Programs/mircallgraph/target/debug/deps -C incremental=/home/hob/Programs/mircallgraph/target/debug/incremental -L dependency=/home/hob/Programs/mircallgraph/target/debug/deps --extern lazy_static=/home/hob/Programs/mircallgraph/target/debug/deps/liblazy_static-b33cd6b8ee13db74.rmeta
# ...
Called by rustc_cb_entry:
        rustc_middle::ty::query::<impl rustc_middle::ty::TyCtxt<'tcx>>::hir_crate_items
        rustc_middle::hir::ModuleItems::definitions
        std::iter::Iterator::map
        std::iter::IntoIterator::into_iter
        std::iter::Iterator::next
        is_fn_with_mir
        handle_fn
Called by handle_fn:
        rustc_middle::ty::print::pretty::<impl rustc_middle::ty::TyCtxt<'t>>::def_path_str
        core::fmt::ArgumentV1::<'a>::new_display
        std::fmt::Arguments::<'a>::new_v1
        std::io::_print
        rustc_middle::ty::query::<impl rustc_middle::ty::TyCtxt<'tcx>>::optimized_mir
        lazy_static::__Deref::deref
        rustc_index::vec::IndexVec::<I, T>::iter_enumerated
        std::iter::IntoIterator::into_iter
        std::iter::Iterator::next
# ...
```
