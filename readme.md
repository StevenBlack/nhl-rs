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
 1. Detroit      37   8   5  16  -3   8
 2. Tampa Bay    36   7  -1  17  22   4
 3. Montréal     36   7   2  12  -5   2
 4. Florida      35   5   5  16   0   5
 5. Ottawa       35   5   2  13   7  12
 6. Boston       37   4   1  14   0   3
 7. Buffalo      35   3   4  12  -6   4
 8. Toronto      36   1   0  12  -6  -5
             👉🏻 287  40  18 112   9  33 0.570

=======================================
         Metropolitan division         
=======================================
                 GP +/- L10  RW  GD L10
 1. Carolina     35  12   3  14  18   2
 2. Philadelphia 35   8   2  11   7   5
 3. Washington   36   7   1  18  18  -1
 4. NY Islanders 36   6   3  13   2   3
 5. New Jersey   36   5  -2  13  -6 -10
 6. Pittsburgh   36   3  -4  13  -9 -18
 7. NY Rangers   38   2   0  11  -5  -7
 8. Columbus     36   0  -3   8 -17 -10
             👉🏻 288  43   0 101   8 -36 0.575

=======================================
           Central division            
=======================================
                 GP +/- L10  RW  GD L10
 1. Colorado     35  24   7  24  61  13
 2. Dallas       37  18   6  21  36  15
 3. Minnesota    37  12   4  16  19  13
 4. Utah         38   1   0  13   5   1
 5. Nashville    35  -1   4  11 -19   9
 6. Winnipeg     35  -2  -3  13  -4  -6
 7. St. Louis    38  -2   1  14 -36  -7
 8. Chicago      35  -3  -5  12 -13 -20
             👉🏻 290  47  14 124  49  18 0.581

=======================================
           Pacific division            
=======================================
                 GP +/- L10  RW  GD L10
 1. Vegas        34   8   4  12   1   2
 2. Anaheim      37   7   1  13   3  -2
 3. Edmonton     37   5   5  13  -2  13
 4. Los Angeles  35   4  -2  10  -4  -2
 5. San Jose     36   1   0   9 -15  -5
 6. Seattle      34   0  -4   9 -19 -12
 7. Calgary      36  -2   3  12 -11   5
 8. Vancouver    36  -3   0  10 -19  -3
             👉🏻 285  20   7  88 -66  -4 0.535

=======================================
          Eastern conference           
=======================================
                 GP +/- L10  RW  GD L10
 1. Carolina     35  12   3  14  18   2
 2. Philadelphia 35   8   2  11   7   5
 3. Detroit      37   8   5  16  -3   8
 4. Washington   36   7   1  18  18  -1
 5. Tampa Bay    36   7  -1  17  22   4
 6. Montréal     36   7   2  12  -5   2
 7. NY Islanders 36   6   3  13   2   3
 8. Florida      35   5   5  16   0   5
 9. Ottawa       35   5   2  13   7  12
10. New Jersey   36   5  -2  13  -6 -10
11. Boston       37   4   1  14   0   3
12. Buffalo      35   3   4  12  -6   4
13. Pittsburgh   36   3  -4  13  -9 -18
14. NY Rangers   38   2   0  11  -5  -7
15. Toronto      36   1   0  12  -6  -5
16. Columbus     36   0  -3   8 -17 -10
             👉🏻 575  83  18 213  17  -3 0.572

=======================================
          Western conference           
=======================================
                 GP +/- L10  RW  GD L10
 1. Colorado     35  24   7  24  61  13
 2. Dallas       37  18   6  21  36  15
 3. Minnesota    37  12   4  16  19  13
 4. Vegas        34   8   4  12   1   2
 5. Anaheim      37   7   1  13   3  -2
 6. Edmonton     37   5   5  13  -2  13
 7. Los Angeles  35   4  -2  10  -4  -2
 8. San Jose     36   1   0   9 -15  -5
 9. Utah         38   1   0  13   5   1
10. Seattle      34   0  -4   9 -19 -12
11. Nashville    35  -1   4  11 -19   9
12. Winnipeg     35  -2  -3  13  -4  -6
13. Calgary      36  -2   3  12 -11   5
14. St. Louis    38  -2   1  14 -36  -7
15. Chicago      35  -3  -5  12 -13 -20
16. Vancouver    36  -3   0  10 -19  -3
             👉🏻 575  67  21 212 -17  14 0.558

