# nhl

National Hockey League (NHL) stats as I like 'em.

`nhl` is a command line utility that fetches the current NHL standings, 
schedule, or current playoff matchups, and displays them in a table format 
in the command window.

Presently `nhl` can display the following things:

* NHL standings, in various ways
* NHL schedules, in various ways
* Playoff matchups, given the current standings

## Motivation

I want to see NHL standings that clearly show team rankings 
despite differences in games played.

I find ranking teams by points, as traditional in media, isn't
great because of discrepancies in games played.  Also some NHL 
games end up worth 3-points, and others just 2-points.  With the 
league running at about .550 win percentage, I feel it makes 
more sense to rank teams in terms of ***wins minus losses***.

**What's wrong with win percentage?**

Win percentage isn't universally available in NHL standings online,
and win percentage is never clear how many games separate two 
teams. With ***wins minus losses***, the number of games separating
teams is unambiguous.

## Quick start

```text
nhl            # NHL standings, 5 different ways
nhl -d         # NHL standings by division
nhl -c         # NHL standings by conference
nhl -f         # NHL league standings
nhl -l10       # NHL league standings by last 10 games
nhl -p         # NHL playoff matchups
nhl -s         # NHL schedule, including broadcasters, next 7 days
nhl -s -t MTL  # Full schedule including broadcasters and results for one team 
```

## Usage

### Version information with `-V` or `--version`

Here's the latest version:

<!-- BEGIN:version -->

```text
$ nhl -V

nhl 0.2.9
```

<!-- END:version -->

### Help information with `-h` or `--help`
<!-- BEGIN:help -->

```text
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

<!-- END:help -->

## Standings

All standings are assembled from results from the following call to the NHL api:

* https://api-web.nhle.com/v1/standings/now

### Default output of `nhl` is standings, sliced and diced

The default output is **standings** segmented segmented in different ways

* Four divisions standings,
* two conference standings, 
* full league standings, and
* Last 10 games performance.

Standings order are determined by

* wins minus losses (descending), 
* games played (ascending), and 
* regulation wins (descending).

The standings groups show

* GP — games played, 
* +/- wins minus losses, 
* L10 — and the last 10 games' wins minus losses,
* RW — number of regulation wins,
* GD — goal differential,
* +/- — goal differential, last 10 games.

Below each standings group is a tally of the columns in the group.

<!-- BEGIN:vanilla -->

```text
$ nhl 


=======================================
           Atlantic division           
=======================================
                 GP +/- L10  RW  GD L10
 1. Tampa Bay    48  18   9  25  49  24
 2. Detroit      50  14   5  21   4   8
 3. Montréal     50  13   3  18   5   4
 4. Buffalo      49  10   3  21  10   7
 5. Boston       50   8   6  21  10  18
 6. Toronto      49   7   4  17   1   2
 7. Florida      48   5  -1  22 -14 -17
 8. Ottawa       49   4   0  18  -1  -3
             👉🏻 393  79  29 163  64  43 0.601

=======================================
         Metropolitan division         
=======================================
                 GP +/- L10  RW  GD L10
 1. Carolina     50  16   5  22  28  15
 2. NY Islanders 49  10   3  18   8   5
 3. Pittsburgh   48   9   4  19   9  10
 4. Philadelphia 48   6  -3  16  -7 -16
 5. Washington   50   4  -3  21  11  -5
 6. New Jersey   50   4   0  17 -20  -9
 7. Columbus     49   2   1  13 -15  -2
 8. NY Rangers   51  -3  -5  14 -21 -17
             👉🏻 395  48   2 140  -7 -19 0.561

=======================================
           Central division            
