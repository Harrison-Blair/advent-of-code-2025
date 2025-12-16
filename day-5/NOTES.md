# Notes

## Cases
Given [a,b] and [x,y] 
  - are two positive integers
  - a < b && x < y

Overlaps
  - a < x && x <= b <= y -> set x = a
  - b > y && x <= a <= y -> set y = b

Contains
  - x = a, y = b

No relation
 - add [a,b] to list