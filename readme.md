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
 1. Tampa Bay    44  15  10  23  43  26
 2. Detroit      47  13   5  20   4   7
 3. Montréal     46  12   5  17   6  11
 4. Buffalo      44   8   6  18   5  13
 5. Toronto      45   8   8  17   9  18
 6. Florida      45   6   1  21  -6  -6
 7. Boston       46   6   1  18   6   2
 8. Ottawa       44   1  -3  15  -7 -10
             👉🏻 361  69  33 149  60  61 0.596

=======================================
         Metropolitan division         
=======================================
                 GP +/- L10  RW  GD L10
 1. Carolina     46  14   3  19  19   4
 2. NY Islanders 45  10   4  16   9   6
 3. Philadelphia 44   8   1  15   1  -3
 4. Pittsburgh   44   7   4  17   5  12
 5. Washington   46   6  -1  21  17  -1
 6. New Jersey   46   2  -3  16 -20 -14
 7. Columbus     45   0   1  11 -18   1
 8. NY Rangers   47  -1  -4  13 -17 -13
             👉🏻 363  46   5 128  -4  -8 0.563

=======================================
           Central division            
=======================================
                 GP +/- L10  RW  GD L10
 1. Colorado     45  29   5  30  79  18
 2. Dallas       46  17   0  23  30  -2
 3. Minnesota    47  14   2  18  18  -1
 4. Utah         46   2   1  16   9   4
 5. Nashville    45   1   2  15 -23  -4
 6. Chicago      46  -1   3  15 -16  -1
 7. St. Louis    46  -4  -2  17 -47 -10
 8. Winnipeg     44  -5  -3  15  -8  -5
             👉🏻 365  53   8 149  42  -1 0.573

=======================================
           Pacific division            
=======================================
                 GP +/- L10  RW  GD L10
 1. Vegas        44  10   2  16   8   7
 2. Edmonton     46   7   3  18   3   6
 3. Seattle      44   6   6  15  -5  14
 4. San Jose     45   4   2  12 -19  -6
 5. Los Angeles  45   3  -1  12  -8  -4
 6. Anaheim      45   0  -7  13 -18 -22
 7. Calgary      45  -3   0  15 -19  -5
 8. Vancouver    45  -8  -6  10 -40 -24
             👉🏻 359  19  -1 111 -98 -34 0.526

=======================================
          Eastern conference           
=======================================
                 GP +/- L10  RW  GD L10
 1. Tampa Bay    44  15  10  23  43  26
 2. Carolina     46  14   3  19  19   4
 3. Detroit      47  13   5  20   4   7
 4. Montréal     46  12   5  17   6  11
 5. NY Islanders 45  10   4  16   9   6
 6. Buffalo      44   8   6  18   5  13
 7. Philadelphia 44   8   1  15   1  -3
 8. Toronto      45   8   8  17   9  18
 9. Pittsburgh   44   7   4  17   5  12
10. Florida      45   6   1  21  -6  -6
11. Washington   46   6  -1  21  17  -1
12. Boston       46   6   1  18   6   2
13. New Jersey   46   2  -3  16 -20 -14
14. Ottawa       44   1  -3  15  -7 -10
15. Columbus     45   0   1  11 -18   1
16. NY Rangers   47  -1  -4  13 -17 -13
             👉🏻 724 115  38 277  56  53 0.579

=======================================
          Western conference           
=======================================
                 GP +/- L10  RW  GD L10
 1. Colorado     45  29   5  30  79  18
 2. Dallas       46  17   0  23  30  -2
 3. Minnesota    47  14   2  18  18  -1
 4. Vegas        44  10   2  16   8   7
 5. Edmonton     46   7   3  18   3   6
 6. Seattle      44   6   6  15  -5  14
 7. San Jose     45   4   2  12 -19  -6
 8. Los Angeles  45   3  -1  12  -8  -4
 9. Utah         46   2   1  16   9   4
10. Nashville    45   1   2  15 -23  -4
11. Anaheim      45   0  -7  13 -18 -22
12. Chicago      46  -1   3  15 -16  -1
13. Calgary      45  -3   0  15 -19  -5
14. St. Louis    46  -4  -2  17 -47 -10
15. Winnipeg     44  -5  -3  15  -8  -5
16. Vancouver    45  -8  -6  10 -40 -24
             👉🏻 724  72   7 260 -56 -35 0.550

=======================================
              Full league              