=======================================
                 GP +/- L10  RW  GD L10
 1. Colorado     47  29   3  31  78  15
 2. Dallas       50  15  -2  24  28  -3
 3. Minnesota    51  14   0  19  17  -4
 4. Utah         49   5   5  19  18  14
 5. Nashville    49   1   0  16 -25  -9
 6. Chicago      49  -2   2  16 -19   0
 7. Winnipeg     49  -3   2  18  -4   5
 8. St. Louis    50  -4  -2  18 -50 -13
             👉🏻 394  55   8 161  43   5 0.570

=======================================
           Pacific division            
=======================================
                 GP +/- L10  RW  GD L10
 1. Vegas        48  12   5  17  14  14
 2. Edmonton     51   7   2  20  11  12
 3. Anaheim      49   4  -1  15 -12  -9
 4. San Jose     49   4   2  14 -20  -4
 5. Los Angeles  49   4   2  13 -10  -5
 6. Seattle      48   3   0  15 -14  -1
 7. Calgary      49  -2  -1  17 -18  -5
 8. Vancouver    49 -12  -8  10 -51 -27
             👉🏻 392  20   1 121 -100 -25 0.526

=======================================
          Eastern conference           
=======================================
                 GP +/- L10  RW  GD L10
 1. Tampa Bay    48  18   9  25  49  24
 2. Carolina     50  16   5  22  28  15
 3. Detroit      50  14   5  21   4   8
 4. Montréal     50  13   3  18   5   4
 5. Buffalo      49  10   3  21  10   7
 6. NY Islanders 49  10   3  18   8   5
 7. Pittsburgh   48   9   4  19   9  10
 8. Boston       50   8   6  21  10  18
 9. Toronto      49   7   4  17   1   2
10. Philadelphia 48   6  -3  16  -7 -16
11. Florida      48   5  -1  22 -14 -17
12. Ottawa       49   4   0  18  -1  -3
13. Washington   50   4  -3  21  11  -5
14. New Jersey   50   4   0  17 -20  -9
15. Columbus     49   2   1  13 -15  -2
16. NY Rangers   51  -3  -5  14 -21 -17
             👉🏻 788 127  31 303  57  24 0.581

=======================================
          Western conference           
=======================================
                 GP +/- L10  RW  GD L10
 1. Colorado     47  29   3  31  78  15
 2. Dallas       50  15  -2  24  28  -3
 3. Minnesota    51  14   0  19  17  -4
 4. Vegas        48  12   5  17  14  14
 5. Edmonton     51   7   2  20  11  12
 6. Utah         49   5   5  19  18  14
 7. Anaheim      49   4  -1  15 -12  -9
 8. San Jose     49   4   2  14 -20  -4
 9. Los Angeles  49   4   2  13 -10  -5
10. Seattle      48   3   0  15 -14  -1
11. Nashville    49   1   0  16 -25  -9
12. Calgary      49  -2  -1  17 -18  -5
13. Chicago      49  -2   2  16 -19   0
14. Winnipeg     49  -3   2  18  -4   5
15. St. Louis    50  -4  -2  18 -50 -13
16. Vancouver    49 -12  -8  10 -51 -27
             👉🏻 786  75   9 282 -57 -20 0.548

=======================================
              Full league              
=======================================
                 GP +/- L10  RW  GD L10
 1. Colorado     47  29   3  31  78  15
 2. Tampa Bay    48  18   9  25  49  24
 3. Carolina     50  16   5  22  28  15
 4. Dallas       50  15  -2  24  28  -3
 5. Detroit      50  14   5  21   4   8
 6. Minnesota    51  14   0  19  17  -4
 7. Montréal     50  13   3  18   5   4
 8. Vegas        48  12   5  17  14  14
 9. Buffalo      49  10   3  21  10   7
