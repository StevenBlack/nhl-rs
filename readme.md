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
 1. Florida      68  17   4  35  37  11
 2. Tampa Bay    66  15   3  33  55   8
 3. Toronto      66  15   1  31  13  -3
 4. Ottawa       66  11   5  27   8   6
 5. Montréal     66   5   6  23 -19  12
 6. Detroit      67   3  -4  24 -17  -6
 7. Boston       68   0  -3  23 -34  -8
 8. Buffalo      65  -7  -3  21 -24 -16
                 👉🏻  59   9 217  19   4 0.555

=======================================
         Metropolitan division
=======================================
                 GP +/- L10  RW  GD L10
 1. Washington   67  29   2  37  69   4
 2. Carolina     67  19   6  36  39  12
 3. New Jersey   68  10   0  32  27  -4
 4. Columbus     66   4   0  23  -4  -2
 5. NY Rangers   68   4   0  30   0   3
 6. NY Islanders 66   2   1  24 -17  -6
 7. Pittsburgh   69  -3   1  19 -48  -4
 8. Philadelphia 68  -4  -3  17 -40 -13
                 👉🏻  61   7 218  26 -10 0.557

=======================================
           Central division
=======================================
                 GP +/- L10  RW  GD L10
 1. Winnipeg     68  30   3  38  81  11
 2. Dallas       66  21   3  35  51   6
 3. Colorado     68  17   7  35  31  20
 4. Minnesota    67  12  -3  29 -10 -15
 5. St. Louis    68   5   5  24   0  16
 6. Utah         67   4   4  22  -8   6
 7. Nashville    66  -8   1  21 -43  -4
 8. Chicago      67 -18  -2  17 -55  -6
                 👉🏻  63  18 221  47  34 0.559

=======================================
           Pacific division
=======================================
                 GP +/- L10  RW  GD L10
 1. Vegas        67  19   2  36  38   6
 2. Los Angeles  65  16   2  31  16  -4
 3. Edmonton     67  15   0  28  20  -3
 4. Calgary      65   6   1  24 -21  -4
 5. Vancouver    67   6   0  24 -18  -4
 6. Anaheim      67  -2  -2  21 -32  -3
 7. Seattle      68  -5  -1  23 -18  -5
 8. San Jose     68 -23  -2  13 -77  -8
                 👉🏻  32   0 200 -92 -25 0.530

=======================================
          Eastern conference
=======================================
                 GP +/- L10  RW  GD L10
 1. Washington   67  29   2  37  69   4
 2. Carolina     67  19   6  36  39  12
 3. Florida      68  17   4  35  37  11
 4. Tampa Bay    66  15   3  33  55   8
 5. Toronto      66  15   1  31  13  -3
 6. Ottawa       66  11   5  27   8   6
 7. New Jersey   68  10   0  32  27  -4
 8. Montréal     66   5   6  23 -19  12
 9. Columbus     66   4   0  23  -4  -2
10. NY Rangers   68   4   0  30   0   3
11. Detroit      67   3  -4  24 -17  -6
12. NY Islanders 66   2   1  24 -17  -6
13. Boston       68   0  -3  23 -34  -8
14. Pittsburgh   69  -3   1  19 -48  -4
15. Philadelphia 68  -4  -3  17 -40 -13
16. Buffalo      65  -7  -3  21 -24 -16
                 👉🏻 120  16 435  45  -6 0.556

=======================================
          Western conference
=======================================
                 GP +/- L10  RW  GD L10
 1. Winnipeg     68  30   3  38  81  11
 2. Dallas       66  21   3  35  51   6
 3. Vegas        67  19   2  36  38   6
 4. Colorado     68  17   7  35  31  20
 5. Los Angeles  65  16   2  31  16  -4
 6. Edmonton     67  15   0  28  20  -3
 7. Minnesota    67  12  -3  29 -10 -15
 8. Calgary      65   6   1  24 -21  -4
 9. Vancouver    67   6   0  24 -18  -4
10. St. Louis    68   5   5  24   0  16
11. Utah         67   4   4  22  -8   6
12. Anaheim      67  -2  -2  21 -32  -3
13. Seattle      68  -5  -1  23 -18  -5
14. Nashville    66  -8   1  21 -43  -4
15. Chicago      67 -18  -2  17 -55  -6
16. San Jose     68 -23  -2  13 -77  -8
                 👉🏻  95  18 421 -45   9 0.544