=======================================
                 GP +/- L10  RW  GD L10
 1. Colorado     45  29   5  30  79  18
 2. Dallas       46  17   0  23  30  -2
 3. Tampa Bay    44  15  10  23  43  26
 4. Carolina     46  14   3  19  19   4
 5. Minnesota    47  14   2  18  18  -1
 6. Detroit      47  13   5  20   4   7
 7. Montréal     46  12   5  17   6  11
 8. Vegas        44  10   2  16   8   7
 9. NY Islanders 45  10   4  16   9   6
10. Buffalo      44   8   6  18   5  13
11. Philadelphia 44   8   1  15   1  -3
12. Toronto      45   8   8  17   9  18
13. Pittsburgh   44   7   4  17   5  12
14. Edmonton     46   7   3  18   3   6
15. Seattle      44   6   6  15  -5  14
16. Florida      45   6   1  21  -6  -6
17. Washington   46   6  -1  21  17  -1
18. Boston       46   6   1  18   6   2
19. San Jose     45   4   2  12 -19  -6
20. Los Angeles  45   3  -1  12  -8  -4
21. Utah         46   2   1  16   9   4
22. New Jersey   46   2  -3  16 -20 -14
23. Ottawa       44   1  -3  15  -7 -10
24. Nashville    45   1   2  15 -23  -4
25. Anaheim      45   0  -7  13 -18 -22
26. Columbus     45   0   1  11 -18   1
27. Chicago      46  -1   3  15 -16  -1
28. NY Rangers   47  -1  -4  13 -17 -13
29. Calgary      45  -3   0  15 -19  -5
30. St. Louis    46  -4  -2  17 -47 -10
31. Winnipeg     44  -5  -3  15  -8  -5
32. Vancouver    45  -8  -6  10 -40 -24
             👉🏻1448 187  45 537   0  18 0.565

=======================================
      Full league (last 10 games)      
=======================================
                 GP +/- L10  RW  GD L10
 1. Tampa Bay    44  15  10  23  43  26
 2. Toronto      45   8   8  17   9  18
 3. Buffalo      44   8   6  18   5  13
 4. Seattle      44   6   6  15  -5  14
 5. Colorado     45  29   5  30  79  18
 6. Detroit      47  13   5  20   4   7
 7. Montréal     46  12   5  17   6  11
 8. NY Islanders 45  10   4  16   9   6
 9. Pittsburgh   44   7   4  17   5  12
10. Carolina     46  14   3  19  19   4
11. Edmonton     46   7   3  18   3   6
12. Chicago      46  -1   3  15 -16  -1
13. Minnesota    47  14   2  18  18  -1
14. Vegas        44  10   2  16   8   7
15. San Jose     45   4   2  12 -19  -6
16. Nashville    45   1   2  15 -23  -4
17. Philadelphia 44   8   1  15   1  -3
18. Florida      45   6   1  21  -6  -6
19. Boston       46   6   1  18   6   2
20. Utah         46   2   1  16   9   4
21. Columbus     45   0   1  11 -18   1
22. Dallas       46  17   0  23  30  -2
23. Calgary      45  -3   0  15 -19  -5
24. Washington   46   6  -1  21  17  -1
25. Los Angeles  45   3  -1  12  -8  -4
26. St. Louis    46  -4  -2  17 -47 -10
27. New Jersey   46   2  -3  16 -20 -14
28. Ottawa       44   1  -3  15  -7 -10
29. Winnipeg     44  -5  -3  15  -8  -5
30. NY Rangers   47  -1  -4  13 -17 -13
31. Vancouver    45  -8  -6  10 -40 -24
32. Anaheim      45   0  -7  13 -18 -22
             👉🏻1448 187  45 537   0  18 0.565
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
 1. Tampa Bay    44  15  10  23  43  26
 2. Detroit      47  13   5  20   4   7
 3. Montréal     46  12   5  17   6  11
 4. Buffalo      44   8   6  18   5  13
 5. Toronto      45   8   8  17   9  18
 6. Florida      45   6   1  21  -6  -6
 7. Boston       46   6   1  18   6   2
 8. Ottawa       44   1  -3  15  -7 -10
             👉🏻 361  69  33 149  60  61 0.596

=======================================
         Metropolitan division         
=======================================
                 GP +/- L10  RW  GD L10
 1. Carolina     46  14   3  19  19   4
 2. NY Islanders 45  10   4  16   9   6
 3. Philadelphia 44   8   1  15   1  -3
 4. Pittsburgh   44   7   4  17   5  12
 5. Washington   46   6  -1  21  17  -1
 6. New Jersey   46   2  -3  16 -20 -14
 7. Columbus     45   0   1  11 -18   1
 8. NY Rangers   47  -1  -4  13 -17 -13
             👉🏻 363  46   5 128  -4  -8 0.563

=======================================
           Central division            
