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



=======================================
           Atlantic division
=======================================
                 GP +/- L10  RW  GD L10
 1. Montréal     13   6   5   4   7   5
 2. Detroit      14   4   2   6   2  -1
 3. Toronto      13   2   3   5   0   1
 4. Tampa Bay    13   1   2   4   1   3
 5. Buffalo      13   1   4   4  -2   6
 6. Ottawa       13   1   2   4  -3   3
 7. Boston       15   1   0   5  -2  -5
 8. Florida      13   0  -3   5  -7 -13
                 👉🏻  16  15  37  -4  -1 0.575

=======================================
         Metropolitan division
=======================================
                 GP +/- L10  RW  GD L10
 1. New Jersey   13   5   4   8   6   6
 2. Carolina     12   4   2   5  12   8
 3. Pittsburgh   14   4   4   8   9  11
 4. Columbus     12   2   2   5   4   2
 5. Philadelphia 13   2   2   3   3   2
 6. Washington   12   1   1   5   2   2
 7. NY Islanders 13   1   4   6  -3   3
 8. NY Rangers   14   0   0   4  -3  -8
                 👉🏻  19  19  44  30  26 0.592

=======================================
           Central division
=======================================
                 GP +/- L10  RW  GD L10
 1. Colorado     14   7   4   8  13   8
 2. Winnipeg     13   5   4   8  12   9
 3. Utah         13   5   6   6   9  11
 4. Dallas       13   4   1   5  -2  -7
 5. Chicago      13   0   2   4   1   4
 6. Minnesota    14  -1  -1   3 -11 -11
 7. Nashville    15  -1  -2   4 -12 -11
 8. St. Louis    13  -3  -4   4 -16 -16
                 👉🏻  16  10  42  -6 -13 0.574

=======================================
           Pacific division
=======================================
                 GP +/- L10  RW  GD L10
 1. Anaheim      12   5   5   6  11  12
 2. Vegas        12   5   4   6   8   8
 3. Seattle      12   4   2   4   0  -3
 4. Los Angeles  14   2   3   3  -4   0
 5. Edmonton     15   1   1   4  -2  -1
 6. Vancouver    14   0   0   3  -5  -6
 7. San Jose     13  -2  -1   1 -11  -5
 8. Calgary      14  -6  -4   2 -17 -10
                 👉🏻   9  10  29 -20  -5 0.542

=======================================
          Eastern conference
=======================================
                 GP +/- L10  RW  GD L10
 1. Montréal     13   6   5   4   7   5
 2. New Jersey   13   5   4   8   6   6
 3. Carolina     12   4   2   5  12   8
 4. Pittsburgh   14   4   4   8   9  11
 5. Detroit      14   4   2   6   2  -1
 6. Columbus     12   2   2   5   4   2
 7. Toronto      13   2   3   5   0   1
 8. Philadelphia 13   2   2   3   3   2
 9. Washington   12   1   1   5   2   2
10. NY Islanders 13   1   4   6  -3   3
11. Tampa Bay    13   1   2   4   1   3
12. Buffalo      13   1   4   4  -2   6
13. Ottawa       13   1   2   4  -3   3
14. Boston       15   1   0   5  -2  -5
15. Florida      13   0  -3   5  -7 -13
16. NY Rangers   14   0   0   4  -3  -8
                 👉🏻  35  34  81  26  25 0.583

=======================================
          Western conference
=======================================
                 GP +/- L10  RW  GD L10
 1. Colorado     14   7   4   8  13   8
 2. Anaheim      12   5   5   6  11  12
 3. Vegas        12   5   4   6   8   8
 4. Winnipeg     13   5   4   8  12   9
 5. Utah         13   5   6   6   9  11
 6. Seattle      12   4   2   4   0  -3
 7. Dallas       13   4   1   5  -2  -7
 8. Los Angeles  14   2   3   3  -4   0
 9. Edmonton     15   1   1   4  -2  -1
10. Chicago      13   0   2   4   1   4
11. Vancouver    14   0   0   3  -5  -6
12. Minnesota    14  -1  -1   3 -11 -11
13. Nashville    15  -1  -2   4 -12 -11
14. San Jose     13  -2  -1   1 -11  -5
15. St. Louis    13  -3  -4   4 -16 -16
16. Calgary      14  -6  -4   2 -17 -10
                 👉🏻  25  20  71 -26 -18 0.558