10. NY Islanders 49  10   3  18   8   5
11. Pittsburgh   48   9   4  19   9  10
12. Boston       50   8   6  21  10  18
13. Toronto      49   7   4  17   1   2
14. Edmonton     51   7   2  20  11  12
15. Philadelphia 48   6  -3  16  -7 -16
16. Florida      48   5  -1  22 -14 -17
17. Utah         49   5   5  19  18  14
18. Ottawa       49   4   0  18  -1  -3
19. Anaheim      49   4  -1  15 -12  -9
20. San Jose     49   4   2  14 -20  -4
21. Los Angeles  49   4   2  13 -10  -5
22. Washington   50   4  -3  21  11  -5
23. New Jersey   50   4   0  17 -20  -9
24. Seattle      48   3   0  15 -14  -1
25. Columbus     49   2   1  13 -15  -2
26. Nashville    49   1   0  16 -25  -9
27. Calgary      49  -2  -1  17 -18  -5
28. Chicago      49  -2   2  16 -19   0
29. Winnipeg     49  -3   2  18  -4   5
30. NY Rangers   51  -3  -5  14 -21 -17
31. St. Louis    50  -4  -2  18 -50 -13
32. Vancouver    49 -12  -8  10 -51 -27
             👉🏻1574 202  40 585   0   4 0.564

=======================================
      Full league (last 10 games)      
=======================================
                 GP +/- L10  RW  GD L10
 1. Tampa Bay    48  18   9  25  49  24
 2. Boston       50   8   6  21  10  18
 3. Carolina     50  16   5  22  28  15
 4. Detroit      50  14   5  21   4   8
 5. Vegas        48  12   5  17  14  14
 6. Utah         49   5   5  19  18  14
 7. Pittsburgh   48   9   4  19   9  10
 8. Toronto      49   7   4  17   1   2
 9. Colorado     47  29   3  31  78  15
10. Montréal     50  13   3  18   5   4
11. Buffalo      49  10   3  21  10   7
12. NY Islanders 49  10   3  18   8   5
13. Edmonton     51   7   2  20  11  12
14. San Jose     49   4   2  14 -20  -4
15. Los Angeles  49   4   2  13 -10  -5
16. Chicago      49  -2   2  16 -19   0
17. Winnipeg     49  -3   2  18  -4   5
18. Columbus     49   2   1  13 -15  -2
19. Minnesota    51  14   0  19  17  -4
20. Ottawa       49   4   0  18  -1  -3
21. New Jersey   50   4   0  17 -20  -9
22. Seattle      48   3   0  15 -14  -1
23. Nashville    49   1   0  16 -25  -9
24. Florida      48   5  -1  22 -14 -17
25. Anaheim      49   4  -1  15 -12  -9
26. Calgary      49  -2  -1  17 -18  -5
27. Dallas       50  15  -2  24  28  -3
28. St. Louis    50  -4  -2  18 -50 -13
29. Philadelphia 48   6  -3  16  -7 -16
30. Washington   50   4  -3  21  11  -5
31. NY Rangers   51  -3  -5  14 -21 -17
32. Vancouver    49 -12  -8  10 -51 -27
             👉🏻1574 202  40 585   0   4 0.564
```

<!-- END:vanilla -->

### Division standings with `--division` or `-d`

<!-- BEGIN:division -->

```text
$ nhl -d


=======================================
           Atlantic division           
=======================================
                 GP +/- L10  RW  GD L10
 1. Tampa Bay    48  18   9  25  49  24
 2. Detroit      50  14   5  21   4   8
 3. Montréal     50  13   3  18   5   4
 4. Buffalo      49  10   3  21  10   7
 5. Boston       50   8   6  21  10  18
 6. Toronto      49   7   4  17   1   2
 7. Florida      48   5  -1  22 -14 -17
 8. Ottawa       49   4   0  18  -1  -3
             👉🏻 393  79  29 163  64  43 0.601

=======================================
         Metropolitan division         