=======================================
                 GP +/- L10  RW  GD L10
 1. Colorado     45  29   5  30  79  18
 2. Dallas       46  17   0  23  30  -2
 3. Minnesota    47  14   2  18  18  -1
 4. Utah         46   2   1  16   9   4
 5. Nashville    45   1   2  15 -23  -4
 6. Chicago      46  -1   3  15 -16  -1
 7. St. Louis    46  -4  -2  17 -47 -10
 8. Winnipeg     44  -5  -3  15  -8  -5
             👉🏻 365  53   8 149  42  -1 0.573

=======================================
           Pacific division            
=======================================
                 GP +/- L10  RW  GD L10
 1. Vegas        44  10   2  16   8   7
 2. Edmonton     46   7   3  18   3   6
 3. Seattle      44   6   6  15  -5  14
 4. San Jose     45   4   2  12 -19  -6
 5. Los Angeles  45   3  -1  12  -8  -4
 6. Anaheim      45   0  -7  13 -18 -22
 7. Calgary      45  -3   0  15 -19  -5
 8. Vancouver    45  -8  -6  10 -40 -24
             👉🏻 359  19  -1 111 -98 -34 0.526
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
 1. Tampa Bay    44  15  10  23  43  26
 2. Carolina     46  14   3  19  19   4
 3. Detroit      47  13   5  20   4   7
 4. Montréal     46  12   5  17   6  11
 5. NY Islanders 45  10   4  16   9   6
 6. Buffalo      44   8   6  18   5  13
 7. Philadelphia 44   8   1  15   1  -3
 8. Toronto      45   8   8  17   9  18
 9. Pittsburgh   44   7   4  17   5  12
10. Florida      45   6   1  21  -6  -6
11. Washington   46   6  -1  21  17  -1
12. Boston       46   6   1  18   6   2
13. New Jersey   46   2  -3  16 -20 -14
14. Ottawa       44   1  -3  15  -7 -10
15. Columbus     45   0   1  11 -18   1
16. NY Rangers   47  -1  -4  13 -17 -13
             👉🏻 724 115  38 277  56  53 0.579

=======================================
          Western conference           
=======================================
                 GP +/- L10  RW  GD L10
 1. Colorado     45  29   5  30  79  18
 2. Dallas       46  17   0  23  30  -2
 3. Minnesota    47  14   2  18  18  -1
 4. Vegas        44  10   2  16   8   7
 5. Edmonton     46   7   3  18   3   6
 6. Seattle      44   6   6  15  -5  14
 7. San Jose     45   4   2  12 -19  -6
 8. Los Angeles  45   3  -1  12  -8  -4
 9. Utah         46   2   1  16   9   4
10. Nashville    45   1   2  15 -23  -4
11. Anaheim      45   0  -7  13 -18 -22
12. Chicago      46  -1   3  15 -16  -1
13. Calgary      45  -3   0  15 -19  -5
14. St. Louis    46  -4  -2  17 -47 -10
15. Winnipeg     44  -5  -3  15  -8  -5
16. Vancouver    45  -8  -6  10 -40 -24
             👉🏻 724  72   7 260 -56 -35 0.550
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
 2. Dallas       46  17   0  23  30  -2
 3. Tampa Bay    44  15  10  23  43  26
 4. Carolina     46  14   3  19  19   4
 5. Minnesota    47  14   2  18  18  -1
 6. Detroit      47  13   5  20   4   7
 7. Montréal     46  12   5  17   6  11
 8. Vegas        44  10   2  16   8   7
 9. NY Islanders 45  10   4  16   9   6
10. Buffalo      44   8   6  18   5  13
11. Philadelphia 44   8   1  15   1  -3
12. Toronto      45   8   8  17   9  18
13. Pittsburgh   44   7   4  17   5  12
14. Edmonton     46   7   3  18   3   6
15. Seattle      44   6   6  15  -5  14
16. Florida      45   6   1  21  -6  -6
17. Washington   46   6  -1  21  17  -1
18. Boston       46   6   1  18   6   2
19. San Jose     45   4   2  12 -19  -6
20. Los Angeles  45   3  -1  12  -8  -4
21. Utah         46   2   1  16   9   4
22. New Jersey   46   2  -3  16 -20 -14
23. Ottawa       44   1  -3  15  -7 -10
24. Nashville    45   1   2  15 -23  -4
25. Anaheim      45   0  -7  13 -18 -22
26. Columbus     45   0   1  11 -18   1
27. Chicago      46  -1   3  15 -16  -1
28. NY Rangers   47  -1  -4  13 -17 -13
29. Calgary      45  -3   0  15 -19  -5
30. St. Louis    46  -4  -2  17 -47 -10
31. Winnipeg     44  -5  -3  15  -8  -5
32. Vancouver    45  -8  -6  10 -40 -24
             👉🏻1448 187  45 537   0  18 0.565
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
 1. Tampa Bay    44  15  10  23  43  26
 2. Toronto      45   8   8  17   9  18
 3. Buffalo      44   8   6  18   5  13
 4. Seattle      44   6   6  15  -5  14
 5. Colorado     45  29   5  30  79  18
 6. Detroit      47  13   5  20   4   7
 7. Montréal     46  12   5  17   6  11
 8. NY Islanders 45  10   4  16   9   6
 9. Pittsburgh   44   7   4  17   5  12
