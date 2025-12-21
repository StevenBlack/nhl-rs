# nhl

National Hockey League (NHL) stats as I like 'em.

The `nhl` executable is a simple command-line tool that fetches the current NHL standings or schedule, and displays them in a table format.


## Usage

### Version information with `-V` or `--version`

Here's the latest version:

<!-- BEGIN:version -->

```text
$ ./target/release/nhl -V

nhl 0.2.8
```

<!-- END:version -->

### Help information with `-h` or `--help`
<!-- BEGIN:help -->

```text
$ ./target/release/nhl -h

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

<!-- END:help -->

### Default output - standings sliced and diced

The default output is **standings** segmented segmented in different ways

* Four divisions standings
* two conference standings, and
* full league standings.

Standings are determined by wins minus losses, games played, and the standings show games played, wins minus losses, and the last ten games' wins minus losses.

<!-- BEGIN:vanilla -->

```text
$ ./target/release/nhl 


=======================================
           Atlantic division           
=======================================
                 GP +/- L10  RW  GD L10
 1. Detroit      36   7   5  16  -4   8
 2. Montréal     34   6   1  11  -8  -3
 3. Tampa Bay    34   5  -3  15  17   1
 4. Florida      35   5   5  16   0   5
 5. Boston       35   5   2  14   5   5
 6. Ottawa       34   4   0  12   3   3
 7. Toronto      33   2   2  11  -3   4
 8. Buffalo      34   2   4  11  -8   3
             👉🏻 275  36  16 106   2  26 0.565

=======================================
         Metropolitan division         
=======================================
                 GP +/- L10  RW  GD L10
 1. Carolina     34  13   5  14  20   5
 2. Philadelphia 34   7   0  10   4  -2
 3. Washington   35   7   2  18  19   3
 4. New Jersey   35   6  -2  13  -4 -10
 5. NY Islanders 36   6   3  13   2   3
 6. Pittsburgh   33   4  -2  13  -3 -12
 7. NY Rangers   37   3   2  11  -4  -5
 8. Columbus     34   0  -2   7 -18 -10
             👉🏻 278  46   6  99  16 -28 0.583

=======================================
           Central division            
=======================================
                 GP +/- L10  RW  GD L10
 1. Colorado     34  23   7  23  57  14
 2. Dallas       36  17   5  20  32  10
 3. Minnesota    36  13   6  16  23  18
 4. Utah         37   0   0  13   4   7
 5. St. Louis    37  -1   1  14 -33  -7
 6. Winnipeg     34  -2  -4  13  -3  -9
 7. Nashville    33  -3   2   9 -22   4
 8. Chicago      35  -3  -5  12 -13 -20
             👉🏻 282  44  12 120  45  17 0.578

=======================================
           Pacific division            
=======================================
                 GP +/- L10  RW  GD L10
 1. Vegas        32  10   5  12   5   2
 2. Anaheim      35   7   1  12   4  -5
 3. Los Angeles  34   5   0  10  -2   1
 4. Edmonton     36   4   3  12  -3  11
 5. San Jose     35   2   0   9 -13  -4
 6. Seattle      32  -2  -8   7 -23 -21
 7. Vancouver    34  -3  -1  10 -17  -3
 8. Calgary      35  -3   3  11 -14   4
             👉🏻 273  20   3  83 -63 -15 0.537

=======================================
          Eastern conference           
=======================================
                 GP +/- L10  RW  GD L10
 1. Carolina     34  13   5  14  20   5
 2. Philadelphia 34   7   0  10   4  -2
 3. Washington   35   7   2  18  19   3
 4. Detroit      36   7   5  16  -4   8
 5. Montréal     34   6   1  11  -8  -3
 6. New Jersey   35   6  -2  13  -4 -10
 7. NY Islanders 36   6   3  13   2   3
 8. Tampa Bay    34   5  -3  15  17   1
 9. Florida      35   5   5  16   0   5
