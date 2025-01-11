# nhl-rs

NHL stats as I like 'em.

## Usage and sample output

The `nhl-rs` executable is a simple command-line tool that fetches the current NHL standings or schedule, and displays them in a tabular format.

```bash
$ nhl-rs -h

Usage: nhl-rs [OPTIONS]

Options:
  -l, --local       use local data
  -p, --playoffs    display the playoffs picture
  -s, --schedule    display the proximate schedule
  -d, --division    display just division standings
  -c, --conference  display just conference standings
  -f, --full        display just the full league standings
      --save        Save sample data to file
  -h, --help        Print help
  -V, --version     Print version
```

The output is segmented into the four divisions, two conferences, and the full league. The teams are sorted by wins minus losses, and the display is games played, wins minus losses, and the last ten games' wins minus losses.

```bash
$ nhl-rs

===============================
       Atlantic division
===============================
                 GP +/- L10  GD
 1. Toronto      43  13   2  14
 2. Florida      43  10   1  11
 3. Tampa Bay    39   7   0  36
 4. Montréal     41   2   6 -13
 5. Boston       45   2  -3 -26
 6. Ottawa       40   1  -1  -4
 7. Detroit      41   1   2 -16
 8. Buffalo      42  -5   1 -10
                 👉🏻  31   8  -8 0.546

===============================
     Metropolitan division
===============================
                 GP +/- L10  GD
 1. Washington   42  17   3  40
 2. Carolina     43  11   1  25
 3. New Jersey   44  10  -1  27
 4. Columbus     42   2   5  -3
 5. Pittsburgh   43   1   1 -26
 6. NY Rangers   41  -1  -1 -10
 7. NY Islanders 41  -2  -2 -16
 8. Philadelphia 42  -3  -3 -25
                 👉🏻  35   3  12 0.552

===============================
       Central division
===============================
                 GP +/- L10  GD
 1. Winnipeg     43  16   2  45
 2. Minnesota    42  14   2  11
 3. Dallas       40  13   5  32
 4. Colorado     43  10   7  12
 5. Utah         41   2  -2  -7
 6. St. Louis    43   1   1  -7
 7. Nashville    41  -8   1 -28
 8. Chicago      42 -12  -4 -35
                 👉🏻  36  12  23 0.554

===============================
       Pacific division
===============================
                 GP +/- L10  GD
 1. Vegas        41  18   6  32
 2. Los Angeles  39  14   6  23
 3. Edmonton     41  12   5  21
 4. Calgary      40   5   2 -14
 5. Vancouver    41   5  -2  -9
 6. Anaheim      41  -2   1 -22
 7. Seattle      42  -5  -5 -16
 8. San Jose     44 -12  -5 -42
                 👉🏻  35   8 -27 0.553

===============================
      Eastern conference
===============================
                 GP +/- L10  GD
 1. Washington   42  17   3  40
 2. Toronto      43  13   2  14
 3. Carolina     43  11   1  25
 4. Florida      43  10   1  11
 5. New Jersey   44  10  -1  27
 6. Tampa Bay    39   7   0  36
 7. Montréal     41   2   6 -13
 8. Columbus     42   2   5  -3
 9. Boston       45   2  -3 -26
10. Ottawa       40   1  -1  -4
11. Detroit      41   1   2 -16
12. Pittsburgh   43   1   1 -26
13. NY Rangers   41  -1  -1 -10
14. NY Islanders 41  -2  -2 -16
15. Philadelphia 42  -3  -3 -25
16. Buffalo      42  -5   1 -10
                 👉🏻  66  11   4 0.549

===============================
      Western conference
===============================
                 GP +/- L10  GD
 1. Vegas        41  18   6  32
 2. Winnipeg     43  16   2  45
 3. Los Angeles  39  14   6  23
 4. Minnesota    42  14   2  11
 5. Dallas       40  13   5  32
 6. Edmonton     41  12   5  21
 7. Colorado     43  10   7  12
 8. Calgary      40   5   2 -14
 9. Vancouver    41   5  -2  -9
10. Utah         41   2  -2  -7
11. St. Louis    43   1   1  -7
12. Anaheim      41  -2   1 -22
13. Seattle      42  -5  -5 -16
14. Nashville    41  -8   1 -28
15. Chicago      42 -12  -4 -35
16. San Jose     44 -12  -5 -42
                 👉🏻  71  20  -4 0.553

===============================
          Full league
===============================
                 GP +/- L10  GD
 1. Vegas        41  18   6  32
 2. Washington   42  17   3  40
 3. Winnipeg     43  16   2  45
 4. Los Angeles  39  14   6  23
 5. Minnesota    42  14   2  11
 6. Dallas       40  13   5  32
 7. Toronto      43  13   2  14
 8. Edmonton     41  12   5  21
 9. Carolina     43  11   1  25
10. Colorado     43  10   7  12
11. Florida      43  10   1  11
12. New Jersey   44  10  -1  27
13. Tampa Bay    39   7   0  36
14. Calgary      40   5   2 -14
15. Vancouver    41   5  -2  -9
16. Montréal     41   2   6 -13
17. Utah         41   2  -2  -7
18. Columbus     42   2   5  -3
19. Boston       45   2  -3 -26
20. Ottawa       40   1  -1  -4
21. Detroit      41   1   2 -16
22. St. Louis    43   1   1  -7
23. Pittsburgh   43   1   1 -26
24. NY Rangers   41  -1  -1 -10
25. Anaheim      41  -2   1 -22
26. NY Islanders 41  -2  -2 -16
27. Philadelphia 42  -3  -3 -25
28. Seattle      42  -5  -5 -16
29. Buffalo      42  -5   1 -10
30. Nashville    41  -8   1 -28
31. Chicago      42 -12  -4 -35
32. San Jose     44 -12  -5 -42
                 👉🏻 137  31   0 0.551
```

## Resources

* New API docs: <https://gitlab.com/dword4/nhlapi/-/blob/master/new-api.md>
* Great docs: <https://github.com/Zmalski/NHL-API-Reference>
* Transform tools, JSON to Rust Serde: <https://transform.tools/json-to-rust-serde>
