# nhl-rs

NHL stats as I like 'em.

## Usage and sample output

The `nhl-rs` executable is a simple command-line tool that fetches the current NHL standings and displays them in a tabular format.

The output is segmented into the four divisions, two conferences, and the full league. The teams are sorted by wins minus losses, and the display is games played, wins minus losses, and the last ten games' wins minus losses.

```bash
$ nhl-rs

==============================
      Atlantic division
==============================
                    GP +/- L10
 1. Bruins          49  22   6
 2. Panthers        49  17   4
 3. Maple Leafs     47  11   1
 4. Lightning       50   9   6
 5. Red Wings       50   8   4
 6. Sabres          49  -1   2
 7. Canadiens       49  -1  -1
 8. Senators        47  -5   4

==============================
    Metropolitan division
==============================
                    GP +/- L10
 1. Rangers         49  14  -1
 2. Hurricanes      48  13   5
 3. Flyers          50   6   0
 4. Penguins        46   5   1
 5. Devils          47   4  -3
 6. Capitals        47   4  -1
 7. Islanders       49   3  -4
 8. Blue Jackets    50  -8  -2

==============================
       Central division
==============================
                    GP +/- L10
 1. Jets            47  18   3
 2. Avalanche       49  18   4
 3. Stars           49  17   5
 4. Blues           49   5   3
 5. Predators       51   3  -1
 6. Coyotes         48   1  -1
 7. Wild            49  -2  -1
 8. Blackhawks      50 -20  -6

==============================
       Pacific division
==============================
                    GP +/- L10
 1. Canucks         49  22   8
 2. Oilers          45  14  10
 3. Golden Knights  50  14   3
 4. Kings           48   8  -2
 5. Kraken          50   2  -1
 6. Flames          49   0   0
 7. Ducks           50 -12  -1
 8. Sharks          51 -18   2

==============================
      Eastern conference
==============================
                    GP +/- L10
 1. Bruins          49  22   6
 2. Panthers        49  17   4
 3. Rangers         49  14  -1
 4. Hurricanes      48  13   5
 5. Maple Leafs     47  11   1
 6. Lightning       50   9   6
 7. Red Wings       50   8   4
 8. Flyers          50   6   0
 9. Penguins        46   5   1
10. Devils          47   4  -3
11. Capitals        47   4  -1
12. Islanders       49   3  -4
13. Sabres          49  -1   2
14. Canadiens       49  -1  -1
15. Senators        47  -5   4
16. Blue Jackets    50  -8  -2

==============================
      Western conference
==============================
                    GP +/- L10
 1. Canucks         49  22   8
 2. Jets            47  18   3
 3. Avalanche       49  18   4
 4. Stars           49  17   5
 5. Oilers          45  14  10
 6. Golden Knights  50  14   3
 7. Kings           48   8  -2
 8. Blues           49   5   3
 9. Predators       51   3  -1
10. Kraken          50   2  -1
11. Coyotes         48   1  -1
12. Flames          49   0   0
13. Wild            49  -2  -1
14. Ducks           50 -12  -1
15. Sharks          51 -18   2
16. Blackhawks      50 -20  -6

==============================
         Full league
==============================
                    GP +/- L10
 1. Canucks         49  22   8
 2. Bruins          49  22   6
 3. Jets            47  18   3
 4. Avalanche       49  18   4
 5. Panthers        49  17   4
 6. Stars           49  17   5
 7. Oilers          45  14  10
 8. Rangers         49  14  -1
 9. Golden Knights  50  14   3
10. Hurricanes      48  13   5
11. Maple Leafs     47  11   1
12. Lightning       50   9   6
13. Kings           48   8  -2
14. Red Wings       50   8   4
15. Flyers          50   6   0
16. Penguins        46   5   1
17. Blues           49   5   3
18. Devils          47   4  -3
19. Capitals        47   4  -1
20. Islanders       49   3  -4
21. Predators       51   3  -1
22. Kraken          50   2  -1
23. Coyotes         48   1  -1
24. Flames          49   0   0
25. Sabres          49  -1   2
26. Canadiens       49  -1  -1
27. Wild            49  -2  -1
28. Senators        47  -5   4
29. Blue Jackets    50  -8  -2
30. Ducks           50 -12  -1
31. Sharks          51 -18   2
32. Blackhawks      50 -20  -6
```

## Resources

* New API docs: <https://gitlab.com/dword4/nhlapi/-/blob/master/new-api.md>
* Transform tools, JSON to Rust Serde: <https://transform.tools/json-to-rust-serde>
