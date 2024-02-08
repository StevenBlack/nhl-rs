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
 1. Boston       50  21   5
 2. Florida      50  16   2
 3. Toronto      49  11   0
 4. Detroit      50   8   4
 5. Tampa Bay    51   8   6
 6. Montréal     50   0   0
 7. Buffalo      50  -2   0
 8. Ottawa       47  -5   4

===========================
   Metropolitan division
===========================
                 GP +/- L10
 1. NY Rangers   51  16   3
 2. Carolina     49  12   3
 3. Philadelphia 51   7   0
 4. Pittsburgh   47   6   1
 5. New Jersey   48   5  -1
 6. NY Islanders 50   4  -2
 7. Washington   48   3  -3
 8. Columbus     50  -8  -2

===========================
     Central division
===========================
                 GP +/- L10
 1. Winnipeg     48  17   1
 2. Colorado     51  17   3
 3. Dallas       51  17   5
 4. St Louis     49   5   3
 5. Nashville    51   3  -1
 6. Arizona      48   1  -1
 7. Minnesota    50  -1   1
 8. Chicago      51 -21  -6

===========================
     Pacific division
===========================
                 GP +/- L10
 1. Vancouver    50  23   8
 2. Vegas        51  15   5
 3. Edmonton     46  13   8
 4. Los Angeles  48   8  -2
 5. Seattle      50   2  -1
 6. Calgary      50   1   2
 7. Anaheim      50 -12  -1
 8. San Jose     51 -18   2

===========================
    Eastern conference
===========================
                 GP +/- L10
 1. Boston       50  21   5
 2. Florida      50  16   2
 3. NY Rangers   51  16   3
 4. Carolina     49  12   3
 5. Toronto      49  11   0
 6. Detroit      50   8   4
 7. Tampa Bay    51   8   6
 8. Philadelphia 51   7   0
 9. Pittsburgh   47   6   1
10. New Jersey   48   5  -1
11. NY Islanders 50   4  -2
12. Washington   48   3  -3
13. Montréal     50   0   0
14. Buffalo      50  -2   0
15. Ottawa       47  -5   4
16. Columbus     50  -8  -2

===========================
    Western conference
===========================
                 GP +/- L10
 1. Vancouver    50  23   8
 2. Winnipeg     48  17   1
 3. Colorado     51  17   3
 4. Dallas       51  17   5
 5. Vegas        51  15   5
 6. Edmonton     46  13   8
 7. Los Angeles  48   8  -2
 8. St Louis     49   5   3
 9. Nashville    51   3  -1
10. Seattle      50   2  -1
11. Arizona      48   1  -1
12. Calgary      50   1   2
13. Minnesota    50  -1   1
14. Anaheim      50 -12  -1
15. San Jose     51 -18   2
16. Chicago      51 -21  -6

===========================
        Full league
===========================
                 GP +/- L10
 1. Vancouver    50  23   8
 2. Boston       50  21   5
 3. Winnipeg     48  17   1
 4. Colorado     51  17   3
 5. Dallas       51  17   5
 6. Florida      50  16   2
 7. NY Rangers   51  16   3
 8. Vegas        51  15   5
 9. Edmonton     46  13   8
10. Carolina     49  12   3
11. Toronto      49  11   0
12. Los Angeles  48   8  -2
13. Detroit      50   8   4
14. Tampa Bay    51   8   6
15. Philadelphia 51   7   0
16. Pittsburgh   47   6   1
17. New Jersey   48   5  -1
18. St Louis     49   5   3
19. NY Islanders 50   4  -2
20. Washington   48   3  -3
21. Nashville    51   3  -1
22. Seattle      50   2  -1
23. Arizona      48   1  -1
24. Calgary      50   1   2
25. Montréal     50   0   0
26. Minnesota    50  -1   1
27. Buffalo      50  -2   0
28. Ottawa       47  -5   4
29. Columbus     50  -8  -2
30. Anaheim      50 -12  -1
31. San Jose     51 -18   2
32. Chicago      51 -21  -6
```

## Resources

* New API docs: <https://gitlab.com/dword4/nhlapi/-/blob/master/new-api.md>
* Great docs: <https://github.com/Zmalski/NHL-API-Reference>
* Transform tools, JSON to Rust Serde: <https://transform.tools/json-to-rust-serde>
