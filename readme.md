# nhl

National Hockey League (NHL) stats as I like 'em.

`nhl` is a command line utility that fetches the current NHL standings, 
schedule, or current playoff matchups, and displays them in a table format 
in the command window.

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
 1. Tampa Bay    41  12   5  20  32  10
 2. Montréal     42  10   4  14   0   8
 3. Detroit      43   9   3  17  -7  -2
 4. Buffalo      40   6   8  15  -1  13
 5. Florida      41   6   3  19  -1   0
 6. Ottawa       40   5   3  15   4   7
 7. Toronto      41   4   2  15  -1  -1
 8. Boston       42   4  -2  15  -3 -11
             👉🏻 330  56  26 130  23  24 0.585

=======================================
         Metropolitan division         
=======================================
                 GP +/- L10  RW  GD L10
 1. Carolina     41  10  -1  15  11  -6
 2. Philadelphia 40   9   2  14   8   3
 3. Pittsburgh   41   8   2  16   4   1
 4. NY Islanders 42   8   1  15   0  -6
 5. Washington   42   6  -2  19  14  -9
 6. New Jersey   41   5   1  15  -8   0
 7. Columbus     41   2   1  11 -10   5
 8. NY Rangers   43   2  -1  13  -3  -3
             👉🏻 331  50   3 118  16 -15 0.576

=======================================
           Central division            
=======================================
                 GP +/- L10  RW  GD L10
 1. Colorado     41  28   8  28  72  20
 2. Dallas       42  16   1  21  29   3
 3. Minnesota    43  15   5  18  23  11
 4. Nashville    41   1   4  14 -18   4
 5. Utah         42  -1   0  14   5   3
 6. St. Louis    43  -1   1  17 -39  -5
 7. Chicago      41  -2  -3  13 -17 -14
 8. Winnipeg     40  -6  -5  13 -11  -7
             👉🏻 333  50  11 138  44  15 0.575

=======================================
           Pacific division            
=======================================
                 GP +/- L10  RW  GD L10
 1. Vegas        39   6  -2  13  -1  -5
 2. Seattle      39   4   3  12 -12   5
 3. Anaheim      41   4  -4  13  -7 -17
 4. Edmonton     42   4   0  15  -4  -3
 5. Los Angeles  40   3  -3  11  -4  -6
 6. San Jose     41   2   2  11 -19  -3
 7. Calgary      41  -1   2  14 -10   3
 8. Vancouver    41  -4   2  10 -26  -2
             👉🏻 324  18   0  99 -83 -28 0.528

=======================================
          Eastern conference           
=======================================
                 GP +/- L10  RW  GD L10
 1. Tampa Bay    41  12   5  20  32  10
 2. Carolina     41  10  -1  15  11  -6
 3. Montréal     42  10   4  14   0   8
 4. Philadelphia 40   9   2  14   8   3
 5. Detroit      43   9   3  17  -7  -2
 6. Pittsburgh   41   8   2  16   4   1
 7. NY Islanders 42   8   1  15   0  -6
 8. Buffalo      40   6   8  15  -1  13
 9. Florida      41   6   3  19  -1   0
10. Washington   42   6  -2  19  14  -9
11. Ottawa       40   5   3  15   4   7
12. New Jersey   41   5   1  15  -8   0
13. Toronto      41   4   2  15  -1  -1
14. Boston       42   4  -2  15  -3 -11
15. Columbus     41   2   1  11 -10   5
16. NY Rangers   43   2  -1  13  -3  -3
             👉🏻 661 106  29 248  39   9 0.580

=======================================
          Western conference           
=======================================
                 GP +/- L10  RW  GD L10
 1. Colorado     41  28   8  28  72  20
 2. Dallas       42  16   1  21  29   3
 3. Minnesota    43  15   5  18  23  11
 4. Vegas        39   6  -2  13  -1  -5
 5. Seattle      39   4   3  12 -12   5
 6. Anaheim      41   4  -4  13  -7 -17
 7. Edmonton     42   4   0  15  -4  -3
 8. Los Angeles  40   3  -3  11  -4  -6
 9. San Jose     41   2   2  11 -19  -3
