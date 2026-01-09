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
 1. Tampa Bay    42  13   6  21  34  13
 2. Montréal     43  11   6  15   3  14
 3. Detroit      44  10   3  18  -5  -1
 4. Buffalo      41   7   8  16   1  14
 5. Florida      42   5   1  19  -4  -6
 6. Toronto      42   5   2  16   2   1
 7. Ottawa       42   3   1  15   0   3
 8. Boston       43   3  -2  15  -6 -10
             👉🏻 339  57  25 135  25  28 0.584

=======================================
         Metropolitan division         
=======================================
                 GP +/- L10  RW  GD L10
 1. Carolina     43  12  -1  17  16  -5
 2. Philadelphia 41  10   3  15  11   7
 3. NY Islanders 43   9   1  16   9   2
 4. Pittsburgh   41   8   2  16   4   1
 5. Washington   44   6  -2  20  14  -8
 6. New Jersey   43   3  -1  15 -19 -13
 7. NY Rangers   44   2   0  13  -4  -1
 8. Columbus     42   1   1  11 -13   3
             👉🏻 341  51   3 123  18 -14 0.575

=======================================
           Central division            
=======================================
                 GP +/- L10  RW  GD L10
 1. Colorado     42  27   6  28  70  16
 2. Dallas       44  16   1  22  29   4
 3. Minnesota    44  14   3  18  21   4
 4. Utah         44   1   0  15   8   3
 5. Nashville    42   0   2  14 -22  -3
 6. Chicago      43   0   1  14 -12  -4
 7. St. Louis    44  -2   1  17 -43  -6
 8. Winnipeg     41  -6  -6  13 -12 -12
             👉🏻 344  50   8 141  39   2 0.573

=======================================
           Pacific division            
=======================================
                 GP +/- L10  RW  GD L10
 1. Vegas        41   7  -3  13  -1  -7
 2. Seattle      41   6   7  14  -5  16
 3. Edmonton     43   5   2  16   0   4
 4. Los Angeles  42   4  -1  12  -3  -1
 5. San Jose     43   4   2  12 -15  -1
 6. Anaheim      43   2  -6  13 -13 -23
 7. Calgary      43  -3   0  14 -17  -4
 8. Vancouver    42  -5   0  10 -28  -5
             👉🏻 338  20   1 104 -82 -21 0.530

=======================================
          Eastern conference           
=======================================
                 GP +/- L10  RW  GD L10
 1. Tampa Bay    42  13   6  21  34  13
 2. Carolina     43  12  -1  17  16  -5
 3. Montréal     43  11   6  15   3  14
 4. Philadelphia 41  10   3  15  11   7
 5. Detroit      44  10   3  18  -5  -1
 6. NY Islanders 43   9   1  16   9   2
 7. Pittsburgh   41   8   2  16   4   1
 8. Buffalo      41   7   8  16   1  14
 9. Washington   44   6  -2  20  14  -8
10. Florida      42   5   1  19  -4  -6
11. Toronto      42   5   2  16   2   1
12. Ottawa       42   3   1  15   0   3
13. Boston       43   3  -2  15  -6 -10
14. New Jersey   43   3  -1  15 -19 -13
15. NY Rangers   44   2   0  13  -4  -1
16. Columbus     42   1   1  11 -13   3
             👉🏻 680 108  28 258  43  14 0.579

=======================================
          Western conference           
=======================================
                 GP +/- L10  RW  GD L10
 1. Colorado     42  27   6  28  70  16
 2. Dallas       44  16   1  22  29   4
 3. Minnesota    44  14   3  18  21   4
 4. Vegas        41   7  -3  13  -1  -7
 5. Seattle      41   6   7  14  -5  16
 6. Edmonton     43   5   2  16   0   4
 7. Los Angeles  42   4  -1  12  -3  -1
 8. San Jose     43   4   2  12 -15  -1
 9. Anaheim      43   2  -6  13 -13 -23
10. Utah         44   1   0  15   8   3
11. Nashville    42   0   2  14 -22  -3
12. Chicago      43   0   1  14 -12  -4
13. St. Louis    44  -2   1  17 -43  -6
14. Calgary      43  -3   0  14 -17  -4
15. Vancouver    42  -5   0  10 -28  -5
16. Winnipeg     41  -6  -6  13 -12 -12
             👉🏻 682  70   9 245 -43 -19 0.551

=======================================
              Full league              
=======================================
                 GP +/- L10  RW  GD L10
 1. Colorado     42  27   6  28  70  16
 2. Dallas       44  16   1  22  29   4
 3. Minnesota    44  14   3  18  21   4
 4. Tampa Bay    42  13   6  21  34  13
 5. Carolina     43  12  -1  17  16  -5
 6. Montréal     43  11   6  15   3  14
 7. Philadelphia 41  10   3  15  11   7
 8. Detroit      44  10   3  18  -5  -1
 9. NY Islanders 43   9   1  16   9   2
