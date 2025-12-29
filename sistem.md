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

## Slučajevi korišćenja

## Osetljivi resursi