10. Nashville    41   1   4  14 -18   4
11. Calgary      41  -1   2  14 -10   3
12. Utah         42  -1   0  14   5   3
13. St. Louis    43  -1   1  17 -39  -5
14. Chicago      41  -2  -3  13 -17 -14
15. Vancouver    41  -4   2  10 -26  -2
16. Winnipeg     40  -6  -5  13 -11  -7
             👉🏻 657  68  11 237 -39 -13 0.552

=======================================
              Full league              
=======================================
                 GP +/- L10  RW  GD L10
 1. Colorado     41  28   8  28  72  20
 2. Dallas       42  16   1  21  29   3
 3. Minnesota    43  15   5  18  23  11
 4. Tampa Bay    41  12   5  20  32  10
 5. Carolina     41  10  -1  15  11  -6
 6. Montréal     42  10   4  14   0   8
 7. Philadelphia 40   9   2  14   8   3
 8. Detroit      43   9   3  17  -7  -2
 9. Pittsburgh   41   8   2  16   4   1
10. NY Islanders 42   8   1  15   0  -6
11. Vegas        39   6  -2  13  -1  -5
12. Buffalo      40   6   8  15  -1  13
13. Florida      41   6   3  19  -1   0
14. Washington   42   6  -2  19  14  -9
15. Ottawa       40   5   3  15   4   7
16. New Jersey   41   5   1  15  -8   0
17. Seattle      39   4   3  12 -12   5
18. Toronto      41   4   2  15  -1  -1
19. Anaheim      41   4  -4  13  -7 -17
20. Boston       42   4  -2  15  -3 -11
21. Edmonton     42   4   0  15  -4  -3
22. Los Angeles  40   3  -3  11  -4  -6
23. Columbus     41   2   1  11 -10   5
24. San Jose     41   2   2  11 -19  -3
25. NY Rangers   43   2  -1  13  -3  -3
26. Nashville    41   1   4  14 -18   4
27. Calgary      41  -1   2  14 -10   3
28. Utah         42  -1   0  14   5   3
29. St. Louis    43  -1   1  17 -39  -5
30. Chicago      41  -2  -3  13 -17 -14
31. Vancouver    41  -4   2  10 -26  -2
32. Winnipeg     40  -6  -5  13 -11  -7
             👉🏻1318 174  40 485   0  -4 0.566

=======================================
      Full league (last 10 games)      
=======================================
                 GP +/- L10  RW  GD L10
 1. Colorado     41  28   8  28  72  20
 2. Buffalo      40   6   8  15  -1  13
 3. Minnesota    43  15   5  18  23  11
 4. Tampa Bay    41  12   5  20  32  10
 5. Montréal     42  10   4  14   0   8
 6. Nashville    41   1   4  14 -18   4
 7. Detroit      43   9   3  17  -7  -2
 8. Florida      41   6   3  19  -1   0
 9. Ottawa       40   5   3  15   4   7
10. Seattle      39   4   3  12 -12   5
11. Philadelphia 40   9   2  14   8   3
12. Pittsburgh   41   8   2  16   4   1
13. Toronto      41   4   2  15  -1  -1
14. San Jose     41   2   2  11 -19  -3
15. Calgary      41  -1   2  14 -10   3
16. Vancouver    41  -4   2  10 -26  -2
17. Dallas       42  16   1  21  29   3
18. NY Islanders 42   8   1  15   0  -6
19. New Jersey   41   5   1  15  -8   0
20. Columbus     41   2   1  11 -10   5
21. St. Louis    43  -1   1  17 -39  -5
22. Edmonton     42   4   0  15  -4  -3
23. Utah         42  -1   0  14   5   3
24. Carolina     41  10  -1  15  11  -6
25. NY Rangers   43   2  -1  13  -3  -3
26. Vegas        39   6  -2  13  -1  -5
27. Washington   42   6  -2  19  14  -9
28. Boston       42   4  -2  15  -3 -11
29. Los Angeles  40   3  -3  11  -4  -6
30. Chicago      41  -2  -3  13 -17 -14
31. Anaheim      41   4  -4  13  -7 -17
32. Winnipeg     40  -6  -5  13 -11  -7
             👉🏻1318 174  40 485   0  -4 0.566
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
 1. Tampa Bay    41  12   5  20  32  10
 2. Montréal     42  10   4  14   0   8
 3. Detroit      43   9   3  17  -7  -2
 4. Buffalo      40   6   8  15  -1  13
 5. Florida      41   6   3  19  -1   0
 6. Ottawa       40   5   3  15   4   7
 7. Toronto      41   4   2  15  -1  -1
 8. Boston       42   4  -2  15  -3 -11
             👉🏻 330  56  26 130  23  24 0.585

