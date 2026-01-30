# InfluxDB

InfluxDB je baza podataka napravljena za prikupljanje, obradu i skladištenje događaja i vremenskih serija. Podaci vremenskih serija (Time Series Data) predstavljaju merenja ili događaje koji se prate, nadgledaju i evidentiraju tokom vremena.

Baze podataka za rad sa podacima vremenskih serija (Time Series Databases) su posebno pogodne za slučajeve korišćenja koji zahtevaju čuvanje podataka u realnom ili skoro realnom vremenu. Zbog toga se često koriste za nadgledanje i skladištenje podataka dobijenih pomoću senzora, praćenje stanja servera i performansi aplikacija, praćenje analitike ponašanja,...

InfluxDB je optimizovan i za scenarije u kojima je bitno da se upiti izvršavaju brzo kako bi se podržala korisnička iskustva poput kontrolnih tabli i interaktivnog korisničkog interfejsa. Vremena odziva za upite variraju u rasponu od 10 do 30ms.

Baze podataka za vremenske serije se jasno razlikuju od ostalih tipova baza podataka po tome što poseduju specifičnu arhitekturu prilagođenu radu sa velikim količinama vremenski zavisnih podataka.

InfluxDB koristi poseban model podataka prilagođen vremenskim serijama, koji omogućava veoma brzo pretraživanje zahvaljujući indeksiranju oznaka (tagova) i efikasnoj organizaciji podataka na disku. Za razliku od drugih rešenja (Graphite, RRD, OpenTSDB), InfluxDB podržava više tipova podataka, veliki broj tagova i polja i visoku vremensku preciznost. Ovo ga čini pogodnim za zahtevne primene poput IoT sistema, finansija i tako dalje.

IoT (Internet of Things) uređaji generišu ogromne količine podataka vremenskih serija sa senzora koji mere sve, od temperature i vlažnosti preko vibracija i pritiska do biometrijskih podataka čoveka. Kada se radi o internet stvarima, ovakve baze podataka se lako nose sa izazovima koji se javljaju. To mogu biti česta ažuriranja, dolazak podataka van redosleda i slično. 

##

## Arhitektura
Arhitektura InfluxDB-a može se podeliti na četiri ključne komponente, to jest Ingester, Querier, Compactor i Garbage Collection; i dva glavna sloja skladištenja Catalog i Object Storage.

<img width="792" height="585" alt="Snimak ekrana (594)" src="https://github.com/user-attachments/assets/3b87ac71-18ca-48f7-b36c-6dcb8c3b5346" />

### Data Ingestion (Ingester)
Ova komponenta je zadužena da prima podatke koje klijenti ili IoT uređaji šalju ka sistemu, da ih validira i da izvrši inicijalnu obradu pre trajnog skladištenja. Podaci se prvo prihvataju kroz Ingest Router, nakon čega se vrši validacija šeme i automatsko otkrivanje novih tabela. Nakon toga, podaci se logički razdeljuju prema vremenskim intervalima (na primer po danima). U ovom delu procesa se vrši i uklanjanje duplih zapisa da ne bi došlo do višestrukog skladištenja istih podataka. Obrađeni podaci se zatim upisuju u Parquet fajlove koji se čuvaju u Object Storage-u, a informacije o novonastalim fajlovima i ostali metapodaci čuvaju u katalogu metapodataka, tj. u Catalog-u. Ingest komponenta radi konstantno i napravljena je tako da obezbedi približno real-time dostupnost najnovijih podataka za upite.

### Data Querying (Querier)
To je komponenta koja služi za obradu korisničkih upita i pomoću nje se pristupa podacima unutar InfluxDB-a. Upiti su najčešće kreirani pomoću InfluxQL, a mogu se kreirati i pomoću klasičnog SQL-a. Upiti prvo nailaze na Query Router i on ih prosleđuje Querier-u na obradu. Querier pre izvršavanja prvo učita metapodatke u keš memoriju, a zatim u toku izvršavanja učitava potrebne podatke iz Object Storage-a. Za učitavanje najnovijih podataka komunicira direktno sa Ingester-om. Querier sadrži optimizator upita koji služi za kreiranje optimalong plana upita koji će se izvršiti nad učitanim podacima. Querier koristi DataFusion i Apache Arrow za izgradnju i izvršavanje prilagođenih planova upita. Querier izbacuje nepotrebne podatke pre samog izvršavanja upita tako što koristi prednosti toga što je Ingester prethodno izvršio particionisanje, tj. razdeljivamje podataka. Iako podaci u okviru svakog pojedinačnog fajla ne sadrže duplikate, podaci u različitim fajlovima, kao i podaci koji još uvek nisu trajno sačuvani nego ih Ingester prosleđuje Querier-u, mogu da sadrže duplikate. Zbog toga se proces uklanjanja duplikata radi i u ovoj komponenti. 

