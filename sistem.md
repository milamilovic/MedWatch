# MedWatch

## Domen problema
Softverski sistem se nalazi u domenu digitalnog zdravstva i IoT sistema za praćenje zdravstvenog stanja korisnika. Sistem se koristi za prikupljanje, obradu i analizu zdravstvenih podataka koji se čitaju pomoću pametnih uređaja poput pametnih satova, fitnes narukvica i medicinskih senzora. Cilj je da se korisnicima omogući real-time uvid u sopstveno zdravstveno stanje, kao i automatska reakcija u slučaju kritičnog zdravstvenog stanja.

Osnovna ideja je da se zdravstveni podaci prikupljaju, čuvaju i analiziraju u realnom ili skoro realnom vremenu. SOS uzbuna se aktivira isključivo u slučaju detekcije kritičnih parametara, čime se smanjuje broj lažnih uzbuna. Sistem je posebno koristan za starije osobe, hronične bolesnike i korisnike koji žive sami, gde je pravovremena reakcija od presudnog značaja.

Sistem spada u bezbednosno osetljive sisteme zbog toga što obrađuje lične zdravstvene podatke, tako da ima visoke zahteve u vezi poverljivosti, integriteta i dostupnosti informacija.

### Učesnici
U okviru domena postoje sledeći ključni učesnici:

- Pacijent

  Osoba koja koristi uređaje za praćenje zdravstvenih parametara. Korisnik ima uvid u sopstvene podatke putem mobilne aplikacije i definiše pragove za kritične vrednosti, kao i kontakt osobu za SOS uzbune.

- Kontakt osoba za hitne slučajeve

  Osoba (član porodice, staratelj ili druga bliska osoba) koja prima SOS notifikacije u slučaju detekcije kritičnih zdravstvenih parametara.

- Zdravstveni (IoT) uređaji 

  Nosivi medicinski uređaji koji očitavaju zdravstvene podatke poput pulsa, krvnog pritiska, nivoa kiseonika i telesne temperature.

- Eksterni servisi za notifikacije

  Third party servisi koji omogućavaju slanje push notifikacija, SMS poruka ili drugih vidova uzbuna kontakt osobama.

### Poslovni procesi koje softver podržava

- Prikupljanje zdravstvenih podataka

  Kontinuirano preuzimanje podataka sa IoT uređaja putem komunikacionih protokola prilagođenih za rad u realnom vremenu.

- Skladištenje i analiza podataka

  Čuvanje podataka u time series bazi i njihovo kasnije analiziranje, pregled trendova i istorije merenja.

- Praćenje i vizualizacija zdravstvenog stanja

  Omogućavanje korisniku da putem mobilne aplikacije prati svoje zdravstvene parametre kroz grafike i tabele.

- Detekcija kritičnih stanja

  Automatska analiza očitanih podataka i poređenje sa unapred definisanim pragovima ili pravilima za identifikaciju potencijalno opasnih zdravstvenih stanja.

- Slanje SOS uzbuna

  U slučaju detekcije kritičnih vrednosti, sistem generiše i prosleđuje uzbunu kontakt osobi uz pomoć eksternog servisa.

- Upravljanje korisničkim podešavanjima

  Definisanje kontakt osoba i pragova za uzbune.

## Arhitektura sistema
Zamišljeni softver je projektovan kao event-driven distribuirani sistem. Njegove komponente prikupljaju podatke, obrađuju događaje, skladište informacije i komuniciraju sa krajnjim korisnicima.

Osnovne arhitekturalne karakteristike sistema su:

- Event-driven arhitektura

  Zdravstveni podaci su event-ovi koji nastaju merenjem na IoT uređajima. Ovo omogućava skalabilnost, slabu povezanost komponenti i efikasnu obradu podataka u realnom vremenu.

- Asinhrona komunikacija

  Većina komunikacije između komponentid se odvija asinhrono, putem message broker-a. Na taj način se smanjuje zavisnost između servisa i povećava otpornost sistema na greške.

- Mikroservisna arhitektura

  Sistem je podeljen na više logičkih servisa, gde svaki servis ima jasno definisanu odgovornost i može se nezavisno razvijati i skalirati.

- Podrška za rad u realnom vremenu

  Arhitektura sistema je takva da je kašnjenje u obradi podataka minimalno, naročito u kontekstu detekcije kritičnih zdravstvenih stanja.

- Bezbednost i privatnost po dizajnu (security & privacy by design)

  Arhitektura je osmišljena uzimajući u obzir činjenicu da se obrađuju zdravstveni podaci. Ona omogućava kontrolu pristupa, segmentaciju sistema i minimizaciju izloženosti osetljivih informacija.

### Tehnologije i njihova uloga u sistemu

1. Rust backend servisi

Backend sistema je implementiran u programskom jeziku Rust. Rust smo izabrale zbog visokih performansi i niske latencije, bezbednosnih garancija na nivou memorije (memory safety) i dobre podrške za konkurentno i asinhrono programiranje. Backend je podeljen na četiri mikroservisa.

2. AMQP (RabbitMQ)

AMQP protokol, uz RabbitMQ kao implementaciju, koristi se za komunikaciju između IoT uređaja i sistema. Ovaj protokol je posebno pogodan za IoT okruženja zbog male potrošnje mrežnih i računarskih resursa, podrške za publish/subscribe model i tolerancije na nestabilne mrežne veze. 

