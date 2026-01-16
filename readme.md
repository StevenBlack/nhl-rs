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
 1. Tampa Bay    45  16  10  23  44  25
 2. Detroit      48  12   3  20   1   3
 3. Montréal     48  11   3  17   3   5
 4. Buffalo      46  10   6  20  10  15
 5. Toronto      46   7   6  17   4  10
 6. Boston       47   7   3  19   9   9
 7. Florida      45   6   1  21  -6  -6
 8. Ottawa       46   3  -2  17  -2  -8
             👉🏻 371  72  30 154  63  53 0.597

=======================================
         Metropolitan division         
=======================================
                 GP +/- L10  RW  GD L10
 1. Carolina     47  13   1  19  16  -2
 2. NY Islanders 46   9   3  16   8   6
 3. Pittsburgh   46   8   5  18   7  16
 4. Philadelphia 46   6  -3  15  -5 -14
 5. Washington   48   6  -1  21  17   2
 6. New Jersey   47   3  -1  16 -19 -12
 7. Columbus     47   2   1  13 -13   2
 8. NY Rangers   48  -2  -4  13 -21 -16
             👉🏻 375  45   1 131 -10 -18 0.560

=======================================
           Central division            
=======================================
                 GP +/- L10  RW  GD L10
 1. Colorado     45  29   5  30  79  18
 2. Dallas       47  16  -2  23  28  -8
 3. Minnesota    47  14   2  18  18  -1
 4. Utah         47   3   3  17  14  10
 5. Nashville    46   2   2  15 -22  -4
 6. Chicago      46  -1   3  15 -16  -1
 7. St. Louis    47  -3  -2  18 -44 -11
 8. Winnipeg     45  -4  -2  16  -7  -3
             👉🏻 370  56   9 152  50   0 0.576

=======================================
           Pacific division            
=======================================
                 GP +/- L10  RW  GD L10
 1. Vegas        45  11   2  16   9   3
 2. Edmonton     47   7   2  18   2   4
 3. Seattle      45   6   5  15  -6  12
 4. San Jose     46   5   4  13 -18  -3
 5. Los Angeles  46   3   0  12  -9  -4
 6. Anaheim      46   1  -7  14 -16 -21
 7. Calgary      46  -4  -2  15 -21 -10
 8. Vancouver    47 -10  -6  10 -44 -22
             👉🏻 368  19  -2 113 -103 -41 0.526

=======================================
          Eastern conference           
=======================================
                 GP +/- L10  RW  GD L10
 1. Tampa Bay    45  16  10  23  44  25
 2. Carolina     47  13   1  19  16  -2
 3. Detroit      48  12   3  20   1   3
 4. Montréal     48  11   3  17   3   5
 5. Buffalo      46  10   6  20  10  15
 6. NY Islanders 46   9   3  16   8   6
 7. Pittsburgh   46   8   5  18   7  16
 8. Toronto      46   7   6  17   4  10
 9. Boston       47   7   3  19   9   9
10. Florida      45   6   1  21  -6  -6
11. Philadelphia 46   6  -3  15  -5 -14
12. Washington   48   6  -1  21  17   2
13. Ottawa       46   3  -2  17  -2  -8
14. New Jersey   47   3  -1  16 -19 -12
15. Columbus     47   2   1  13 -13   2
16. NY Rangers   48  -2  -4  13 -21 -16
             👉🏻 746 117  31 285  53  35 0.578

=======================================
          Western conference           
=======================================
                 GP +/- L10  RW  GD L10
 1. Colorado     45  29   5  30  79  18
 2. Dallas       47  16  -2  23  28  -8
 3. Minnesota    47  14   2  18  18  -1
 4. Vegas        45  11   2  16   9   3
 5. Edmonton     47   7   2  18   2   4
 6. Seattle      45   6   5  15  -6  12
 7. San Jose     46   5   4  13 -18  -3
 8. Los Angeles  46   3   0  12  -9  -4
 9. Utah         47   3   3  17  14  10
10. Nashville    46   2   2  15 -22  -4
11. Anaheim      46   1  -7  14 -16 -21
12. Chicago      46  -1   3  15 -16  -1
13. St. Louis    47  -3  -2  18 -44 -11
14. Winnipeg     45  -4  -2  16  -7  -3
15. Calgary      46  -4  -2  15 -21 -10
16. Vancouver    47 -10  -6  10 -44 -22
             👉🏻 738  75   7 265 -53 -41 0.551