=======================================
                 GP +/- L10  RW  GD L10
 1. Carolina     50  16   5  22  28  15
 2. NY Islanders 49  10   3  18   8   5
 3. Pittsburgh   48   9   4  19   9  10
 4. Philadelphia 48   6  -3  16  -7 -16
 5. Washington   50   4  -3  21  11  -5
 6. New Jersey   50   4   0  17 -20  -9
 7. Columbus     49   2   1  13 -15  -2
 8. NY Rangers   51  -3  -5  14 -21 -17
             👉🏻 395  48   2 140  -7 -19 0.561

=======================================
           Central division            
=======================================
                 GP +/- L10  RW  GD L10
 1. Colorado     47  29   3  31  78  15
 2. Dallas       50  15  -2  24  28  -3
 3. Minnesota    51  14   0  19  17  -4
 4. Utah         49   5   5  19  18  14
 5. Nashville    49   1   0  16 -25  -9
 6. Chicago      49  -2   2  16 -19   0
 7. Winnipeg     49  -3   2  18  -4   5
 8. St. Louis    50  -4  -2  18 -50 -13
             👉🏻 394  55   8 161  43   5 0.570

=======================================
           Pacific division            
=======================================
                 GP +/- L10  RW  GD L10
 1. Vegas        48  12   5  17  14  14
 2. Edmonton     51   7   2  20  11  12
 3. Anaheim      49   4  -1  15 -12  -9
 4. San Jose     49   4   2  14 -20  -4
 5. Los Angeles  49   4   2  13 -10  -5
 6. Seattle      48   3   0  15 -14  -1
 7. Calgary      49  -2  -1  17 -18  -5
 8. Vancouver    49 -12  -8  10 -51 -27
             👉🏻 392  20   1 121 -100 -25 0.526
```

<!-- END:division -->

### Conference standings with `--conference` or `-c`

<!-- BEGIN:conference -->

```text
$ nhl -c


=======================================
          Eastern conference           
=======================================
                 GP +/- L10  RW  GD L10
 1. Tampa Bay    48  18   9  25  49  24
 2. Carolina     50  16   5  22  28  15
 3. Detroit      50  14   5  21   4   8
 4. Montréal     50  13   3  18   5   4
 5. Buffalo      49  10   3  21  10   7
 6. NY Islanders 49  10   3  18   8   5
 7. Pittsburgh   48   9   4  19   9  10
 8. Boston       50   8   6  21  10  18
 9. Toronto      49   7   4  17   1   2
10. Philadelphia 48   6  -3  16  -7 -16
11. Florida      48   5  -1  22 -14 -17
12. Ottawa       49   4   0  18  -1  -3
13. Washington   50   4  -3  21  11  -5
14. New Jersey   50   4   0  17 -20  -9
15. Columbus     49   2   1  13 -15  -2
16. NY Rangers   51  -3  -5  14 -21 -17
             👉🏻 788 127  31 303  57  24 0.581

=======================================
          Western conference           
=======================================
                 GP +/- L10  RW  GD L10
 1. Colorado     47  29   3  31  78  15
 2. Dallas       50  15  -2  24  28  -3
 3. Minnesota    51  14   0  19  17  -4
 4. Vegas        48  12   5  17  14  14
 5. Edmonton     51   7   2  20  11  12
 6. Utah         49   5   5  19  18  14
 7. Anaheim      49   4  -1  15 -12  -9
 8. San Jose     49   4   2  14 -20  -4
 9. Los Angeles  49   4   2  13 -10  -5
10. Seattle      48   3   0  15 -14  -1
11. Nashville    49   1   0  16 -25  -9
12. Calgary      49  -2  -1  17 -18  -5
13. Chicago      49  -2   2  16 -19   0
14. Winnipeg     49  -3   2  18  -4   5
15. St. Louis    50  -4  -2  18 -50 -13
16. Vancouver    49 -12  -8  10 -51 -27
             👉🏻 786  75   9 282 -57 -20 0.548
```

<!-- END:conference -->

### Full league standings- `--full` or `-f`

#### Conventional full league standings

<!-- BEGIN:full -->

```text
$ nhl -f