IoT uređaji objavljuju zdravstvene podatke na odgovarajuće AMQP redove, dok backend servis čita iz tih redova i preuzima podatke za dalju obradu.

3. MQTT (Mosquitto)

Mosquitto se, kao broker za MQTT protokol, koristi za real time stream podataka. Zbog toga što funkcioniše na publish/subscribe modelu je pogodan za live monitoring i grafički prikaz podataka korisniku.

4. InfluxDB

InfluxDB se koristi kao baza podataka za skladištenje zdravstvenih podataka zavisnih u vremenu. Razlozi za izbor ove baze su to što je optimizovana za rad sa vremenskim serijama i velikim brojem merenja, kao i podrške za agragaciju i analizu trendova kroz vreme.

InfluxDB skladišti istorijske zdravstvene podatke korisnika, koji se kasnije prikazuju u mobilnoj aplikaciji.

5. Servis za notifikacije

Servis za notifikacije je zadužen za slanje SOS uzbuna kontakt osobama. On se integriše sa Simple Notification Service-om na AWS-u i aktivira se isključivo kada backend detektuje kritične zdravstvene vrednosti.

6. Mobilna aplikacija

Mobilna aplikacija komunicira sa backend servisima putem bezbednih API poziva. Ona predstavlja glavni interfejs za krajnje korisnike. Funkcionalnosti uključuju prikaz trenutnih i istorijskih zdravstvenih podataka, upravljanje pragovima za kritične vrednosti i definisanje kontakt osobe za SOS uzbune. 

## Slučajevi korišćenja
1. Praćenje i pregled zdravstvenih podataka

  
    Ovde spadaju funkcionalnosti koje omogućavaju krajnjem korisniku praćenje i vizualizaciju zdravstvenih parametara. To su:
    - prikupljanje podataka sa nosivih uređaja,
    - prikaz trenutnih vrednosti,
    - pregled istorijskih merenja i trendova kroz vreme,
    - filtriranje i agregacija podataka po vremenskim periodima ili tipovima merenja.

2. Upravljanje pragovima i SOS kontaktima

    Ova grupa funkcionalnosti omogućava korisniku da definiše pravila za detekciju kritičnih stanja i dodaje osobe koje će biti obaveštene. Tu spadaju:
    - postavljanje individualnih pragova za kritične zdravstvene vrednosti,
    - registracija i upravljanje kontakt osobama za SOS uzbune,
    - aktivacija i deaktivacija SOS servisa.

3. Detekcija kritičnih stanja i podizanje uzbuna

    Obuhvata backend procese i logiku koji omogućavaju automatsko prepoznavanje potencijalno opasnih zdravstvenih situacija i obaveštavanje kontakt osoba:
    - kontinuirana analiza zdravstvenih podataka u realnom vremenu,
    - podizanje uzbuna kada merenja prelaze definisane pragove,
    - slanje SOS notifikacije kontakt osobi,
    - logovanje kritičnih događaja radi revizije i kasnije analize.

4. Upravljanje korisničkim nalogom i konfiguracijom

    Ova grupa služi za administraciju korisničkih naloga i osnovnih podešavanja:
    - registracija i autentifikacija korisnika,  
    - promena ličnih podataka i lozinki,
    - povezivanje nosivog uređaja.

5. Notifikacije i komunikacija sa korisnikom

    Ova grupa obuhvata interakciju sa korisnikom u obliku obaveštenja i vizualnih indikacija:
    - slanje push notifikacija o kritičnim stanjima,
    - prikaz statusa uređaja i konekcije sa sistemom u mobilnoj aplikaciji.

## Osetljivi resursi

| # | Resurs                                                          | Opis                                                                                                                      | Bezbednosni cilj                                                                                                  | Regulativa / standard             |
| - | --------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------- | --------------------------------- |
| 1 | **Zdravstveni podaci korisnika**                                | Vremenske serije zdravstvenih parametara prikupljenih sa IoT uređaja (puls, krvni pritisak, nivo kiseonika, temperatura). | **Poverljivost, integritet** - zaštita od neovlašćenog pristupa i neautorizovanih izmena.                         | GDPR                              |
| 2 | **Korisnički lični podaci**                                     | Informacije kao što su ime, kontakt telefon, email i drugi lični podaci.                              | **Poverljivost, integritet** - sprečavanje curenja ličnih podataka i neovlašćenih promena.                        | GDPR                              |
| 3 | **SOS konfiguracija**             | Podaci o tome kome se šalju SOS notifikacije.                                                    | **Integritet, dostupnost** - sprečavanje neovlašćenih promena koje bi mogle izazvati lažne ili propuštene uzbune. | -                                 |
| 4 | **Istorija kritičnih događaja i logovi sistema**                | Zapisi o detekcijama kritičnih stanja i prosleđenim notifikacijama.                                                       | **Integritet, dostupnost** - očuvanje tačnosti evidencije za reviziju i analizu.                                     | -                                 |
| 5 | **Tokeni i pristupni ključevi za mobilnu aplikaciju i servise** | Tokeni za autentifikaciju i autorizaciju korisnika i servisa.                                                             | **Poverljivost, integritet** - sprečavanje neovlašćenog pristupa sistemu i podacima.                              | -                                 |
| 6 | **Podaci sa IoT uređaja pre obrade**                            | Sirovi senzorski podaci pre validacije i skladištenja u InfluxDB.                                                         | **Integritet, poverljivost** - sprečavanje manipulacije podacima koji mogu uticati na detekciju kritičnih stanja. | -                                 |