=======================================
              Full league              
=======================================
                 GP +/- L10  RW  GD L10
 1. Colorado     45  29   5  30  79  18
 2. Tampa Bay    45  16  10  23  44  25
 3. Dallas       47  16  -2  23  28  -8
 4. Minnesota    47  14   2  18  18  -1
 5. Carolina     47  13   1  19  16  -2
 6. Detroit      48  12   3  20   1   3
 7. Vegas        45  11   2  16   9   3
 8. Montréal     48  11   3  17   3   5
 9. Buffalo      46  10   6  20  10  15
10. NY Islanders 46   9   3  16   8   6
11. Pittsburgh   46   8   5  18   7  16
12. Toronto      46   7   6  17   4  10
13. Boston       47   7   3  19   9   9
14. Edmonton     47   7   2  18   2   4
15. Florida      45   6   1  21  -6  -6
16. Seattle      45   6   5  15  -6  12
17. Philadelphia 46   6  -3  15  -5 -14
18. Washington   48   6  -1  21  17   2
19. San Jose     46   5   4  13 -18  -3
20. Ottawa       46   3  -2  17  -2  -8
21. Los Angeles  46   3   0  12  -9  -4
22. Utah         47   3   3  17  14  10
23. New Jersey   47   3  -1  16 -19 -12
24. Nashville    46   2   2  15 -22  -4
25. Columbus     47   2   1  13 -13   2
26. Anaheim      46   1  -7  14 -16 -21
27. Chicago      46  -1   3  15 -16  -1
28. NY Rangers   48  -2  -4  13 -21 -16
29. St. Louis    47  -3  -2  18 -44 -11
30. Winnipeg     45  -4  -2  16  -7  -3
31. Calgary      46  -4  -2  15 -21 -10
32. Vancouver    47 -10  -6  10 -44 -22
             👉🏻1484 192  38 550   0  -6 0.565

=======================================
      Full league (last 10 games)      
=======================================
                 GP +/- L10  RW  GD L10
 1. Tampa Bay    45  16  10  23  44  25
 2. Buffalo      46  10   6  20  10  15
 3. Toronto      46   7   6  17   4  10
 4. Colorado     45  29   5  30  79  18
 5. Pittsburgh   46   8   5  18   7  16
 6. Seattle      45   6   5  15  -6  12
 7. San Jose     46   5   4  13 -18  -3
 8. Detroit      48  12   3  20   1   3
 9. Montréal     48  11   3  17   3   5
10. NY Islanders 46   9   3  16   8   6
11. Boston       47   7   3  19   9   9
12. Utah         47   3   3  17  14  10
13. Chicago      46  -1   3  15 -16  -1
14. Minnesota    47  14   2  18  18  -1
15. Vegas        45  11   2  16   9   3
16. Edmonton     47   7   2  18   2   4
17. Nashville    46   2   2  15 -22  -4
18. Carolina     47  13   1  19  16  -2
19. Florida      45   6   1  21  -6  -6
20. Columbus     47   2   1  13 -13   2
21. Los Angeles  46   3   0  12  -9  -4
22. Washington   48   6  -1  21  17   2
23. New Jersey   47   3  -1  16 -19 -12
24. Dallas       47  16  -2  23  28  -8
25. Ottawa       46   3  -2  17  -2  -8
26. St. Louis    47  -3  -2  18 -44 -11
27. Winnipeg     45  -4  -2  16  -7  -3
28. Calgary      46  -4  -2  15 -21 -10
29. Philadelphia 46   6  -3  15  -5 -14
30. NY Rangers   48  -2  -4  13 -21 -16
31. Vancouver    47 -10  -6  10 -44 -22
32. Anaheim      46   1  -7  14 -16 -21
             👉🏻1484 192  38 550   0  -6 0.565
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
 1. Tampa Bay    45  16  10  23  44  25
 2. Detroit      48  12   3  20   1   3
 3. Montréal     48  11   3  17   3   5
 4. Buffalo      46  10   6  20  10  15
 5. Toronto      46   7   6  17   4  10
 6. Boston       47   7   3  19   9   9
 7. Florida      45   6   1  21  -6  -6
 8. Ottawa       46   3  -2  17  -2  -8
             👉🏻 371  72  30 154  63  53 0.597

