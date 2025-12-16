# nhl

National Hockey League (NHL) stats as I like 'em.

## Usage and sample output

The `nhl` executable is a simple command-line tool that fetches the current NHL standings or schedule, and displays them in a table format.

```bash
$ nhl -h
Usage: nhl [OPTIONS]

Options:
  -p, --playoffs     display the playoffs picture
  -s, --schedule     display the league proximate schedule
  -t, --team <TEAM>  used with --schedule to display just one team's full schedule
  -d, --division     display just division standings
  -c, --conference   display just conference standings
  -f, --full         display just the full league standings
      --l10          display full league standings by last 10 games
  -h, --help         Print help
  -V, --version      Print version
```

The output is segmented into the four divisions, two conferences, and the full league. The teams are sorted by wins minus losses, and the display is games played, wins minus losses, and the last ten games' wins minus losses.

```bash
$ nhl



=======================================
           Atlantic division           
=======================================
                 GP +/- L10  RW  GD L10
 1. Montréal     32   6   1  10  -8  -5
 2. Tampa Bay    33   6  -1  15  18   5
 3. Detroit      33   6   2  14  -5   0
 4. Boston       33   5   2  13   4   4
 5. Florida      32   4   1  15   2   0
 6. Toronto      31   2   2  10   0   5
 7. Ottawa       32   2  -2  10  -3  -3
 8. Buffalo      32   0   0  10 -11  -7
                 👉🏻  31   5  97  -3  -1 0.560

=======================================
         Metropolitan division         
=======================================
                 GP +/- L10  RW  GD L10
 1. Carolina     32  12   4  13  18   5
 2. Washington   32   8   6  17  23  14
 3. NY Islanders 33   8   3  13   7   2
 4. Philadelphia 31   7   3   9   4   3
 5. Pittsburgh   31   6   2  13   3  -3
 6. New Jersey   33   4  -4  12  -6  -9
 7. NY Rangers   34   2   2  11  -3  -1
 8. Columbus     32   0  -3   7 -16 -13
                 👉🏻  47  13  95  30  -2 0.591

=======================================
           Central division            
=======================================
                 GP +/- L10  RW  GD L10
 1. Colorado     32  21   6  21  54  16
 2. Dallas       34  15   5  18  25   9
 3. Minnesota    33  10   5  13  12   8
 4. Utah         34   1  -2  12   5   3
 5. Winnipeg     32   0  -2  13  -1  -8
 6. Chicago      32   0  -2  12  -7 -14
 7. Nashville    32  -2   4   9 -19  10
 8. St. Louis    34  -3   0  12 -37 -12
                 👉🏻  42  14 110  32  12 0.580

=======================================
           Pacific division            
=======================================
                 GP +/- L10  RW  GD L10
 1. Vegas        31  10   4  12   6  -1
 2. Anaheim      33   8   2  12  10   0
 3. Los Angeles  32   5   1   9  -2   1
 4. Edmonton     33   3   3  10  -4  12
 5. San Jose     33   2   0   8 -14 -10
 6. Seattle      30   0  -5   7 -19 -17
 7. Calgary      33  -3   3  10 -13   4
 8. Vancouver    32  -5  -3   8 -23 -11
                 👉🏻  20   5  76 -59 -22 0.539

=======================================
          Eastern conference           
=======================================
                 GP +/- L10  RW  GD L10
 1. Carolina     32  12   4  13  18   5
 2. Washington   32   8   6  17  23  14
 3. NY Islanders 33   8   3  13   7   2
 4. Philadelphia 31   7   3   9   4   3
 5. Pittsburgh   31   6   2  13   3  -3
 6. Montréal     32   6   1  10  -8  -5
 7. Tampa Bay    33   6  -1  15  18   5
 8. Detroit      33   6   2  14  -5   0
 9. Boston       33   5   2  13   4   4
10. Florida      32   4   1  15   2   0
11. New Jersey   33   4  -4  12  -6  -9
12. Toronto      31   2   2  10   0   5
13. Ottawa       32   2  -2  10  -3  -3
14. NY Rangers   34   2   2  11  -3  -1
15. Buffalo      32   0   0  10 -11  -7
16. Columbus     32   0  -3   7 -16 -13
                 👉🏻  78  18 192  27  -3 0.576

=======================================
          Western conference           
=======================================
                 GP +/- L10  RW  GD L10
 1. Colorado     32  21   6  21  54  16
 2. Dallas       34  15   5  18  25   9
 3. Vegas        31  10   4  12   6  -1
 4. Minnesota    33  10   5  13  12   8
 5. Anaheim      33   8   2  12  10   0
 6. Los Angeles  32   5   1   9  -2   1
 7. Edmonton     33   3   3  10  -4  12
 8. San Jose     33   2   0   8 -14 -10
 9. Utah         34   1  -2  12   5   3
10. Seattle      30   0  -5   7 -19 -17
11. Winnipeg     32   0  -2  13  -1  -8
12. Chicago      32   0  -2  12  -7 -14
13. Nashville    32  -2   4   9 -19  10
14. Calgary      33  -3   3  10 -13   4
15. St. Louis    34  -3   0  12 -37 -12
16. Vancouver    32  -5  -3   8 -23 -11
                 👉🏻  62  19 186 -27 -10 0.560

=======================================
              Full league              
=======================================
                 GP +/- L10  RW  GD L10
 1. Colorado     32  21   6  21  54  16
 2. Dallas       34  15   5  18  25   9
 3. Carolina     32  12   4  13  18   5
 4. Vegas        31  10   4  12   6  -1
 5. Minnesota    33  10   5  13  12   8
 6. Washington   32   8   6  17  23  14
 7. NY Islanders 33   8   3  13   7   2
 8. Anaheim      33   8   2  12  10   0
 9. Philadelphia 31   7   3   9   4   3
10. Pittsburgh   31   6   2  13   3  -3
11. Montréal     32   6   1  10  -8  -5
12. Tampa Bay    33   6  -1  15  18   5
13. Detroit      33   6   2  14  -5   0
14. Los Angeles  32   5   1   9  -2   1
15. Boston       33   5   2  13   4   4
16. Florida      32   4   1  15   2   0
17. New Jersey   33   4  -4  12  -6  -9
18. Edmonton     33   3   3  10  -4  12
19. Toronto      31   2   2  10   0   5
20. Ottawa       32   2  -2  10  -3  -3
21. San Jose     33   2   0   8 -14 -10
22. NY Rangers   34   2   2  11  -3  -1
23. Utah         34   1  -2  12   5   3
24. Seattle      30   0  -5   7 -19 -17
25. Winnipeg     32   0  -2  13  -1  -8
26. Chicago      32   0  -2  12  -7 -14
27. Buffalo      32   0   0  10 -11  -7
28. Columbus     32   0  -3   7 -16 -13
29. Nashville    32  -2   4   9 -19  10
30. Calgary      33  -3   3  10 -13   4
31. St. Louis    34  -3   0  12 -37 -12
32. Vancouver    32  -5  -3   8 -23 -11
                 👉🏻 140  37 378   0 -13 0.568

=======================================
      Full league (last 10 games)      
=======================================
                 GP +/- L10  RW  GD L10
 1. Colorado     32  21   6  21  54  16
 2. Washington   32   8   6  17  23  14
 3. Dallas       34  15   5  18  25   9
 4. Minnesota    33  10   5  13  12   8
 5. Carolina     32  12   4  13  18   5
 6. Vegas        31  10   4  12   6  -1
 7. Nashville    32  -2   4   9 -19  10
 8. NY Islanders 33   8   3  13   7   2
 9. Philadelphia 31   7   3   9   4   3
10. Edmonton     33   3   3  10  -4  12
11. Calgary      33  -3   3  10 -13   4
12. Anaheim      33   8   2  12  10   0
13. Pittsburgh   31   6   2  13   3  -3
14. Detroit      33   6   2  14  -5   0
15. Boston       33   5   2  13   4   4
16. Toronto      31   2   2  10   0   5
17. NY Rangers   34   2   2  11  -3  -1
18. Montréal     32   6   1  10  -8  -5
19. Los Angeles  32   5   1   9  -2   1
20. Florida      32   4   1  15   2   0
21. San Jose     33   2   0   8 -14 -10
22. Buffalo      32   0   0  10 -11  -7
23. St. Louis    34  -3   0  12 -37 -12
24. Tampa Bay    33   6  -1  15  18   5
25. Ottawa       32   2  -2  10  -3  -3
26. Utah         34   1  -2  12   5   3
27. Winnipeg     32   0  -2  13  -1  -8
28. Chicago      32   0  -2  12  -7 -14
29. Columbus     32   0  -3   7 -16 -13
30. Vancouver    32  -5  -3   8 -23 -11
31. New Jersey   33   4  -4  12  -6  -9
32. Seattle      30   0  -5   7 -19 -17
                 👉🏻 140  37 378   0 -13 0.568

```

## Resources

* New API docs: <https://gitlab.com/dword4/nhlapi/-/blob/master/new-api.md>
* Great docs: <https://github.com/Zmalski/NHL-API-Reference>
* Transform tools, JSON to Rust Serde: <https://transform.tools/json-to-rust-serde>
