(module
  (func $internal (param i32 i32) (result i32)
    local.get 0
    local.get 1

    i32.add
  )

  (func (export "call_internal") (result i32)
    i32.const 2
    i32.const 5

    call $internal
  )
)
