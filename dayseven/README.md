# Day Seven

## Part 1
- iterate over all the crap positions
- calculate the absolute difference
- make a hashmap which maps position to cost
- return the lowest cost

## Part 2
- iterate over the range of positions
- implement new cost formula
- everything else is like part 1

### Values

|Start | End| Costs| Diff |
|---|--|------:|------:| 
|1 | 2 |  1 |  1|
|1 | 3 | 1 + 2 = 3 |  2|
|1 | 4 | 1 + 2 + 3 = 6 | 3|
|1 | 5 | 1 + 2 + 3 + 4 = 10 | 4 |

### Sum from 1 to N
S = (n*(n+1)) / 2


### Test
Use Gauß sum formula with difference

#### 1 -> 3 (difference 2) : 
```
  (2*(2+1)) / 2 
= (2*3) / 2 
= 6 / 2 
= 3 ✓
```

#### 1 -> 5 (difference 4) : 
```
  (4*(4+1)) / 2 
= (4*5) / 2 
= 20 / 2 
= 10 ✓
```

