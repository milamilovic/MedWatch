# InfluxDB

InfluxDB je baza podataka napravljena za prikupljanje, obradu i skladištenje događaja i vremenskih serija. Podaci vremenskih serija (Time Series Data) predstavljaju merenja ili događaje koji se prate, nadgledaju i evidentiraju tokom vremena.

Baze podataka za rad sa podacima vremenskih serija (Time Series Databases) su posebno pogodne za slučajeve korišćenja koji zahtevaju čuvanje podataka u realnom ili skoro realnom vremenu. Zbog toga se često koriste za nadgledanje i skladištenje podataka dobijenih pomoću senzora, praćenje stanja servera i performansi aplikacija, praćenje analitike ponašanja,...

InfluxDB je optimizovan i za scenarije u kojima je bitno da se upiti izvršavaju brzo kako bi se podržala korisnička iskustva poput kontrolnih tabli i interaktivnog korisničkog interfejsa. Vremena odziva za upite variraju u rasponu od 10 do 30ms.

Baze podataka za vremenske serije se jasno razlikuju od ostalih tipova baza podataka po tome što poseduju specifičnu arhitekturu prilagođenu radu sa velikim količinama vremenski zavisnih podataka.

InfluxDB koristi poseban model podataka prilagođen vremenskim serijama, koji omogućava veoma brzo pretraživanje zahvaljujući indeksiranju oznaka (tagova) i efikasnoj organizaciji podataka na disku. Za razliku od drugih rešenja (Graphite, RRD, OpenTSDB), InfluxDB podržava više tipova podataka, veliki broj tagova i polja i visoku vremensku preciznost. Ovo ga čini pogodnim za zahtevne primene poput IoT sistema, finansija i tako dalje.

IoT (Internet of Things) uređaji generišu ogromne količine podataka vremenskih serija sa senzora koji mere sve, od temperature i vlažnosti preko vibracija i pritiska do biometrijskih podataka čoveka. Kada se radi o internet stvarima, ovakve baze podataka se lako nose sa izazovima koji se javljaju. To mogu biti česta ažuriranja, dolazak podataka van redosleda i slično. 

## Reference
- https://docs.influxdata.com/influxdb3/core/
- https://www.influxdata.com/time-series-database