=======================================
         Metropolitan division         
=======================================
                 GP +/- L10  RW  GD L10
 1. Carolina     47  13   1  19  16  -2
 2. NY Islanders 46   9   3  16   8   6
 3. Pittsburgh   46   8   5  18   7  16
 4. Philadelphia 46   6  -3  15  -5 -14
 5. Washington   48   6  -1  21  17   2
 6. New Jersey   47   3  -1  16 -19 -12
 7. Columbus     47   2   1  13 -13   2
 8. NY Rangers   48  -2  -4  13 -21 -16
             👉🏻 375  45   1 131 -10 -18 0.560

=======================================
           Central division            
=======================================
                 GP +/- L10  RW  GD L10
 1. Colorado     45  29   5  30  79  18
 2. Dallas       47  16  -2  23  28  -8
 3. Minnesota    47  14   2  18  18  -1
 4. Utah         47   3   3  17  14  10
 5. Nashville    46   2   2  15 -22  -4
 6. Chicago      46  -1   3  15 -16  -1
 7. St. Louis    47  -3  -2  18 -44 -11
 8. Winnipeg     45  -4  -2  16  -7  -3
             👉🏻 370  56   9 152  50   0 0.576

=======================================
           Pacific division            
=======================================
                 GP +/- L10  RW  GD L10
 1. Vegas        45  11   2  16   9   3
 2. Edmonton     47   7   2  18   2   4
 3. Seattle      45   6   5  15  -6  12
 4. San Jose     46   5   4  13 -18  -3
 5. Los Angeles  46   3   0  12  -9  -4
 6. Anaheim      46   1  -7  14 -16 -21
 7. Calgary      46  -4  -2  15 -21 -10
 8. Vancouver    47 -10  -6  10 -44 -22
             👉🏻 368  19  -2 113 -103 -41 0.526
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
 1. Tampa Bay    45  16  10  23  44  25
 2. Carolina     47  13   1  19  16  -2
 3. Detroit      48  12   3  20   1   3
 4. Montréal     48  11   3  17   3   5
 5. Buffalo      46  10   6  20  10  15
 6. NY Islanders 46   9   3  16   8   6
 7. Pittsburgh   46   8   5  18   7  16
 8. Toronto      46   7   6  17   4  10
 9. Boston       47   7   3  19   9   9
10. Florida      45   6   1  21  -6  -6
11. Philadelphia 46   6  -3  15  -5 -14
12. Washington   48   6  -1  21  17   2
13. Ottawa       46   3  -2  17  -2  -8
14. New Jersey   47   3  -1  16 -19 -12
15. Columbus     47   2   1  13 -13   2
16. NY Rangers   48  -2  -4  13 -21 -16
             👉🏻 746 117  31 285  53  35 0.578

=======================================
          Western conference           
=======================================
                 GP +/- L10  RW  GD L10
 1. Colorado     45  29   5  30  79  18
 2. Dallas       47  16  -2  23  28  -8
 3. Minnesota    47  14   2  18  18  -1
 4. Vegas        45  11   2  16   9   3
 5. Edmonton     47   7   2  18   2   4
 6. Seattle      45   6   5  15  -6  12
 7. San Jose     46   5   4  13 -18  -3
 8. Los Angeles  46   3   0  12  -9  -4
 9. Utah         47   3   3  17  14  10
10. Nashville    46   2   2  15 -22  -4
11. Anaheim      46   1  -7  14 -16 -21
12. Chicago      46  -1   3  15 -16  -1
13. St. Louis    47  -3  -2  18 -44 -11
14. Winnipeg     45  -4  -2  16  -7  -3
15. Calgary      46  -4  -2  15 -21 -10
16. Vancouver    47 -10  -6  10 -44 -22
             👉🏻 738  75   7 265 -53 -41 0.551
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
 1. Colorado     45  29   5  30  79  18
 2. Tampa Bay    45  16  10  23  44  25
 3. Dallas       47  16  -2  23  28  -8
 4. Minnesota    47  14   2  18  18  -1
 5. Carolina     47  13   1  19  16  -2
 6. Detroit      48  12   3  20   1   3
 7. Vegas        45  11   2  16   9   3
 8. Montréal     48  11   3  17   3   5
 9. Buffalo      46  10   6  20  10  15