=======================================
         Metropolitan division         
=======================================
                 GP +/- L10  RW  GD L10
 1. Carolina     41  10  -1  15  11  -6
 2. Philadelphia 40   9   2  14   8   3
 3. Pittsburgh   41   8   2  16   4   1
 4. NY Islanders 42   8   1  15   0  -6
 5. Washington   42   6  -2  19  14  -9
 6. New Jersey   41   5   1  15  -8   0
 7. Columbus     41   2   1  11 -10   5
 8. NY Rangers   43   2  -1  13  -3  -3
             👉🏻 331  50   3 118  16 -15 0.576

=======================================
           Central division            
=======================================
                 GP +/- L10  RW  GD L10
 1. Colorado     41  28   8  28  72  20
 2. Dallas       42  16   1  21  29   3
 3. Minnesota    43  15   5  18  23  11
 4. Nashville    41   1   4  14 -18   4
 5. Utah         42  -1   0  14   5   3
 6. St. Louis    43  -1   1  17 -39  -5
 7. Chicago      41  -2  -3  13 -17 -14
 8. Winnipeg     40  -6  -5  13 -11  -7
             👉🏻 333  50  11 138  44  15 0.575

=======================================
           Pacific division            
=======================================
                 GP +/- L10  RW  GD L10
 1. Vegas        39   6  -2  13  -1  -5
 2. Seattle      39   4   3  12 -12   5
 3. Anaheim      41   4  -4  13  -7 -17
 4. Edmonton     42   4   0  15  -4  -3
 5. Los Angeles  40   3  -3  11  -4  -6
 6. San Jose     41   2   2  11 -19  -3
 7. Calgary      41  -1   2  14 -10   3
 8. Vancouver    41  -4   2  10 -26  -2
             👉🏻 324  18   0  99 -83 -28 0.528
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
 1. Tampa Bay    41  12   5  20  32  10
 2. Carolina     41  10  -1  15  11  -6
 3. Montréal     42  10   4  14   0   8
 4. Philadelphia 40   9   2  14   8   3
 5. Detroit      43   9   3  17  -7  -2
 6. Pittsburgh   41   8   2  16   4   1
 7. NY Islanders 42   8   1  15   0  -6
 8. Buffalo      40   6   8  15  -1  13
 9. Florida      41   6   3  19  -1   0
10. Washington   42   6  -2  19  14  -9
11. Ottawa       40   5   3  15   4   7
12. New Jersey   41   5   1  15  -8   0
13. Toronto      41   4   2  15  -1  -1
14. Boston       42   4  -2  15  -3 -11
15. Columbus     41   2   1  11 -10   5
16. NY Rangers   43   2  -1  13  -3  -3
             👉🏻 661 106  29 248  39   9 0.580

=======================================
          Western conference           
=======================================
                 GP +/- L10  RW  GD L10
 1. Colorado     41  28   8  28  72  20
 2. Dallas       42  16   1  21  29   3
 3. Minnesota    43  15   5  18  23  11
 4. Vegas        39   6  -2  13  -1  -5
 5. Seattle      39   4   3  12 -12   5
 6. Anaheim      41   4  -4  13  -7 -17
 7. Edmonton     42   4   0  15  -4  -3
 8. Los Angeles  40   3  -3  11  -4  -6
 9. San Jose     41   2   2  11 -19  -3
10. Nashville    41   1   4  14 -18   4
11. Calgary      41  -1   2  14 -10   3
12. Utah         42  -1   0  14   5   3
13. St. Louis    43  -1   1  17 -39  -5
14. Chicago      41  -2  -3  13 -17 -14
15. Vancouver    41  -4   2  10 -26  -2
16. Winnipeg     40  -6  -5  13 -11  -7
             👉🏻 657  68  11 237 -39 -13 0.552
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
 1. Colorado     41  28   8  28  72  20
 2. Dallas       42  16   1  21  29   3
 3. Minnesota    43  15   5  18  23  11
 4. Tampa Bay    41  12   5  20  32  10
 5. Carolina     41  10  -1  15  11  -6
 6. Montréal     42  10   4  14   0   8
 7. Philadelphia 40   9   2  14   8   3
 8. Detroit      43   9   3  17  -7  -2
 9. Pittsburgh   41   8   2  16   4   1
