(module
  (func (export "local_get") (result i32)
    (local i32)

    i32.const 1234
    local.set 0

    local.get 0
  )
)
