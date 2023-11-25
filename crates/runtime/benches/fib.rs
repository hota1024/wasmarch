use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};

macro_rules! instantiate_wat {
    ($wat_path: tt) => {{
        let wat = include_bytes!(concat!("wasm/", $wat_path, ".wat"));
        let wasm = wabt::wat2wasm(wat).unwrap();
        runtime::instantiate(&wasm).unwrap()
    }};
}

fn fibonacci(n: i32) -> i32 {
    if n == 1 {
        return 1;
    }

    if n == 2 {
        return 1;
    }

    fibonacci(n - 1) + fibonacci(n - 2)
}

fn bench_fibs(c: &mut Criterion) {
    let mut group = c.benchmark_group("Fibonacci");

    let mut wasmarch = instantiate_wat!("fib");

    let mut chibiwasm_runtime =
        chibiwasm::Runtime::from_bytes(include_bytes!("wasm/fib.wasm"), None).unwrap();

    let wasmtime_engine = wasmtime::Engine::default();
    let wasmtime_module =
        wasmtime::Module::new(&wasmtime_engine, include_bytes!("wasm/fib.wat")).unwrap();
    let wasmtime_linker = wasmtime::Linker::new(&wasmtime_engine);
    let mut wasmtime_store = wasmtime::Store::new(&wasmtime_engine, 4);
    let wasmtime_instance = wasmtime_linker
        .instantiate(&mut wasmtime_store, &wasmtime_module)
        .unwrap();
    let wasmtime_fib = wasmtime_instance
        .get_func(&mut wasmtime_store, "fib")
        .unwrap();
    let mut wasmtime_results = [wasmtime::Val::I32(0)];
    wasmtime_fib
        .call(
            &mut wasmtime_store,
            &[wasmtime::Val::I32(10)],
            &mut wasmtime_results,
        )
        .unwrap();

    for i in [20].iter() {
        group.bench_with_input(BenchmarkId::new("wasmarch", i), i, |b, i| {
            b.iter(|| wasmarch.invoke("fib", &[runtime::value::Val::I32(*i)]));
        });
        group.bench_with_input(BenchmarkId::new("chibiwasm", i), i, |b, i| {
            b.iter(|| chibiwasm_runtime.call("fib".to_string(), vec![chibiwasm::Value::from(*i)]));
        });
        group.bench_with_input(BenchmarkId::new("wasmtime", i), i, |b, i| {
            b.iter(|| {
                wasmtime_fib.call(
                    &mut wasmtime_store,
                    &[wasmtime::Val::I32(*i)],
                    &mut wasmtime_results,
                )
            });
        });
    }
}

criterion_group!(benches, bench_fibs);
criterion_main!(benches);
