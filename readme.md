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

nhl 0.2.12
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
 1. Tampa Bay    51  19   7  26  49  17
 2. Detroit      54  15   5  22   6  11
 3. Buffalo      52  13   5  24  20  16
 4. Montréal     53  12   1  18   3   0
 5. Boston       54  11   7  23  12  15
 6. Florida      52   7   2  23  -9  -5
 7. Toronto      53   4  -2  17  -9 -12
 8. Ottawa       52   3   0  19   0   0
             👉🏻 421  84  25 172  72  42 0.600

=======================================
         Metropolitan division         
=======================================
                 GP +/- L10  RW  GD L10
 1. Carolina     52  17   6  23  30  17
 2. Pittsburgh   51  12   4  22  17  13
 3. NY Islanders 52   9   1  19   4   4
 4. Philadelphia 51   6  -4  17  -8 -19
 5. Columbus     51   4   2  15 -11  -1
 6. New Jersey   53   3   0  18 -22  -3
 7. Washington   54   3  -3  22   7  -7
 8. NY Rangers   53  -3  -5  14 -22 -19
             👉🏻 417  51   1 150  -5 -15 0.561

=======================================
           Central division            
=======================================
                 GP +/- L10  RW  GD L10
 1. Colorado     50  29   0  32  76   3
 2. Dallas       53  16   1  26  29   3
 3. Minnesota    54  16   2  19  18  -3
 4. Utah         53   7   7  21  21  15
 5. Nashville    52   1   1  17 -27  -5
 6. Chicago      53  -2  -2  16 -24 -12
 7. Winnipeg     52  -3   4  19  -8   5
 8. St. Louis    53  -6  -5  18 -53 -14
             👉🏻 420  58   8 168  32  -8 0.569

=======================================
           Pacific division            
=======================================
                 GP +/- L10  RW  GD L10
 1. Vegas        52  11   3  18   9   8
 2. Edmonton     54   8   2  21  11  10
 3. San Jose     51   6   4  16 -15   4
 4. Los Angeles  51   6   2  14  -7  -5
 5. Anaheim      53   6   4  16 -11   2
 6. Seattle      52   5  -1  18  -7  -1
 7. Calgary      52  -4  -2  17 -24 -10
 8. Vancouver    53 -14  -8  11 -55 -23
             👉🏻 418  24   4 131 -99 -15 0.529

=======================================
          Eastern conference           
=======================================
                 GP +/- L10  RW  GD L10
 1. Tampa Bay    51  19   7  26  49  17
 2. Carolina     52  17   6  23  30  17
 3. Detroit      54  15   5  22   6  11
 4. Buffalo      52  13   5  24  20  16
 5. Pittsburgh   51  12   4  22  17  13
 6. Montréal     53  12   1  18   3   0
 7. Boston       54  11   7  23  12  15
 8. NY Islanders 52   9   1  19   4   4
 9. Florida      52   7   2  23  -9  -5
10. Philadelphia 51   6  -4  17  -8 -19
11. Columbus     51   4   2  15 -11  -1
12. Toronto      53   4  -2  17  -9 -12
13. Ottawa       52   3   0  19   0   0
14. New Jersey   53   3   0  18 -22  -3
15. Washington   54   3  -3  22   7  -7
16. NY Rangers   53  -3  -5  14 -22 -19
             👉🏻 838 135  26 322  67  27 0.581

=======================================
          Western conference           
=======================================
                 GP +/- L10  RW  GD L10
 1. Colorado     50  29   0  32  76   3
 2. Dallas       53  16   1  26  29   3
 3. Minnesota    54  16   2  19  18  -3
 4. Vegas        52  11   3  18   9   8
 5. Edmonton     54   8   2  21  11  10
 6. Utah         53   7   7  21  21  15
 7. San Jose     51   6   4  16 -15   4
 8. Los Angeles  51   6   2  14  -7  -5
 9. Anaheim      53   6   4  16 -11   2
10. Seattle      52   5  -1  18  -7  -1
11. Nashville    52   1   1  17 -27  -5
12. Chicago      53  -2  -2  16 -24 -12
13. Winnipeg     52  -3   4  19  -8   5
14. Calgary      52  -4  -2  17 -24 -10
15. St. Louis    53  -6  -5  18 -53 -14
16. Vancouver    53 -14  -8  11 -55 -23
             👉🏻 838  82  12 299 -67 -23 0.549