10. Pittsburgh   41   8   2  16   4   1
11. Buffalo      41   7   8  16   1  14
12. Vegas        41   7  -3  13  -1  -7
13. Seattle      41   6   7  14  -5  16
14. Washington   44   6  -2  20  14  -8
15. Florida      42   5   1  19  -4  -6
16. Toronto      42   5   2  16   2   1
17. Edmonton     43   5   2  16   0   4
18. Los Angeles  42   4  -1  12  -3  -1
19. San Jose     43   4   2  12 -15  -1
20. Ottawa       42   3   1  15   0   3
21. Boston       43   3  -2  15  -6 -10
22. New Jersey   43   3  -1  15 -19 -13
23. Anaheim      43   2  -6  13 -13 -23
24. NY Rangers   44   2   0  13  -4  -1
25. Columbus     42   1   1  11 -13   3
26. Utah         44   1   0  15   8   3
27. Nashville    42   0   2  14 -22  -3
28. Chicago      43   0   1  14 -12  -4
29. St. Louis    44  -2   1  17 -43  -6
30. Calgary      43  -3   0  14 -17  -4
31. Vancouver    42  -5   0  10 -28  -5
32. Winnipeg     41  -6  -6  13 -12 -12
             👉🏻1362 178  37 503   0  -5 0.565

=======================================
      Full league (last 10 games)      
=======================================
                 GP +/- L10  RW  GD L10
 1. Buffalo      41   7   8  16   1  14
 2. Seattle      41   6   7  14  -5  16
 3. Colorado     42  27   6  28  70  16
 4. Tampa Bay    42  13   6  21  34  13
 5. Montréal     43  11   6  15   3  14
 6. Minnesota    44  14   3  18  21   4
 7. Philadelphia 41  10   3  15  11   7
 8. Detroit      44  10   3  18  -5  -1
 9. Pittsburgh   41   8   2  16   4   1
10. Toronto      42   5   2  16   2   1
11. Edmonton     43   5   2  16   0   4
12. San Jose     43   4   2  12 -15  -1
13. Nashville    42   0   2  14 -22  -3
14. Dallas       44  16   1  22  29   4
15. NY Islanders 43   9   1  16   9   2
16. Florida      42   5   1  19  -4  -6
17. Ottawa       42   3   1  15   0   3
18. Columbus     42   1   1  11 -13   3
19. Chicago      43   0   1  14 -12  -4
20. St. Louis    44  -2   1  17 -43  -6
21. NY Rangers   44   2   0  13  -4  -1
22. Utah         44   1   0  15   8   3
23. Calgary      43  -3   0  14 -17  -4
24. Vancouver    42  -5   0  10 -28  -5
25. Carolina     43  12  -1  17  16  -5
26. Los Angeles  42   4  -1  12  -3  -1
27. New Jersey   43   3  -1  15 -19 -13
28. Washington   44   6  -2  20  14  -8
29. Boston       43   3  -2  15  -6 -10
30. Vegas        41   7  -3  13  -1  -7
31. Anaheim      43   2  -6  13 -13 -23
32. Winnipeg     41  -6  -6  13 -12 -12
             👉🏻1362 178  37 503   0  -5 0.565
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
 1. Tampa Bay    42  13   6  21  34  13
 2. Montréal     43  11   6  15   3  14
 3. Detroit      44  10   3  18  -5  -1
 4. Buffalo      41   7   8  16   1  14
 5. Florida      42   5   1  19  -4  -6
 6. Toronto      42   5   2  16   2   1
 7. Ottawa       42   3   1  15   0   3
 8. Boston       43   3  -2  15  -6 -10
             👉🏻 339  57  25 135  25  28 0.584

=======================================
         Metropolitan division         
=======================================
                 GP +/- L10  RW  GD L10
 1. Carolina     43  12  -1  17  16  -5
 2. Philadelphia 41  10   3  15  11   7
 3. NY Islanders 43   9   1  16   9   2
 4. Pittsburgh   41   8   2  16   4   1
 5. Washington   44   6  -2  20  14  -8
 6. New Jersey   43   3  -1  15 -19 -13
 7. NY Rangers   44   2   0  13  -4  -1
 8. Columbus     42   1   1  11 -13   3
             👉🏻 341  51   3 123  18 -14 0.575

=======================================
           Central division            
