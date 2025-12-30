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
 1. Tampa Bay    38   9   3  18  25  10
 2. Detroit      40   9   4  16  -4   3
 3. Montréal     38   8   3  13  -2   5
 4. Florida      38   6   4  18   3   5
 5. Buffalo      38   6   8  14   0  12
 6. Ottawa       38   3   1  13   1   6
 7. Toronto      38   2   0  13  -5  -7
 8. Boston       40   2  -2  14  -8 -10
             👉🏻 308  45  21 119  10  24 0.573

=======================================
         Metropolitan division         
=======================================
                 GP +/- L10  RW  GD L10
 1. Carolina     38  13   5  15  19   7
 2. Philadelphia 37   8   2  12   6   2
 3. NY Islanders 39   7   1  15   3  -2
 4. Washington   39   6  -2  18  13 -13
 5. Pittsburgh   37   4  -3  14  -5 -13
 6. New Jersey   38   4  -1  13  -8  -6
 7. Columbus     38   2  -2  10 -12  -5
 8. NY Rangers   41   2  -1  12  -4  -6
             👉🏻 307  46  -1 109  12 -36 0.575

=======================================
           Central division            
=======================================
                 GP +/- L10  RW  GD L10
 1. Colorado     38  27   9  26  66  18
 2. Dallas       39  18   4  21  34   7
 3. Minnesota    40  14   7  17  22  18
 4. Nashville    38   0   4  12 -18   8
 5. Utah         40  -1  -2  13   3  -2
 6. St. Louis    40  -2  -1  15 -37 -10
 7. Winnipeg     37  -3  -5  13  -7 -11
 8. Chicago      38  -4  -6  12 -18 -19
             👉🏻 310  49  10 129  45   9 0.579

=======================================
           Pacific division            
=======================================
                 GP +/- L10  RW  GD L10
 1. Vegas        37   8   1  13   2  -2
 2. Edmonton     40   6   4  15   3  10
 3. Anaheim      39   5  -3  13  -3 -15
 4. Los Angeles  38   3  -2  11  -3  -4
 5. Seattle      37   2   1  11 -16   0
 6. San Jose     39   2   2  11 -16   0
 7. Calgary      39  -1   4  13 -13   5
 8. Vancouver    38  -3   2  10 -21   0
             👉🏻 307  22   9  97 -67  -6 0.536

=======================================
          Eastern conference           
=======================================
                 GP +/- L10  RW  GD L10
 1. Carolina     38  13   5  15  19   7
 2. Tampa Bay    38   9   3  18  25  10
 3. Detroit      40   9   4  16  -4   3
 4. Philadelphia 37   8   2  12   6   2
 5. Montréal     38   8   3  13  -2   5
 6. NY Islanders 39   7   1  15   3  -2
 7. Florida      38   6   4  18   3   5
 8. Buffalo      38   6   8  14   0  12
 9. Washington   39   6  -2  18  13 -13
10. Pittsburgh   37   4  -3  14  -5 -13
11. New Jersey   38   4  -1  13  -8  -6
12. Ottawa       38   3   1  13   1   6
13. Toronto      38   2   0  13  -5  -7
14. Columbus     38   2  -2  10 -12  -5
15. Boston       40   2  -2  14  -8 -10
16. NY Rangers   41   2  -1  12  -4  -6
             👉🏻 615  91  20 228  22 -12 0.574

=======================================
          Western conference           
=======================================
                 GP +/- L10  RW  GD L10
 1. Colorado     38  27   9  26  66  18
 2. Dallas       39  18   4  21  34   7
 3. Minnesota    40  14   7  17  22  18
 4. Vegas        37   8   1  13   2  -2
 5. Edmonton     40   6   4  15   3  10
 6. Anaheim      39   5  -3  13  -3 -15
 7. Los Angeles  38   3  -2  11  -3  -4
 8. Seattle      37   2   1  11 -16   0
 9. San Jose     39   2   2  11 -16   0
10. Nashville    38   0   4  12 -18   8
11. Calgary      39  -1   4  13 -13   5
12. Utah         40  -1  -2  13   3  -2
13. St. Louis    40  -2  -1  15 -37 -10
14. Winnipeg     37  -3  -5  13  -7 -11
15. Vancouver    38  -3   2  10 -21   0
16. Chicago      38  -4  -6  12 -18 -19
             👉🏻 617  71  19 226 -22   3 0.558

