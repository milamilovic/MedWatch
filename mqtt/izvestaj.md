# MQTT izvestaj

Analiziran je sistem za zdravstveni monitoring gde IoT uređaji šalju medicinske podatke pacijenata putem MQTT message broker-a, konkretno Mosquitto brokera. Kao bezbednosna pretnja identifikovana je kategorija napda koji ciljaju integritet sistema prema CIA trijadi.

## Stablo napada

<img width="3887" height="1308" alt="image" src="https://github.com/user-attachments/assets/701d0b6b-d277-4495-9d35-0c1e256bd829" />

### Praktično realizovan napad
Napad eksploatiše ranjivost CVE-2017-7650 u Eclipse Mosquitto brokerima verzije pre 1.4.12, gde pattern-based ACL mehanizam ne vrši proveru pristupa za klijente čiji client_id sadrži MQTT wildcard karaktere # ili +. Sistem koji je implementiran se sastoji iz tri komponente koje su kontejnerizovane i organizovane pomoću docker compose-a. Ovaj napad spada u CWE-284 (Improper Access Control) po MITRE terminologiji

- #### Implementacija ranjive aplikacije
    Ranjiv sistem se sastoji iz Eclipse Mosquitto brokera verzije 1.4.10 i aplikacije implementirane u Rust programskom jeziku. Mosquitto je pokrenut sa autentifikacijom putem password fajla i pattern-based ACL pravilima koja treba da ograniče svakog korisnika isključivo na sopstveni topic oblika health/data/<username>. Aplikacija simulira dva IoT uređaja koji periodično šalju vitalne znakove pacijenata u JSON formatu, kao i monitoring stanicu koja prima i prikazuje pristigle podatke.

- #### Implementacija napada
    Napad se izvodi pokretanjem aplikacije implementirane u Rust-u koja se konektuje na broker sa legitimnim kredencijalima korisnika patient1, ali sa client_id postavljenim na vrednost #. Zbog baga u pattern matching mehanizmu ranjive verzije brokera, ACL provera se u potpunosti preskače za takvog klijenta. Napad se odvija u dve faze: u prvoj fazi napadač se pretplaćuje na wildcard topic # čime dobija uvid u medicinske podatke svih pacijenata na brokeru, a u drugoj fazi publish-uje lažne medicinske podatke na topic health/data/patient1 simulirajući kritično stanje pacijenta. Ovi lažni podaci se ne mogu razlikovati od legitimnih, što u realnom scenariju može izazvati pogrešne medicinske odluke ili lažne SOS uzbune.

- #### Implementacija mitigovane aplikacije
    Mitigovan sistem se kao i ranjiv sastoji iz Eclipse Mosquitto brokera i aplikacije u Rust-u, ali je broker zamenjen zakrpljenom verzijom 2.0.18 koja eksplicitno validira client_id i username vrednosti pre ACL provere i odbija konekciju svakog klijenta čiji identifikator sadrži karaktere #, + ili /. Pored toga, ACL konfiguracija sadrži eksplicitna deny pravila kao dodatni sloj zaštite. Kada napadač pokuša isti napad, broker vraća NotAuthorized grešku već u fazi uspostavljanja konekcije i napad bude blokiran pre nego što napadač dobije ikakav pristup sistemu.

- #### Video demonstracija

https://github.com/user-attachments/assets/a32ce19d-5d46-44a2-ba5a-10b9c52bd285


### Napadi koji nisu realizovani
Preostala četiri napada iz stabla nisu implementirani u praktičnom delu, ali su teorijski relevantni za potpunu bezbednosnu analizu sistema. 

- #### Kršenje pristupa retained porukama
    CVE-2018-12546

    TODO

- #### Prazna ACL politika, podrazumevani allow all
    CVE-2018-12550
    
    TODO

- #### Autentifikacija putem loše kreiranog password fajla
    CVE-2018-12551
    
    TODO

- #### Subscription bypass za offline durable klijente
    CVE-2021-34434
  
    TODO


## Reference
https://nvd.nist.gov/vuln/detail/cve-2017-7650

https://nvd.nist.gov/vuln/detail/cve-2018-12546

https://nvd.nist.gov/vuln/detail/cve-2018-12550

https://nvd.nist.gov/vuln/detail/cve-2018-12551

https://nvd.nist.gov/vuln/detail/cve-2021-34434
