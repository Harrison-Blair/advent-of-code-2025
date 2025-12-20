# Notes

`n = 1000` Electrical Boxes

This smells to me like a variant of the *Traveling salesperson* problem?

## Calculating closest paths

Calculating every paths distance with every other path would be O(n^2)... doesn't seem very efficient

Sorting the list initialy would make this much easier, need to ensure there is a quicksort that would account for this

Could calculate distance from `(0,0,0)` ? if distances are close, odds are they are close together?
 - on second thought since this is 3D space, I dont think this would help much