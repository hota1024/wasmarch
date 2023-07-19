(module
  (global $global (mut i32) (i32.const 10))

  (func (export "get_global") (result i32)
    global.get $global
  )
  (func (export "set_global") (param $v i32)
    local.get $v
    global.set $global
  )
)