10. Carolina     46  14   3  19  19   4
11. Edmonton     46   7   3  18   3   6
12. Chicago      46  -1   3  15 -16  -1
13. Minnesota    47  14   2  18  18  -1
14. Vegas        44  10   2  16   8   7
15. San Jose     45   4   2  12 -19  -6
16. Nashville    45   1   2  15 -23  -4
17. Philadelphia 44   8   1  15   1  -3
18. Florida      45   6   1  21  -6  -6
19. Boston       46   6   1  18   6   2
20. Utah         46   2   1  16   9   4
21. Columbus     45   0   1  11 -18   1
22. Dallas       46  17   0  23  30  -2
23. Calgary      45  -3   0  15 -19  -5
24. Washington   46   6  -1  21  17  -1
25. Los Angeles  45   3  -1  12  -8  -4
26. St. Louis    46  -4  -2  17 -47 -10
27. New Jersey   46   2  -3  16 -20 -14
28. Ottawa       44   1  -3  15  -7 -10
29. Winnipeg     44  -5  -3  15  -8  -5
30. NY Rangers   47  -1  -4  13 -17 -13
31. Vancouver    45  -8  -6  10 -40 -24
32. Anaheim      45   0  -7  13 -18 -22
             👉🏻1448 187  45 537   0  18 0.565
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

[8-w] Toronto (8) at Tampa Bay (15)
[7-w] Buffalo (8) at Carolina (14)
[3-2] Montréal (12) at Detroit (13)
[3-2] Philadelphia (8) at NY Islanders (10)

Outside looking-in: PIT (7) FLA (6) WSH (6) BOS (6) 

Western Conference:

[8-w] Los Angeles (3) at Colorado (29)
[3-2] Minnesota (14) at Dallas (17)
[7-w] San Jose (4) at Vegas (10)
[3-2] Seattle (6) at Edmonton (7)

Outside looking-in: UTA (2) NSH (1) ANA (0) 
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

              2026-01-13 (TUE)               
=============================================
VAN at OTT  19:00   (SNP, RDS2, TSN5)
TBL at PIT  19:00   (The Spot, SN-PIT)
MTL at WSH  19:00   (TSN2, RDS, MNMT)
CGY at CBJ  19:00   (SNW, FDSNOH)
DET at BOS  19:30   (TNT, HBO MAX, NESN)
CAR at STL  19:30   (ESPN+, HULU)
EDM at NSH  20:00   (SNO, SN1, FDSNSO)
NYI at WPG  20:00   (TSN3, MSGSN)
TOR at UTA  22:00   (ESPN+, HULU, TVAS, TSN4)
DAL at ANA  22:00   (TNT, truTV, HBO MAX, SN)

              2026-01-14 (WED)               
=============================================
SEA at NJD  19:00   (KHN/Prime, MSGSN, KONG)
PHI at BUF  19:30   (TNT, truTV, HBO MAX, TVAS)
OTT at NYR  19:30   (SN, RDS2, MSG)
VGK at LAK  22:00   (TNT, truTV, HBO MAX, SN360, SN (JIP), TVAS)

              2026-01-15 (THU)               
=============================================
MTL at BUF  19:00   (TSN2, RDS, MSG-B)
PHI at PIT  19:00   (ESPN, SN)
SJS at WSH  19:00   (NBCSCA, MNMT)
VAN at CBJ  19:00   (SNP, FDSNOH)
SEA at BOS  20:00   (KHN/Prime, KING 5, NESN, KONG)
WPG at MIN  20:00   (TSN3, FDSNNO, FDSNWIX)
CGY at CHI  20:30   (SNW, CHSN)
DAL at UTA  21:00   (Utah16, Victory+)
NYI at EDM  21:00   (SN1, TVAS, MSGSN)
TOR at VGK  21:30   (ESPN, TSN4)

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
TOR at WPG  19:00   (SN, CBC)
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