=======================================
              Full league              
=======================================
                 GP +/- L10  RW  GD L10
 1. Colorado     38  27   9  26  66  18
 2. Dallas       39  18   4  21  34   7
 3. Minnesota    40  14   7  17  22  18
 4. Carolina     38  13   5  15  19   7
 5. Tampa Bay    38   9   3  18  25  10
 6. Detroit      40   9   4  16  -4   3
 7. Vegas        37   8   1  13   2  -2
 8. Philadelphia 37   8   2  12   6   2
 9. Montréal     38   8   3  13  -2   5
10. NY Islanders 39   7   1  15   3  -2
11. Florida      38   6   4  18   3   5
12. Buffalo      38   6   8  14   0  12
13. Washington   39   6  -2  18  13 -13
14. Edmonton     40   6   4  15   3  10
15. Anaheim      39   5  -3  13  -3 -15
16. Pittsburgh   37   4  -3  14  -5 -13
17. New Jersey   38   4  -1  13  -8  -6
18. Ottawa       38   3   1  13   1   6
19. Los Angeles  38   3  -2  11  -3  -4
20. Seattle      37   2   1  11 -16   0
21. Toronto      38   2   0  13  -5  -7
22. Columbus     38   2  -2  10 -12  -5
23. San Jose     39   2   2  11 -16   0
24. Boston       40   2  -2  14  -8 -10
25. NY Rangers   41   2  -1  12  -4  -6
26. Nashville    38   0   4  12 -18   8
27. Calgary      39  -1   4  13 -13   5
28. Utah         40  -1  -2  13   3  -2
29. St. Louis    40  -2  -1  15 -37 -10
30. Winnipeg     37  -3  -5  13  -7 -11
31. Vancouver    38  -3   2  10 -21   0
32. Chicago      38  -4  -6  12 -18 -19
             👉🏻1232 162  39 454   0  -9 0.566

=======================================
      Full league (last 10 games)      
=======================================
                 GP +/- L10  RW  GD L10
 1. Colorado     38  27   9  26  66  18
 2. Buffalo      38   6   8  14   0  12
 3. Minnesota    40  14   7  17  22  18
 4. Carolina     38  13   5  15  19   7
 5. Dallas       39  18   4  21  34   7
 6. Detroit      40   9   4  16  -4   3
 7. Florida      38   6   4  18   3   5
 8. Edmonton     40   6   4  15   3  10
 9. Nashville    38   0   4  12 -18   8
10. Calgary      39  -1   4  13 -13   5
11. Tampa Bay    38   9   3  18  25  10
12. Montréal     38   8   3  13  -2   5
13. Philadelphia 37   8   2  12   6   2
14. San Jose     39   2   2  11 -16   0
15. Vancouver    38  -3   2  10 -21   0
16. Vegas        37   8   1  13   2  -2
17. NY Islanders 39   7   1  15   3  -2
18. Ottawa       38   3   1  13   1   6
19. Seattle      37   2   1  11 -16   0
20. Toronto      38   2   0  13  -5  -7
21. New Jersey   38   4  -1  13  -8  -6
22. NY Rangers   41   2  -1  12  -4  -6
23. St. Louis    40  -2  -1  15 -37 -10
24. Washington   39   6  -2  18  13 -13
25. Los Angeles  38   3  -2  11  -3  -4
26. Columbus     38   2  -2  10 -12  -5
27. Boston       40   2  -2  14  -8 -10
28. Utah         40  -1  -2  13   3  -2
29. Anaheim      39   5  -3  13  -3 -15
30. Pittsburgh   37   4  -3  14  -5 -13
31. Winnipeg     37  -3  -5  13  -7 -11
32. Chicago      38  -4  -6  12 -18 -19
             👉🏻1232 162  39 454   0  -9 0.566
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
 1. Tampa Bay    38   9   3  18  25  10
 2. Detroit      40   9   4  16  -4   3
 3. Montréal     38   8   3  13  -2   5
 4. Florida      38   6   4  18   3   5
 5. Buffalo      38   6   8  14   0  12
 6. Ottawa       38   3   1  13   1   6
 7. Toronto      38   2   0  13  -5  -7
 8. Boston       40   2  -2  14  -8 -10
             👉🏻 308  45  21 119  10  24 0.573