=======================================
              Full league              
=======================================
                 GP +/- L10  RW  GD L10
 1. Colorado     35  24   7  24  61  13
 2. Dallas       37  18   6  21  36  15
 3. Carolina     35  12   3  14  18   2
 4. Minnesota    37  12   4  16  19  13
 5. Vegas        34   8   4  12   1   2
 6. Philadelphia 35   8   2  11   7   5
 7. Detroit      37   8   5  16  -3   8
 8. Washington   36   7   1  18  18  -1
 9. Tampa Bay    36   7  -1  17  22   4
10. Montréal     36   7   2  12  -5   2
11. Anaheim      37   7   1  13   3  -2
12. NY Islanders 36   6   3  13   2   3
13. Florida      35   5   5  16   0   5
14. Ottawa       35   5   2  13   7  12
15. New Jersey   36   5  -2  13  -6 -10
16. Edmonton     37   5   5  13  -2  13
17. Los Angeles  35   4  -2  10  -4  -2
18. Boston       37   4   1  14   0   3
19. Buffalo      35   3   4  12  -6   4
20. Pittsburgh   36   3  -4  13  -9 -18
21. NY Rangers   38   2   0  11  -5  -7
22. Toronto      36   1   0  12  -6  -5
23. San Jose     36   1   0   9 -15  -5
24. Utah         38   1   0  13   5   1
25. Seattle      34   0  -4   9 -19 -12
26. Columbus     36   0  -3   8 -17 -10
27. Nashville    35  -1   4  11 -19   9
28. Winnipeg     35  -2  -3  13  -4  -6
29. Calgary      36  -2   3  12 -11   5
30. St. Louis    38  -2   1  14 -36  -7
31. Chicago      35  -3  -5  12 -13 -20
32. Vancouver    36  -3   0  10 -19  -3
             👉🏻1150 150  39 425   0  11 0.565

=======================================
      Full league (last 10 games)      
=======================================
                 GP +/- L10  RW  GD L10
 1. Colorado     35  24   7  24  61  13
 2. Dallas       37  18   6  21  36  15
 3. Detroit      37   8   5  16  -3   8
 4. Florida      35   5   5  16   0   5
 5. Edmonton     37   5   5  13  -2  13
 6. Minnesota    37  12   4  16  19  13
 7. Vegas        34   8   4  12   1   2
 8. Buffalo      35   3   4  12  -6   4
 9. Nashville    35  -1   4  11 -19   9
10. Carolina     35  12   3  14  18   2
11. NY Islanders 36   6   3  13   2   3
12. Calgary      36  -2   3  12 -11   5
13. Philadelphia 35   8   2  11   7   5
14. Montréal     36   7   2  12  -5   2
15. Ottawa       35   5   2  13   7  12
16. Washington   36   7   1  18  18  -1
17. Anaheim      37   7   1  13   3  -2
18. Boston       37   4   1  14   0   3
19. St. Louis    38  -2   1  14 -36  -7
20. NY Rangers   38   2   0  11  -5  -7
21. Toronto      36   1   0  12  -6  -5
22. San Jose     36   1   0   9 -15  -5
23. Utah         38   1   0  13   5   1
24. Vancouver    36  -3   0  10 -19  -3
25. Tampa Bay    36   7  -1  17  22   4
26. New Jersey   36   5  -2  13  -6 -10
27. Los Angeles  35   4  -2  10  -4  -2
28. Columbus     36   0  -3   8 -17 -10
29. Winnipeg     35  -2  -3  13  -4  -6
30. Pittsburgh   36   3  -4  13  -9 -18
31. Seattle      34   0  -4   9 -19 -12
32. Chicago      35  -3  -5  12 -13 -20
             👉🏻1150 150  39 425   0  11 0.565
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
 1. Detroit      37   8   5  16  -3   8
 2. Tampa Bay    36   7  -1  17  22   4
 3. Montréal     36   7   2  12  -5   2
 4. Florida      35   5   5  16   0   5
 5. Ottawa       35   5   2  13   7  12
 6. Boston       37   4   1  14   0   3
 7. Buffalo      35   3   4  12  -6   4
 8. Toronto      36   1   0  12  -6  -5
             👉🏻 287  40  18 112   9  33 0.570