=======================================
              Full league
=======================================
                 GP +/- L10  RW  GD L10
 1. Winnipeg     68  30   3  38  81  11
 2. Washington   67  29   2  37  69   4
 3. Dallas       66  21   3  35  51   6
 4. Carolina     67  19   6  36  39  12
 5. Vegas        67  19   2  36  38   6
 6. Colorado     68  17   7  35  31  20
 7. Florida      68  17   4  35  37  11
 8. Los Angeles  65  16   2  31  16  -4
 9. Tampa Bay    66  15   3  33  55   8
10. Toronto      66  15   1  31  13  -3
11. Edmonton     67  15   0  28  20  -3
12. Minnesota    67  12  -3  29 -10 -15
13. Ottawa       66  11   5  27   8   6
14. New Jersey   68  10   0  32  27  -4
15. Calgary      65   6   1  24 -21  -4
16. Vancouver    67   6   0  24 -18  -4
17. Montréal     66   5   6  23 -19  12
18. St. Louis    68   5   5  24   0  16
19. Columbus     66   4   0  23  -4  -2
20. Utah         67   4   4  22  -8   6
21. NY Rangers   68   4   0  30   0   3
22. Detroit      67   3  -4  24 -17  -6
23. NY Islanders 66   2   1  24 -17  -6
24. Boston       68   0  -3  23 -34  -8
25. Anaheim      67  -2  -2  21 -32  -3
26. Pittsburgh   69  -3   1  19 -48  -4
27. Philadelphia 68  -4  -3  17 -40 -13
28. Seattle      68  -5  -1  23 -18  -5
29. Buffalo      65  -7  -3  21 -24 -16
30. Nashville    66  -8   1  21 -43  -4
31. Chicago      67 -18  -2  17 -55  -6
32. San Jose     68 -23  -2  13 -77  -8
                 👉🏻 215  34 856   0   3 0.550

=======================================
      Full league (last 10 games)
=======================================
                 GP +/- L10  RW  GD L10
 1. Colorado     68  17   7  35  31  20
 2. Carolina     67  19   6  36  39  12
 3. Montréal     66   5   6  23 -19  12
 4. Ottawa       66  11   5  27   8   6
 5. St. Louis    68   5   5  24   0  16
 6. Florida      68  17   4  35  37  11
 7. Utah         67   4   4  22  -8   6
 8. Winnipeg     68  30   3  38  81  11
 9. Dallas       66  21   3  35  51   6
10. Tampa Bay    66  15   3  33  55   8
11. Washington   67  29   2  37  69   4
12. Vegas        67  19   2  36  38   6
13. Los Angeles  65  16   2  31  16  -4
14. Toronto      66  15   1  31  13  -3
15. Calgary      65   6   1  24 -21  -4
16. NY Islanders 66   2   1  24 -17  -6
17. Pittsburgh   69  -3   1  19 -48  -4
18. Nashville    66  -8   1  21 -43  -4
19. Edmonton     67  15   0  28  20  -3
20. New Jersey   68  10   0  32  27  -4
21. Vancouver    67   6   0  24 -18  -4
22. Columbus     66   4   0  23  -4  -2
23. NY Rangers   68   4   0  30   0   3
24. Seattle      68  -5  -1  23 -18  -5
25. Anaheim      67  -2  -2  21 -32  -3
26. Chicago      67 -18  -2  17 -55  -6
27. San Jose     68 -23  -2  13 -77  -8
28. Minnesota    67  12  -3  29 -10 -15
29. Boston       68   0  -3  23 -34  -8
30. Philadelphia 68  -4  -3  17 -40 -13
31. Buffalo      65  -7  -3  21 -24 -16
32. Detroit      67   3  -4  24 -17  -6
                 👉🏻 215  34 856   0   3 0.550
                 
```

## Resources

* New API docs: <https://gitlab.com/dword4/nhlapi/-/blob/master/new-api.md>
* Great docs: <https://github.com/Zmalski/NHL-API-Reference>
* Transform tools, JSON to Rust Serde: <https://transform.tools/json-to-rust-serde>