=======================================
              Full league              
=======================================
                 GP +/- L10  RW  GD L10
 1. Colorado     50  29   0  32  76   3
 2. Tampa Bay    51  19   7  26  49  17
 3. Carolina     52  17   6  23  30  17
 4. Dallas       53  16   1  26  29   3
 5. Minnesota    54  16   2  19  18  -3
 6. Detroit      54  15   5  22   6  11
 7. Buffalo      52  13   5  24  20  16
 8. Pittsburgh   51  12   4  22  17  13
 9. Montréal     53  12   1  18   3   0
10. Vegas        52  11   3  18   9   8
11. Boston       54  11   7  23  12  15
12. NY Islanders 52   9   1  19   4   4
13. Edmonton     54   8   2  21  11  10
14. Florida      52   7   2  23  -9  -5
15. Utah         53   7   7  21  21  15
16. Philadelphia 51   6  -4  17  -8 -19
17. San Jose     51   6   4  16 -15   4
18. Los Angeles  51   6   2  14  -7  -5
19. Anaheim      53   6   4  16 -11   2
20. Seattle      52   5  -1  18  -7  -1
21. Columbus     51   4   2  15 -11  -1
22. Toronto      53   4  -2  17  -9 -12
23. Ottawa       52   3   0  19   0   0
24. New Jersey   53   3   0  18 -22  -3
25. Washington   54   3  -3  22   7  -7
26. Nashville    52   1   1  17 -27  -5
27. Chicago      53  -2  -2  16 -24 -12
28. Winnipeg     52  -3   4  19  -8   5
29. NY Rangers   53  -3  -5  14 -22 -19
30. Calgary      52  -4  -2  17 -24 -10
31. St. Louis    53  -6  -5  18 -53 -14
32. Vancouver    53 -14  -8  11 -55 -23
             👉🏻1676 217  38 621   0   4 0.565

=======================================
      Full league (last 10 games)      
=======================================
                 GP +/- L10  RW  GD L10
 1. Tampa Bay    51  19   7  26  49  17
 2. Boston       54  11   7  23  12  15
 3. Utah         53   7   7  21  21  15
 4. Carolina     52  17   6  23  30  17
 5. Detroit      54  15   5  22   6  11
 6. Buffalo      52  13   5  24  20  16
 7. Pittsburgh   51  12   4  22  17  13
 8. San Jose     51   6   4  16 -15   4
 9. Anaheim      53   6   4  16 -11   2
10. Winnipeg     52  -3   4  19  -8   5
11. Vegas        52  11   3  18   9   8
12. Minnesota    54  16   2  19  18  -3
13. Edmonton     54   8   2  21  11  10
14. Florida      52   7   2  23  -9  -5
15. Los Angeles  51   6   2  14  -7  -5
16. Columbus     51   4   2  15 -11  -1
17. Dallas       53  16   1  26  29   3
18. Montréal     53  12   1  18   3   0
19. NY Islanders 52   9   1  19   4   4
20. Nashville    52   1   1  17 -27  -5
21. Colorado     50  29   0  32  76   3
22. Ottawa       52   3   0  19   0   0
23. New Jersey   53   3   0  18 -22  -3
24. Seattle      52   5  -1  18  -7  -1
25. Toronto      53   4  -2  17  -9 -12
26. Chicago      53  -2  -2  16 -24 -12
27. Calgary      52  -4  -2  17 -24 -10
28. Washington   54   3  -3  22   7  -7
29. Philadelphia 51   6  -4  17  -8 -19
30. NY Rangers   53  -3  -5  14 -22 -19
31. St. Louis    53  -6  -5  18 -53 -14
32. Vancouver    53 -14  -8  11 -55 -23
             👉🏻1676 217  38 621   0   4 0.565
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
 1. Tampa Bay    51  19   7  26  49  17
 2. Detroit      54  15   5  22   6  11
 3. Buffalo      52  13   5  24  20  16
 4. Montréal     53  12   1  18   3   0
 5. Boston       54  11   7  23  12  15
 6. Florida      52   7   2  23  -9  -5
 7. Toronto      53   4  -2  17  -9 -12
 8. Ottawa       52   3   0  19   0   0
             👉🏻 421  84  25 172  72  42 0.600

