(module
  (func $fib_rec (export "fib") (param i32) (result i32)
    local.get 0
    i32.const 1
    i32.eq
    if
      i32.const 1
      return
    end

    local.get 0
    i32.const 2
    i32.eq
    if
      i32.const 1
      return
    end

    local.get 0
    i32.const 1
    i32.sub
    call $fib_rec

    local.get 0
    i32.const 2
    i32.sub
    call $fib_rec

    i32.add
  )
)
