(module
  (func (export "returns_i32") (result i32)
    (i32.const 1234)
  )
  (func (export "returns_i64") (result i64)
    (i64.const 9223372036854775807) ;; i64::MAX
  )
  (func (export "returns_f32") (result f32)
    (f32.const 3.14)
  )
  (func (export "returns_f64") (result f64)
    (f64.const -3.14)
  )
)