(module
  (import
    "std"
    "newline"
    (func $newline)
  )
  (import
    "std"
    "log_i32"
    (func
      $log_i32
      (param i32)
    )
  )
  (import
    "std"
    "log_i32x3"
    (func
      $log_i32x3
      (param i32)
      (param i32)
      (param i32)
    )
  )
  (import
    "std"
    "random_bool"
    (func
      $random_bool
      (result i32)
    )
  )
  (global
    $grid_width
    (export "grid_width")
    i32
    (i32.const 48)
  )
  (global
    $grid_height
    (export "grid_height")
    i32
    (i32.const 48)
  )
  (global
    $grid_pixel_size
    (export "grid_pixel_size")
    f32
    (f32.const 16)
  )
  (global
    $CELL_LIVE
    i32
    (i32.const 43775)
  )
  (global
    $CELL_DEAD
    i32
    (i32.const 16777215)
  )
  (global
    $P_BOARD
    i32
    (i32.const 0)
  )
  (global
    $P_NEXT
    (mut i32)
    (i32.const 0)
  )
  (global
    $frame
    (mut i32)
    (i32.const 0)
  )
  (memory $grid_memory 1)
  (export
    "grid_memory"
    (memory $grid_memory)
  )
  (func
    $point_to_index
    (param $p i32)
    (param $x i32)
    (param $y i32)
    (result i32)
    (i32.add
      (local.get $p)
      (i32.mul
        (i32.add
          (i32.mul
            (local.get $y)
            (global.get $grid_width)
          )
          (local.get $x)
        )
        (i32.const 4)
      )
    )
  )
  (func
    $get
    (param $p i32)
    (param $x i32)
    (param $y i32)
    (result i32)
    (if
      (i32.or
        (i32.or
          (i32.or
            (i32.lt_s
              (local.get $x)
              (i32.const 0)
            )
            (i32.gt_s
              (local.get $x)
              (i32.sub
                (global.get $grid_width)
                (i32.const 1)
              )
            )
          )
          (i32.lt_s
            (local.get $y)
            (i32.const 0)
          )
        )
        (i32.gt_s
          (local.get $y)
          (i32.sub
            (global.get $grid_height)
            (i32.const 1)
          )
        )
      )
      (then
        (return
          (global.get $CELL_DEAD)
        )
      )
    )
    (i32.load
      (call
        $point_to_index
        (local.get $p)
        (local.get $x)
        (local.get $y)
      )
    )
  )
  (func
    $set
    (param $p i32)
    (param $x i32)
    (param $y i32)
    (param $color i32)
    (i32.store
      (call
        $point_to_index
        (local.get $p)
        (local.get $x)
        (local.get $y)
      )
      (local.get $color)
    )
  )
  (func
    $count_around
    (param $p i32)
    (param $x i32)
    (param $y i32)
    (param $c i32)
    (result i32)
    (local $count i32)
    (local.set
      $count
      (i32.const 0)
    )
    (if
      (i32.eq
        (call
          $get
          (local.get $p)
          (i32.add
            (local.get $x)
            (i32.const 1)
          )
          (local.get $y)
        )
        (local.get $c)
      )
      (then
        (local.set
          $count
          (i32.add
            (local.get $count)
            (i32.const 1)
          )
        )
      )
    )
    (if
      (i32.eq
        (call
          $get
          (local.get $p)
          (i32.sub
            (local.get $x)
            (i32.const 1)
          )
          (local.get $y)
        )
        (local.get $c)
      )
      (then
        (local.set
          $count
          (i32.add
            (local.get $count)
            (i32.const 1)
          )
        )
      )
    )
    (if
      (i32.eq
        (call
          $get
          (local.get $p)
          (local.get $x)
          (i32.add
            (local.get $y)
            (i32.const 1)
          )
        )
        (local.get $c)
      )
      (then
        (local.set
          $count
          (i32.add
            (local.get $count)
            (i32.const 1)
          )
        )
      )
    )
    (if
      (i32.eq
        (call
          $get
          (local.get $p)
          (local.get $x)
          (i32.sub
            (local.get $y)
            (i32.const 1)
          )
        )
        (local.get $c)
      )
      (then
        (local.set
          $count
          (i32.add
            (local.get $count)
            (i32.const 1)
          )
        )
      )
    )
    (if
      (i32.eq
        (call
          $get
          (local.get $p)
          (i32.add
            (local.get $x)
            (i32.const 1)
          )
          (i32.add
            (local.get $y)
            (i32.const 1)
          )
        )
        (local.get $c)
      )
      (then
        (local.set
          $count
          (i32.add
            (local.get $count)
            (i32.const 1)
          )
        )
      )
    )
    (if
      (i32.eq
        (call
          $get
          (local.get $p)
          (i32.sub
            (local.get $x)
            (i32.const 1)
          )
          (i32.add
            (local.get $y)
            (i32.const 1)
          )
        )
        (local.get $c)
      )
      (then
        (local.set
          $count
          (i32.add
            (local.get $count)
            (i32.const 1)
          )
        )
      )
    )
    (if
      (i32.eq
        (call
          $get
          (local.get $p)
          (i32.add
            (local.get $x)
            (i32.const 1)
          )
          (i32.sub
            (local.get $y)
            (i32.const 1)
          )
        )
        (local.get $c)
      )
      (then
        (local.set
          $count
          (i32.add
            (local.get $count)
            (i32.const 1)
          )
        )
      )
    )
    (if
      (i32.eq
        (call
          $get
          (local.get $p)
          (i32.sub
            (local.get $x)
            (i32.const 1)
          )
          (i32.sub
            (local.get $y)
            (i32.const 1)
          )
        )
        (local.get $c)
      )
      (then
        (local.set
          $count
          (i32.add
            (local.get $count)
            (i32.const 1)
          )
        )
      )
    )
    (local.get $count)
  )
  (func
    $mem_copy
    (param $src i32)
    (param $dest i32)
    (local $x i32)
    (local $y i32)
    (local $c i32)
    (local.set
      $x
      (i32.const 0)
    )
    (local.set
      $y
      (i32.const 0)
    )
    (local.set
      $c
      (i32.const 0)
    )
    (loop
      $wasabi_lang/label_continue_0
      (local.set
        $y
        (i32.const 0)
      )
      (loop
        $wasabi_lang/label_continue_2
        (call
          $set
          (local.get $dest)
          (local.get $x)
          (local.get $y)
          (call
            $get
            (local.get $src)
            (local.get $x)
            (local.get $y)
          )
        )
        (local.set
          $y
          (i32.add
            (local.get $y)
            (i32.const 1)
          )
        )
        (i32.lt_s
          (local.get $y)
          (global.get $grid_height)
        )
        (br_if $wasabi_lang/label_continue_2)
      )
      (local.set
        $x
        (i32.add
          (local.get $x)
          (i32.const 1)
        )
      )
      (i32.lt_s
        (local.get $x)
        (global.get $grid_width)
      )
      (br_if $wasabi_lang/label_continue_0)
    )
  )
  (func $gen_next)
  (func
    $grid_frame
    (export "grid_frame")
    (local $x i32)
    (local $y i32)
    (local $cur i32)
    (local $count i32)
    (local $next_state i32)
    (global.set
      $frame
      (i32.add
        (global.get $frame)
        (i32.const 1)
      )
    )
    (memory.copy
      (global.get $P_NEXT)
      (global.get $P_BOARD)
      (i32.mul
        (i32.mul
          (global.get $grid_width)
          (global.get $grid_height)
        )
        (i32.const 4)
      )
    )
    (local.set
      $x
      (i32.const 0)
    )
    (local.set
      $y
      (i32.const 0)
    )
    (local.set
      $cur
      (i32.const 0)
    )
    (local.set
      $count
      (i32.const 0)
    )
    (local.set
      $next_state
      (i32.const 0)
    )
    (loop
      $wasabi_lang/label_continue_4
      (local.set
        $y
        (i32.const 0)
      )
      (loop
        $wasabi_lang/label_continue_6
        (local.set
          $cur
          (call
            $get
            (global.get $P_BOARD)
            (local.get $x)
            (local.get $y)
          )
        )
        (local.set
          $count
          (call
            $count_around
            (global.get $P_BOARD)
            (local.get $x)
            (local.get $y)
            (global.get $CELL_LIVE)
          )
        )
        (local.set
          $next_state
          (i32.sub
            (i32.const 0)
            (i32.const 1)
          )
        )
        (if
          (i32.and
            (i32.eq
              (local.get $cur)
              (global.get $CELL_LIVE)
            )
            (i32.lt_s
              (local.get $count)
              (i32.const 2)
            )
          )
          (then
            (local.set
              $next_state
              (global.get $CELL_DEAD)
            )
          )
        )
        (if
          (i32.and
            (i32.and
              (i32.eq
                (local.get $next_state)
                (i32.sub
                  (i32.const 0)
                  (i32.const 1)
                )
              )
              (i32.eq
                (local.get $cur)
                (global.get $CELL_LIVE)
              )
            )
            (i32.gt_s
              (local.get $count)
              (i32.const 3)
            )
          )
          (then
            (local.set
              $next_state
              (global.get $CELL_DEAD)
            )
          )
        )
        (if
          (i32.and
            (i32.and
              (i32.eq
                (local.get $next_state)
                (i32.sub
                  (i32.const 0)
                  (i32.const 1)
                )
              )
              (i32.eq
                (local.get $cur)
                (global.get $CELL_DEAD)
              )
            )
            (i32.eq
              (local.get $count)
              (i32.const 3)
            )
          )
          (then
            (local.set
              $next_state
              (global.get $CELL_LIVE)
            )
          )
        )
        (if
          (i32.eq
            (local.get $next_state)
            (i32.sub
              (i32.const 0)
              (i32.const 1)
            )
          )
          (then
            (local.set
              $next_state
              (local.get $cur)
            )
          )
        )
        (call
          $set
          (global.get $P_NEXT)
          (local.get $x)
          (local.get $y)
          (local.get $next_state)
        )
        (local.set
          $y
          (i32.add
            (local.get $y)
            (i32.const 1)
          )
        )
        (i32.lt_s
          (local.get $y)
          (global.get $grid_height)
        )
        (br_if $wasabi_lang/label_continue_6)
      )
      (local.set
        $x
        (i32.add
          (local.get $x)
          (i32.const 1)
        )
      )
      (i32.lt_s
        (local.get $x)
        (global.get $grid_width)
      )
      (br_if $wasabi_lang/label_continue_4)
    )
    (memory.copy
      (global.get $P_BOARD)
      (global.get $P_NEXT)
      (i32.mul
        (i32.mul
          (global.get $grid_width)
          (global.get $grid_height)
        )
        (i32.const 4)
      )
    )
  )
  (func
    $wasabi_lang/start_fn
    (local $x i32)
    (local $y i32)
    (drop
      (memory.grow
        (i32.const 1)
      )
    )
    (global.set
      $P_NEXT
      (i32.mul
        (i32.mul
          (global.get $grid_width)
          (global.get $grid_height)
        )
        (i32.const 4)
      )
    )
    (local.set
      $x
      (i32.const 0)
    )
    (local.set
      $y
      (i32.const 0)
    )
    (loop
      $wasabi_lang/label_continue_8
      (local.set
        $y
        (i32.const 0)
      )
      (loop
        $wasabi_lang/label_continue_10
        (call
          $set
          (global.get $P_NEXT)
          (local.get $x)
          (local.get $y)
          (global.get $CELL_DEAD)
        )
        (if
          (call $random_bool)
          (then
            (call
              $set
              (global.get $P_NEXT)
              (local.get $x)
              (local.get $y)
              (global.get $CELL_LIVE)
            )
          )
        )
        (local.set
          $y
          (i32.add
            (local.get $y)
            (i32.const 1)
          )
        )
        (i32.lt_s
          (local.get $y)
          (global.get $grid_height)
        )
        (br_if $wasabi_lang/label_continue_10)
      )
      (local.set
        $x
        (i32.add
          (local.get $x)
          (i32.const 1)
        )
      )
      (i32.lt_s
        (local.get $x)
        (global.get $grid_width)
      )
      (br_if $wasabi_lang/label_continue_8)
    )
    (memory.copy
      (global.get $P_BOARD)
      (global.get $P_NEXT)
      (i32.mul
        (i32.mul
          (global.get $grid_width)
          (global.get $grid_height)
        )
        (i32.const 4)
      )
    )
  )
  (start $wasabi_lang/start_fn)
)