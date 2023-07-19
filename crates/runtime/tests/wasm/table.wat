(module
  (table 2 anyfunc)
  (type $return_i32 (func (result i32)))

  (func $fn1 (result i32)
    i32.const 10
  )
  (func $fn2 (result i32)
    i32.const 20
  )

  (func (export "call_by_index") (param $i i32) (result i32)
    local.get $i
    call_indirect (type $return_i32)
  )

  (elem (i32.const 0) $fn1 $fn2)
)
