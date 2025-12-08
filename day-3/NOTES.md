# Part 3 Notes

- 12/n Chosen `Voltages`
- Leading number is most important factor in value

The below math seems to not be applicable, given that the positions are immutable.

**455 Possible solutions** given the formula: 
```math
C(n,r) =\binom{n}{r} = \frac{n!}{(r!(n-r)!)}
```

Where `n` is the total number of total options and `r` is the number chosen options

*for the cases of this, it seems that the following are true:*
- `n` = 100
- `r` = 88
    - `r` = `n` - 12

The goal seems to be to limit the number of candidate possibilities through various means. Below I have constructed an order of operations to help limit those possiblities.

## Oder of Ops

### Find the Leading # - Recursive
- Recursively find the highest next number that leaves the `batteryBank` with >= 12 charges.
    - Recursion ends when `batteryBank.Count` == 12

This seems to work when there are different numbers, but take a case like:

`222222222222222222222222222222222222222222222222212122`

Special consideration needs to be added here, as the numbers to remove are towards the end. The previously stated formula would leave you with the following:

`222222212122`

This is clearly not the highest combination of values possible here.