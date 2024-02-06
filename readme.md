# nhl-rs

NHL stats as I like 'em.

## Usage and sample output

The `nhl-rs` executable is a simple command-line tool that fetches the current NHL standings and displays them in a tabular format.

The output is segmented into the four divisions, two conferences, and the full league. The teams are sorted by wins minus losses, and the display is games played, wins minus losses, and the last ten games in the format `W-L-OT`.

```bash
$ nhl-rs                                                                                                23:18:23

===================================
         Atlantic division
===================================
 1. Bruins          49  22   7-1-2
 2. Panthers        49  17   6-2-2
 3. Maple Leafs     48  10   4-5-1
 4. Lightning       50   9   8-2-0
 5. Red Wings       50   8   6-2-2
 6. Sabres          49  -1   6-4-0
 7. Canadiens       49  -1   3-4-3
 8. Senators        47  -5   6-2-2

===================================
       Metropolitan division
===================================
 1. Rangers         50  15   5-4-1
 2. Hurricanes      48  13   7-2-1
 3. Flyers          50   6   5-5-0
 4. Penguins        46   5   4-3-3
 5. Capitals        47   4   4-5-1
 6. Devils          47   4   3-6-1
 7. Islanders       50   4   3-5-2
 8. Blue Jackets    50  -8   3-5-2

===================================
         Central division
===================================
 1. Jets            47  18   6-3-1
 2. Avalanche       50  18   7-2-1
 3. Stars           49  17   7-2-1
 4. Blues           49   5   6-3-1
 5. Predators       51   3   4-5-1
 6. Coyotes         48   1   4-5-1
 7. Wild            49  -2   4-5-1
 8. Blackhawks      50 -20   2-8-0

===================================
         Pacific division
===================================
 1. Canucks         49  22   8-0-2
 2. Oilers          45  14   10-0-0
 3. Golden Knights  50  14   6-3-1
 4. Kings           48   8   3-5-2
 5. Kraken          50   2   4-5-1
 6. Flames          49   0   5-5-0
 7. Ducks           50 -12   4-5-1
 8. Sharks          51 -18   5-3-2

===================================
        Eastern conference
===================================
 1. Bruins          49  22   7-1-2
 2. Panthers        49  17   6-2-2
 3. Rangers         50  15   5-4-1
 4. Hurricanes      48  13   7-2-1
 5. Maple Leafs     48  10   4-5-1
 6. Lightning       50   9   8-2-0
 7. Red Wings       50   8   6-2-2
 8. Flyers          50   6   5-5-0
 9. Penguins        46   5   4-3-3
10. Capitals        47   4   4-5-1
11. Devils          47   4   3-6-1
12. Islanders       50   4   3-5-2
13. Sabres          49  -1   6-4-0
14. Canadiens       49  -1   3-4-3
15. Senators        47  -5   6-2-2
16. Blue Jackets    50  -8   3-5-2

===================================
        Western conference
===================================
 1. Canucks         49  22   8-0-2
 2. Jets            47  18   6-3-1
 3. Avalanche       50  18   7-2-1
 4. Stars           49  17   7-2-1
 5. Oilers          45  14   10-0-0
 6. Golden Knights  50  14   6-3-1
 7. Kings           48   8   3-5-2
 8. Blues           49   5   6-3-1
 9. Predators       51   3   4-5-1
10. Kraken          50   2   4-5-1
11. Coyotes         48   1   4-5-1
12. Flames          49   0   5-5-0
13. Wild            49  -2   4-5-1
14. Ducks           50 -12   4-5-1
15. Sharks          51 -18   5-3-2
16. Blackhawks      50 -20   2-8-0

===================================
            Full league
===================================
 1. Bruins          49  22   7-1-2
 2. Canucks         49  22   8-0-2
 3. Jets            47  18   6-3-1
 4. Avalanche       50  18   7-2-1
 5. Panthers        49  17   6-2-2
 6. Stars           49  17   7-2-1
 7. Rangers         50  15   5-4-1
 8. Oilers          45  14   10-0-0
 9. Golden Knights  50  14   6-3-1
10. Hurricanes      48  13   7-2-1
11. Maple Leafs     48  10   4-5-1
12. Lightning       50   9   8-2-0
13. Kings           48   8   3-5-2
14. Red Wings       50   8   6-2-2
15. Flyers          50   6   5-5-0
16. Penguins        46   5   4-3-3
17. Blues           49   5   6-3-1
18. Capitals        47   4   4-5-1
19. Devils          47   4   3-6-1
20. Islanders       50   4   3-5-2
21. Predators       51   3   4-5-1
22. Kraken          50   2   4-5-1
23. Coyotes         48   1   4-5-1
24. Flames          49   0   5-5-0
25. Sabres          49  -1   6-4-0
26. Canadiens       49  -1   3-4-3
27. Wild            49  -2   4-5-1
28. Senators        47  -5   6-2-2
29. Blue Jackets    50  -8   3-5-2
30. Ducks           50 -12   4-5-1
31. Sharks          51 -18   5-3-2
32. Blackhawks      50 -20   2-8-0
```

## Resources

* New API docs: <https://gitlab.com/dword4/nhlapi/-/blob/master/new-api.md>
* Transform tools, JSON to Rust Serde: <https://transform.tools/json-to-rust-serde>