=======================================
              Full league              
=======================================
                 GP +/- L10  RW  GD L10
 1. Colorado     47  29   3  31  78  15
 2. Tampa Bay    48  18   9  25  49  24
 3. Carolina     50  16   5  22  28  15
 4. Dallas       50  15  -2  24  28  -3
 5. Detroit      50  14   5  21   4   8
 6. Minnesota    51  14   0  19  17  -4
 7. Montréal     50  13   3  18   5   4
 8. Vegas        48  12   5  17  14  14
 9. Buffalo      49  10   3  21  10   7
10. NY Islanders 49  10   3  18   8   5
11. Pittsburgh   48   9   4  19   9  10
12. Boston       50   8   6  21  10  18
13. Toronto      49   7   4  17   1   2
14. Edmonton     51   7   2  20  11  12
15. Philadelphia 48   6  -3  16  -7 -16
16. Florida      48   5  -1  22 -14 -17
17. Utah         49   5   5  19  18  14
18. Ottawa       49   4   0  18  -1  -3
19. Anaheim      49   4  -1  15 -12  -9
20. San Jose     49   4   2  14 -20  -4
21. Los Angeles  49   4   2  13 -10  -5
22. Washington   50   4  -3  21  11  -5
23. New Jersey   50   4   0  17 -20  -9
24. Seattle      48   3   0  15 -14  -1
25. Columbus     49   2   1  13 -15  -2
26. Nashville    49   1   0  16 -25  -9
27. Calgary      49  -2  -1  17 -18  -5
28. Chicago      49  -2   2  16 -19   0
29. Winnipeg     49  -3   2  18  -4   5
30. NY Rangers   51  -3  -5  14 -21 -17
31. St. Louis    50  -4  -2  18 -50 -13
32. Vancouver    49 -12  -8  10 -51 -27
             👉🏻1574 202  40 585   0   4 0.564
```

<!-- END:full -->

#### Full league standings by last 10 games
<!-- BEGIN:last10 -->

```text
$ nhl --l10


=======================================
      Full league (last 10 games)      
=======================================
                 GP +/- L10  RW  GD L10
 1. Tampa Bay    48  18   9  25  49  24
 2. Boston       50   8   6  21  10  18
 3. Carolina     50  16   5  22  28  15
 4. Detroit      50  14   5  21   4   8
 5. Vegas        48  12   5  17  14  14
 6. Utah         49   5   5  19  18  14
 7. Pittsburgh   48   9   4  19   9  10
 8. Toronto      49   7   4  17   1   2
 9. Colorado     47  29   3  31  78  15
10. Montréal     50  13   3  18   5   4
11. Buffalo      49  10   3  21  10   7
12. NY Islanders 49  10   3  18   8   5
13. Edmonton     51   7   2  20  11  12
14. San Jose     49   4   2  14 -20  -4
15. Los Angeles  49   4   2  13 -10  -5
16. Chicago      49  -2   2  16 -19   0
17. Winnipeg     49  -3   2  18  -4   5
18. Columbus     49   2   1  13 -15  -2
19. Minnesota    51  14   0  19  17  -4
20. Ottawa       49   4   0  18  -1  -3
21. New Jersey   50   4   0  17 -20  -9
22. Seattle      48   3   0  15 -14  -1
23. Nashville    49   1   0  16 -25  -9
24. Florida      48   5  -1  22 -14 -17
25. Anaheim      49   4  -1  15 -12  -9
26. Calgary      49  -2  -1  17 -18  -5
27. Dallas       50  15  -2  24  28  -3
28. St. Louis    50  -4  -2  18 -50 -13
29. Philadelphia 48   6  -3  16  -7 -16
30. Washington   50   4  -3  21  11  -5
31. NY Rangers   51  -3  -5  14 -21 -17
32. Vancouver    49 -12  -8  10 -51 -27
             👉🏻1574 202  40 585   0   4 0.564