=======================================
         Metropolitan division         
=======================================
                 GP +/- L10  RW  GD L10
 1. Carolina     35  12   3  14  18   2
 2. Philadelphia 35   8   2  11   7   5
 3. Washington   36   7   1  18  18  -1
 4. NY Islanders 36   6   3  13   2   3
 5. New Jersey   36   5  -2  13  -6 -10
 6. Pittsburgh   36   3  -4  13  -9 -18
 7. NY Rangers   38   2   0  11  -5  -7
 8. Columbus     36   0  -3   8 -17 -10
             👉🏻 288  43   0 101   8 -36 0.575

=======================================
           Central division            
=======================================
                 GP +/- L10  RW  GD L10
 1. Colorado     35  24   7  24  61  13
 2. Dallas       37  18   6  21  36  15
 3. Minnesota    37  12   4  16  19  13
 4. Utah         38   1   0  13   5   1
 5. Nashville    35  -1   4  11 -19   9
 6. Winnipeg     35  -2  -3  13  -4  -6
 7. St. Louis    38  -2   1  14 -36  -7
 8. Chicago      35  -3  -5  12 -13 -20
             👉🏻 290  47  14 124  49  18 0.581

=======================================
           Pacific division            
=======================================
                 GP +/- L10  RW  GD L10
 1. Vegas        34   8   4  12   1   2
 2. Anaheim      37   7   1  13   3  -2
 3. Edmonton     37   5   5  13  -2  13
 4. Los Angeles  35   4  -2  10  -4  -2
 5. San Jose     36   1   0   9 -15  -5
 6. Seattle      34   0  -4   9 -19 -12
 7. Calgary      36  -2   3  12 -11   5
 8. Vancouver    36  -3   0  10 -19  -3
             👉🏻 285  20   7  88 -66  -4 0.535
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
 1. Carolina     35  12   3  14  18   2
 2. Philadelphia 35   8   2  11   7   5
 3. Detroit      37   8   5  16  -3   8
 4. Washington   36   7   1  18  18  -1
 5. Tampa Bay    36   7  -1  17  22   4
 6. Montréal     36   7   2  12  -5   2
 7. NY Islanders 36   6   3  13   2   3
 8. Florida      35   5   5  16   0   5
 9. Ottawa       35   5   2  13   7  12
10. New Jersey   36   5  -2  13  -6 -10
11. Boston       37   4   1  14   0   3
12. Buffalo      35   3   4  12  -6   4
13. Pittsburgh   36   3  -4  13  -9 -18
14. NY Rangers   38   2   0  11  -5  -7
15. Toronto      36   1   0  12  -6  -5
16. Columbus     36   0  -3   8 -17 -10
             👉🏻 575  83  18 213  17  -3 0.572

=======================================
          Western conference           
=======================================
                 GP +/- L10  RW  GD L10
 1. Colorado     35  24   7  24  61  13
 2. Dallas       37  18   6  21  36  15
 3. Minnesota    37  12   4  16  19  13
 4. Vegas        34   8   4  12   1   2
 5. Anaheim      37   7   1  13   3  -2
 6. Edmonton     37   5   5  13  -2  13
 7. Los Angeles  35   4  -2  10  -4  -2
 8. San Jose     36   1   0   9 -15  -5
 9. Utah         38   1   0  13   5   1
10. Seattle      34   0  -4   9 -19 -12
11. Nashville    35  -1   4  11 -19   9
12. Winnipeg     35  -2  -3  13  -4  -6
13. Calgary      36  -2   3  12 -11   5
14. St. Louis    38  -2   1  14 -36  -7
15. Chicago      35  -3  -5  12 -13 -20
16. Vancouver    36  -3   0  10 -19  -3
             👉🏻 575  67  21 212 -17  14 0.558
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
 1. Colorado     35  24   7  24  61  13
 2. Dallas       37  18   6  21  36  15
 3. Carolina     35  12   3  14  18   2
 4. Minnesota    37  12   4  16  19  13
 5. Vegas        34   8   4  12   1   2
 6. Philadelphia 35   8   2  11   7   5
 7. Detroit      37   8   5  16  -3   8
 8. Washington   36   7   1  18  18  -1
 9. Tampa Bay    36   7  -1  17  22   4
