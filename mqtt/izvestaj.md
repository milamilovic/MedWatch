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

    Ranjivost pogađa Eclipse Mosquitto verzije od 1.0 do 1.5.5 uključujući i utiče na integritet podataka. Kada klijent objavi retained poruku odnosno poruku koja se zadržava, a zatim mu se ukine pristup tom topic-u te poruke će i dalje biti objavljene klijentima koji se pretplate na taj topic u budućnosti. Mitigovane verzije mosquitto-a su >= 1.5.6 i uvedena je opcija da se konfiguriše 'check_retain_source' koja ukoliko je podršena proverava prava izvora poruke pre objavljivanja. Koraci za izvođenje ovog napada su:
    - kreiranje acl fajla sa sledećim sadržajem:
       ```
       topic read a/a
       topic write a/a
       ```
    - pokretanje mosquitto brokera koji koristi taj acl fajl
    - korišćenje MQTT klijenta za pretplatu na poruke iz teme „a/a“
       ```
       mosquitto_sub -t 'a/a'
       ```
    - korišćenje drugog klijenta da se objavi (retained) poruka na temu „a/a“
    - brisanje linije „topic write a/a" iz ACL fajla i slanje SIGHUP signala mosquitto-u da bi se ponovo učitala konfiguracija:
       ```
       kill -HUP <mosquitto_pid>
       ```
    - ponovno povezivanje prvog klijenta i pretplaćivanje na temu „a/a“. Broker će isporučiti retained poruku koju je sačuvao u koraku 4, iako izvorni publisher više nema pravo pisanja na taj topic. 

- #### Prazna ACL politika, podrazumevani allow all
    CVE-2018-12550
    
    Ranjivost pogađa Eclipse Mosquitto verzije od 1.0 do 1.5.5. Ako je ACL datoteka prazna ili ima samo prazne redove ili komentare, onda mosquitto tretira ACL datoteku kao da nije definisana, što znači da nikakav pristup nije zabranjen. Iako uskraćivanje pristupa svim temama nije korisna konfiguracija, ovo ponašanje je neočekivano i može dovesti do toga da pristup bude nepravilno odobren u nekim okolnostima. Ova ranjivost se ispoljava tako što na primer klijent ima pravo pristupa topic-u „a/a“, odnosno ACL fajl sadrži 
    ```
       topic readwrite a/a
    ```
    Da bi se klijentu uklonila ta permisija, administrator može da zakomentariše taj red. ACL fajl sada sadrži samo komentar, odnosno efektivno je prazan. Očekivano ponašanje bi bilo da klijent više nema pravo pristupa tom fajlu, ali broker tumači prazan ACL fajl kao da ACL uopšte nije definisan i primenjuje podrazumevanu allow-all politiku. Klijent koji se sada konektuje i pretplati na bilo koji topic, uključujući i one koji nikada nisu bili definisani u ACL fajlu, dobija neograničen pristup. Ovo je suprotno nameri administratora koji je hteo da opozove sva prava. Mitigacija za ovaj napad je upgrade na verziju >= 1.5.6 gde prazan ACL fajl rezultuje deny-all politikom, što je bezbednije i intuitivnije ponašanje.

