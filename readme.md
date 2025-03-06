# nhl

National Hockey League (NHL) stats as I like 'em.

## Usage and sample output

The `nhl` executable is a simple command-line tool that fetches the current NHL standings or schedule, and displays them in a table format.

```bash
$ nhl -h

Usage: nhl [OPTIONS]

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
$ nhl

===================================
         Atlantic division
===================================
                 GP +/- L10  GD L10
 1. Toronto      61  18   7  21  13
 2. Florida      62  17   6  34  16
 3. Tampa Bay    61  15   8  58  24
 4. Ottawa       60   5  -1  -3  -4
 5. Detroit      61   5   1 -16  -4
 6. Montréal     61   4   2 -20   2
 7. Boston       63   1  -2 -31  -2
 8. Buffalo      60  -6   1 -17  -2
                 👉🏻  59  22  26  43 0.560

===================================
       Metropolitan division
===================================
                 GP +/- L10  GD L10
 1. Washington   61  25   2  59   5
 2. Carolina     62  14  -2  26  -6
 3. New Jersey   63   9  -2  31   0
 4. Columbus     61   7   1   5   3
 5. NY Rangers   61   5   4   4  10
 6. NY Islanders 61   2  -2 -14 -10
 7. Philadelphia 62   0   0 -25  -1
 8. Pittsburgh   64  -6  -4 -53 -17
                 👉🏻  56  -3  33 -16 0.557

===================================
         Central division
===================================
                 GP +/- L10  GD L10
 1. Winnipeg     62  26   5  70  11
 2. Dallas       61  21   5  52  14
 3. Minnesota    62  14   0  -2 -11
 4. Colorado     62  12   4  19  14
 5. Utah         61   2   2  -9   1
 6. St. Louis    62   2   4  -8   9
 7. Nashville    61 -10  -2 -45 -15
 8. Chicago      61 -16  -2 -45  -5
                 👉🏻  51  16  32  18 0.552

===================================
         Pacific division
===================================
                 GP +/- L10  GD L10
 1. Vegas        60  18   2  33   1
 2. Edmonton     61  13  -4  17 -13
 3. Los Angeles  59  11   2   8  -2
 4. Calgary      61   6   0 -18  -6
 5. Vancouver    60   5  -1 -19  -6
 6. Anaheim      60   1   3 -25   3
 7. Seattle      62  -6  -1 -19  -4
 8. San Jose     63 -20  -3 -68 -11
                 👉🏻  28  -2 -91 -38 0.529

===================================
        Eastern conference
===================================
                 GP +/- L10  GD L10
 1. Washington   61  25   2  59   5
 2. Toronto      61  18   7  21  13
 3. Florida      62  17   6  34  16
 4. Tampa Bay    61  15   8  58  24
 5. Carolina     62  14  -2  26  -6
 6. New Jersey   63   9  -2  31   0
 7. Columbus     61   7   1   5   3
 8. Ottawa       60   5  -1  -3  -4
 9. NY Rangers   61   5   4   4  10
10. Detroit      61   5   1 -16  -4
11. Montréal     61   4   2 -20   2
12. NY Islanders 61   2  -2 -14 -10
13. Boston       63   1  -2 -31  -2
14. Philadelphia 62   0   0 -25  -1
15. Buffalo      60  -6   1 -17  -2
16. Pittsburgh   64  -6  -4 -53 -17
                 👉🏻 115  19  59  27 0.558

===================================
        Western conference
===================================
                 GP +/- L10  GD L10
 1. Winnipeg     62  26   5  70  11
 2. Dallas       61  21   5  52  14
 3. Vegas        60  18   2  33   1
 4. Minnesota    62  14   0  -2 -11
 5. Edmonton     61  13  -4  17 -13
 6. Colorado     62  12   4  19  14
 7. Los Angeles  59  11   2   8  -2
 8. Calgary      61   6   0 -18  -6
 9. Vancouver    60   5  -1 -19  -6
10. Utah         61   2   2  -9   1
11. St. Louis    62   2   4  -8   9
12. Anaheim      60   1   3 -25   3
13. Seattle      62  -6  -1 -19  -4
14. Nashville    61 -10  -2 -45 -15
15. Chicago      61 -16  -2 -45  -5
16. San Jose     63 -20  -3 -68 -11
                 👉🏻  79  14 -59 -20 0.540

===================================
            Full league
===================================
                 GP +/- L10  GD L10
 1. Winnipeg     62  26   5  70  11
 2. Washington   61  25   2  59   5
 3. Dallas       61  21   5  52  14
 4. Vegas        60  18   2  33   1
 5. Toronto      61  18   7  21  13
 6. Florida      62  17   6  34  16
 7. Tampa Bay    61  15   8  58  24
 8. Carolina     62  14  -2  26  -6
 9. Minnesota    62  14   0  -2 -11
10. Edmonton     61  13  -4  17 -13
11. Colorado     62  12   4  19  14
12. Los Angeles  59  11   2   8  -2
13. New Jersey   63   9  -2  31   0
14. Columbus     61   7   1   5   3
15. Calgary      61   6   0 -18  -6
16. Ottawa       60   5  -1  -3  -4
17. Vancouver    60   5  -1 -19  -6
18. NY Rangers   61   5   4   4  10
19. Detroit      61   5   1 -16  -4
20. Montréal     61   4   2 -20   2
21. NY Islanders 61   2  -2 -14 -10
22. Utah         61   2   2  -9   1
23. St. Louis    62   2   4  -8   9
24. Anaheim      60   1   3 -25   3
25. Boston       63   1  -2 -31  -2
26. Philadelphia 62   0   0 -25  -1
27. Buffalo      60  -6   1 -17  -2
28. Seattle      62  -6  -1 -19  -4
29. Pittsburgh   64  -6  -4 -53 -17
30. Nashville    61 -10  -2 -45 -15
31. Chicago      61 -16  -2 -45  -5
32. San Jose     63 -20  -3 -68 -11
                 👉🏻 194  33   0   7 0.549

===================================
    Full league (last 10 games)
===================================
                 GP +/- L10  GD L10
 1. Tampa Bay    61  15   8  58  24
 2. Toronto      61  18   7  21  13
 3. Florida      62  17   6  34  16
 4. Winnipeg     62  26   5  70  11
 5. Dallas       61  21   5  52  14
 6. Colorado     62  12   4  19  14
 7. NY Rangers   61   5   4   4  10
 8. St. Louis    62   2   4  -8   9
 9. Anaheim      60   1   3 -25   3
10. Washington   61  25   2  59   5
11. Vegas        60  18   2  33   1
12. Los Angeles  59  11   2   8  -2
13. Montréal     61   4   2 -20   2
14. Utah         61   2   2  -9   1
15. Columbus     61   7   1   5   3
16. Detroit      61   5   1 -16  -4
17. Buffalo      60  -6   1 -17  -2
18. Minnesota    62  14   0  -2 -11
19. Calgary      61   6   0 -18  -6
20. Philadelphia 62   0   0 -25  -1
21. Ottawa       60   5  -1  -3  -4
22. Vancouver    60   5  -1 -19  -6
23. Seattle      62  -6  -1 -19  -4
24. Carolina     62  14  -2  26  -6
25. New Jersey   63   9  -2  31   0
26. NY Islanders 61   2  -2 -14 -10
27. Boston       63   1  -2 -31  -2
28. Nashville    61 -10  -2 -45 -15
29. Chicago      61 -16  -2 -45  -5
30. San Jose     63 -20  -3 -68 -11
31. Edmonton     61  13  -4  17 -13
32. Pittsburgh   64  -6  -4 -53 -17
                 👉🏻 194  33   0   7 0.549
                 
```

## Resources

* New API docs: <https://gitlab.com/dword4/nhlapi/-/blob/master/new-api.md>
* Great docs: <https://github.com/Zmalski/NHL-API-Reference>
* Transform tools, JSON to Rust Serde: <https://transform.tools/json-to-rust-serde>
