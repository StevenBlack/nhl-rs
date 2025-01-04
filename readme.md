# nhl-rs

NHL stats as I like 'em.

## Usage and sample output

The `nhl-rs` executable is a simple command-line tool that fetches the current NHL standings or schedule, and displays them in a tabular format.

```bash
 nhl-rs -h
Usage: nhl-rs [OPTIONS]

Options:
  -l, --local     use local data
  -p, --playoffs  display the playoffs picture
  -s, --schedule  display the proximate schedule
      --save      Save sample data to file
  -h, --help      Print help
  -V, --version   Print version
```

The output is segmented into the four divisions, two conferences, and the full league. The teams are sorted by wins minus losses, and the display is games played, wins minus losses, and the last ten games' wins minus losses.

```bash
$ nhl-rs

===============================
       Atlantic division
===============================
                 GP +/- L10  GD
 1. Toronto      39  11   2  14
 2. Florida      39   9   0   2
 3. Tampa Bay    35   7   2  23
 4. Boston       40   4   1   7
 5. Ottawa       37   3   4  11
 6. Montréal     37   0   4  -4
 7. Detroit      38  -2   0 -11
 8. Buffalo      39  -6  -3  -6
                 👉🏻  26  10  -1 0.543

===============================
     Metropolitan division
===============================
                 GP +/- L10  GD
 1. Washington   38  15   1  17
 2. Carolina     38  10   1  28
 3. New Jersey   41  10   2  15
 4. Pittsburgh   39   0   1 -11
 5. Columbus     39  -1   0  17
 6. Philadelphia 39  -1  -2  -9
 7. NY Rangers   37  -2  -4 -14
 8. NY Islanders 39  -4  -4 -13
                 👉🏻  27  -5   3 0.544

===============================
       Central division
===============================
                 GP +/- L10  GD
 1. Winnipeg     40  16   4  32
 2. Minnesota    39  13   0  -8
 3. Dallas       37  10   3  24
 4. Colorado     39   9   6  -2
 5. Utah         38   2   1 -11
 6. St. Louis    39   1   0 -13
 7. Nashville    38  -9  -1   0
 8. Chicago      38 -12  -4 -11
                 👉🏻  30   9  11 0.549

===============================
       Pacific division
===============================
                 GP +/- L10  GD
 1. Vegas        38  17   6  28
 2. Los Angeles  37  12   4  23
 3. Edmonton     37  10   5   6
 4. Vancouver    37   7   1 -12
 5. Calgary      38   5   2  10
 6. Anaheim      37  -1   2 -15
 7. Seattle      39  -2  -2  -2
 8. San Jose     41 -11  -5 -12
                 👉🏻  37  13 -13 0.561

===============================
      Eastern conference
===============================
                 GP +/- L10  GD
 1. Washington   38  15   1  17
 2. Toronto      39  11   2  14
 3. Carolina     38  10   1  28
 4. New Jersey   41  10   2  15
 5. Florida      39   9   0   2
 6. Tampa Bay    35   7   2  23
 7. Boston       40   4   1   7
 8. Ottawa       37   3   4  11
 9. Montréal     37   0   4  -4
10. Pittsburgh   39   0   1 -11
11. Columbus     39  -1   0  17
12. Philadelphia 39  -1  -2  -9
13. NY Rangers   37  -2  -4 -14
14. Detroit      38  -2   0 -11
15. NY Islanders 39  -4  -4 -13
16. Buffalo      39  -6  -3  -6
                 👉🏻  53   5   2 0.543

===============================
      Western conference
===============================
                 GP +/- L10  GD
 1. Vegas        38  17   6  28
 2. Winnipeg     40  16   4  32
 3. Minnesota    39  13   0  -8
 4. Los Angeles  37  12   4  23
 5. Dallas       37  10   3  24
 6. Edmonton     37  10   5   6
 7. Colorado     39   9   6  -2
 8. Vancouver    37   7   1 -12
 9. Calgary      38   5   2  10
10. Utah         38   2   1 -11
11. St. Louis    39   1   0 -13
12. Anaheim      37  -1   2 -15
13. Seattle      39  -2  -2  -2
14. Nashville    38  -9  -1   0
15. San Jose     41 -11  -5 -12
16. Chicago      38 -12  -4 -11
                 👉🏻  67  22  -2 0.555

===============================
          Full league
===============================
                 GP +/- L10  GD
 1. Vegas        38  17   6  28
 2. Winnipeg     40  16   4  32
 3. Washington   38  15   1  17
 4. Minnesota    39  13   0  -8
 5. Los Angeles  37  12   4  23
 6. Toronto      39  11   2  14
 7. Dallas       37  10   3  24
 8. Edmonton     37  10   5   6
 9. Carolina     38  10   1  28
10. New Jersey   41  10   2  15
11. Colorado     39   9   6  -2
12. Florida      39   9   0   2
13. Tampa Bay    35   7   2  23
14. Vancouver    37   7   1 -12
15. Calgary      38   5   2  10
16. Boston       40   4   1   7
17. Ottawa       37   3   4  11
18. Utah         38   2   1 -11
19. St. Louis    39   1   0 -13
20. Montréal     37   0   4  -4
21. Pittsburgh   39   0   1 -11
22. Anaheim      37  -1   2 -15
23. Columbus     39  -1   0  17
24. Philadelphia 39  -1  -2  -9
25. NY Rangers   37  -2  -4 -14
26. Detroit      38  -2   0 -11
27. Seattle      39  -2  -2  -2
28. NY Islanders 39  -4  -4 -13
29. Buffalo      39  -6  -3  -6
30. Nashville    38  -9  -1   0
31. San Jose     41 -11  -5 -12
32. Chicago      38 -12  -4 -11
                 👉🏻 120  27   0 0.549
```

## Resources

* New API docs: <https://gitlab.com/dword4/nhlapi/-/blob/master/new-api.md>
* Great docs: <https://github.com/Zmalski/NHL-API-Reference>
* Transform tools, JSON to Rust Serde: <https://transform.tools/json-to-rust-serde>