10. Boston       35   5   2  14   5   5
11. Pittsburgh   33   4  -2  13  -3 -12
12. Ottawa       34   4   0  12   3   3
13. NY Rangers   37   3   2  11  -4  -5
14. Toronto      33   2   2  11  -3   4
15. Buffalo      34   2   4  11  -8   3
16. Columbus     34   0  -2   7 -18 -10
             👉🏻 553  82  22 205  18  -2 0.574

=======================================
          Western conference           
=======================================
                 GP +/- L10  RW  GD L10
 1. Colorado     34  23   7  23  57  14
 2. Dallas       36  17   5  20  32  10
 3. Minnesota    36  13   6  16  23  18
 4. Vegas        32  10   5  12   5   2
 5. Anaheim      35   7   1  12   4  -5
 6. Los Angeles  34   5   0  10  -2   1
 7. Edmonton     36   4   3  12  -3  11
 8. San Jose     35   2   0   9 -13  -4
 9. Utah         37   0   0  13   4   7
10. St. Louis    37  -1   1  14 -33  -7
11. Seattle      32  -2  -8   7 -23 -21
12. Winnipeg     34  -2  -4  13  -3  -9
13. Nashville    33  -3   2   9 -22   4
14. Vancouver    34  -3  -1  10 -17  -3
15. Chicago      35  -3  -5  12 -13 -20
16. Calgary      35  -3   3  11 -14   4
             👉🏻 555  64  15 203 -18   2 0.558

=======================================
              Full league              
=======================================
                 GP +/- L10  RW  GD L10
 1. Colorado     34  23   7  23  57  14
 2. Dallas       36  17   5  20  32  10
 3. Carolina     34  13   5  14  20   5
 4. Minnesota    36  13   6  16  23  18
 5. Vegas        32  10   5  12   5   2
 6. Philadelphia 34   7   0  10   4  -2
 7. Washington   35   7   2  18  19   3
 8. Anaheim      35   7   1  12   4  -5
 9. Detroit      36   7   5  16  -4   8
10. Montréal     34   6   1  11  -8  -3
11. New Jersey   35   6  -2  13  -4 -10
12. NY Islanders 36   6   3  13   2   3
13. Tampa Bay    34   5  -3  15  17   1
14. Los Angeles  34   5   0  10  -2   1
15. Florida      35   5   5  16   0   5
16. Boston       35   5   2  14   5   5
17. Pittsburgh   33   4  -2  13  -3 -12
18. Ottawa       34   4   0  12   3   3
19. Edmonton     36   4   3  12  -3  11
20. NY Rangers   37   3   2  11  -4  -5
21. Toronto      33   2   2  11  -3   4
22. Buffalo      34   2   4  11  -8   3
23. San Jose     35   2   0   9 -13  -4
24. Columbus     34   0  -2   7 -18 -10
25. Utah         37   0   0  13   4   7
26. St. Louis    37  -1   1  14 -33  -7
27. Seattle      32  -2  -8   7 -23 -21
28. Winnipeg     34  -2  -4  13  -3  -9
29. Nashville    33  -3   2   9 -22   4
30. Vancouver    34  -3  -1  10 -17  -3
31. Chicago      35  -3  -5  12 -13 -20
32. Calgary      35  -3   3  11 -14   4
             👉🏻1108 146  37 408   0   0 0.566

=======================================
      Full league (last 10 games)      
=======================================
                 GP +/- L10  RW  GD L10
 1. Colorado     34  23   7  23  57  14
 2. Minnesota    36  13   6  16  23  18
 3. Dallas       36  17   5  20  32  10
 4. Carolina     34  13   5  14  20   5
 5. Vegas        32  10   5  12   5   2
 6. Detroit      36   7   5  16  -4   8
 7. Florida      35   5   5  16   0   5
 8. Buffalo      34   2   4  11  -8   3
 9. NY Islanders 36   6   3  13   2   3
