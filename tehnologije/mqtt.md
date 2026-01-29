# MQTT

MQTT (Message Queuing Telemetry Transport) je mrežni protokol koji je baziran na publish-subscribe modelu i koristi se za razmenu poruka, prvenstveno sa udaljenim uređajima koji imaju ograničene resursime kao što su na primer IoT uređaji. MQTT je binarni protokol i oslanja se na TCP/IP da bi obezbedio pouzdan transport poruka. Moguća su tri nivoa kvaliteta usluge (QoS):
- *at-most-once* gde se poruka šalje samo jednom i ne postoji potvrda o prijemu
- *at-least-once* gde se poruka isporučuje najmanje jednom ali je moguće da dođe do duplikata
- *exactly-once* gde se poruka isporučuje tačno jednom koristeći four way handshake
MQTT sam po sebi šalje kredencijale u plain text-u, ali može da se koristi TLS/SSL za enkripciju. Dokazano je da je bezbednost ovog protokola narušena 2020. godine kada su naučnici izveli Slow Denial of Service nad njim.

<br/>
<img src="https://github.com/user-attachments/assets/1f8d9323-8a1c-468b-a5da-9f1f3ac53822" />
<br/><br/>

Arhitektura MQTT-a se zasniva na tome da centralni server (broker) posreduje u komunikaciji između više klijenata. Klijenti mogu biti publishers odnosno mogu da objavljuju poruke, subscribers tj da su pretplaćeni na poruke ili oboje. Komunikacija se odvija preko topic-a. MQTT broker prima sve poruke od publisher-a i prosleđuje ih njihovim subscriber-ima. Broker ne čuva poruke trajno već funkcioniše samo kao posrednik u komunikaciji. Publisher ne mora da zna ko su subscriber-i, već kada šalje poruku navede samo topic, playload, QoS nivo i retain flag (koji navodi da li broker treba da sačuva poslednju poruku za topic). Publisher može da objavi poruke na više različitih topic-a ili može odmah da zatvori konekciju nakon slanja. Subscriber se pretplaćuje na topic-e sa kojih želi da dobija poruke i prima ih od brokera. On može biti pretplaćen na više topic-a i može dodavati ili uklanjati pretplate tokom aktivne sesije. Topic je UTF-8 string koji broker koristi za rutiranje poruka. Organizovan je hijerarhijski sa znakom ,,/" za separator nivoa. 

## Mosquitto

Mosquitto je open source MQTT broker. Razvila ga je Eclipse fondacija i on je napisan u C programskom jeziku. Mosquitto je jedan od najpopularnijih brokera zbog svoje jednostavnosti, performansi i lakog konfiguracisanja.

## Arhitektura

### Broker
Mosquitto broker je centralna komponenta koja prima klijentske konekcije preko TCP soketa (uglavnom na portu 1883 ili 8883 u zavisnosti od toga da li nema ili ima enkripciju). Broker mapira topic-e na listu pretplaćenih klijenata i kada primi poruku od publisher-a prosleđuje poruku svim klijentima koji su pretplaćeni na taj topic. Mosquitto upravlja sesijom za svaku konekciju sa klijentom, retained porukama na svakom topic-u, in-flight porukama za QoS 1 i QoS 2, kao i klijentskim subskripcijama i njihovim QoS nivoima. Broker podržava više načina za autentifikaciju odnosno username i password kroz password fajl, TLS klijentske sertifikate (X.509), ali i plugin sistem za custom autentifikaciju. Autorizacija se konfiguriše kroz ACL (Access Control List) fajlove koji definišu koja konekcija može da pristupa kojim topic-ima sa kojim permisijama. Mosquitto može da čuva podatke na disku da se ne bi izgubile u slučaju restarta, i mogu da se čivaju retained poruke, stanje trajnih poruka i in-flight poruke koje nisu još isporučene klijentima.

### Subscriber-i
*mosquitto_sub* je klijent za pretplaćivanje na topic-e i primanje poruka kroz komandnu liniju i on može da se pretplati na više topic-a odjednom. Alat podržava wildcard karaktere u imenima topic-a gde znak ,,+" zamenjuje jedan nivo u hijerarhiji a ,,#" zamenjuje sve nivoe od te tačke na dalje. *mosquitto_sub* može da prikaže poruke u različitim formatima na primer sa -v opcijom se uključuje verbose mode koji prikazuje topic uz poruku. Opcija -F omogućava custom format string-a koji podržava između ostalog ispisivanje topic-a, payload-a, QoS-a, retain flag-a i timestamp-a. Alat podržava filtriranje poruka po topic-u i mogu se koristiti RegEx-i za pattern matching sa -T opcijom. *mosquitto_sub* može da skladišti poruke koje pristignu u fajlovima pomoću --output opcije ili da ih prosleđuje na stdout kako bi one otišle na dalju obradu sa drugim alatima.

*mosquitto_rr* je klijent koji iz komandne linije implementira request-response obrazac, čime se omogućava sinhrona komunikacija preko MQTT protokola. Ovaj alat radi tako što objavljuje poruku na određeni topic i čeka odgovor na response topic-u. On je posebno koristan za implementaciju RPC obrasca (Remote Procedure Call) preko MQTT-a gde klijent šalje zahtev i očekuje odgovor sa servera. *mosquitto_rr* je koristan za testiranje MQTT servisa koji implementiraju request-response obrazac bez pisanja klijentskog koda.

### Publisher
*mosquitto_pub* je klijenti za objavljivanje poruka na MQTT broker koji radi iz komandne linije. On omogućava slanje poruka ili čitanje podataka sa standardnog ulaza i podržava sva tri QoS nivoa i može da postavlja retain flag. *mosquitto_pub* može i da učitava poruke iz fajlova koristeći -f opciju ili da pročita jednu liniju sa stdin-a pomoću -l opcije, što omogućava pipe-ovanje podataka iz drugih programa. Alat podržava enkripciju pomoću TLS/SSL-a kroz sertifikate i privatne ključeve, kao i pre-shared key autentifikaciju. Korisnik može da konfiguriše ,,Last Will Testament" poruku koja će biti poslata ako se neočekivano izgubi konekcija sa brokerom. *mosquitto_pub* omogućava slanje null poruka pomoću -n opcije što se koristi za brisanje retained poruka sa topic-a.

## Reference
https://en.wikipedia.org/wiki/MQTT

https://en.wikipedia.org/wiki/Slow_DoS_attack

https://mosquitto.org/documentation/

https://mosquitto.org/man/mosquitto_rr-1.html

https://mosquitto.org/man/mosquitto_sub-1.html

https://mosquitto.org/man/mosquitto_pub-1.html