10. NY Islanders 46   9   3  16   8   6
11. Pittsburgh   46   8   5  18   7  16
12. Toronto      46   7   6  17   4  10
13. Boston       47   7   3  19   9   9
14. Edmonton     47   7   2  18   2   4
15. Florida      45   6   1  21  -6  -6
16. Seattle      45   6   5  15  -6  12
17. Philadelphia 46   6  -3  15  -5 -14
18. Washington   48   6  -1  21  17   2
19. San Jose     46   5   4  13 -18  -3
20. Ottawa       46   3  -2  17  -2  -8
21. Los Angeles  46   3   0  12  -9  -4
22. Utah         47   3   3  17  14  10
23. New Jersey   47   3  -1  16 -19 -12
24. Nashville    46   2   2  15 -22  -4
25. Columbus     47   2   1  13 -13   2
26. Anaheim      46   1  -7  14 -16 -21
27. Chicago      46  -1   3  15 -16  -1
28. NY Rangers   48  -2  -4  13 -21 -16
29. St. Louis    47  -3  -2  18 -44 -11
30. Winnipeg     45  -4  -2  16  -7  -3
31. Calgary      46  -4  -2  15 -21 -10
32. Vancouver    47 -10  -6  10 -44 -22
             👉🏻1484 192  38 550   0  -6 0.565
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
 1. Tampa Bay    45  16  10  23  44  25
 2. Buffalo      46  10   6  20  10  15
 3. Toronto      46   7   6  17   4  10
 4. Colorado     45  29   5  30  79  18
 5. Pittsburgh   46   8   5  18   7  16
 6. Seattle      45   6   5  15  -6  12
 7. San Jose     46   5   4  13 -18  -3
 8. Detroit      48  12   3  20   1   3
 9. Montréal     48  11   3  17   3   5
10. NY Islanders 46   9   3  16   8   6
11. Boston       47   7   3  19   9   9
12. Utah         47   3   3  17  14  10
13. Chicago      46  -1   3  15 -16  -1
14. Minnesota    47  14   2  18  18  -1
15. Vegas        45  11   2  16   9   3
16. Edmonton     47   7   2  18   2   4
17. Nashville    46   2   2  15 -22  -4
18. Carolina     47  13   1  19  16  -2
19. Florida      45   6   1  21  -6  -6
20. Columbus     47   2   1  13 -13   2
21. Los Angeles  46   3   0  12  -9  -4
22. Washington   48   6  -1  21  17   2
23. New Jersey   47   3  -1  16 -19 -12
24. Dallas       47  16  -2  23  28  -8
25. Ottawa       46   3  -2  17  -2  -8
26. St. Louis    47  -3  -2  18 -44 -11
27. Winnipeg     45  -4  -2  16  -7  -3
28. Calgary      46  -4  -2  15 -21 -10
29. Philadelphia 46   6  -3  15  -5 -14
30. NY Rangers   48  -2  -4  13 -21 -16
31. Vancouver    47 -10  -6  10 -44 -22
32. Anaheim      46   1  -7  14 -16 -21
             👉🏻1484 192  38 550   0  -6 0.565
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

[8-w] Toronto (7) at Tampa Bay (16)
[7-w] Buffalo (10) at Carolina (13)
[3-2] Montréal (11) at Detroit (12)
[3-2] Pittsburgh (8) at NY Islanders (9)

Outside looking-in: BOS (7) FLA (6) PHI (6) WSH (6) 

Western Conference:

[8-w] Los Angeles (3) at Colorado (29)
[3-2] Minnesota (14) at Dallas (16)
[7-w] San Jose (5) at Vegas (11)
[3-2] Seattle (6) at Edmonton (7)

Outside looking-in: UTA (3) NSH (2) ANA (1) 
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

              2026-01-15 (THU)               