10. Edmonton     36   4   3  12  -3  11
11. Calgary      35  -3   3  11 -14   4
12. Washington   35   7   2  18  19   3
13. Boston       35   5   2  14   5   5
14. NY Rangers   37   3   2  11  -4  -5
15. Toronto      33   2   2  11  -3   4
16. Nashville    33  -3   2   9 -22   4
17. Anaheim      35   7   1  12   4  -5
18. Montréal     34   6   1  11  -8  -3
19. St. Louis    37  -1   1  14 -33  -7
20. Philadelphia 34   7   0  10   4  -2
21. Los Angeles  34   5   0  10  -2   1
22. Ottawa       34   4   0  12   3   3
23. San Jose     35   2   0   9 -13  -4
24. Utah         37   0   0  13   4   7
25. Vancouver    34  -3  -1  10 -17  -3
26. New Jersey   35   6  -2  13  -4 -10
27. Pittsburgh   33   4  -2  13  -3 -12
28. Columbus     34   0  -2   7 -18 -10
29. Tampa Bay    34   5  -3  15  17   1
30. Winnipeg     34  -2  -4  13  -3  -9
31. Chicago      35  -3  -5  12 -13 -20
32. Seattle      32  -2  -8   7 -23 -21
             👉🏻1108 146  37 408   0   0 0.566
```

<!-- END:vanilla -->

### Division standings with `--division` or `-d`

<!-- BEGIN:division -->

```text
$ ./target/release/nhl -d


=======================================
           Atlantic division           
=======================================
                 GP +/- L10  RW  GD L10
 1. Detroit      36   7   5  16  -4   8
 2. Montréal     34   6   1  11  -8  -3
 3. Tampa Bay    34   5  -3  15  17   1
 4. Florida      35   5   5  16   0   5
 5. Boston       35   5   2  14   5   5
 6. Ottawa       34   4   0  12   3   3
 7. Toronto      33   2   2  11  -3   4
 8. Buffalo      34   2   4  11  -8   3
             👉🏻 275  36  16 106   2  26 0.565

=======================================
         Metropolitan division         
=======================================
                 GP +/- L10  RW  GD L10
 1. Carolina     34  13   5  14  20   5
 2. Philadelphia 34   7   0  10   4  -2
 3. Washington   35   7   2  18  19   3
 4. New Jersey   35   6  -2  13  -4 -10
 5. NY Islanders 36   6   3  13   2   3
 6. Pittsburgh   33   4  -2  13  -3 -12
 7. NY Rangers   37   3   2  11  -4  -5
 8. Columbus     34   0  -2   7 -18 -10
             👉🏻 278  46   6  99  16 -28 0.583

=======================================
           Central division            
=======================================
                 GP +/- L10  RW  GD L10
 1. Colorado     34  23   7  23  57  14
 2. Dallas       36  17   5  20  32  10
 3. Minnesota    36  13   6  16  23  18
 4. Utah         37   0   0  13   4   7
 5. St. Louis    37  -1   1  14 -33  -7
 6. Winnipeg     34  -2  -4  13  -3  -9
 7. Nashville    33  -3   2   9 -22   4
 8. Chicago      35  -3  -5  12 -13 -20
             👉🏻 282  44  12 120  45  17 0.578

=======================================
           Pacific division            
=======================================
                 GP +/- L10  RW  GD L10
 1. Vegas        32  10   5  12   5   2
 2. Anaheim      35   7   1  12   4  -5
 3. Los Angeles  34   5   0  10  -2   1
 4. Edmonton     36   4   3  12  -3  11
 5. San Jose     35   2   0   9 -13  -4
 6. Seattle      32  -2  -8   7 -23 -21
 7. Vancouver    34  -3  -1  10 -17  -3
 8. Calgary      35  -3   3  11 -14   4
             👉🏻 273  20   3  83 -63 -15 0.537
```

<!-- END:division -->

### Conference standings with `--conference` or `-c`

<!-- BEGIN:conference -->

```text
$ ./target/release/nhl -c