=======================================
         Metropolitan division         
=======================================
                 GP +/- L10  RW  GD L10
 1. Carolina     38  13   5  15  19   7
 2. Philadelphia 37   8   2  12   6   2
 3. NY Islanders 39   7   1  15   3  -2
 4. Washington   39   6  -2  18  13 -13
 5. Pittsburgh   37   4  -3  14  -5 -13
 6. New Jersey   38   4  -1  13  -8  -6
 7. Columbus     38   2  -2  10 -12  -5
 8. NY Rangers   41   2  -1  12  -4  -6
             👉🏻 307  46  -1 109  12 -36 0.575

=======================================
           Central division            
=======================================
                 GP +/- L10  RW  GD L10
 1. Colorado     38  27   9  26  66  18
 2. Dallas       39  18   4  21  34   7
 3. Minnesota    40  14   7  17  22  18
 4. Nashville    38   0   4  12 -18   8
 5. Utah         40  -1  -2  13   3  -2
 6. St. Louis    40  -2  -1  15 -37 -10
 7. Winnipeg     37  -3  -5  13  -7 -11
 8. Chicago      38  -4  -6  12 -18 -19
             👉🏻 310  49  10 129  45   9 0.579

=======================================
           Pacific division            
=======================================
                 GP +/- L10  RW  GD L10
 1. Vegas        37   8   1  13   2  -2
 2. Edmonton     40   6   4  15   3  10
 3. Anaheim      39   5  -3  13  -3 -15
 4. Los Angeles  38   3  -2  11  -3  -4
 5. Seattle      37   2   1  11 -16   0
 6. San Jose     39   2   2  11 -16   0
 7. Calgary      39  -1   4  13 -13   5
 8. Vancouver    38  -3   2  10 -21   0
             👉🏻 307  22   9  97 -67  -6 0.536
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
 1. Carolina     38  13   5  15  19   7
 2. Tampa Bay    38   9   3  18  25  10
 3. Detroit      40   9   4  16  -4   3
 4. Philadelphia 37   8   2  12   6   2
 5. Montréal     38   8   3  13  -2   5
 6. NY Islanders 39   7   1  15   3  -2
 7. Florida      38   6   4  18   3   5
 8. Buffalo      38   6   8  14   0  12
 9. Washington   39   6  -2  18  13 -13
10. Pittsburgh   37   4  -3  14  -5 -13
11. New Jersey   38   4  -1  13  -8  -6
12. Ottawa       38   3   1  13   1   6
13. Toronto      38   2   0  13  -5  -7
14. Columbus     38   2  -2  10 -12  -5
15. Boston       40   2  -2  14  -8 -10
16. NY Rangers   41   2  -1  12  -4  -6
             👉🏻 615  91  20 228  22 -12 0.574

=======================================
          Western conference           
=======================================
                 GP +/- L10  RW  GD L10
 1. Colorado     38  27   9  26  66  18
 2. Dallas       39  18   4  21  34   7
 3. Minnesota    40  14   7  17  22  18
 4. Vegas        37   8   1  13   2  -2
 5. Edmonton     40   6   4  15   3  10
 6. Anaheim      39   5  -3  13  -3 -15
 7. Los Angeles  38   3  -2  11  -3  -4
 8. Seattle      37   2   1  11 -16   0
 9. San Jose     39   2   2  11 -16   0
10. Nashville    38   0   4  12 -18   8
11. Calgary      39  -1   4  13 -13   5
12. Utah         40  -1  -2  13   3  -2
13. St. Louis    40  -2  -1  15 -37 -10
14. Winnipeg     37  -3  -5  13  -7 -11
15. Vancouver    38  -3   2  10 -21   0
16. Chicago      38  -4  -6  12 -18 -19
             👉🏻 617  71  19 226 -22   3 0.558
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
 1. Colorado     38  27   9  26  66  18
 2. Dallas       39  18   4  21  34   7
 3. Minnesota    40  14   7  17  22  18
 4. Carolina     38  13   5  15  19   7
 5. Tampa Bay    38   9   3  18  25  10
 6. Detroit      40   9   4  16  -4   3
 7. Vegas        37   8   1  13   2  -2
 8. Philadelphia 37   8   2  12   6   2
 9. Montréal     38   8   3  13  -2   5
