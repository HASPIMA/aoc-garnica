File ← &i "../utils.ua" "File"

ParseReg ← /↥∵(⋕▽↧⊃(≥@0)(≤@9).°□)regex$"\\d+_"

Solve ← /+≡(
  ⊃⊃⊃(
    ParseReg" red"
  )(
    ParseReg" green"
  )(
    ParseReg" blue"
  )(
    ParseReg":"
  )
  /↧≤12_13_14⊂⊂
  ⊢×
)

Solve File "inputs/input1.txt"

---
⍤.=8 Solve File "inputs/example1.txt"
---
