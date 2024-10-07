~ "../utils.ua" ~ File

# keep first empty item for consistency with indexes
N ← {"" "one" "two" "three" "four" "five" "six" "seven" "eight" "nine"}

Solve ← (
  ∵(
    ≡⊂⇡⧻.N # setup data
    ≡◇(
      ⊃⊃(
        ◇⌕⊢⇌ # find number using words
      )(
        ⌕°⋕⊢ # find number using digit
      )(
        °□⊢ # keep index
      )
      ↥ # join same word and number in position
      × # use index number
    )
    ▽>0./↥      # join all numbers and delete zeroes
    ⋕⊂∩°⋕⊃⊢(⊢⇌) # get first and last digit and parse them
  )
  /+ # Add all line results
)

# Compacted version
Solve ← /+(
  ∵(
    ⋕⊂∩°⋕⊃⊢(⊢⇌)▽>0./↥≡◇(
      ×↥⊃⊃(◇⌕⊢⇌)(⌕°⋕⊢)(°□⊢)
    )≡⊂⇡⧻.N
  )
)

Solve File "inputs/input1.txt"

---test
⍤.=281 Solve File "inputs/example2.txt"
---
