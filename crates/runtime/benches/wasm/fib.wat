(module
  (func
    $fib
    (export "fib")
    (param $n i32)
    (result i32)
    (local $r i32)
    (local.set
      $r
      (i32.sub
        (i32.const 0)
        (i32.const 1)
      )
    )
    (if
      (i32.le_s
        (local.get $n)
        (i32.const 1)
      )
      (then
        (local.set
          $r
          (local.get $n)
        )
      )
    )
    (if
      (i32.eq
        (local.get $r)
        (i32.sub
          (i32.const 0)
          (i32.const 1)
        )
      )
      (then
        (local.set
          $r
          (i32.add
            (call
              $fib
              (i32.sub
                (local.get $n)
                (i32.const 1)
              )
            )
            (call
              $fib
              (i32.sub
                (local.get $n)
                (i32.const 2)
              )
            )
          )
        )
      )
    )
    (local.get $r)
  )
)
