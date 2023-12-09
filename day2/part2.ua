File ← &i "../utils.ua" "File"

ParseReg ← /↥∵(⋕▽↧⊃(≥@0)(≤@9).°□)regex$"\\d+_"

Solve ← /+≡(
  ⊃⊃(
    ParseReg" red"
  )(
    ParseReg" green"
  )(
    ParseReg" blue"
  )
  ⊂⊂
  /×
)

Solve File "inputs/input1.txt"

---
⍤.=2286 Solve File "inputs/example1.txt"
---