=======================================
          Eastern conference           
=======================================
                 GP +/- L10  RW  GD L10
 1. Carolina     34  13   5  14  20   5
 2. Philadelphia 34   7   0  10   4  -2
 3. Washington   35   7   2  18  19   3
 4. Detroit      36   7   5  16  -4   8
 5. Montréal     34   6   1  11  -8  -3
 6. New Jersey   35   6  -2  13  -4 -10
 7. NY Islanders 36   6   3  13   2   3
 8. Tampa Bay    34   5  -3  15  17   1
 9. Florida      35   5   5  16   0   5
10. Boston       35   5   2  14   5   5
11. Pittsburgh   33   4  -2  13  -3 -12
12. Ottawa       34   4   0  12   3   3
13. NY Rangers   37   3   2  11  -4  -5
14. Toronto      33   2   2  11  -3   4
15. Buffalo      34   2   4  11  -8   3
16. Columbus     34   0  -2   7 -18 -10
             👉🏻 553  82  22 205  18  -2 0.574

=======================================
          Western conference           
=======================================
                 GP +/- L10  RW  GD L10
 1. Colorado     34  23   7  23  57  14
 2. Dallas       36  17   5  20  32  10
 3. Minnesota    36  13   6  16  23  18
 4. Vegas        32  10   5  12   5   2
 5. Anaheim      35   7   1  12   4  -5
 6. Los Angeles  34   5   0  10  -2   1
 7. Edmonton     36   4   3  12  -3  11
 8. San Jose     35   2   0   9 -13  -4
 9. Utah         37   0   0  13   4   7
10. St. Louis    37  -1   1  14 -33  -7
11. Seattle      32  -2  -8   7 -23 -21
12. Winnipeg     34  -2  -4  13  -3  -9
13. Nashville    33  -3   2   9 -22   4
14. Vancouver    34  -3  -1  10 -17  -3
15. Chicago      35  -3  -5  12 -13 -20
16. Calgary      35  -3   3  11 -14   4
             👉🏻 555  64  15 203 -18   2 0.558
```

<!-- END:conference -->

### Full league standings- `--full` or `-f`

#### Conventional full league standings

<!-- BEGIN:full -->

```text
$ ./target/release/nhl -f


=======================================
              Full league              
=======================================
                 GP +/- L10  RW  GD L10
 1. Colorado     34  23   7  23  57  14
 2. Dallas       36  17   5  20  32  10
 3. Carolina     34  13   5  14  20   5
 4. Minnesota    36  13   6  16  23  18
 5. Vegas        32  10   5  12   5   2
 6. Philadelphia 34   7   0  10   4  -2
 7. Washington   35   7   2  18  19   3
 8. Anaheim      35   7   1  12   4  -5
 9. Detroit      36   7   5  16  -4   8
10. Montréal     34   6   1  11  -8  -3
11. New Jersey   35   6  -2  13  -4 -10
12. NY Islanders 36   6   3  13   2   3
13. Tampa Bay    34   5  -3  15  17   1
14. Los Angeles  34   5   0  10  -2   1
15. Florida      35   5   5  16   0   5
16. Boston       35   5   2  14   5   5
17. Pittsburgh   33   4  -2  13  -3 -12
18. Ottawa       34   4   0  12   3   3
19. Edmonton     36   4   3  12  -3  11
20. NY Rangers   37   3   2  11  -4  -5
21. Toronto      33   2   2  11  -3   4
22. Buffalo      34   2   4  11  -8   3
23. San Jose     35   2   0   9 -13  -4
24. Columbus     34   0  -2   7 -18 -10
25. Utah         37   0   0  13   4   7
26. St. Louis    37  -1   1  14 -33  -7
27. Seattle      32  -2  -8   7 -23 -21
28. Winnipeg     34  -2  -4  13  -3  -9
29. Nashville    33  -3   2   9 -22   4
30. Vancouver    34  -3  -1  10 -17  -3
31. Chicago      35  -3  -5  12 -13 -20
32. Calgary      35  -3   3  11 -14   4
             👉🏻1108 146  37 408   0   0 0.566