=============================================
MTL 3 - 5 BUF  19:00   (TSN2, RDS, MSG-B)
PHI 3 - 6 PIT  19:00   (ESPN, SN)
SJS 3 - 2 WSH  19:00   (NBCSCA, MNMT)
VAN 1 - 4 CBJ  19:00   (SNP, FDSNOH)
SEA 2 - 3 BOS  20:00   (KHN/Prime, KING 5, NESN, KONG)
WPG 6 - 1 MIN  20:00   (TSN3, FDSNNO, FDSNWIX)
CGY 2 - 1 CHI  20:30   (SNW, CHSN)
DAL 0 - 0 UTA  21:00   (Utah16, Victory+)
NYI 0 - 0 EDM  21:00   (SN1, TVAS, MSGSN)
TOR 3 - 1 VGK  21:30   (ESPN, TSN4)

              2026-01-16 (FRI)               
=============================================
SJS at DET  19:00   (SN, NHLN, FDSNDET, NBCSCA)
FLA at CAR  19:00   (SNO, TVAS, FDSNSO, SCRIPPS)
TBL at STL  20:00   (FDSNMW, The Spot)
NSH at COL  21:00   (SNE (JIP), SN360, TVAS, FDSNSO, ALT, KTVD)
ANA at LAK  22:30   (SNE (JIP), SN, FDSNW, Victory+, KCOP-13)

              2026-01-17 (SAT)               
=============================================
MIN at BUF  12:30   (NHLN, FDSNNO, MSG-B, FDSNWI)
NYR at PHI  13:00   (NBCSP, MSG)
NYI at CGY  15:00   (CBC, SN, TVAS, MSGSN)
SEA at UTA  17:00   (KHN/Prime, Utah16, KONG)
MTL at OTT  19:00   (CITY, SNE, TVAS)
CAR at NJD  19:00   (FDSNSO, MSGSN)
CBJ at PIT  19:00   (FDSNOH, SN-PIT)
FLA at WSH  19:00   (MNMT, SCRIPPS)
TOR at WPG  19:00   (SN, CBC, NHLN)
BOS at CHI  20:00   (CHSN, NESN)
EDM at VAN  22:00   (CBC, CITY, SN, TVAS)
NSH at VGK  22:00   (FDSNSO, SCRIPPS)
LAK at ANA  22:00   (FDSNW, Victory+)

              2026-01-18 (SUN)               
=============================================
TBL at DAL  14:00   (NHLN, SN, TVAS, The Spot, Victory+)
OTT at DET  17:00   (RDS2, TSN5, FDSNDET)
STL at EDM  20:00   (SN, TVAS, FDSNMW)

              2026-01-19 (MON)               
=============================================
BUF at CAR  13:30   (TNT, truTV, HBO MAX)
WSH at COL  16:00   (TNT, truTV, HBO MAX, MNMT)
PIT at SEA  17:00   (TVAS, KHN/Prime, SN-PIT, KONG)
SJS at FLA  18:00   (NBCSCA, SCRIPPS)
MIN at TOR  19:30   (Prime, FDSNNO, FDSNWI)
PHI at VGK  20:00   (NBCSP+, SCRIPPS)
WPG at CHI  20:30   (TVAS, TSN3, CHSN)
NJD at CGY  21:00   (SNW, MSGSN)
NYI at VAN  22:00   (SNP, MSGSN2)
NYR at ANA  22:00   (Victory+, MSG, KCOP-13)

              2026-01-20 (TUE)               
=============================================
MIN at MTL  19:00   (TSN2, RDS, FDSNNO, FDSNWI)
SJS at TBL  19:00   (SN, The Spot, NBCSCA)
OTT at CBJ  19:00   (RDS2, TSN5, FDSNOH)
BOS at DAL  19:30   (TNT, HBO MAX, SNE, NESN)
BUF at NSH  20:00   (FDSNSO, MSG-B)
STL at WPG  20:00   (TSN3, FDSNMW)
NJD at EDM  22:00   (TNT, truTV, HBO MAX, SNW, TVAS)
NYR at LAK  22:00   (SN, FDSNW, MSG)

              2026-01-21 (WED)               
=============================================
DET at TOR  19:00   (TNT, truTV, HBO MAX, SN, TVAS)
ANA at COL  21:00   (Victory+, ALT, KCOP-13)
PHI at UTA  21:00   (Utah16, NBCSP)
PIT at CGY  21:30   (SNW, SN1, TVAS, SN-PIT)
NYI at SEA  21:30   (TNT, truTV, HBO MAX)
WSH at VAN  22:00   (SNP, MNMT)
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