10. NY Islanders 39   7   1  15   3  -2
11. Florida      38   6   4  18   3   5
12. Buffalo      38   6   8  14   0  12
13. Washington   39   6  -2  18  13 -13
14. Edmonton     40   6   4  15   3  10
15. Anaheim      39   5  -3  13  -3 -15
16. Pittsburgh   37   4  -3  14  -5 -13
17. New Jersey   38   4  -1  13  -8  -6
18. Ottawa       38   3   1  13   1   6
19. Los Angeles  38   3  -2  11  -3  -4
20. Seattle      37   2   1  11 -16   0
21. Toronto      38   2   0  13  -5  -7
22. Columbus     38   2  -2  10 -12  -5
23. San Jose     39   2   2  11 -16   0
24. Boston       40   2  -2  14  -8 -10
25. NY Rangers   41   2  -1  12  -4  -6
26. Nashville    38   0   4  12 -18   8
27. Calgary      39  -1   4  13 -13   5
28. Utah         40  -1  -2  13   3  -2
29. St. Louis    40  -2  -1  15 -37 -10
30. Winnipeg     37  -3  -5  13  -7 -11
31. Vancouver    38  -3   2  10 -21   0
32. Chicago      38  -4  -6  12 -18 -19
             👉🏻1232 162  39 454   0  -9 0.566
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
 1. Colorado     38  27   9  26  66  18
 2. Buffalo      38   6   8  14   0  12
 3. Minnesota    40  14   7  17  22  18
 4. Carolina     38  13   5  15  19   7
 5. Dallas       39  18   4  21  34   7
 6. Detroit      40   9   4  16  -4   3
 7. Florida      38   6   4  18   3   5
 8. Edmonton     40   6   4  15   3  10
 9. Nashville    38   0   4  12 -18   8
10. Calgary      39  -1   4  13 -13   5
11. Tampa Bay    38   9   3  18  25  10
12. Montréal     38   8   3  13  -2   5
13. Philadelphia 37   8   2  12   6   2
14. San Jose     39   2   2  11 -16   0
15. Vancouver    38  -3   2  10 -21   0
16. Vegas        37   8   1  13   2  -2
17. NY Islanders 39   7   1  15   3  -2
18. Ottawa       38   3   1  13   1   6
19. Seattle      37   2   1  11 -16   0
20. Toronto      38   2   0  13  -5  -7
21. New Jersey   38   4  -1  13  -8  -6
22. NY Rangers   41   2  -1  12  -4  -6
23. St. Louis    40  -2  -1  15 -37 -10
24. Washington   39   6  -2  18  13 -13
25. Los Angeles  38   3  -2  11  -3  -4
26. Columbus     38   2  -2  10 -12  -5
27. Boston       40   2  -2  14  -8 -10
28. Utah         40  -1  -2  13   3  -2
29. Anaheim      39   5  -3  13  -3 -15
30. Pittsburgh   37   4  -3  14  -5 -13
31. Winnipeg     37  -3  -5  13  -7 -11
32. Chicago      38  -4  -6  12 -18 -19
             👉🏻1232 162  39 454   0  -9 0.566
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

[8-w] Buffalo (6) at Carolina (13)
[7-w] Florida (6) at Tampa Bay (9)
[3-2] Montréal (8) at Detroit (9)
[3-2] NY Islanders (7) at Philadelphia (8)

Outside looking-in: WSH (6) PIT (4) NJD (4) OTT (3) 

Western Conference:

[8-w] Seattle (2) at Colorado (27)
[3-2] Minnesota (14) at Dallas (18)
[7-w] Los Angeles (3) at Vegas (8)
[3-2] Anaheim (5) at Edmonton (6)

Outside looking-in: SJS (2) NSH (0) CGY (-1) UTA (-1) 
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

              2025-12-30 (TUE)               