```

<!-- END:full -->

#### Full league standings by last 10 games
<!-- BEGIN:last10 -->

```text
$ ./target/release/nhl --l10


=======================================
      Full league (last 10 games)      
=======================================
                 GP +/- L10  RW  GD L10
 1. Colorado     34  23   7  23  57  14
 2. Minnesota    36  13   6  16  23  18
 3. Dallas       36  17   5  20  32  10
 4. Carolina     34  13   5  14  20   5
 5. Vegas        32  10   5  12   5   2
 6. Detroit      36   7   5  16  -4   8
 7. Florida      35   5   5  16   0   5
 8. Buffalo      34   2   4  11  -8   3
 9. NY Islanders 36   6   3  13   2   3
10. Edmonton     36   4   3  12  -3  11
11. Calgary      35  -3   3  11 -14   4
12. Washington   35   7   2  18  19   3
13. Boston       35   5   2  14   5   5
14. NY Rangers   37   3   2  11  -4  -5
15. Toronto      33   2   2  11  -3   4
16. Nashville    33  -3   2   9 -22   4
17. Anaheim      35   7   1  12   4  -5
18. Montréal     34   6   1  11  -8  -3
19. St. Louis    37  -1   1  14 -33  -7
20. Philadelphia 34   7   0  10   4  -2
21. Los Angeles  34   5   0  10  -2   1
22. Ottawa       34   4   0  12   3   3
23. San Jose     35   2   0   9 -13  -4
24. Utah         37   0   0  13   4   7
25. Vancouver    34  -3  -1  10 -17  -3
26. New Jersey   35   6  -2  13  -4 -10
27. Pittsburgh   33   4  -2  13  -3 -12
28. Columbus     34   0  -2   7 -18 -10
29. Tampa Bay    34   5  -3  15  17   1
30. Winnipeg     34  -2  -4  13  -3  -9
31. Chicago      35  -3  -5  12 -13 -20
32. Seattle      32  -2  -8   7 -23 -21
             👉🏻1108 146  37 408   0   0 0.566
```

<!-- END:last10 -->

## Current schedule segment (7-days) with `--schedule` or `-s`

By default, shows the next 7 days of NHL games, including today, and broadcast networks.

<!-- BEGIN:schedule -->

```text
$ ./target/release/nhl -s


=======================================================
             Upcoming league-wide schedule             
=======================================================

              2025-12-20 (SAT)               
=============================================
PHI 4 - 5 NYR  12:30   (NBCSP, MSG)
DET 5 - 2 WSH  12:30   (SN, NHLN, FDSNDET, MNMT)
CHI 4 - 6 OTT  15:00   (TVAS, TSN5, CHSN)
EDM 2 - 5 MIN  15:00   (SN, FDSNNO, FDSNWI)
NYI 2 - 3 BUF  17:00   (MSG-B, MSGSN)
STL 6 - 2 FLA  18:00   (FDSNMW, SCRIPPS)
VAN 2 - 3 BOS  19:00   (SNP, NHLN, SN360, NESN)
PIT 0 - 3 MTL  19:00   (CITY, SNE, TVAS, SN-PIT)
CAR 3 - 3 TBL  19:00   (FDSNSO, The Spot)
TOR 2 - 2 NSH  19:00   (SN, CBC, FDSNSO)
VGK at CGY  22:00   (CBC, CITY, SN, SCRIPPS)
CBJ at ANA  22:00   (FDSNOH, Victory+, KCOP-13)
SEA at SJS  22:00   (KHN/Prime, NBCSCA, KONG)

              2025-12-21 (SUN)               
