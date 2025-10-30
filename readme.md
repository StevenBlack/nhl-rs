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
 1. Montréal     11   5   6   4   7  10
 2. Detroit      10   4   4   6   4   4
 3. Buffalo      10   0   0   4  -1  -1
 4. Tampa Bay    10   0   0   3  -1  -1
 5. Florida      11   0  -1   5  -4  -5
 6. Ottawa       11   0  -1   4  -3  -4
 7. Toronto      11   0  -1   3  -4  -7
 8. Boston       12  -2  -4   4  -5  -8
                 👉🏻   7   3  33  -7 -12 0.541

=======================================
         Metropolitan division         
=======================================
                 GP +/- L10  RW  GD L10
 1. New Jersey   10   6   6   7   9   9
 2. Pittsburgh   11   5   4   7  10   7
 3. Carolina      9   3   3   3   6   6
 4. Philadelphia  9   2   2   2   3   3
 5. Washington   10   2   2   5   5   5
 6. Columbus     10   2   2   4   4   4
 7. NY Islanders  9   0   0   4  -1  -1
 8. NY Rangers   11  -1   0   4  -2   1
                 👉🏻  19  19  36  34  34 0.620

=======================================
           Central division            
=======================================
                 GP +/- L10  RW  GD L10
 1. Colorado     11   5   4   6  11   8
 2. Utah         11   5   6   6  10  11
 3. Winnipeg     10   4   4   6   9   9
 4. Dallas       10   3   3   5  -1  -1
 5. Chicago      10   2   2   4   7   7
 6. Nashville    11  -1  -2   3  -9 -10
 7. Minnesota    11  -2  -3   2 -12 -17
 8. St. Louis    10  -3  -3   3 -15 -15
                 👉🏻  13  11  35   0  -8 0.577

=======================================
           Pacific division            
=======================================
                 GP +/- L10  RW  GD L10
 1. Vegas        10   5   5   5   9   9
 2. Seattle      10   3   3   3  -1  -1
 3. Anaheim       9   2   2   3   1   1
 4. Los Angeles  11   2   3   2  -3   0
 5. Edmonton     11   1   1   4   0   1
 6. Vancouver    11  -1  -2   3  -4  -8
 7. San Jose     10  -4  -4   0 -14 -14
 8. Calgary      11  -6  -7   1 -15 -16
                 👉🏻   2   1  21 -27 -28 0.512

=======================================
          Eastern conference           
=======================================
                 GP +/- L10  RW  GD L10
 1. New Jersey   10   6   6   7   9   9
 2. Pittsburgh   11   5   4   7  10   7
 3. Montréal     11   5   6   4   7  10
 4. Detroit      10   4   4   6   4   4
 5. Carolina      9   3   3   3   6   6
 6. Philadelphia  9   2   2   2   3   3
 7. Washington   10   2   2   5   5   5
 8. Columbus     10   2   2   4   4   4
 9. NY Islanders  9   0   0   4  -1  -1
10. Buffalo      10   0   0   4  -1  -1
11. Tampa Bay    10   0   0   3  -1  -1
12. Florida      11   0  -1   5  -4  -5
13. Ottawa       11   0  -1   4  -3  -4
14. Toronto      11   0  -1   3  -4  -7
15. NY Rangers   11  -1   0   4  -2   1
16. Boston       12  -2  -4   4  -5  -8
                 👉🏻  26  22  69  27  22 0.579

=======================================
          Western conference           
=======================================
                 GP +/- L10  RW  GD L10
 1. Vegas        10   5   5   5   9   9
 2. Colorado     11   5   4   6  11   8
 3. Utah         11   5   6   6  10  11
 4. Winnipeg     10   4   4   6   9   9
 5. Dallas       10   3   3   5  -1  -1
 6. Seattle      10   3   3   3  -1  -1
 7. Anaheim       9   2   2   3   1   1
 8. Chicago      10   2   2   4   7   7
 9. Los Angeles  11   2   3   2  -3   0
10. Edmonton     11   1   1   4   0   1
11. Vancouver    11  -1  -2   3  -4  -8
12. Nashville    11  -1  -2   3  -9 -10
13. Minnesota    11  -2  -3   2 -12 -17
14. St. Louis    10  -3  -3   3 -15 -15
15. San Jose     10  -4  -4   0 -14 -14
16. Calgary      11  -6  -7   1 -15 -16
                 👉🏻  15  12  56 -27 -36 0.545