=======================================
              Full league
=======================================
                 GP +/- L10  RW  GD L10
 1. Colorado     14   7   4   8  13   8
 2. Montréal     13   6   5   4   7   5
 3. Anaheim      12   5   5   6  11  12
 4. Vegas        12   5   4   6   8   8
 5. Winnipeg     13   5   4   8  12   9
 6. New Jersey   13   5   4   8   6   6
 7. Utah         13   5   6   6   9  11
 8. Carolina     12   4   2   5  12   8
 9. Seattle      12   4   2   4   0  -3
10. Dallas       13   4   1   5  -2  -7
11. Pittsburgh   14   4   4   8   9  11
12. Detroit      14   4   2   6   2  -1
13. Columbus     12   2   2   5   4   2
14. Toronto      13   2   3   5   0   1
15. Philadelphia 13   2   2   3   3   2
16. Los Angeles  14   2   3   3  -4   0
17. Washington   12   1   1   5   2   2
18. NY Islanders 13   1   4   6  -3   3
19. Tampa Bay    13   1   2   4   1   3
20. Buffalo      13   1   4   4  -2   6
21. Ottawa       13   1   2   4  -3   3
22. Boston       15   1   0   5  -2  -5
23. Edmonton     15   1   1   4  -2  -1
24. Florida      13   0  -3   5  -7 -13
25. Chicago      13   0   2   4   1   4
26. NY Rangers   14   0   0   4  -3  -8
27. Vancouver    14   0   0   3  -5  -6
28. Minnesota    14  -1  -1   3 -11 -11
29. Nashville    15  -1  -2   4 -12 -11
30. San Jose     13  -2  -1   1 -11  -5
31. St. Louis    13  -3  -4   4 -16 -16
32. Calgary      14  -6  -4   2 -17 -10
                 👉🏻  60  54 152   0   7 0.571

=======================================
      Full league (last 10 games)
=======================================
                 GP +/- L10  RW  GD L10
 1. Utah         13   5   6   6   9  11
 2. Montréal     13   6   5   4   7   5
 3. Anaheim      12   5   5   6  11  12
 4. Colorado     14   7   4   8  13   8
 5. Vegas        12   5   4   6   8   8
 6. Winnipeg     13   5   4   8  12   9
 7. New Jersey   13   5   4   8   6   6
 8. Pittsburgh   14   4   4   8   9  11
 9. NY Islanders 13   1   4   6  -3   3
10. Buffalo      13   1   4   4  -2   6
11. Toronto      13   2   3   5   0   1
12. Los Angeles  14   2   3   3  -4   0
13. Carolina     12   4   2   5  12   8
14. Seattle      12   4   2   4   0  -3
15. Detroit      14   4   2   6   2  -1
16. Columbus     12   2   2   5   4   2
17. Philadelphia 13   2   2   3   3   2
18. Tampa Bay    13   1   2   4   1   3
19. Ottawa       13   1   2   4  -3   3
20. Chicago      13   0   2   4   1   4
21. Dallas       13   4   1   5  -2  -7
22. Washington   12   1   1   5   2   2
23. Edmonton     15   1   1   4  -2  -1
24. Boston       15   1   0   5  -2  -5
25. NY Rangers   14   0   0   4  -3  -8
26. Vancouver    14   0   0   3  -5  -6
27. Minnesota    14  -1  -1   3 -11 -11
28. San Jose     13  -2  -1   1 -11  -5
29. Nashville    15  -1  -2   4 -12 -11
30. Florida      13   0  -3   5  -7 -13
31. St. Louis    13  -3  -4   4 -16 -16
32. Calgary      14  -6  -4   2 -17 -10
                 👉🏻  60  54 152   0   7 0.571

```

## Resources

* New API docs: <https://gitlab.com/dword4/nhlapi/-/blob/master/new-api.md>
* Great docs: <https://github.com/Zmalski/NHL-API-Reference>
* Transform tools, JSON to Rust Serde: <https://transform.tools/json-to-rust-serde>