10. Montréal     36   7   2  12  -5   2
11. Anaheim      37   7   1  13   3  -2
12. NY Islanders 36   6   3  13   2   3
13. Florida      35   5   5  16   0   5
14. Ottawa       35   5   2  13   7  12
15. New Jersey   36   5  -2  13  -6 -10
16. Edmonton     37   5   5  13  -2  13
17. Los Angeles  35   4  -2  10  -4  -2
18. Boston       37   4   1  14   0   3
19. Buffalo      35   3   4  12  -6   4
20. Pittsburgh   36   3  -4  13  -9 -18
21. NY Rangers   38   2   0  11  -5  -7
22. Toronto      36   1   0  12  -6  -5
23. San Jose     36   1   0   9 -15  -5
24. Utah         38   1   0  13   5   1
25. Seattle      34   0  -4   9 -19 -12
26. Columbus     36   0  -3   8 -17 -10
27. Nashville    35  -1   4  11 -19   9
28. Winnipeg     35  -2  -3  13  -4  -6
29. Calgary      36  -2   3  12 -11   5
30. St. Louis    38  -2   1  14 -36  -7
31. Chicago      35  -3  -5  12 -13 -20
32. Vancouver    36  -3   0  10 -19  -3
             👉🏻1150 150  39 425   0  11 0.565
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
 1. Colorado     35  24   7  24  61  13
 2. Dallas       37  18   6  21  36  15
 3. Detroit      37   8   5  16  -3   8
 4. Florida      35   5   5  16   0   5
 5. Edmonton     37   5   5  13  -2  13
 6. Minnesota    37  12   4  16  19  13
 7. Vegas        34   8   4  12   1   2
 8. Buffalo      35   3   4  12  -6   4
 9. Nashville    35  -1   4  11 -19   9
10. Carolina     35  12   3  14  18   2
11. NY Islanders 36   6   3  13   2   3
12. Calgary      36  -2   3  12 -11   5
13. Philadelphia 35   8   2  11   7   5
14. Montréal     36   7   2  12  -5   2
15. Ottawa       35   5   2  13   7  12
16. Washington   36   7   1  18  18  -1
17. Anaheim      37   7   1  13   3  -2
18. Boston       37   4   1  14   0   3
19. St. Louis    38  -2   1  14 -36  -7
20. NY Rangers   38   2   0  11  -5  -7
21. Toronto      36   1   0  12  -6  -5
22. San Jose     36   1   0   9 -15  -5
23. Utah         38   1   0  13   5   1
24. Vancouver    36  -3   0  10 -19  -3
25. Tampa Bay    36   7  -1  17  22   4
26. New Jersey   36   5  -2  13  -6 -10
27. Los Angeles  35   4  -2  10  -4  -2
28. Columbus     36   0  -3   8 -17 -10
29. Winnipeg     35  -2  -3  13  -4  -6
30. Pittsburgh   36   3  -4  13  -9 -18
31. Seattle      34   0  -4   9 -19 -12
32. Chicago      35  -3  -5  12 -13 -20
             👉🏻1150 150  39 425   0  11 0.565
```

<!-- END:last10 -->

### Playoff matchups with `--playoffs` or `-p`

Playoff matchups are derived from the current standings on a ***wins minus losses** basis.

Playoff matchups are followed by a list of proximate teams "on the outside, looking in."

<!-- BEGIN:playoffs -->

```text
$ nhl -p


===================================
         Playoff Matchups          
===================================
Eastern Conference:

[8-w] Florida (5) at Carolina (12)
[3-2] Washington (7) at Philadelphia (8)
[7-w] NY Islanders (6) at Detroit (8)
[3-2] Montréal (7) at Tampa Bay (7)

Outside looking-in: OTT (5) NJD (5) BOS (4) BUF (3) PIT (3) NYR (2) 

Western Conference:

[8-w] San Jose (1) at Colorado (24)
[3-2] Minnesota (12) at Dallas (18)
[7-w] Los Angeles (4) at Vegas (8)
[3-2] Edmonton (5) at Anaheim (7)