10. NY Islanders 42   8   1  15   0  -6
11. Vegas        39   6  -2  13  -1  -5
12. Buffalo      40   6   8  15  -1  13
13. Florida      41   6   3  19  -1   0
14. Washington   42   6  -2  19  14  -9
15. Ottawa       40   5   3  15   4   7
16. New Jersey   41   5   1  15  -8   0
17. Seattle      39   4   3  12 -12   5
18. Toronto      41   4   2  15  -1  -1
19. Anaheim      41   4  -4  13  -7 -17
20. Boston       42   4  -2  15  -3 -11
21. Edmonton     42   4   0  15  -4  -3
22. Los Angeles  40   3  -3  11  -4  -6
23. Columbus     41   2   1  11 -10   5
24. San Jose     41   2   2  11 -19  -3
25. NY Rangers   43   2  -1  13  -3  -3
26. Nashville    41   1   4  14 -18   4
27. Calgary      41  -1   2  14 -10   3
28. Utah         42  -1   0  14   5   3
29. St. Louis    43  -1   1  17 -39  -5
30. Chicago      41  -2  -3  13 -17 -14
31. Vancouver    41  -4   2  10 -26  -2
32. Winnipeg     40  -6  -5  13 -11  -7
             👉🏻1318 174  40 485   0  -4 0.566
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
 1. Colorado     41  28   8  28  72  20
 2. Buffalo      40   6   8  15  -1  13
 3. Minnesota    43  15   5  18  23  11
 4. Tampa Bay    41  12   5  20  32  10
 5. Montréal     42  10   4  14   0   8
 6. Nashville    41   1   4  14 -18   4
 7. Detroit      43   9   3  17  -7  -2
 8. Florida      41   6   3  19  -1   0
 9. Ottawa       40   5   3  15   4   7
10. Seattle      39   4   3  12 -12   5
11. Philadelphia 40   9   2  14   8   3
12. Pittsburgh   41   8   2  16   4   1
13. Toronto      41   4   2  15  -1  -1
14. San Jose     41   2   2  11 -19  -3
15. Calgary      41  -1   2  14 -10   3
16. Vancouver    41  -4   2  10 -26  -2
17. Dallas       42  16   1  21  29   3
18. NY Islanders 42   8   1  15   0  -6
19. New Jersey   41   5   1  15  -8   0
20. Columbus     41   2   1  11 -10   5
21. St. Louis    43  -1   1  17 -39  -5
22. Edmonton     42   4   0  15  -4  -3
23. Utah         42  -1   0  14   5   3
24. Carolina     41  10  -1  15  11  -6
25. NY Rangers   43   2  -1  13  -3  -3
26. Vegas        39   6  -2  13  -1  -5
27. Washington   42   6  -2  19  14  -9
28. Boston       42   4  -2  15  -3 -11
29. Los Angeles  40   3  -3  11  -4  -6
30. Chicago      41  -2  -3  13 -17 -14
31. Anaheim      41   4  -4  13  -7 -17
32. Winnipeg     40  -6  -5  13 -11  -7
             👉🏻1318 174  40 485   0  -4 0.566
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

[8-w] Buffalo (6) at Tampa Bay (12)
[7-w] NY Islanders (8) at Carolina (10)
[3-2] Detroit (9) at Montréal (10)
[3-2] Pittsburgh (8) at Philadelphia (9)

Outside looking-in: FLA (6) WSH (6) OTT (5) NJD (5) TOR (4) BOS (4) 

Western Conference:

[8-w] Los Angeles (3) at Colorado (28)
[3-2] Minnesota (15) at Dallas (16)
[7-w] Edmonton (4) at Vegas (6)
[3-2] Anaheim (4) at Seattle (4)

Outside looking-in: SJS (2) NSH (1) 
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

              2026-01-04 (SUN)               
=============================================
MTL 4 - 3 DAL  14:00   (TSN2, RDS, Victory+)
PIT 5 - 4 CBJ  15:00   (SNW, SN1, FDSNOH, SN-PIT)
COL 1 - 2 FLA  17:00   (SN, ALT, SCRIPPS)
CAR 3 - 1 NJD  19:00   (FDSNSO, MSGSN)
VGK 2 - 2 CHI  19:00   (SN1, TVAS, CHSN, SCRIPPS)

              2026-01-05 (MON)               
