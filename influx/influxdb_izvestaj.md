# InfluxDB Izveštaj

Analiziran je sistem za zdravstveni monitoring gde IoT uređaji šalju medicinske podatke pacijenata putem InfluxDB vremenske baze podataka. Kao bezbednosna pretnja identifikovana je kategorija napada koji ciljaju **poverljivost (confidentiality)** sistema prema CIA trijadi.


## Stablo napada

<img width="6787" height="1196" alt="influxdb-diagram" src="https://github.com/user-attachments/assets/29a4ffb3-9ae7-4231-a9a4-657510486aea" />

## Praktično realizovan napad

Napad eksploatiše CVE-2019-20933 u InfluxDB verzijama pre 1.7.6 i podrazumevanu konfiguraciju InfluxDB 1.x gde je autentifikacija isključena (`auth-enabled=false`). Sistem koji je implementiran se sastoji iz tri komponente koje su kontejnerizovane i organizovane pomoću docker compose-a. Ovaj napad spada u **CWE-287** (Improper Authentication) i **CWE-306** (Missing Authentication for Critical Function) po MITRE terminologiji.