```

<!-- END:last10 -->

### Playoff matchups with `--playoffs` or `-p`

Playoff matchups are derived from the current standings on a ***wins minus losses** basis.

Playoff matchups are followed by a list of proximate teams that are "on the outside, looking-in."

<!-- BEGIN:playoffs -->

```text
$ nhl -p


===================================
         Playoff Matchups          
===================================
Eastern Conference:

[8-w] Boston (8) at Tampa Bay (18)
[7-w] Buffalo (10) at Carolina (16)
[3-2] Montréal (13) at Detroit (14)
[3-2] Pittsburgh (9) at NY Islanders (10)

Outside looking-in: TOR (7) PHI (6) FLA (5) 

Western Conference:

[8-w] San Jose (4) at Colorado (29)
[3-2] Minnesota (14) at Dallas (15)
[7-w] Utah (5) at Vegas (12)
[3-2] Anaheim (4) at Edmonton (7)

Outside looking-in: LAK (4) SEA (3) NSH (1) 
```

<!-- END:playoffs -->

## Schedules

### Current schedule segment (7-days) with `--schedule` or `-s`

The current schedule uses the following call to the NHL api:

* https://api-web.nhle.com/v1/schedule/now

Shows the next 7 days of NHL games (including today), strat times, and broadcast networks.

The output shows scores of games currently in progress, and scores of games very
recently completed.

<!-- BEGIN:schedule -->

```text
$ nhl -s


=======================================================
             Upcoming league-wide schedule             
=======================================================

              2026-01-21 (WED)               
=============================================
DET at TOR  19:00   (TNT, truTV, HBO MAX, SN, TVAS)
ANA at COL  21:00   (Victory+, ALT, KCOP-13)
PHI at UTA  21:00   (Utah16, NBCSP)
PIT at CGY  21:30   (SNW, SN1, TVAS, SN-PIT)
NYI at SEA  21:30   (TNT, truTV, HBO MAX)
WSH at VAN  22:00   (SNP, MNMT)

              2026-01-22 (THU)               
=============================================
VGK at BOS  19:00   (SN, NESN, SCRIPPS)
BUF at MTL  19:00   (TSN2, RDS, MSG-B)
CHI at CAR  19:00   (ESPN+, HULU, SN1)
DAL at CBJ  19:00   (FDSNOH, Victory+)
OTT at NSH  20:00   (RDS2, TSN5, FDSNSO)
FLA at WPG  20:00   (TSN3, SCRIPPS)
PIT at EDM  21:00   (SNW, TVAS, SN-PIT)
DET at MIN  21:30   (ESPN, SNE, SN1)

              2026-01-23 (FRI)               
=============================================
VGK at TOR  19:00   (SN, TVAS, SCRIPPS)
TBL at CHI  19:00   (ESPN)
STL at DAL  20:00   (FDSNMW, Victory+)
PHI at COL  21:00   (NBCSP, ALT2)
WSH at CGY  21:00   (SN1, MNMT)
NJD at VAN  22:00   (SNP, MSGSN)
NYR at SJS  22:00   (SNE, TVAS, NBCSCA, MSG)
ANA at SEA  22:00   (KHN/Prime, Victory+, KONG, KCOP-13)

              2026-01-24 (SAT)               
=============================================
BUF at NYI  13:00   (SN, MSG-B, MSGSN)
UTA at NSH  15:30   (SN, SN1, FDSNSO, Utah16)
MTL at BOS  19:00   (SN, CBC, NHLN, TVAS, NESN)
CAR at OTT  19:00   (CITY, SN1, FDSNSO)
TBL at CBJ  19:00   (The Spot, FDSNOH)
DET at WPG  19:00   (SNW, FDSNDET)
LAK at STL  20:00   (FDSNMW, FDSNW, KCAL)
FLA at MIN  21:00   (FDSNNO, FDSNWI, SCRIPPS)
WSH at EDM  22:00   (CBC, CITY, SN, TVAS2, MNMT)

              2026-01-25 (SUN)               