=============================================
UTA at NYR  19:00   (Utah16, MSG)
ANA at WSH  19:00   (Victory+, MNMT, KCOP-13)
DET at OTT  19:30   (Prime, TVAS, FDSNDET)
SEA at CGY  21:30   (SNW, KHN/Prime, KING 5, KONG)
MIN at LAK  22:30   (FDSNNO, FDSNW, FDSNWI)

              2026-01-06 (TUE)               
=============================================
VAN at BUF  19:00   (SNP, MSG-B)
COL at TBL  19:00   (SNE, TVAS, The Spot, ALT)
ANA at PHI  19:00   (Victory+, NBCSP, KCOP-13)
DAL at CAR  19:00   (FDSNSO, Victory+)
NJD at NYI  19:30   (ESPN+, HULU)
FLA at TOR  19:30   (TNT, HBO MAX, SNO)
VGK at WPG  20:00   (TSN3, SCRIPPS)
NSH at EDM  21:00   (SNW, TVAS, FDSNSO)
CBJ at SJS  22:00   (FDSNOH, NBCSCA)
BOS at SEA  22:00   (SN, KHN/Prime, NESN, KONG)

              2026-01-07 (WED)               
=============================================
DAL at WSH  19:00   (TNT, HBO MAX, Victory+)
CGY at MTL  19:30   (SN, RDS)
STL at CHI  21:30   (TNT, truTV, HBO MAX)
OTT at UTA  21:30   (RDS2, TSN5, Utah16)
SJS at LAK  22:30   (SN, FDSNW, NBCSCA)

              2026-01-08 (THU)               
=============================================
CGY at BOS  19:00   (SNO, SN1, NESN)
FLA at MTL  19:00   (TSN2, RDS, SCRIPPS)
VAN at DET  19:00   (SNP, FDSNDET)
BUF at NYR  19:00   (MSG-B, MSG)
TOR at PHI  19:00   (TVAS, TSN4, NBCSP)
NJD at PIT  19:00   (SN-PIT, MSGSN)
ANA at CAR  19:00   (FDSNSO, Victory+, KCOP-13)
NYI at NSH  20:00   (ESPN+, HULU)
EDM at WPG  20:00   (SNW, TSN3)
OTT at COL  21:00   (RDS2, TSN5, ALT, KTVD)
CBJ at VGK  22:00   (SN, SN1, TVAS, FDSNOH, SCRIPPS)
MIN at SEA  22:00   (FDSNNOX, KHN/Prime, FDSNWI, KONG)

              2026-01-09 (FRI)               
=============================================
WSH at CHI  20:00   (SNP, NHLN, TVAS, CHSN, MNMT2)
LAK at WPG  20:00   (TSN3, FDSNW)
STL at UTA  21:00   (FDSNMW, KMOV-TV, Matrix-MW, Utah16)

              2026-01-10 (SAT)               
=============================================
NYR at BOS  13:00   (ABC, SN)
CGY at PIT  15:30   (SN, TVAS, SN-PIT)
CBJ at COL  16:00   (FDSNOH, ALT)
DAL at SJS  16:00   (Victory+, NBCSCA)
ANA at BUF  19:00   (Victory+, MSG-B, KCOP-13)
VAN at TOR  19:00   (SN, CBC)
DET at MTL  19:00   (CITY, SNE, TVAS, FDSNDET)
FLA at OTT  19:00   (NHLN, SN1, TVAS2, SCRIPPS)
TBL at PHI  19:00   (The Spot, NBCSP)
SEA at CAR  19:00   (FDSNSO, KHN/Prime, KING 5, KONG)
CHI at NSH  20:00   (FDSNSO, CHSN)
NYI at MIN  20:00   (FDSNNO, MSGSN, FDSNWI)
LAK at EDM  22:00   (CBC, CITY, SN, FDSNW)
STL at VGK  22:00   (FDSNMW, SCRIPPS)
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


----

## Resources

* New API docs: <https://gitlab.com/dword4/nhlapi/-/blob/master/new-api.md>
* Great docs: <https://github.com/Zmalski/NHL-API-Reference>
* Transform tools, JSON to Rust Serde: <https://transform.tools/json-to-rust-serde>