=======================================
                 GP +/- L10  RW  GD L10
 1. Colorado     42  27   6  28  70  16
 2. Dallas       44  16   1  22  29   4
 3. Minnesota    44  14   3  18  21   4
 4. Utah         44   1   0  15   8   3
 5. Nashville    42   0   2  14 -22  -3
 6. Chicago      43   0   1  14 -12  -4
 7. St. Louis    44  -2   1  17 -43  -6
 8. Winnipeg     41  -6  -6  13 -12 -12
             👉🏻 344  50   8 141  39   2 0.573

=======================================
           Pacific division            
=======================================
                 GP +/- L10  RW  GD L10
 1. Vegas        41   7  -3  13  -1  -7
 2. Seattle      41   6   7  14  -5  16
 3. Edmonton     43   5   2  16   0   4
 4. Los Angeles  42   4  -1  12  -3  -1
 5. San Jose     43   4   2  12 -15  -1
 6. Anaheim      43   2  -6  13 -13 -23
 7. Calgary      43  -3   0  14 -17  -4
 8. Vancouver    42  -5   0  10 -28  -5
             👉🏻 338  20   1 104 -82 -21 0.530
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
 1. Tampa Bay    42  13   6  21  34  13
 2. Carolina     43  12  -1  17  16  -5
 3. Montréal     43  11   6  15   3  14
 4. Philadelphia 41  10   3  15  11   7
 5. Detroit      44  10   3  18  -5  -1
 6. NY Islanders 43   9   1  16   9   2
 7. Pittsburgh   41   8   2  16   4   1
 8. Buffalo      41   7   8  16   1  14
 9. Washington   44   6  -2  20  14  -8
10. Florida      42   5   1  19  -4  -6
11. Toronto      42   5   2  16   2   1
12. Ottawa       42   3   1  15   0   3
13. Boston       43   3  -2  15  -6 -10
14. New Jersey   43   3  -1  15 -19 -13
15. NY Rangers   44   2   0  13  -4  -1
16. Columbus     42   1   1  11 -13   3
             👉🏻 680 108  28 258  43  14 0.579

=======================================
          Western conference           
=======================================
                 GP +/- L10  RW  GD L10
 1. Colorado     42  27   6  28  70  16
 2. Dallas       44  16   1  22  29   4
 3. Minnesota    44  14   3  18  21   4
 4. Vegas        41   7  -3  13  -1  -7
 5. Seattle      41   6   7  14  -5  16
 6. Edmonton     43   5   2  16   0   4
 7. Los Angeles  42   4  -1  12  -3  -1
 8. San Jose     43   4   2  12 -15  -1
 9. Anaheim      43   2  -6  13 -13 -23
10. Utah         44   1   0  15   8   3
11. Nashville    42   0   2  14 -22  -3
12. Chicago      43   0   1  14 -12  -4
13. St. Louis    44  -2   1  17 -43  -6
14. Calgary      43  -3   0  14 -17  -4
15. Vancouver    42  -5   0  10 -28  -5
16. Winnipeg     41  -6  -6  13 -12 -12
             👉🏻 682  70   9 245 -43 -19 0.551
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
 1. Colorado     42  27   6  28  70  16
 2. Dallas       44  16   1  22  29   4
 3. Minnesota    44  14   3  18  21   4
 4. Tampa Bay    42  13   6  21  34  13
 5. Carolina     43  12  -1  17  16  -5
 6. Montréal     43  11   6  15   3  14
 7. Philadelphia 41  10   3  15  11   7
 8. Detroit      44  10   3  18  -5  -1
 9. NY Islanders 43   9   1  16   9   2
10. Pittsburgh   41   8   2  16   4   1
11. Buffalo      41   7   8  16   1  14
12. Vegas        41   7  -3  13  -1  -7
13. Seattle      41   6   7  14  -5  16
14. Washington   44   6  -2  20  14  -8
15. Florida      42   5   1  19  -4  -6
16. Toronto      42   5   2  16   2   1
17. Edmonton     43   5   2  16   0   4
18. Los Angeles  42   4  -1  12  -3  -1
19. San Jose     43   4   2  12 -15  -1
20. Ottawa       42   3   1  15   0   3
21. Boston       43   3  -2  15  -6 -10
22. New Jersey   43   3  -1  15 -19 -13
23. Anaheim      43   2  -6  13 -13 -23
24. NY Rangers   44   2   0  13  -4  -1
25. Columbus     42   1   1  11 -13   3
26. Utah         44   1   0  15   8   3
27. Nashville    42   0   2  14 -22  -3
28. Chicago      43   0   1  14 -12  -4
29. St. Louis    44  -2   1  17 -43  -6
30. Calgary      43  -3   0  14 -17  -4
31. Vancouver    42  -5   0  10 -28  -5
32. Winnipeg     41  -6  -6  13 -12 -12
             👉🏻1362 178  37 503   0  -5 0.565
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
 1. Buffalo      41   7   8  16   1  14
 2. Seattle      41   6   7  14  -5  16
 3. Colorado     42  27   6  28  70  16
 4. Tampa Bay    42  13   6  21  34  13
 5. Montréal     43  11   6  15   3  14
 6. Minnesota    44  14   3  18  21   4
 7. Philadelphia 41  10   3  15  11   7
 8. Detroit      44  10   3  18  -5  -1
 9. Pittsburgh   41   8   2  16   4   1
