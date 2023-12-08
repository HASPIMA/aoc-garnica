File ← &i "../utils.ua" "File"

Solve ← /+/+∵(
  ⊃⊃⊃(
    /↥∵(⋕⊢⊜□≠,@ °□)regex"\\d+ red"
  )(
    /↥∵(⋕⊢⊜□≠,@ °□)regex"\\d+ green"
  )(
    /↥∵(⋕⊢⊜□≠,@ °□)regex"\\d+ blue"
  )(
    /↥∵(⋕⊢⇌⊜□≠,@ °□)regex"Game \\d+"
  )
  ⊂⊂
  12_13_14
  ≤
  /↧
  ×
)

Solve File "inputs/input1.txt"

---
⍤.=8 Solve File "inputs/example1.txt"
---