=============================================
WSH at DET  13:00   (NHLN, TVAS, FDSNDET, MNMT)
COL at MIN  18:00   (FDSNNO, ALT, FDSNWIX)
OTT at BOS  19:00   (TSN5, RDSI, NESN)
BUF at NJD  19:00   (MSG-B, MSGSN)
MTL at PIT  19:00   (TSN2, RDS, SN-PIT)
NYR at NSH  19:00   (FDSNSO, MSG 2)
TOR at DAL  19:00   (SNO, NHLN, Victory+)
WPG at UTA  19:00   (TSN3, Utah16)
VGK at EDM  20:00   (SN, SN1, SCRIPPS)

              2025-12-22 (MON)               
=============================================
STL at TBL  19:00   (TVAS, FDSNMW, The Spot)
VAN at PHI  19:30   (NHLN, Prime, TVAS2, NBCSP)
SEA at ANA  22:00   (TVAS, KHN/Prime, KING 5, Victory+, KONG, KCOP-13)
CBJ at LAK  22:00   (TVAS2, FDSNOH, FDSNW)

              2025-12-23 (TUE)               
=============================================
PIT at TOR  16:00   (TNT, truTV, HBO MAX, SNO)
DAL at DET  18:30   (TNT, truTV, HBO MAX, Victory+)
NYR at WSH  19:00   (SN, MSG 2, MNMT2)
MTL at BOS  19:00   (TSN2, RDS, NESN)
BUF at OTT  19:00   (RDS2, TSN5, MSG-B)
NJD at NYI  19:00   (MSGSN2, MSGSN)
FLA at CAR  19:00   (FDSNSO, SCRIPPS)
NSH at MIN  20:00   (ESPN+, HULU)
PHI at CHI  21:00   (TNT, truTV, HBO MAX)
UTA at COL  21:00   (Utah16, ALT)
CGY at EDM  21:00   (SNW, SN360)
SJS at VGK  22:00   (NBCSCA, SCRIPPS)
SEA at LAK  22:00   (FDSNW, KHN/Prime, KONG)

              2025-12-24 (WED)               
=============================================
No games

              2025-12-25 (THU)               
=============================================
No games

              2025-12-26 (FRI)               
=============================================
No games
```

<!-- END:schedule -->

## Schedule for a particular team with (`--schedule` or `-s`) and (`--team <TEAM>` or `-t <TEAM>`)

Shows the full schedule for a particular team. The is specified by its common 3-letter code, e.g., "BOS", "NYR", "STL", etc.

<!-- BEGIN:teamschedule -->

```text
$ ./target/release/nhl -s -t MTL


=======================================================
                  MTL season schedule                  
=======================================================
 1 Wed Oct  8 2025  MTL 2 - 5 TOR   
 2 Thu Oct  9 2025  MTL 5 - 1 DET   
 3 Sat Oct 11 2025  MTL 3 - 2 CHI   
 4 Tue Oct 14 2025  SEA 4 - 5 MTL (OT)
 5 Thu Oct 16 2025  NSH 2 - 3 MTL (OT)
 6 Sat Oct 18 2025  NYR 4 - 3 MTL   
 7 Mon Oct 20 2025  BUF 2 - 4 MTL   
 8 Wed Oct 22 2025  MTL 2 - 1 CGY (OT)
 9 Thu Oct 23 2025  MTL 5 - 6 EDM   