- #### Autentifikacija putem loše kreiranog password fajla
    CVE-2018-12551
    
    Ranjivost pogađa mosquitto verzije 1.0 do 1.5.5. Kada je broker konfigurisan da koristi password fajl za autentifikaciju, svaki unos pogrešnog oblika u tom fajlu tretira se kao validan. Konkretno, prazna linija u password fajlu tretira se kao validan korisnik sa praznim korisničkim imenom i bez lozinke, što znači da napadač može da se konektuje sa praznim username poljem i bez lozinke. Ova ranjivost nastaje najčešće usled ručnih izmena password fajla, grešaka pri kreiranju backup-a ili neispravnog generisanja fajla automatizovanim skriptama. Ranjivost ne utiče na sisteme gde je password fajl kreiran i modifikovan isključivo putem mosquitto_passwd alata. Da bi se ovaj napad reprodukovao potrebno je jedino konfigurisati /etc/mosquitto.conf fajl:
    ```
    password_file /etc/mosquitto/mosquitto.users
    acl_file /etc/mosquitto/mosquitto.acl
    allow_anonymous false
    ```
    zatim i /etc/mosquitto.acl
    ```
    user test
    readwrite #
    ```
    Očekuje se i da postoji fajl mosquitto.users koji sadrži korisnika test sa lozinkom test i prazan red
    ```
    test:$6$BHZEmbaA2YgNtNRI$qJ399QBKyrhnzEQCyoL3qU0N8VopPEGZwCjXb8fALz/cFP+ICnJi7cIIIdm3if08qc/0YbI3Ete0md2GqUjG7Q==
    
    ```
    Kada se pokrene mosquitto sa ovom konfiguracijom i pokuša da se pošalje poruka sa praznim username-om komandom 
    ```
    mosquitto_pub -d -u '' -t DoorControl -m UNLOCK
    ```
    poruka bude uspešno poslata, iako ne bi trebalo. Bez prazne linije u mosquitto.users fajlu se ranjivost ne može demonstrirati. Mitigacija za ovu ranjivost je upgrade na verziju >= 1.5.6, kao i obavezno korišćenje mosquitto_passwd alata za sve izmene password fajla.

- #### Subscription bypass za offline durable klijente
    CVE-2021-34434
  
    Ranjivost pogađa Eclipse Mosquitto verzije od 2.0.0 do 2.0.11 i nastaje u dynamic security pluginu koji je uveden u verziji 2.0 kao moderniji mehanizam za upravljanje korisnicima i dozvolama. U MQTT protokolu, durable klijent je klijent koji se konektuje sa clean_session=false, što znači da broker čuva aktivne subscriptione i neprimljene poruke dok je klijent offline. Ranjivost se ispoljava u situaciji kada administrator oduzme klijentu pravo pretplate na određeni topic dok je taj klijent offline zbog toga što broker ne revokuje postojeće subscriptione za tog klijenta, već ih zadržava u internoj strukturi. Kada se klijent ponovo konektuje, on nastavlja da prima poruke na topicima za koje mu je pristup oduzet, kao da revokacija nikada nije izvršena. Dynamic Security Plugin postavlja acl vrednosti na
    ```
    * publishClientSend: deny
    * publishClientReceive: allow
    * subscribe: deny
    * unsubscribe: allow
    ```
    Da bi se ova ranjivost demonstrirala, klijent se najpre konektuje na broker sa trajnom sesijom i dovoljno dugim intervalom sesije (`cleanStart=false`, `sessionExpiryInterval=10000`) i pretplaćuje se na topic `message/state`. Zatim se klijent diskonektuje, a administrator u međuvremenu opoziva pravo pretplate putem dynamic security plugina komandom `mosquitto_ctrl dynsec removeClientSubscription <klijent> message/state`. Kada se klijent ponovo konektuje sa `cleanStart=false`, broker restauriše prethodnu sesiju uključujući subscription na `message/state` bez slanja novog SUBSCRIBE paketa. Zbog toga što je podrazumevana vrednost za `publishClientReceive` postavljena na `allow`, klijent i dalje prima poruke sa topica `message/state`, iako mu je pristup formalno oduzet. Mitigacija za ovaj napad je upgrade na verziju >= 2.0.12.

## Reference
https://nvd.nist.gov/vuln/detail/cve-2017-7650

https://nvd.nist.gov/vuln/detail/cve-2018-12546

https://bugs.eclipse.org/bugs/show_bug.cgi?id=543127

https://nvd.nist.gov/vuln/detail/cve-2018-12550

https://nvd.nist.gov/vuln/detail/cve-2018-12551

https://security.snyk.io/vuln/SNYK-COCOAPODS-MOSQUITTO-470675

https://nvd.nist.gov/vuln/detail/cve-2021-34434

https://bugs.eclipse.org/bugs/show_bug.cgi?id=543401

https://bugs.launchpad.net/ubuntu/+source/mosquitto/+bug/1814931

https://bugs.eclipse.org/bugs/show_bug.cgi?id=541870

https://bugs.eclipse.org/bugs/show_bug.cgi?id=575324