=======================================
         Metropolitan division         
=======================================
                 GP +/- L10  RW  GD L10
 1. Carolina     52  17   6  23  30  17
 2. Pittsburgh   51  12   4  22  17  13
 3. NY Islanders 52   9   1  19   4   4
 4. Philadelphia 51   6  -4  17  -8 -19
 5. Columbus     51   4   2  15 -11  -1
 6. New Jersey   53   3   0  18 -22  -3
 7. Washington   54   3  -3  22   7  -7
 8. NY Rangers   53  -3  -5  14 -22 -19
             👉🏻 417  51   1 150  -5 -15 0.561

=======================================
           Central division            
=======================================
                 GP +/- L10  RW  GD L10
 1. Colorado     50  29   0  32  76   3
 2. Dallas       53  16   1  26  29   3
 3. Minnesota    54  16   2  19  18  -3
 4. Utah         53   7   7  21  21  15
 5. Nashville    52   1   1  17 -27  -5
 6. Chicago      53  -2  -2  16 -24 -12
 7. Winnipeg     52  -3   4  19  -8   5
 8. St. Louis    53  -6  -5  18 -53 -14
             👉🏻 420  58   8 168  32  -8 0.569

=======================================
           Pacific division            
=======================================
                 GP +/- L10  RW  GD L10
 1. Vegas        52  11   3  18   9   8
 2. Edmonton     54   8   2  21  11  10
 3. San Jose     51   6   4  16 -15   4
 4. Los Angeles  51   6   2  14  -7  -5
 5. Anaheim      53   6   4  16 -11   2
 6. Seattle      52   5  -1  18  -7  -1
 7. Calgary      52  -4  -2  17 -24 -10
 8. Vancouver    53 -14  -8  11 -55 -23
             👉🏻 418  24   4 131 -99 -15 0.529
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
 1. Tampa Bay    51  19   7  26  49  17
 2. Carolina     52  17   6  23  30  17
 3. Detroit      54  15   5  22   6  11
 4. Buffalo      52  13   5  24  20  16
 5. Pittsburgh   51  12   4  22  17  13
 6. Montréal     53  12   1  18   3   0
 7. Boston       54  11   7  23  12  15
 8. NY Islanders 52   9   1  19   4   4
 9. Florida      52   7   2  23  -9  -5
10. Philadelphia 51   6  -4  17  -8 -19
11. Columbus     51   4   2  15 -11  -1
12. Toronto      53   4  -2  17  -9 -12
13. Ottawa       52   3   0  19   0   0
14. New Jersey   53   3   0  18 -22  -3
15. Washington   54   3  -3  22   7  -7
16. NY Rangers   53  -3  -5  14 -22 -19
             👉🏻 838 135  26 322  67  27 0.581

=======================================
          Western conference           
=======================================
                 GP +/- L10  RW  GD L10
 1. Colorado     50  29   0  32  76   3
 2. Dallas       53  16   1  26  29   3
 3. Minnesota    54  16   2  19  18  -3
 4. Vegas        52  11   3  18   9   8
 5. Edmonton     54   8   2  21  11  10
 6. Utah         53   7   7  21  21  15
 7. San Jose     51   6   4  16 -15   4
 8. Los Angeles  51   6   2  14  -7  -5
 9. Anaheim      53   6   4  16 -11   2
10. Seattle      52   5  -1  18  -7  -1
11. Nashville    52   1   1  17 -27  -5
12. Chicago      53  -2  -2  16 -24 -12
13. Winnipeg     52  -3   4  19  -8   5
14. Calgary      52  -4  -2  17 -24 -10
15. St. Louis    53  -6  -5  18 -53 -14
16. Vancouver    53 -14  -8  11 -55 -23
             👉🏻 838  82  12 299 -67 -23 0.549
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
 1. Colorado     50  29   0  32  76   3
 2. Tampa Bay    51  19   7  26  49  17
 3. Carolina     52  17   6  23  30  17
 4. Dallas       53  16   1  26  29   3
 5. Minnesota    54  16   2  19  18  -3
 6. Detroit      54  15   5  22   6  11
 7. Buffalo      52  13   5  24  20  16
 8. Pittsburgh   51  12   4  22  17  13
 9. Montréal     53  12   1  18   3   0
