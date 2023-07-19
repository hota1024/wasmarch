(module
  (memory $memory 1)
  (export "memory" (memory $memory))

  (func (export "set_memory") (param $v i32)
    i32.const 0
    local.get $v
    i32.store
  )
)