=============================================
NJD at TOR  19:00   (SNO, MSG)
MTL at FLA  19:00   (TSN2, RDS, SCRIPPS)
CAR at PIT  19:00   (SN, FDSNSO, SN-PIT)
NYI at CHI  20:30   (NHLN, CHSN, MSGSN)
PHI at VAN  22:00   (SN, NBCSP)

              2025-12-31 (WED)               
=============================================
NYR at WSH  12:30   (SN, TVAS, MNMT, MSG)
NSH at VGK  15:00   (SN, TVAS, FDSNSO, SCRIPPS)
TBL at ANA  16:00   (The Spot, Victory+, KCOP-13)
MIN at SJS  16:00   (FDSNNO, NBCSCA, FDSNWIX)
WPG at DET  18:30   (SN, FDSNDET)
NJD at CBJ  19:00   (FDSNOH, MSGSN)
BUF at DAL  20:00   (Victory+, MSG-B)
STL at COL  21:00   (FDSNMW, ALT)
PHI at CGY  21:30   (SN1, NBCSP)
BOS at EDM  21:30   (SN, NESN)

              2026-01-01 (THU)               
=============================================
WSH at OTT  13:00   (RDS, TSN5, MNMT)
UTA at NYI  15:00   (NHLN, SN, Utah16, MSGSN)
WPG at TOR  19:00   (SNO, NHLN, TSN3)
DET at PIT  19:00   (SN, FDSNDET, SN-PIT)
MTL at CAR  19:00   (TSN2, RDS, FDSNSO)
TBL at LAK  19:00   (The Spot, FDSNW)
DAL at CHI  20:30   (SN1, CHSN, Victory+)
NSH at SEA  22:00   (SN, FDSNSO, KHN/Prime, KONG)

              2026-01-02 (FRI)               
=============================================
VGK at STL  15:00   (SNW, SN1, TVAS, FDSNMW, KMOV-TV, Matrix-MW, SCRIPPS)
NYR at FLA ✨  20:00   (TNT, truTV, HBO MAX, SN, TVAS)
SEA at VAN  22:30   (SNP, KHN/Prime, KING 5, KONG)
MIN at ANA  22:30   (FDSNNO, Victory+, FDSNWIX, KCOP-13)

              2026-01-03 (SAT)               
=============================================
PIT at DET  12:00   (ABC, SN, TVAS)
UTA at NJD  15:00   (Utah16, MSGSN)
BUF at CBJ  15:00   (FDSNOH, MSG-B)
PHI at EDM  15:30   (SN, NBCSP)
MTL at STL  16:00   (TSN2, RDS, FDSNMW)
TBL at SJS  16:00   (The Spot, NBCSCA)
WPG at OTT  19:00   (SN1, TVAS)
TOR at NYI  19:00   (SN, CBC, NHLN, MSGSN)
CHI at WSH  19:00   (CHSN, MNMT)
COL at CAR  19:00   (FDSNSO, ALT)
NSH at CGY  19:00   (CITY, SNW, FDSNSO)
MIN at LAK  21:00   (FDSNNO, FDSNW, FDSNWI)
BOS at VAN  22:00   (CBC, CITY, SN, TVAS, NESN)

              2026-01-04 (SUN)               
=============================================
MTL at DAL  14:00   (TSN2, RDS, Victory+)
PIT at CBJ  15:00   (SNW, SN1, FDSNOH, SN-PIT)
COL at FLA  17:00   (SN, ALT, SCRIPPS)
CAR at NJD  19:00   (FDSNSO, MSGSN)
VGK at CHI  19:00   (SN1, TVAS, CHSN, SCRIPPS)

              2026-01-05 (MON)               
=============================================
UTA at NYR  19:00   (Utah16, MSG)
ANA at WSH  19:00   (Victory+, MNMT, KCOP-13)
DET at OTT  19:30   (Prime, TVAS, FDSNDET)
SEA at CGY  21:30   (SNW, KHN/Prime, KING 5, KONG)
MIN at LAK  22:30   (FDSNNO, FDSNW, FDSNWI)
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


----

## Resources

* New API docs: <https://gitlab.com/dword4/nhlapi/-/blob/master/new-api.md>
* Great docs: <https://github.com/Zmalski/NHL-API-Reference>
* Transform tools, JSON to Rust Serde: <https://transform.tools/json-to-rust-serde>
