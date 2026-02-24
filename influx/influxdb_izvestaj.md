# InfluxDB Izveštaj

Analiziran je sistem za zdravstveni monitoring gde IoT uređaji šalju medicinske podatke pacijenata putem InfluxDB vremenske baze podataka. Kao bezbednosna pretnja identifikovana je kategorija napada koji ciljaju poverljivost (confidentiality) sistema prema CIA trijadi.


## Stablo napada

<img width="6787" height="1196" alt="influxdb-diagram" src="https://github.com/user-attachments/assets/29a4ffb3-9ae7-4231-a9a4-657510486aea" />

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
