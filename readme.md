# nhl-rs

NHL stats as I like 'em.

## Usage and sample output

The `nhl-rs` executable is a simple command-line tool that fetches the current NHL standings or schedule, and displays them in a table format.

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
 1. Toronto      46  12   2   9
 2. Florida      46   9  -1   8
 3. Tampa Bay    43   8  -1  35
 4. Ottawa       44   4   0   3
 5. Montréal     43   3   5 -12
 6. Boston       46   3  -3 -22
 7. Detroit      44   2   6 -12
 8. Buffalo      44  -5   3 -12
                 👉🏻  36  11  -3 0.551

===============================
     Metropolitan division
===============================
                 GP +/- L10  GD
 1. Washington   45  20   5  47
 2. New Jersey   47  11  -1  26
 3. Carolina     45  10   2  22
 4. Columbus     45   5   6   2
 5. NY Rangers   43   0   0 -10
 6. Philadelphia 45  -1   0 -19
 7. NY Islanders 43  -2   0 -17
 8. Pittsburgh   46  -2  -3 -36
                 👉🏻  41   9  15 0.557

===============================
       Central division
===============================
                 GP +/- L10  GD
 1. Winnipeg     45  18   4  53
 2. Dallas       43  14   7  35
 3. Minnesota    45  13   2   8
 4. Colorado     45  10   5  10
 5. St. Louis    45   1   2  -7
 6. Utah         43   0  -5 -10
 7. Nashville    43  -8   0 -29
 8. Chicago      44 -14  -6 -39
                 👉🏻  34   9  21 0.548

===============================
       Pacific division
===============================
                 GP +/- L10  GD
 1. Vegas        44  17   2  32
 2. Edmonton     44  15   5  25
 3. Los Angeles  41  12   3  21
 4. Calgary      43   6   2 -11
 5. Vancouver    43   5  -1 -11
 6. Anaheim      45  -3   0 -31
 7. Seattle      45  -4  -1 -14
 8. San Jose     47 -13  -4 -44
                 👉🏻  35   6 -33 0.550

===============================
      Eastern conference
===============================
                 GP +/- L10  GD
 1. Washington   45  20   5  47
 2. Toronto      46  12   2   9
 3. New Jersey   47  11  -1  26
 4. Carolina     45  10   2  22
 5. Florida      46   9  -1   8
 6. Tampa Bay    43   8  -1  35
 7. Columbus     45   5   6   2
 8. Ottawa       44   4   0   3
 9. Montréal     43   3   5 -12
10. Boston       46   3  -3 -22
11. Detroit      44   2   6 -12
12. NY Rangers   43   0   0 -10
13. Philadelphia 45  -1   0 -19
14. NY Islanders 43  -2   0 -17
15. Pittsburgh   46  -2  -3 -36
16. Buffalo      44  -5   3 -12
                 👉🏻  77  20  12 0.554

===============================
      Western conference
===============================
                 GP +/- L10  GD
 1. Winnipeg     45  18   4  53
 2. Vegas        44  17   2  32
 3. Edmonton     44  15   5  25
 4. Dallas       43  14   7  35
 5. Minnesota    45  13   2   8
 6. Los Angeles  41  12   3  21
 7. Colorado     45  10   5  10
 8. Calgary      43   6   2 -11
 9. Vancouver    43   5  -1 -11
10. St. Louis    45   1   2  -7
11. Utah         43   0  -5 -10
12. Anaheim      45  -3   0 -31
13. Seattle      45  -4  -1 -14
14. Nashville    43  -8   0 -29
15. San Jose     47 -13  -4 -44
16. Chicago      44 -14  -6 -39
                 👉🏻  69  15 -12 0.549

===============================
          Full league
===============================
                 GP +/- L10  GD
 1. Washington   45  20   5  47
 2. Winnipeg     45  18   4  53
 3. Vegas        44  17   2  32
 4. Edmonton     44  15   5  25
 5. Dallas       43  14   7  35
 6. Minnesota    45  13   2   8
 7. Los Angeles  41  12   3  21
 8. Toronto      46  12   2   9
 9. New Jersey   47  11  -1  26
10. Carolina     45  10   2  22
11. Colorado     45  10   5  10
12. Florida      46   9  -1   8
13. Tampa Bay    43   8  -1  35
14. Calgary      43   6   2 -11
15. Vancouver    43   5  -1 -11
16. Columbus     45   5   6   2
17. Ottawa       44   4   0   3
18. Montréal     43   3   5 -12
19. Boston       46   3  -3 -22
20. Detroit      44   2   6 -12
21. St. Louis    45   1   2  -7
22. NY Rangers   43   0   0 -10
23. Utah         43   0  -5 -10
24. Philadelphia 45  -1   0 -19
25. NY Islanders 43  -2   0 -17
26. Pittsburgh   46  -2  -3 -36
27. Anaheim      45  -3   0 -31
28. Seattle      45  -4  -1 -14
29. Buffalo      44  -5   3 -12
30. Nashville    43  -8   0 -29
31. San Jose     47 -13  -4 -44
32. Chicago      44 -14  -6 -39
                 👉🏻 146  35   0 0.551

```

## Resources

* New API docs: <https://gitlab.com/dword4/nhlapi/-/blob/master/new-api.md>
* Great docs: <https://github.com/Zmalski/NHL-API-Reference>
* Transform tools, JSON to Rust Serde: <https://transform.tools/json-to-rust-serde>
