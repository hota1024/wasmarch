(module
  (import "env" "log_i32" (func $log_i32 (param i32)))
  (import "env" "cos" (func $cos (param f32) (result f32)))

  (func (export "main") (result f32)
    i32.const 10
    call $log_i32

    f32.const 3.14
    call $cos
  )
)
