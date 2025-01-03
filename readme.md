# nhl-rs

NHL stats as I like 'em.

## Usage and sample output

The `nhl-rs` executable is a simple command-line tool that fetches the current NHL standings and displays them in a tabular format.

The output is segmented into the four divisions, two conferences, and the full league. The teams are sorted by wins minus losses, and the display is games played, wins minus losses, and the last ten games' wins minus losses.

```bash
$ nhl-rs

===========================
     Atlantic division
===========================
                 GP +/- L10
 1. Toronto      39  11   2
 2. Florida      39   9   0
 3. Tampa Bay    35   7   2
 4. Boston       40   4   1
 5. Ottawa       37   3   4
 6. Montréal     37   0   4
 7. Detroit      38  -2   0
 8. Buffalo      39  -6  -3
                 👉🏻  26  10 0.543

===========================
   Metropolitan division
===========================
                 GP +/- L10
 1. Washington   38  15   1
 2. Carolina     38  10   1
 3. New Jersey   41  10   2
 4. Pittsburgh   39   0   1
 5. Columbus     39  -1   0
 6. Philadelphia 39  -1  -2
 7. NY Rangers   37  -2  -4
 8. NY Islanders 39  -4  -4
                 👉🏻  27  -5 0.544

===========================
     Central division
===========================
                 GP +/- L10
 1. Winnipeg     40  16   4
 2. Minnesota    39  13   0
 3. Dallas       37  10   3
 4. Colorado     39   9   6
 5. Utah         38   2   1
 6. St. Louis    39   1   0
 7. Nashville    38  -9  -1
 8. Chicago      38 -12  -4
                 👉🏻  30   9 0.549

===========================
     Pacific division
===========================
                 GP +/- L10
 1. Vegas        38  17   6
 2. Los Angeles  37  12   4
 3. Edmonton     37  10   5
 4. Vancouver    37   7   1
 5. Calgary      38   5   2
 6. Anaheim      37  -1   2
 7. Seattle      39  -2  -2
 8. San Jose     41 -11  -5
                 👉🏻  37  13 0.561

===========================
    Eastern conference
===========================
                 GP +/- L10
 1. Washington   38  15   1
 2. Toronto      39  11   2
 3. Carolina     38  10   1
 4. New Jersey   41  10   2
 5. Florida      39   9   0
 6. Tampa Bay    35   7   2
 7. Boston       40   4   1
 8. Ottawa       37   3   4
 9. Montréal     37   0   4
10. Pittsburgh   39   0   1
11. Columbus     39  -1   0
12. Philadelphia 39  -1  -2
13. NY Rangers   37  -2  -4
14. Detroit      38  -2   0
15. NY Islanders 39  -4  -4
16. Buffalo      39  -6  -3
                 👉🏻  53   5 0.543

===========================
    Western conference
===========================
                 GP +/- L10
 1. Vegas        38  17   6
 2. Winnipeg     40  16   4
 3. Minnesota    39  13   0
 4. Los Angeles  37  12   4
 5. Dallas       37  10   3
 6. Edmonton     37  10   5
 7. Colorado     39   9   6
 8. Vancouver    37   7   1
 9. Calgary      38   5   2
10. Utah         38   2   1
11. St. Louis    39   1   0
12. Anaheim      37  -1   2
13. Seattle      39  -2  -2
14. Nashville    38  -9  -1
15. San Jose     41 -11  -5
16. Chicago      38 -12  -4
                 👉🏻  67  22 0.555

===========================
        Full league
===========================
                 GP +/- L10
 1. Vegas        38  17   6
 2. Winnipeg     40  16   4
 3. Washington   38  15   1
 4. Minnesota    39  13   0
 5. Los Angeles  37  12   4
 6. Toronto      39  11   2
 7. Dallas       37  10   3
 8. Edmonton     37  10   5
 9. Carolina     38  10   1
10. New Jersey   41  10   2
11. Colorado     39   9   6
12. Florida      39   9   0
13. Tampa Bay    35   7   2
14. Vancouver    37   7   1
15. Calgary      38   5   2
16. Boston       40   4   1
17. Ottawa       37   3   4
18. Utah         38   2   1
19. St. Louis    39   1   0
20. Montréal     37   0   4
21. Pittsburgh   39   0   1
22. Anaheim      37  -1   2
23. Columbus     39  -1   0
24. Philadelphia 39  -1  -2
25. NY Rangers   37  -2  -4
26. Detroit      38  -2   0
27. Seattle      39  -2  -2
28. NY Islanders 39  -4  -4
29. Buffalo      39  -6  -3
30. Nashville    38  -9  -1
31. San Jose     41 -11  -5
32. Chicago      38 -12  -4
                 👉🏻 120  27 0.549
```

## Resources

* New API docs: <https://gitlab.com/dword4/nhlapi/-/blob/master/new-api.md>
* Great docs: <https://github.com/Zmalski/NHL-API-Reference>
* Transform tools, JSON to Rust Serde: <https://transform.tools/json-to-rust-serde>