10. Vegas        52  11   3  18   9   8
11. Boston       54  11   7  23  12  15
12. NY Islanders 52   9   1  19   4   4
13. Edmonton     54   8   2  21  11  10
14. Florida      52   7   2  23  -9  -5
15. Utah         53   7   7  21  21  15
16. Philadelphia 51   6  -4  17  -8 -19
17. San Jose     51   6   4  16 -15   4
18. Los Angeles  51   6   2  14  -7  -5
19. Anaheim      53   6   4  16 -11   2
20. Seattle      52   5  -1  18  -7  -1
21. Columbus     51   4   2  15 -11  -1
22. Toronto      53   4  -2  17  -9 -12
23. Ottawa       52   3   0  19   0   0
24. New Jersey   53   3   0  18 -22  -3
25. Washington   54   3  -3  22   7  -7
26. Nashville    52   1   1  17 -27  -5
27. Chicago      53  -2  -2  16 -24 -12
28. Winnipeg     52  -3   4  19  -8   5
29. NY Rangers   53  -3  -5  14 -22 -19
30. Calgary      52  -4  -2  17 -24 -10
31. St. Louis    53  -6  -5  18 -53 -14
32. Vancouver    53 -14  -8  11 -55 -23
             👉🏻1676 217  38 621   0   4 0.565
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
 1. Tampa Bay    51  19   7  26  49  17
 2. Boston       54  11   7  23  12  15
 3. Utah         53   7   7  21  21  15
 4. Carolina     52  17   6  23  30  17
 5. Detroit      54  15   5  22   6  11
 6. Buffalo      52  13   5  24  20  16
 7. Pittsburgh   51  12   4  22  17  13
 8. San Jose     51   6   4  16 -15   4
 9. Anaheim      53   6   4  16 -11   2
10. Winnipeg     52  -3   4  19  -8   5
11. Vegas        52  11   3  18   9   8
12. Minnesota    54  16   2  19  18  -3
13. Edmonton     54   8   2  21  11  10
14. Florida      52   7   2  23  -9  -5
15. Los Angeles  51   6   2  14  -7  -5
16. Columbus     51   4   2  15 -11  -1
17. Dallas       53  16   1  26  29   3
18. Montréal     53  12   1  18   3   0
19. NY Islanders 52   9   1  19   4   4
20. Nashville    52   1   1  17 -27  -5
21. Colorado     50  29   0  32  76   3
22. Ottawa       52   3   0  19   0   0
23. New Jersey   53   3   0  18 -22  -3
24. Seattle      52   5  -1  18  -7  -1
25. Toronto      53   4  -2  17  -9 -12
26. Chicago      53  -2  -2  16 -24 -12
27. Calgary      52  -4  -2  17 -24 -10
28. Washington   54   3  -3  22   7  -7
29. Philadelphia 51   6  -4  17  -8 -19
30. NY Rangers   53  -3  -5  14 -22 -19
31. St. Louis    53  -6  -5  18 -53 -14
32. Vancouver    53 -14  -8  11 -55 -23
             👉🏻1676 217  38 621   0   4 0.565
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

[8-w] Boston (11) at Tampa Bay (19)
[7-w] Montréal (12) at Carolina (17)
[3-2] Buffalo (13) at Detroit (15)
[3-2] NY Islanders (9) at Pittsburgh (12)

Outside looking-in: PHI (6) 

Western Conference:

[8-w] Los Angeles (6) at Colorado (29)
[3-2] Minnesota (16) at Dallas (16)
[7-w] Utah (7) at Vegas (11)
[3-2] San Jose (6) at Edmonton (8)

Outside looking-in: ANA (6) SEA (5) 
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

              2026-01-28 (WED)               
=============================================
NYR 0 - 0 NYI  19:00   (1) (MSGSN, MSG 2)
PHI 0 - 0 CBJ  19:30   (TNT, truTV, HBO MAX)
COL 0 - 0 OTT  19:30   (SN, TVAS, ALT)

              2026-01-29 (THU)               
