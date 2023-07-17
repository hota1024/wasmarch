(module
  (func (export "if_1") (result i32)
    (i32.const 1)

    if (result i32)
      (i32.const 1)
    else
      (i32.const 0)
    end
  )
  (func (export "if_0") (result i32)
    (i32.const 0)

    if (result i32)
      (i32.const 1)
    else
      (i32.const 0)
    end
  )
  (func (export "if_if_1") (result i32)
    (i32.const 1)

    if (result i32)
      (i32.const 1)

      if (result i32)
        (i32.const 1)
      else
        (i32.const 0)
      end
    else
      (i32.const 0)
    end
  )
  (func (export "if_if_0") (result i32)
    (i32.const 0)

    if (result i32)
      (i32.const 1)

      if (result i32)
        (i32.const 1)
      else
        (i32.const 0)
      end
    else
      (i32.const 0)
    end
  )
)