### Data Compaction (Compactor)
Compactor je komponenta koja vrši optimizaciju skladištenja podataka tako što spaja veliki broj malih Parquet fajlova u veće celine. Compactor se pokreće periodično u pozadini i analizira nove Parquet fajlove koji su nastali od strane Ingester-a. Prilikom ovog procesa se spajaju mali fajlovi koji se međusobno preklapaju i na taj način nastaju veći fajlovi bez preklapanja podataka što smanjuje redundanciju i povećava efikasnost. Poboljšavaju se performanse izvršavanja upita, tj. povećava se efikasnost izvršavanja upita jer ima manje fajlova koje treba pročitati. Compactor ne briše stare fajlove odmah, već ih označava kao logički obrisane (soft deleted) u katalogu metapodataka. Fizičko brisanje iz Object Storage-a se vrši u Garbage Collector-u. 

### Garbage Collector (GC)
Garbage Collector služi za održavanje efikasnosti i čišćenje Object Storage-a. Korisnici imaju mogućnost da definišu svoja pravila zadržavanja podataka (data retention policy) i ona se čuvaju u Catalog-u. Garbage Collector uklanjan stare ili više nepotrebne fajlove u skladu sa  tim definisanim pravilima. On periodično čita informacije iz Catalog-a kako bi detektovao fajlove kojima je istekao period zadržavanja ili su prethodno označeni kao logički obrisani (npr od strane Compactor-a). Fajlovi se prvo logički obrišu, tj. označe se da su soft delete dok se ne proveri da se ne koriste više ni u jednoj komponenti sistema, a zatim se i fizički obrišu (hard delete) iz Object Storage-a. Osim toga, obrišu se i njihovi metapodaci u Catalog-u. Na ovaj način se efikasno upravlja zauzećem prostora skladištenja i održavaju se dobre performanse celog sistema.

### Catalog (metadata)
Catalog je centralizovana baza metapodataka. U njemu se čuvaju informacije o strukturi podataka, tačnije definicije tabela i kolona, kao i metapodaci o Parquet fajlovima poput lokacije u Object Storage-u i statusa. InfluxDB koristi baze podataka kompatibilne sa PostgreSQL-om za uptavljanje Catalog-om. 

### Object Storage (Parquet files)
To je glavno i trajno skladište podataka u InfluxDB-u i u njemu se vremenske serije čuvaju u formatu Parquet fajlova. Parquet je kolumnarni format skladištenja koji omogućava visoku kompresiju i efikasno čitanje podataka. Parquet fajlovi mogu da budu skladišteni u Object Storage bazama koje se mogu nalaziti lokalno na disku, ali i u nekom od cloud servisa poput S3 bucket-a na AWS-u ili Azure Blob Storage-a ili Google Cloud Storage-a.

### Apache Arrow
Apache Arrow je dodatna komponenta i ona se koristi kao osnovni in-memory kolumnarni format za obradu podataka. Prilikom upotrebe Apache Arrow-a se izbegava serijalizacija i deserializacija između komponenti sistema i zbog toga je obrada podataka jako brza. To je dobro jer se uz pomoć DataFusion query engine-a može omogućiti paralelna obrada i optimizacija upita nad velikim skupovima podataka (u Querier-u).

<img width="634" height="800" alt="image" src="https://github.com/user-attachments/assets/c5117c78-09d1-47bf-ab9a-cbe5dc3542f3" />




## Reference
- https://docs.influxdata.com/influxdb3/core/
- https://www.influxdata.com/time-series-database
- https://www.influxdata.com/blog/influxdb-3-0-system-architecture/
- https://www.influxdata.com/blog/understanding-influxdb-3.0-part-two/