=============================================
PHI at BOS  19:00   (SNE, NBCSP+, NESN)
LAK at BUF  19:00   (FDSNW, MSG-B)
COL at MTL  19:00   (TSN2, RDS, ALT)
WPG at TBL  19:00   (TSN3, The Spot)
NSH at NJD  19:00   (FDSNSO, MSGSN)
NYI at NYR  19:00   (MSGSN2, MSG)
CHI at PIT  19:00   (SN, TVAS, CHSN, SN-PIT)
UTA at CAR  19:00   (FDSNSO, Utah16)
WSH at DET  19:30   (ESPN+, HULU)
FLA at STL  20:00   (FDSNMW, SCRIPPS)
CGY at MIN  20:00   (SNW, FDSNNO, FDSNWI)
SJS at EDM  21:00   (SN1, NBCSCA)
ANA at VAN  22:00   (SNP, Victory+, KCOP-13)
DAL at VGK  22:00   (Victory+, SCRIPPS)
TOR at SEA  22:00   (ESPN+, HULU, SNO, TVAS)

              2026-01-30 (FRI)               
=============================================
CBJ at CHI  20:30   (NHLN, SN, TVAS, CHSN, FDSNOH)

              2026-01-31 (SAT)               
=============================================
LAK at PHI  12:30   (SN1, FDSNW, NBCSP)
COL at DET  13:00   (ABC, SN)
NYR at PIT  15:30   (ABC, SN, SN1, TVAS)
WPG at FLA  16:00   (TSN3, SCRIPPS)
SJS at CGY  16:00   (SN, NBCSCA)
CAR at WSH  17:00   (FDSNSO, MNMT)
MTL at BUF  19:00   (CITY, SNE, NHLN, TVAS, MSG-B)
NJD at OTT  19:00   (SN1, TVAS2, MSG)
NSH at NYI  19:00   (FDSNSO, MSGSN)
CBJ at STL  19:00   (FDSNMW, FDSNOH)
TOR at VAN  19:00   (SN, CBC)
DAL at UTA  21:00   (Utah16, Victory+)
MIN at EDM  22:00   (CBC, CITY, SN, TVAS, FDSNNOX, FDSNWI)
SEA at VGK  22:00   (KHN/Prime, KING 5, KONG, SCRIPPS)

              2026-02-01 (SUN)               
=============================================
LAK at CAR  15:00   (SN, FDSNSO, FDSNW)
BOS at TBL ✨  18:30   (ESPN, SN, TVAS)
VGK at ANA  21:30   (ESPN, SN360, SN (JIP), TVAS)

              2026-02-02 (MON)               
=============================================
BUF at FLA  19:00   (MSG-B, SCRIPPS)
OTT at PIT  19:00   (TVAS, TSN5, SN-PIT)
NYI at WSH  19:00   (MNMT, MSGSN)
MTL at MIN  19:30   (Prime, RDS, FDSNNO, FDSNWI)
STL at NSH  20:00   (FDSNMW, FDSNSO)
SJS at CHI  20:30   (CHSN, NBCSCA)
WPG at DAL  20:30   (FOX4, TSN3, Victory+)
DET at COL  21:00   (FDSNDET, ALT, KTVD)
VAN at UTA  21:30   (SNP, Utah16)
TOR at CGY  22:00   (SNW, TVAS, TSN4)

              2026-02-03 (TUE)               
=============================================
CBJ at NJD  19:00   (FDSNOH, MSGSN)
WSH at PHI  19:00   (NBCSP, MNMT2)
OTT at CAR  19:00   (RDS, TSN5, FDSNSO)
BUF at TBL  19:30   (ESPN+, HULU)
PIT at NYI  19:30   (TNT, HBO MAX, TVAS)
TOR at EDM  20:30   (SN, TVAS)
SEA at ANA  22:00   (TNT, HBO MAX)
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
51 Thu Jan 22 2026  BUF 4 - 2 MTL   
52 Sat Jan 24 2026  MTL 3 - 4 BOS   
53 Tue Jan 27 2026  VGK 2 - 3 MTL (OT)
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