Outside looking-in: UTA (1) SEA (0) NSH (-1) WPG (-2) CGY (-2) STL (-2) 
```

<!-- END:playoffs -->

## Schedules

### Current schedule segment (7-days) with `--schedule` or `-s`

The current schedule uses the following call to the NHL api:

* https://api-web.nhle.com/v1/schedule/now

Shows the next 7 days of NHL games (including today), strat times, and broadcast networks.

<!-- BEGIN:schedule -->

```text
$ nhl -s


=======================================================
             Upcoming league-wide schedule             
=======================================================

              2025-12-23 (TUE)               
=============================================
PIT 3 - 6 TOR  16:00   (TNT, truTV, HBO MAX, SNO)
DAL 3 - 2 DET  18:30   (TNT, truTV, HBO MAX, Victory+)
NYR 2 - 3 WSH  19:00   (SN, MSG 2, MNMT2)
MTL 2 - 2 BOS  19:00   (TSN2, RDS, NESN)
BUF 2 - 2 OTT  19:00   (RDS2, TSN5, MSG-B)
NJD 1 - 1 NYI  19:00   (MSGSN2, MSGSN)
FLA 1 - 2 CAR  19:00   (FDSNSO, SCRIPPS)
NSH 2 - 2 MIN  20:00   (ESPN+, HULU)
PHI 0 - 0 CHI  21:00   (TNT, truTV, HBO MAX)
UTA 0 - 0 COL  21:00   (Utah16, ALT)
CGY 0 - 0 EDM  21:00   (SNW, SN360)
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

              2025-12-27 (SAT)               
=============================================
NYR at NYI  18:00   (MSGSN, MSG 2)
BOS at BUF  19:00   (MSG-B, NESN)
OTT at TOR  19:00   (SN, CBC, CITY, TVAS)
TBL at FLA  19:00   (The Spot, SCRIPPS)
WSH at NJD  19:00   (MNMT, MSGSN2)
DET at CAR  19:00   (FDSNSO, FDSNDET)
MIN at WPG  19:00   (SNW, SN1, FDSNNO, FDSNWIX)
NSH at STL  20:00   (FDSNMW, FDSNSO, KMOV-TV, Matrix-MW)
CHI at DAL  20:00   (CHSN, Victory+)
ANA at LAK  21:00   (FDSNW, KTTV, Victory+)
EDM at CGY  22:00   (SN, CBC, TVAS)
SJS at VAN  22:00   (CITY, SNP, SN1, NBCSCA)
COL at VGK  22:00   (ALT, SCRIPPS)

              2025-12-28 (SUN)               
=============================================
MTL at TBL  17:00   (TSN2, RDS, The Spot)
NYI at CBJ  17:00   (FDSNOH, MSGSN)
TOR at DET  19:00   (SNO, FDSNDET)
PIT at CHI  19:00   (SN, TVAS, CHSN, SN-PIT)
PHI at SEA  20:00   (NHLN, KHN/Prime, NBCSP, KONG)

              2025-12-29 (MON)               
=============================================
CBJ at OTT  19:00   (TSN5, RDSI, FDSNOH)
WSH at FLA  19:00   (TVAS, MNMT, SCRIPPS)
NYR at CAR  19:00   (FDSNSO, MSGSN)
EDM at WPG  19:30   (Prime)
BUF at STL  20:00   (FDSNMW, MSG-B)
LAK at COL  21:00   (FDSNW, ALT)
NSH at UTA  21:00   (FDSNSO, Utah16)
BOS at CGY  21:00   (SNW, TVAS, NESN)
MIN at VGK  22:00   (FDSNNOX, FDSNWIX, SCRIPPS)
SJS at ANA  22:00   (KTTV, Victory+, NBCSCA)
VAN at SEA  22:00   (SNP, KHN/Prime, KONG)
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
37 Tue Dec 23 2025  MTL 2 - 2 BOS  19:00 (In progress) (TSN2, RDS, NESN)
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


----

## Resources

* New API docs: <https://gitlab.com/dword4/nhlapi/-/blob/master/new-api.md>
* Great docs: <https://github.com/Zmalski/NHL-API-Reference>
* Transform tools, JSON to Rust Serde: <https://transform.tools/json-to-rust-serde>