10. Toronto      42   5   2  16   2   1
11. Edmonton     43   5   2  16   0   4
12. San Jose     43   4   2  12 -15  -1
13. Nashville    42   0   2  14 -22  -3
14. Dallas       44  16   1  22  29   4
15. NY Islanders 43   9   1  16   9   2
16. Florida      42   5   1  19  -4  -6
17. Ottawa       42   3   1  15   0   3
18. Columbus     42   1   1  11 -13   3
19. Chicago      43   0   1  14 -12  -4
20. St. Louis    44  -2   1  17 -43  -6
21. NY Rangers   44   2   0  13  -4  -1
22. Utah         44   1   0  15   8   3
23. Calgary      43  -3   0  14 -17  -4
24. Vancouver    42  -5   0  10 -28  -5
25. Carolina     43  12  -1  17  16  -5
26. Los Angeles  42   4  -1  12  -3  -1
27. New Jersey   43   3  -1  15 -19 -13
28. Washington   44   6  -2  20  14  -8
29. Boston       43   3  -2  15  -6 -10
30. Vegas        41   7  -3  13  -1  -7
31. Anaheim      43   2  -6  13 -13 -23
32. Winnipeg     41  -6  -6  13 -12 -12
             👉🏻1362 178  37 503   0  -5 0.565
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

[8-w] Buffalo (7) at Tampa Bay (13)
[7-w] Pittsburgh (8) at Carolina (12)
[3-2] Detroit (10) at Montréal (11)
[3-2] NY Islanders (9) at Philadelphia (10)

Outside looking-in: WSH (6) FLA (5) TOR (5) 

Western Conference:

[8-w] San Jose (4) at Colorado (27)
[3-2] Minnesota (14) at Dallas (16)
[7-w] Los Angeles (4) at Vegas (7)
[3-2] Edmonton (5) at Seattle (6)

Outside looking-in: ANA (2) UTA (1) 
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

              2026-01-08 (THU)               
=============================================
CGY 0 - 2 BOS  19:00   (SNO, SN1, NESN)
FLA 0 - 2 MTL  19:00   (TSN2, RDS, SCRIPPS)
VAN 0 - 1 DET  19:00   (SNP, FDSNDET)
BUF 1 - 0 NYR  19:00   (MSG-B, MSG)
TOR 0 - 0 PHI  19:00   (TVAS, TSN4, NBCSP)
NJD 0 - 1 PIT  19:00   (SN-PIT, MSGSN)
ANA 1 - 0 CAR  19:00   (FDSNSO, Victory+, KCOP-13)
NYI 0 - 0 NSH  20:00   (ESPN+, HULU)
EDM 0 - 0 WPG  20:00   (SNW, TSN3)
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

              2026-01-11 (SUN)               
=============================================
NJD at WPG  14:00   (NHLN, TSN3, MSGSN)
PIT at BOS  17:00   (SNE, TVAS, SN-PIT, NESN)
WSH at NSH  19:00   (SNE (JIP), SN360, FDSNSO, MNMT)
CBJ at UTA  19:00   (FDSNOH, Utah16)
VGK at SJS  20:00   (SN, NHLN, TVAS, NBCSCA, SCRIPPS)

              2026-01-12 (MON)               
=============================================
FLA at BUF  19:00   (MSG-B, SCRIPPS)
CAR at DET  19:00   (FDSNSO, FDSNDET)
SEA at NYR  19:00   (NHLN, KHN/Prime, MSG, KONG)
TBL at PHI  19:00   (The Spot, NBCSP)
VAN at MTL  19:30   (Prime, RDS)
NJD at MIN  20:00   (FDSNNO, MSGSN, FDSNWI)
EDM at CHI  20:30   (SNW, CHSN)
TOR at COL  22:00   (SNO, TVAS, ALT, KTVD)
DAL at LAK  22:00   (FDSNW, Victory+)

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
44 Thu Jan  8 2026  FLA 0 - 2 MTL  19:00 (In progress) (TSN2, RDS, SCRIPPS)
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