10 Sat Oct 25 2025  MTL 4 - 3 VAN   
11 Tue Oct 28 2025  MTL 4 - 3 SEA (OT)
12 Sat Nov  1 2025  OTT 3 - 4 MTL (OT)
13 Tue Nov  4 2025  PHI 5 - 4 MTL (SO)
14 Thu Nov  6 2025  MTL 3 - 4 NJD (OT)
15 Sat Nov  8 2025  UTA 2 - 6 MTL   
16 Tue Nov 11 2025  LAK 5 - 1 MTL   
17 Thu Nov 13 2025  DAL 7 - 0 MTL   
18 Sat Nov 15 2025  BOS 3 - 2 MTL   
19 Mon Nov 17 2025  MTL 3 - 4 CBJ (SO)
20 Thu Nov 20 2025  WSH 8 - 4 MTL   
21 Sat Nov 22 2025  TOR 2 - 5 MTL   
22 Wed Nov 26 2025  MTL 4 - 3 UTA   
23 Fri Nov 28 2025  MTL 4 - 1 VGK   
24 Sat Nov 29 2025  MTL 2 - 7 COL   
25 Tue Dec  2 2025  OTT 5 - 2 MTL   
26 Wed Dec  3 2025  WPG 2 - 3 MTL (SO)
27 Sat Dec  6 2025  MTL 2 - 1 TOR (SO)
28 Sun Dec  7 2025  STL 4 - 3 MTL   
29 Tue Dec  9 2025  TBL 6 - 1 MTL   
30 Thu Dec 11 2025  MTL 4 - 2 PIT   
31 Sat Dec 13 2025  MTL 4 - 5 NYR (OT)
32 Sun Dec 14 2025  EDM 1 - 4 MTL   
33 Tue Dec 16 2025  PHI 4 - 1 MTL   
34 Thu Dec 18 2025  CHI 1 - 4 MTL   
35 Sat Dec 20 2025  PIT 0 - 3 MTL  19:00 (In progress) (CITY, SNE, TVAS, SN-PIT)
36 Sun Dec 21 2025  MTL at PIT  19:00  (TSN2, RDS, SN-PIT)
37 Tue Dec 23 2025  MTL at BOS  19:00  (TSN2, RDS, NESN)
38 Sun Dec 28 2025  MTL at TBL  17:00  (TSN2, RDS, The Spot)
39 Tue Dec 30 2025  MTL at FLA  19:00  (TSN2, RDS, SCRIPPS)
40 Thu Jan  1 2026  MTL at CAR  19:00  (TSN2, RDS, FDSNSO)
41 Sat Jan  3 2026  MTL at STL  16:00  (TSN2, RDS, FDSNMW)
42 Sun Jan  4 2026  MTL at DAL  14:00  (TSN2, RDS, Victory+)
43 Wed Jan  7 2026  CGY at MTL  19:30  (SN, RDS)
44 Thu Jan  8 2026  FLA at MTL  19:00  (TSN2, RDS, SCRIPPS)
45 Sat Jan 10 2026  DET at MTL  19:00  (CITY, SNE, TVAS, FDSNDET)
46 Mon Jan 12 2026  VAN at MTL  19:30  (Prime, RDS)
47 Tue Jan 13 2026  MTL at WSH  19:00  (TSN2, RDS, MNMT)
48 Thu Jan 15 2026  MTL at BUF  19:00  (TSN2, RDS, MSG-B)
49 Sat Jan 17 2026  MTL at OTT  19:00  (CITY, SNE, TVAS)
50 Tue Jan 20 2026  MIN at MTL  19:00  (TSN2, RDS, FDSNNO, FDSNWI)
51 Thu Jan 22 2026  BUF at MTL  19:00  (TSN2, RDS, MSG-B)
52 Sat Jan 24 2026  MTL at BOS  19:00  (CBC, SNE, SNO, SNP, NHLN, TVAS, NESN)
53 Tue Jan 27 2026  VGK at MTL  19:00  (TSN2, RDS, SCRIPPS)
54 Thu Jan 29 2026  COL at MTL  19:00  (TSN2, RDS, ALT)
55 Sat Jan 31 2026  MTL at BUF  19:00  (CITY, SNE, NHLN, TVAS, MSG-B)
56 Mon Feb  2 2026  MTL at MIN  19:30  (Prime, RDS, FDSNNO, FDSNWI)
57 Wed Feb  4 2026  MTL at WPG  19:00  (SN, RDS)
58 Thu Feb 26 2026  NYI at MTL  19:00  (TSN2, RDS, MSGSN)
59 Sat Feb 28 2026  WSH at MTL  19:00  (CITY, SNE, TVAS, MNMT)
60 Tue Mar  3 2026  MTL at SJS  22:00  (TSN2, RDS, NBCSCA)
61 Fri Mar  6 2026  MTL at ANA  21:00  (TSN2, RDS, Victory+, KCOP-13)
62 Sat Mar  7 2026  MTL at LAK  19:00  (CITY, SNE, TVAS, FDSNW, KCAL)
63 Tue Mar 10 2026  TOR at MTL  18:00  (SNE, SNO, SNP, RDS)
64 Wed Mar 11 2026  MTL at OTT  18:30  (SN, RDS)
65 Sat Mar 14 2026  SJS at MTL  18:00  (CITY, SNE, TVAS, NBCSCA)
66 Sun Mar 15 2026  ANA at MTL  18:00  (TSN2, RDS, Victory+, KCOP-13)
67 Tue Mar 17 2026  BOS at MTL  18:00  (TSN2, RDS, NESN)
68 Thu Mar 19 2026  MTL at DET  18:00  (TSN2, RDS, FDSNDET)
69 Sat Mar 21 2026  NYI at MTL  18:00  (CITY, SNE, TVAS, MSGSN)
70 Tue Mar 24 2026  CAR at MTL  18:00  (TSN2, RDS, FDSNSO)
71 Thu Mar 26 2026  CBJ at MTL  18:00  (TSN2, RDS, FDSNOH)
72 Sat Mar 28 2026  MTL at NSH  18:00  (CITY, SNE, TVAS, FDSNSO)
73 Sun Mar 29 2026  MTL at CAR  16:00  (TSN2, RDS, FDSNSO)
74 Tue Mar 31 2026  MTL at TBL  18:00  (TSN2, RDS, The Spot)
75 Thu Apr  2 2026  MTL at NYR  18:00  (TSN2, RDS, MSG)
76 Sat Apr  4 2026  MTL at NJD  18:00  (CITY, SNE, TVAS, MSG)
77 Sun Apr  5 2026  NJD at MTL  18:00  (TSN2, RDS, MSGSN)
78 Tue Apr  7 2026  FLA at MTL  18:00  (TSN2, RDS, SCRIPPS)
79 Thu Apr  9 2026  TBL at MTL  18:00  (TSN2, RDS, The Spot)
80 Sat Apr 11 2026  CBJ at MTL  18:00  (CITY, SNE, TVAS, FDSNOH)
81 Sun Apr 12 2026  MTL at NYI  17:00  (TSN2, RDS, MSGSN)
82 Tue Apr 14 2026  MTL at PHI  18:00  (TSN2, RDS, NBCSP)
```

<!-- END:teamschedule -->

## Playoff matchups with `--playoffs` or `-p`

Displays the current playoff matchups.

<!-- BEGIN:playoffs -->

```text
$ ./target/release/nhl -p


===================================
          Playoff Picture          
===================================
[8-w] NY Islanders (6) at Carolina (13)
[3-2] Washington (7) at Philadelphia (7)
[7-w] New Jersey (6) at Detroit (7)
[3-2] Tampa Bay (5) at Montréal (6)

Outside looking-in: FLA (5) BOS (5) PIT (4) OTT (4) NYR (3) 

[8-w] San Jose (2) at Colorado (23)
[3-2] Minnesota (13) at Dallas (17)
[7-w] Edmonton (4) at Vegas (10)
[3-2] Los Angeles (5) at Anaheim (7)

Outside looking-in: UTA (0) STL (-1) 
```

<!-- END:playoffs -->
----

## Resources

* New API docs: <https://gitlab.com/dword4/nhlapi/-/blob/master/new-api.md>
* Great docs: <https://github.com/Zmalski/NHL-API-Reference>
* Transform tools, JSON to Rust Serde: <https://transform.tools/json-to-rust-serde>