=============================================
COL at TOR  13:30   (NHLN, TVAS, TSN4, ALT)
NJD at SEA  15:00   (SN, KHN/Prime, MSGSN, KONG)
VGK at OTT  17:00   (RDS2, TSN5, SCRIPPS)
PIT at VAN  18:00   (SN, TVAS, SN-PIT)
FLA at CHI  19:00   (NHLN, SN1, CHSN, SCRIPPS)
ANA at CGY  20:00   (SNE (JIP), SNO (JIP), SNW, SNP (JIP), Victory+, KCOP-13)

              2026-01-26 (MON)               
=============================================
UTA at TBL  19:00   (The Spot, Utah16)
BOS at NYR  19:00   (NHLN, TVAS, NESN, MSG)
NYI at PHI  19:00   (NBCSP, MSGSN)
LAK at CBJ  19:00   (FDSNOH, FDSNW)
ANA at EDM  20:30   (Prime, Victory+, KCOP-13)

              2026-01-27 (TUE)               
=============================================
NSH at BOS  19:00   (SN, FDSNSO, NESN)
BUF at TOR  19:00   (TSN4, MSG-B)
VGK at MTL  19:00   (TSN2, RDS, SCRIPPS)
LAK at DET  19:00   (SN, FDSNDETX, FDSNW)
UTA at FLA  19:00   (Utah16, SCRIPPS)
WPG at NJD  19:00   (TSN3, MSGSN)
DAL at STL  20:00   (ESPN+, HULU)
CHI at MIN  20:00   (CHSN, FDSNNO, FDSNWIX)
SJS at VAN  22:00   (SNP, NBCSCA)
WSH at SEA  22:00   (SN, TVAS, KHN/Prime, MNMT, KONG)
```

<!-- END:schedule -->

### Schedule for a particular team with (`--schedule` or `-s`) and (`--team <TEAM>` or `-t <TEAM>`)

The full-season team-specific schedule uses the following call to the NHL api, showing
the schedule for the Montreal Canadiens "MTL" and the 2025-2026 season.

* https://api-web.nhle.com/v1/club-schedule-season/MTL/20252026

<!-- BEGIN:teamschedule -->

```text
$ nhl -s -t MTL


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
35 Sat Dec 20 2025  PIT 0 - 4 MTL   
36 Sun Dec 21 2025  MTL 3 - 4 PIT (SO)
37 Tue Dec 23 2025  MTL 6 - 2 BOS   
38 Sun Dec 28 2025  MTL 4 - 5 TBL (SO)
39 Tue Dec 30 2025  MTL 3 - 2 FLA (OT)
40 Thu Jan  1 2026  MTL 7 - 5 CAR   
41 Sat Jan  3 2026  MTL 0 - 2 STL   
42 Sun Jan  4 2026  MTL 4 - 3 DAL (OT)
43 Wed Jan  7 2026  CGY 1 - 4 MTL   
44 Thu Jan  8 2026  FLA 2 - 6 MTL   
45 Sat Jan 10 2026  DET 4 - 0 MTL   
46 Mon Jan 12 2026  VAN 3 - 6 MTL   
47 Tue Jan 13 2026  MTL 2 - 3 WSH (OT)
48 Thu Jan 15 2026  MTL 3 - 5 BUF   
49 Sat Jan 17 2026  MTL 6 - 5 OTT (OT)
50 Tue Jan 20 2026  MIN 3 - 4 MTL   
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


----

## Resources

* New API docs: <https://gitlab.com/dword4/nhlapi/-/blob/master/new-api.md>
* Great docs: <https://github.com/Zmalski/NHL-API-Reference>
* Transform tools, JSON to Rust Serde: <https://transform.tools/json-to-rust-serde>
