File ← &i "../utils.ua" "File"

# keep first empty item for consistency with indexes
N ← {"" "one" "two" "three" "four" "five" "six" "seven" "eight" "nine"}

Solve ← (
  ∵(
    ≡⊂⇡⧻.N # setup data
    ≡(
      ⊃(
        ⌕∩°□⊢⇌ # find number using words
      )(
        ⊃(
          ⌕$"_"∩°□⊢ # find number using digit
        )⊢          # keep index
      )
      ↥ # join same word and number in position
      × # use index number
    )
    ⊐▽>0./↥     # join all numbers and delete zeroes
    ⊃(×10⊢)(⊢⇌) # get first and last digit
    +           # add them
  )
  /+ # Add all line results
)

Solve File "inputs/input1.txt"

---
⍤.=281 Solve File "inputs/example2.txt"
---