=======================================
              Full league              
=======================================
                 GP +/- L10  RW  GD L10
 1. New Jersey   10   6   6   7   9   9
 2. Vegas        10   5   5   5   9   9
 3. Pittsburgh   11   5   4   7  10   7
 4. Colorado     11   5   4   6  11   8
 5. Utah         11   5   6   6  10  11
 6. Montréal     11   5   6   4   7  10
 7. Winnipeg     10   4   4   6   9   9
 8. Detroit      10   4   4   6   4   4
 9. Carolina      9   3   3   3   6   6
10. Dallas       10   3   3   5  -1  -1
11. Seattle      10   3   3   3  -1  -1
12. Anaheim       9   2   2   3   1   1
13. Philadelphia  9   2   2   2   3   3
14. Washington   10   2   2   5   5   5
15. Chicago      10   2   2   4   7   7
16. Columbus     10   2   2   4   4   4
17. Los Angeles  11   2   3   2  -3   0
18. Edmonton     11   1   1   4   0   1
19. NY Islanders  9   0   0   4  -1  -1
20. Buffalo      10   0   0   4  -1  -1
21. Tampa Bay    10   0   0   3  -1  -1
22. Florida      11   0  -1   5  -4  -5
23. Ottawa       11   0  -1   4  -3  -4
24. Toronto      11   0  -1   3  -4  -7
25. NY Rangers   11  -1   0   4  -2   1
26. Vancouver    11  -1  -2   3  -4  -8
27. Nashville    11  -1  -2   3  -9 -10
28. Minnesota    11  -2  -3   2 -12 -17
29. Boston       12  -2  -4   4  -5  -8
30. St. Louis    10  -3  -3   3 -15 -15
31. San Jose     10  -4  -4   0 -14 -14
32. Calgary      11  -6  -7   1 -15 -16
                 👉🏻  41  34 125   0 -14 0.562

=======================================
      Full league (last 10 games)      
=======================================
                 GP +/- L10  RW  GD L10
 1. New Jersey   10   6   6   7   9   9
 2. Utah         11   5   6   6  10  11
 3. Montréal     11   5   6   4   7  10
 4. Vegas        10   5   5   5   9   9
 5. Pittsburgh   11   5   4   7  10   7
 6. Colorado     11   5   4   6  11   8
 7. Winnipeg     10   4   4   6   9   9
 8. Detroit      10   4   4   6   4   4
 9. Carolina      9   3   3   3   6   6
10. Dallas       10   3   3   5  -1  -1
11. Seattle      10   3   3   3  -1  -1
12. Los Angeles  11   2   3   2  -3   0
13. Anaheim       9   2   2   3   1   1
14. Philadelphia  9   2   2   2   3   3
15. Washington   10   2   2   5   5   5
16. Chicago      10   2   2   4   7   7
17. Columbus     10   2   2   4   4   4
18. Edmonton     11   1   1   4   0   1
19. NY Islanders  9   0   0   4  -1  -1
20. Buffalo      10   0   0   4  -1  -1
21. Tampa Bay    10   0   0   3  -1  -1
22. NY Rangers   11  -1   0   4  -2   1
23. Florida      11   0  -1   5  -4  -5
24. Ottawa       11   0  -1   4  -3  -4
25. Toronto      11   0  -1   3  -4  -7
26. Vancouver    11  -1  -2   3  -4  -8
27. Nashville    11  -1  -2   3  -9 -10
28. Minnesota    11  -2  -3   2 -12 -17
29. St. Louis    10  -3  -3   3 -15 -15
30. Boston       12  -2  -4   4  -5  -8
31. San Jose     10  -4  -4   0 -14 -14
32. Calgary      11  -6  -7   1 -15 -16
                 👉🏻  41  34 125   0 -14 0.562
                 
```

## Resources

* New API docs: <https://gitlab.com/dword4/nhlapi/-/blob/master/new-api.md>
* Great docs: <https://github.com/Zmalski/NHL-API-Reference>
* Transform tools, JSON to Rust Serde: <https://transform.tools/json-to-rust-serde>
